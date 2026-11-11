//! Resolve once, retain evidence and keep native offset pages on the same spellings.
use crate::{Ctx, Outcome, drug::search_names, source_snapshot, status};
use biodata::Text;
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{
    AdverseEventSearch, Paged, ReplyBody, ReplyData, Request, Source, SourceNotice, StepKind,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Saved {
    names: Vec<Text>,
    sources: Vec<biomcp_types::SourceOutcome>,
    problems: Vec<biomcp_types::Problem>,
    incomplete: bool,
    omitted: usize,
}

pub(super) fn search<'a>(
    cx: &Ctx<'a>,
    query: &AdverseEventSearch,
    deadline: &mut Deadline<'a>,
) -> Outcome {
    let mut out = status::empty(biomcp_types::AdverseEventPage::into_reply(
        query.clone(),
        None,
    ));
    let result = prepare(cx, query, deadline, &mut out);
    match result {
        Ok(Some((mut saved, sent))) => match query.with_literal_drugs(saved.names.clone()) {
            Ok(native) => {
                let mut result = super::search(cx, &native, &sent, *deadline);
                typed_only(cx, query, (&sent, *deadline), &mut saved, &mut result);
                result.sources.extend(saved.sources);
                result.problems.extend(saved.problems);
                disclose(query, &saved.names, saved.omitted, &mut result);
                status::search_for(&mut result, &[Source::Faers]);
                if result.status() == biomcp_types::Status::Complete {
                    status::item(&mut result, true, false, saved.incomplete);
                }
                result.publish_error = out.publish_error;
                out = result;
            }
            Err(_) => out.local = Some(biomcp_fetch::CacheError::Snapshot.into()),
        },
        Ok(None) => {
            out.body = ReplyData::PoolError { mismatch: false };
            restart(query, &mut out);
        }
        Err(error) => out.local = Some(error),
    }
    out
}

/// The caller's own names, which every FAERS search keeps.
fn typed(query: &AdverseEventSearch) -> Vec<Text> {
    query
        .text()
        .into_iter()
        .chain(query.drug())
        .cloned()
        .collect()
}

/// When FAERS refuses the widened request, search the typed names alone and say so.
fn typed_only<'a>(
    cx: &Ctx<'a>,
    query: &AdverseEventSearch,
    (sent, deadline): (&AdverseEventSearch, Deadline<'a>),
    saved: &mut Saved,
    result: &mut Outcome,
) {
    let typed = typed(query);
    if saved.names.len() <= typed.len() || !search_names::refused(result, Source::Faers) {
        return;
    }
    let Ok(native) = query.with_literal_drugs(typed.clone()) else {
        return;
    };
    *result = super::search(cx, &native, sent, deadline);
    saved.names = typed;
    saved.omitted = 0;
    search_names::retract_sources(&mut saved.sources);
    if let Some(source) = result
        .sources
        .iter_mut()
        .find(|s| s.source == Source::Faers)
    {
        for input in query.text().into_iter().chain(query.drug()) {
            source
                .notices
                .push(search_names::typed_only(input, Source::Faers));
        }
    }
}

fn prepare<'a>(
    cx: &Ctx<'a>,
    query: &AdverseEventSearch,
    deadline: &mut Deadline<'a>,
    out: &mut Outcome,
) -> Result<Option<(Saved, AdverseEventSearch)>, RequestError> {
    let key = source_snapshot::key("faers-drug-names", &query.restart())?;
    if let Some(token) = query.name_snapshot() {
        return Ok(source_snapshot::read(cx, &key, token.as_str(), *deadline)?
            .map(|saved| (saved, query.clone())));
    }
    let mut saved = resolve(cx, query, deadline);
    restrict_notices(&mut saved);
    let admitted: Vec<_> = saved
        .sources
        .iter()
        .filter(|source| source.state != biomcp_types::SourceState::Skipped)
        .cloned()
        .collect();
    let token = crate::publish::keep(
        source_snapshot::write(cx, &key, &saved, &admitted, *deadline),
        out,
    )?;
    let sent = token.map_or_else(|| query.clone(), |token| query.with_name_snapshot(token));
    Ok(Some((saved, sent)))
}

fn resolve<'a>(cx: &Ctx<'a>, query: &AdverseEventSearch, deadline: &mut Deadline<'a>) -> Saved {
    let mut out = status::empty(ReplyData::PoolError { mismatch: false });
    let mut names = typed(query);
    let mut incomplete = false;
    let mut omitted = 0;
    search_names::bounded(deadline, |lookup| {
        for input in query.text().into_iter().chain(query.drug()) {
            let found =
                search_names::resolve(cx, input, &mut out, lookup, search_names::openfda, PARTIAL);
            incomplete |= found.failed;
            for name in found.names {
                if names
                    .iter()
                    .any(|prior| prior.as_str().eq_ignore_ascii_case(name.as_str()))
                {
                    continue;
                }
                let mut candidate = names.clone();
                candidate.push(name);
                if query.with_literal_drugs(candidate.clone()).is_ok() {
                    names = candidate;
                } else {
                    omitted += 1;
                }
            }
        }
    });
    incomplete |= [Source::Mychem, Source::Chembl, Source::OpenfdaLabel]
        .iter()
        .all(|source| cx.fetch.disabled(*source));
    Saved {
        names,
        sources: out.sources,
        problems: out.problems,
        incomplete,
        omitted,
    }
}

const PARTIAL: search_names::Unfinished = search_names::Unfinished::Partial;

fn restrict_notices(saved: &mut Saved) {
    for source in &mut saved.sources {
        for notice in &mut source.notices {
            if let SourceNotice::DrugNameSearch { names, message, .. } = notice {
                *names = names
                    .iter()
                    .filter(|name| saved.names.contains(name))
                    .cloned()
                    .collect();
                if saved.omitted != 0 {
                    *message = format!(
                        "{message} Additional names exceeded the whole native query bound."
                    )
                    .into();
                }
            }
        }
    }
}

fn disclose(query: &AdverseEventSearch, names: &[Text], omitted: usize, out: &mut Outcome) {
    if let Some(source) = out
        .sources
        .iter_mut()
        .find(|source| source.source == Source::Faers)
    {
        for input in query.text().into_iter().chain(query.drug()).take(1) {
            source.notices.push(SourceNotice::DrugNameSearch {
                input: input.clone(), names: names.to_vec().into(),
                message: format!(
                    "Searched these literal spellings in one native FAERS OR query; counts are native reports, not sums. \
                     Name discovery is bounded and does not establish drug-wide completeness. \
                     Left out {omitted} additional names at the whole-query bound."
                ).into(),
            });
        }
    }
}

fn restart(query: &AdverseEventSearch, out: &mut Outcome) {
    out.next.push(crate::next::Step::run(
        StepKind::Restart,
        &[Source::Faers],
        Request::SearchAdverseEvent(query.restart()),
        false,
    ));
}
