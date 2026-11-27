//! Retry a drug get or DDInter partner lookup using source-stated names.
use super::{
    alias_lookup::{Stated, usable, with_aliases},
    label_fallback::LabelFallback,
    name_evidence::{Evidence, recovery},
};
use crate::drug::resolve::{lookup_deadline, within};
use crate::drug::{fork, resolve};
use crate::{Ctx, Outcome, status};
use biodata::Text;
use biomcp_types::{
    DrugCard, DrugGet, DrugSearch, DrugSource, ReplyBody, ReplyData, Request, SourceNotice, Status,
    StepKind,
};

pub(in crate::drug) fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let mut out = get_parts(cx, std::slice::from_ref(query));
    out.pop().unwrap_or_else(|| {
        status::empty(ReplyData::Usage(biomcp_types::UsageError::argument(
            "input".into(),
        )))
    })
}

/// Get every part of one drug name, resolving the name once; a name that resolves to no
/// record answers with one outcome for all parts.
pub(in crate::drug) fn get_parts(cx: &Ctx<'_>, queries: &[DrugGet]) -> Vec<Outcome> {
    let deadline = lookup_deadline(cx);
    let mut outcomes = resolve::get_parts(cx, queries);
    let Some(query) = queries.first() else {
        return outcomes;
    };
    let [out] = outcomes.as_slice() else {
        return outcomes;
    };
    if query.uses_default_identity_source()
        && out.status() == Status::Failed
        && let Some(recovered) = chembl_identity(&within(cx, deadline.remaining()), query, out)
    {
        return vec![recovered];
    }
    if !matches!(out.body, ReplyData::Usage(_)) || !usable(out, query.source()) {
        return outcomes;
    }
    let input = query.input();
    let mut label = LabelFallback::default();
    let (found, stated, evidence) = with_aliases(
        &within(cx, deadline.remaining()),
        input,
        query.source(),
        |stated, name, evidence| {
            let cx = &within(cx, deadline.remaining());
            if query.source() == DrugSource::OpenfdaLabel {
                return label.read(cx, queries, stated, name, evidence);
            }
            retry_name(cx, queries, stated, name, evidence)
        },
    );
    let lookup_failed = evidence.lookup_failed();
    if let Some(mut found) = found {
        for (part, requested) in found.iter_mut().zip(queries) {
            evidence.append(part, query.source());
            if lookup_failed {
                match part.status() {
                    Status::Complete => status::item(part, true, false, true),
                    Status::Absent => status::item(part, false, true, false),
                    _ => (),
                }
                recovery(part, Request::GetDrug(requested.clone()));
            }
        }
        return found;
    }
    let Some(mut out) = outcomes.pop() else {
        return outcomes;
    };
    evidence.append(&mut out, query.source());
    if stated.is_none() && !lookup_failed {
        return vec![out];
    }
    // A Usage reply discards source metadata, including the other-source hint and a source the
    // lookup could not use (1087).
    queries
        .iter()
        .map(|query| {
            let mut part = fork(&out, DrugCard::into_reply(query.clone(), None));
            if lookup_failed {
                status::item(&mut part, false, true, false);
                recovery(&mut part, Request::GetDrug(query.clone()));
            } else if let Some(stated) = &stated {
                hint(&mut part, stated, input, query.source(), cx.fresh);
                label.offer_candidate(&mut part, input, cx.fresh);
                status::item(&mut part, false, false, false);
            }
            part
        })
        .collect()
}

fn chembl_identity(cx: &Ctx<'_>, query: &DrugGet, primary: &Outcome) -> Option<Outcome> {
    let ReplyData::DrugCard {
        query: attempted, ..
    } = &primary.body
    else {
        return None;
    };
    let mychem = primary
        .sources
        .iter()
        .find(|row| row.source == DrugSource::Mychem.source())?;
    let chembl = DrugGet::named(query.input().clone(), DrugSource::Chembl).ok()?;
    let mut found = get(cx, &chembl);
    if !matches!(found.body, ReplyData::DrugCard { body: Some(_), .. }) {
        return None;
    }
    // The carried components share MyChem's failure. Only MyChem itself was asked for that record.
    found.sources.push(mychem.clone());
    let failure = if attempted.input() == query.input() {
        "MyChem.info could not resolve the requested name.".into()
    } else {
        format!("MyChem.info could not serve record {}.", attempted.input())
    };
    notice(
        &mut found,
        DrugSource::Mychem,
        format!("{failure} Resolved the requested name through ChEMBL."),
    );
    status::item(&mut found, true, false, true);
    Some(found)
}

