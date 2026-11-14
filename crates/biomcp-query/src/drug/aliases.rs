//! Source-stated aliases may recover an empty source lookup, never substitute its records.
//! A search that widened its name and still found nothing tries them the same way.
use super::resolve::within;
use crate::{Ctx, Outcome, status};
use biodata::Text;
use biomcp_types::{
    DrugCard, DrugGet, DrugNameSelector, DrugSearch, DrugSource, ReplyBody, ReplyData, Request,
    SourceNotice, SourceOutcome, SourceState, StepKind,
};

pub(crate) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let deadline = cx.fetch.deadline(cx.budget);
    let mut out = search_spelling(cx, query);
    // --exact-term searches the typed text only, so no other source's name stands in for it.
    if query.exact_term() || query.offset() != 0 || !zero(&out, query.source()) {
        return out;
    }
    let Some(input) = query.query() else {
        return out;
    };
    let (found, card, evidence) = with_aliases(
        &within(cx, deadline.remaining()),
        input,
        query.source(),
        |card, name, _| {
            if query.explicit_name_selector() {
                return None;
            }
            let retry = query.with_name(name.clone());
            let mut found = search_spelling(&within(cx, deadline.remaining()), &retry);
            if zero(&found, query.source()) {
                return None;
            }
            notice(
                &mut found,
                query.source(),
                format!(
                    "The search for {:?} returned no rows. {} record {} supplies the name {:?}; \
                     tried the selected source with that name. \
                     Returned rows and totals belong to the selected source.",
                    input.as_str(),
                    card.header.source,
                    card.header.id,
                    name.as_str()
                ),
            );
            Some(found)
        },
    );
    if let Some(mut found) = found {
        evidence.append(&mut found, query.source());
        return found;
    }
    evidence.append(&mut out, query.source());
    if evidence.local.is_some() {
        // A local failure while looking up names leaves the search incomplete, as its exit code says.
        status::search(&mut out);
    }
    if let Some(card) = card {
        hint(&mut out, &card, input, query.source(), cx.fresh);
    }
    out
}

pub(super) fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let mut out = get_parts(cx, std::slice::from_ref(query));
    out.pop().unwrap_or_else(|| {
        status::empty(ReplyData::Usage(biomcp_types::UsageError::argument(
            "input".into(),
        )))
    })
}

/// Get every part of one drug name, resolving the name once; a name that resolves to no
/// record answers with one outcome for all parts.
pub(super) fn get_parts(cx: &Ctx<'_>, queries: &[DrugGet]) -> Vec<Outcome> {
    let deadline = cx.fetch.deadline(cx.budget);
    let mut outcomes = super::resolve::get_parts(cx, queries);
    let Some(query) = queries.first() else {
        return outcomes;
    };
    let [out] = outcomes.as_slice() else {
        return outcomes;
    };
    if !matches!(out.body, ReplyData::Usage(_)) || !usable(out, query.source()) {
        return outcomes;
    }
    let input = query.input();
    let (found, card, evidence) = with_aliases(
        &within(cx, deadline.remaining()),
        input,
        query.source(),
        |card, name, evidence| {
            let retries = queries
                .iter()
                .map(|query| {
                    crate::next::checked(
                        query.with_name(name.clone()),
                        card.header.source.source(),
                        &mut evidence.problems,
                    )
                })
                .collect::<Option<Vec<_>>>()?;
            let mut found = super::resolve::get_parts(&within(cx, deadline.remaining()), &retries);
            if found
                .first()
                .is_none_or(|found| matches!(found.body, ReplyData::Usage(_)))
            {
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
                        card.header.source,
                        card.header.id,
                        name.as_str()
                    ),
                );
            }
            Some(found)
        },
    );
    if let Some(mut found) = found {
        for found in &mut found {
            evidence.append(found, query.source());
        }
        return found;
    }
    let Some(mut out) = outcomes.pop() else {
        return outcomes;
    };
    evidence.append(&mut out, query.source());
    let Some(card) = card else {
        return vec![out];
    };
    // A Usage reply discards source metadata, including the other-source hint.
    queries
        .iter()
        .map(|query| {
            let mut part = super::fork(&out, DrugCard::into_reply(query.clone(), None));
            hint(&mut part, &card, input, query.source(), cx.fresh);
            status::item(&mut part, false, false, false);
            part
        })
        .collect()
}

