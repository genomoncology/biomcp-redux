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
    /// Why a typed name stayed as typed, for the one notice on FAERS.
    #[serde(default)]
    unsettled: Vec<String>,
    /// Each stated name left out with its short reason, for the same notice.
    #[serde(default)]
    left_out: Vec<String>,
    /// FAERS refused the query holding the other names, so only the typed names were searched.
    #[serde(default)]
    rejected: bool,
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
                disclose(query, &saved, &mut result);
                result.sources.extend(saved.sources);
                result.problems.extend(saved.problems);
                status::search_for(&mut result, &[Source::Faers]);
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
    let native = match query.with_literal_drugs(typed.clone()) {
        Ok(native) => native,
        // The typed names alone formed the caller's valid query, so this is a local fault.
        Err(_) => {
            result.local = Some(biomcp_fetch::CacheError::Snapshot.into());
            return;
        }
    };
    *result = super::search(cx, &native, sent, deadline);
    saved.names = typed;
    saved.rejected = true;
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
    let saved = resolve(cx, query, deadline);
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
    let mut omitted = 0;
    let mut unsettled = Vec::new();
    let mut left_out = Vec::new();
    search_names::bounded(deadline, |lookup| {
        for input in query.text().into_iter().chain(query.drug()) {
            let found = search_names::resolve(cx, input, &mut out, lookup, search_names::openfda);
            unsettled.extend(found.unsettled.map(str::to_owned));
            left_out.extend(found.left_out);
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
    // The one notice on FAERS says what the lookups found, so theirs are dropped.
    search_names::drop_notices(&mut out.sources);
    if omitted != 0 {
        left_out.push(format!("{omitted} past query size"));
    }
    Saved {
        names,
        sources: out.sources,
        problems: out.problems,
        unsettled,
        left_out,
        rejected: false,
    }
}

/// One notice on FAERS names every name searched and why any other was not.
fn disclose(query: &AdverseEventSearch, saved: &Saved, out: &mut Outcome) {
    let Some(input) = query.text().into_iter().chain(query.drug()).next() else {
        return;
    };
    let widened = saved.names.len() > typed(query).len();
    let message = search_names::compact(
        "one FAERS query",
        (widened, saved.rejected),
        &saved.unsettled,
        &saved.left_out,
    );
    if let Some(source) = out
        .sources
        .iter_mut()
        .find(|source| source.source == Source::Faers)
    {
        source.notices.push(SourceNotice::DrugNameSearch {
            input: input.clone(),
            names: saved.names.clone().into(),
            message: message.into(),
        });
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