pub(super) fn retry_name(
    cx: &Ctx<'_>,
    queries: &[DrugGet],
    stated: &Stated,
    name: &Text,
    evidence: &mut Evidence,
) -> Option<Vec<Outcome>> {
    let query = queries.first()?;
    let input = query.input();
    let retries = queries
        .iter()
        .map(|query| {
            crate::next::checked(
                query.with_name(name.clone()),
                stated.source.source(),
                &mut evidence.problems,
            )
        })
        .collect::<Option<Vec<_>>>()?;
    let mut found = resolve::get_parts(cx, &retries);
    if !answered_alias(&found, query.source(), evidence) {
        return None;
    }
    for found in &mut found {
        notice(
            found,
            query.source(),
            format!(
                "The source did not resolve {:?}. {} record {} supplies the name {:?}; \
                 tried resolving that name in the selected source. \
                 Only that source can supply its own record.",
                input.as_str(),
                stated.source,
                stated.id,
                name.as_str()
            ),
        );
    }
    Some(found)
}

pub(super) fn answered_alias(
    found: &[Outcome],
    source: DrugSource,
    evidence: &mut Evidence,
) -> bool {
    let answered = found
        .first()
        .is_some_and(|part| !matches!(part.body, ReplyData::Usage(_)))
        && found
            .iter()
            .any(|part| !matches!(part.status(), Status::Absent | Status::Failed));
    if !answered {
        for part in found {
            evidence.observe(part, source, false);
        }
    }
    answered
}

/// Retry a DDInter partner DDInter does not name under each exact name another source states
/// for it; the first name that DDInter names is the partner.
pub(in crate::drug) fn partner<T>(
    cx: &Ctx<'_>,
    input: &Text,
    mut attempt: impl FnMut(&Text) -> Option<T>,
) -> (Option<T>, Aliased) {
    let (found, _, evidence) = with_aliases(cx, input, DrugSource::Ddinter, |stated, name, _| {
        attempt(name).map(|found| {
            let message = format!(
                "DDInter's files name no drug {:?}. {} record {} supplies the name {:?}; \
                 the partner is DDInter's drug by that name.",
                input.as_str(),
                stated.source,
                stated.id,
                name.as_str()
            );
            (found, message)
        })
    });
    let (found, message) = match found {
        Some((found, message)) => (Some(found), Some(message)),
        None => (None, None),
    };
    (found, Aliased { message, evidence })
}

/// The other-source lookups a partner retry made, and the name it used, if any.
pub(in crate::drug) struct Aliased {
    message: Option<String>,
    evidence: Evidence,
}
impl Aliased {
    /// Add the lookups' source outcomes and say which name stood in for the partner.
    pub(in crate::drug) fn append(self, out: &mut Outcome) {
        self.evidence.append(out, DrugSource::Ddinter);
        if let Some(message) = self.message {
            notice(out, DrugSource::Ddinter, message);
        }
    }
}

pub(super) fn notice(out: &mut Outcome, selected: DrugSource, message: String) {
    if let Some(source) = out
        .sources
        .iter_mut()
        .find(|s| s.source == selected.source())
    {
        source.notices.push(SourceNotice::RequestGuidance {
            data_date: None,
            message: message.into(),
            retry_after_seconds: None,
        });
    }
}

pub(super) fn hint(
    out: &mut Outcome,
    stated: &Stated,
    input: &Text,
    selected: DrugSource,
    fresh: bool,
) {
    if stated.ambiguous || !stated.literal(input) {
        notice(
            out,
            selected,
            format!(
                "{} has candidate records for {:?}, but no unique literal name match. \
                 No cross-source alias was selected. Search that source to inspect its candidates.",
                stated.source,
                input.as_str()
            ),
        );
        if let Some(query) = crate::next::checked(
            DrugSearch::named(input.clone(), stated.source, 10),
            stated.source.source(),
            &mut out.problems,
        ) {
            out.next.push(crate::next::Step::run(
                StepKind::Disambiguate,
                &[stated.source.source()],
                Request::SearchDrug(query),
                fresh,
            ));
        }
        return;
    }
    notice(
        out,
        selected,
        format!(
            "The completed queries in {selected} found no matching record for {:?}. \
             {} record {} matches the input; its presence does not establish a record in {selected}.",
            input.as_str(),
            stated.source,
            stated.id
        ),
    );
    if let Some(query) = crate::next::checked(
        DrugGet::new(stated.id.clone(), stated.source),
        stated.source.source(),
        &mut out.problems,
    ) {
        out.next.push(crate::next::Step::run(
            StepKind::Disambiguate,
            &[stated.source.source()],
            Request::GetDrug(query),
            fresh,
        ));
    }
    status::search_for(out, &[selected.source()]);
}