/// Retry a DDInter partner DDInter does not name under each exact name another source states
/// for it; the first name that DDInter names is the partner.
pub(super) fn partner<T>(
    cx: &Ctx<'_>,
    input: &Text,
    mut attempt: impl FnMut(&Text) -> Option<T>,
) -> (Option<T>, Aliased) {
    let (found, _, evidence) = with_aliases(cx, input, DrugSource::Ddinter, |card, name, _| {
        attempt(name).map(|found| {
            let message = format!(
                "DDInter's files name no drug {:?}. {} record {} supplies the name {:?}; \
                 the partner is DDInter's drug by that name.",
                input.as_str(),
                card.header.source,
                card.header.id,
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
pub(super) struct Aliased {
    message: Option<String>,
    evidence: Evidence,
}
impl Aliased {
    /// Add the lookups' source outcomes and say which name stood in for the partner.
    pub(super) fn append(self, out: &mut Outcome) {
        self.evidence.append(out, DrugSource::Ddinter);
        if let Some(message) = self.message {
            notice(out, DrugSource::Ddinter, message);
        }
    }
}

#[derive(Default)]
struct Evidence {
    sources: Vec<SourceOutcome>,
    problems: Vec<biomcp_types::Problem>,
    local: Option<biomcp_fetch::RequestError>,
}
impl Evidence {
    fn append(&self, out: &mut Outcome, selected: DrugSource) {
        for source in &self.sources {
            // MyChem component metadata cannot replace the selected source's native outcome.
            if source.source == selected.source() {
                continue;
            }
            if let Some(existing) = out.sources.iter_mut().find(|s| s.source == source.source) {
                status::retain_source_state(existing, source);
            } else {
                out.sources.push(source.clone());
            }
        }
        for problem in &self.problems {
            if !out.problems.contains(problem) {
                out.problems.push(problem.clone())
            }
        }
        out.local = out.local.or(self.local);
    }
}

/// Try each exact name another source states for `input`; the caller adds the evidence to
/// what `attempt` found.
fn with_aliases<T>(
    cx: &Ctx<'_>,
    input: &Text,
    selected: DrugSource,
    mut attempt: impl FnMut(&DrugCard, &Text, &mut Evidence) -> Option<T>,
) -> (Option<T>, Option<DrugCard>, Evidence) {
    let deadline = cx.fetch.deadline(cx.budget);
    let mut evidence = Evidence::default();
    let mut alternatives = None;
    let mut tried = std::collections::BTreeSet::from([input.as_str().to_lowercase()]);
    for source in [
        DrugSource::Mychem,
        DrugSource::OpenfdaLabel,
        DrugSource::Chembl,
    ] {
        if source == selected {
            continue;
        }
        if deadline.remaining().is_zero() {
            break;
        }
        let Some(query) = crate::next::checked(
            DrugGet::named(input.clone(), source),
            source.source(),
            &mut evidence.problems,
        ) else {
            continue;
        };
        let out = super::resolve::get(&within(cx, deadline.remaining()), &query);
        evidence.sources.extend(out.sources.iter().cloned());
        evidence.problems.extend(out.problems.iter().cloned());
        evidence.local = evidence.local.or(out.local);
        if !usable(&out, source) || out.dropped {
            continue;
        }
        if let ReplyData::DrugCard {
            body: Some(card), ..
        } = out.body
        {
            let exact = !ambiguous(&card) && literal_name(&card, input);
            if exact {
                for name in names(&card) {
                    if deadline.remaining().is_zero() {
                        break;
                    }
                    if !tried.insert(name.as_str().to_lowercase()) {
                        continue;
                    }
                    if let Some(found) = attempt(&card, name, &mut evidence) {
                        return (Some(found), None, evidence);
                    }
                }
            }
            if alternatives.as_ref().is_none_or(|previous: &DrugCard| {
                exact && (ambiguous(previous) || !literal_name(previous, input))
            }) {
                alternatives = Some(card);
            }
        }
    }
    (None, alternatives, evidence)
}

fn ambiguous(card: &DrugCard) -> bool {
    card.header
        .name_resolution
        .as_ref()
        .is_none_or(|r| r.total != 1)
}

pub(super) fn literal_name(card: &DrugCard, input: &Text) -> bool {
    names(card)
        .iter()
        .any(|name| biomcp_types::DrugNameMatch::same_name(name.as_str(), input.as_str()))
}

pub(super) fn names(card: &DrugCard) -> Vec<&Text> {
    let mut seen = std::collections::BTreeSet::new();
    card.header
        .name
        .iter()
        .chain(card.alternate_names.iter())
        .filter(|name| seen.insert(name.as_str().to_lowercase()))
        .collect()
}

fn usable(out: &Outcome, source: DrugSource) -> bool {
    out.local.is_none()
        && out
            .sources
            .iter()
            .any(|s| s.source == source.source() && s.state == SourceState::Ok)
}

fn zero(out: &Outcome, source: DrugSource) -> bool {
    usable(out, source)
        && !out.dropped
        && out
            .page
            .as_ref()
            .is_some_and(|page| page.total == 0 && page.returned == 0)
}

fn notice(out: &mut Outcome, selected: DrugSource, message: String) {
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

fn hint(out: &mut Outcome, card: &DrugCard, input: &Text, selected: DrugSource, fresh: bool) {
    if ambiguous(card) || !literal_name(card, input) {
        notice(
            out,
            selected,
            format!(
                "{} has candidate records for {:?}, but no unique literal name match. \
                 No cross-source alias was selected. Search that source to inspect its candidates.",
                card.header.source,
                input.as_str()
            ),
        );
        if let Some(query) = crate::next::checked(
            DrugSearch::named(input.clone(), card.header.source, 10),
            card.header.source.source(),
            &mut out.problems,
        ) {
            out.next.push(crate::next::Step::run(
                StepKind::Disambiguate,
                &[card.header.source.source()],
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
            card.header.source,
            card.header.id
        ),
    );
    if let Some(query) = crate::next::checked(
        DrugGet::new(card.header.id.clone(), card.header.source),
        card.header.source.source(),
        &mut out.problems,
    ) {
        out.next.push(crate::next::Step::run(
            StepKind::Disambiguate,
            &[card.header.source.source()],
            Request::GetDrug(query),
            fresh,
        ));
    }
    status::search_for(out, &[selected.source()]);
}

fn search_spelling(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let deadline = cx.fetch.deadline(cx.budget);
    let mut out = super::search(cx, query);
    if query.source() != DrugSource::Chembl
        || query.explicit_name_selector()
        || query.offset() != 0
        || !zero(&out, query.source())
    {
        return out;
    }
    let Some(synonym) = crate::next::checked(
        query.with_name_selector(DrugNameSelector::Synonym),
        query.source().source(),
        &mut out.problems,
    ) else {
        return out;
    };
    let mut found = super::search(&within(cx, deadline.remaining()), &synonym);
    notice(
        &mut found,
        query.source(),
        format!(
            "The preferred-name search for {:?} returned no rows; tried the native exact synonym filter.",
            query.query().map_or("", Text::as_str)
        ),
    );
    found
}
