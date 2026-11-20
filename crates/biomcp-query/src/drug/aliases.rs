//! Source-stated aliases may recover an empty source lookup, never substitute its records.
//! A search that widened its name and still found nothing tries them the same way.
use super::resolve::within;
use crate::{Ctx, Outcome, status};
use biodata::{DrugId, Text};
use biomcp_types::{
    DrugCard, DrugGet, DrugNameSelector, DrugSearch, DrugSource, Reason, ReplyBody, ReplyData,
    Request, SourceNotice, SourceOutcome, SourceState, SourceStatus, StepKind,
};

#[expect(
    clippy::disallowed_methods,
    reason = "Times nested commands, which charge their own command clocks; it opens no leg (1038)"
)]
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
    let (found, stated, evidence) = with_aliases(
        &within(cx, deadline.remaining()),
        input,
        query.source(),
        |stated, name, _| {
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
                    stated.source,
                    stated.id,
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
    if let Some(stated) = stated {
        hint(&mut out, &stated, input, query.source(), cx.fresh);
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
#[expect(
    clippy::disallowed_methods,
    reason = "Times nested commands, which charge their own command clocks; it opens no leg (1038)"
)]
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
    let (found, stated, evidence) = with_aliases(
        &within(cx, deadline.remaining()),
        input,
        query.source(),
        |stated, name, evidence| {
            if query.source() == DrugSource::OpenfdaLabel {
                if evidence.label_candidate.is_some() {
                    return None;
                }
                evidence.label_candidate = Some(name.clone());
                return label_fallback(cx, &deadline, queries, input, evidence);
            }
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
                        stated.source,
                        stated.id,
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
    let lookup_failed = evidence.lookup_failed();
    let Some(stated) = stated else {
        if lookup_failed {
            status::item(&mut out, false, true, false);
        }
        return vec![out];
    };
    // A Usage reply discards source metadata, including the other-source hint.
    queries
        .iter()
        .map(|query| {
            let mut part = super::fork(&out, DrugCard::into_reply(query.clone(), None));
            hint(&mut part, &stated, input, query.source(), cx.fresh);
            if let Some(name) = &evidence.label_candidate {
                label_candidates(&mut part, input, name, cx.fresh);
            }
            status::item(&mut part, false, lookup_failed, false);
            part
        })
        .collect()
}

fn label_fallback(
    cx: &Ctx<'_>,
    deadline: &biomcp_fetch::Deadline<'_>,
    queries: &[DrugGet],
    input: &Text,
    evidence: &mut Evidence,
) -> Option<Vec<Outcome>> {
    let (lookup, application) =
        super::resolve::brand_application(&within(cx, deadline.remaining()), input)?;
    evidence.sources.extend(lookup.sources);
    evidence.problems.extend(lookup.problems);
    evidence.local = evidence.local.or(lookup.local);
    let application = application?;
    let mut found = super::label_application::fallback(
        &within(cx, deadline.remaining()),
        queries,
        &application,
    )?;
    for out in &mut found {
        notice(
            out,
            DrugSource::OpenfdaLabel,
            format!(
                "openFDA did not resolve {input:?} by name. Drugs@FDA lists that brand under {application}; looked for its openFDA label by application.",
                input = input.as_str(),
            ),
        );
    }
    Some(found)
}

fn label_candidates(out: &mut Outcome, input: &Text, name: &Text, fresh: bool) {
    notice(
        out,
        DrugSource::OpenfdaLabel,
        format!(
            "openFDA did not resolve {input:?} by name. No label could be verified for its application. The name {name:?} may find candidates.",
            input = input.as_str(),
            name = name.as_str(),
        ),
    );
    if let Some(search) = crate::next::checked(
        DrugSearch::named(name.clone(), DrugSource::OpenfdaLabel, 10),
        biomcp_types::Source::OpenfdaLabel,
        &mut out.problems,
    ) {
        out.next.push(crate::next::Step::run(
            StepKind::Disambiguate,
            &[biomcp_types::Source::OpenfdaLabel],
            Request::SearchDrug(search),
            fresh,
        ));
    }
}

/// Retry a DDInter partner DDInter does not name under each exact name another source states
/// for it; the first name that DDInter names is the partner.
pub(super) fn partner<T>(
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
    label_candidate: Option<Text>,
    sources: Vec<SourceOutcome>,
    problems: Vec<biomcp_types::Problem>,
    local: Option<biomcp_fetch::RequestError>,
}
impl Evidence {
    /// Not finding the record is an answer only when every name lookup answered (1059).
    fn lookup_failed(&self) -> bool {
        self.sources.iter().any(|s| {
            s.state == biomcp_types::SourceState::Failed
                && s.reason != Some(biomcp_types::Reason::NotFound)
        })
    }
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

/// One other source's record for a name, with the names it states.
struct Stated {
    source: DrugSource,
    id: DrugId,
    names: Vec<Text>,
    /// The source holds more than one record for the name.
    ambiguous: bool,
}
impl Stated {
    fn card(card: &DrugCard) -> Self {
        Self {
            source: card.header.source,
            id: card.header.id.clone(),
            names: names(card).into_iter().cloned().collect(),
            ambiguous: card
                .header
                .name_resolution
                .as_ref()
                .is_none_or(|r| r.total != 1),
        }
    }
    /// The record states the name itself, ignoring letter case.
    fn literal(&self, input: &Text) -> bool {
        self.names
            .iter()
            .any(|name| biomcp_types::DrugNameMatch::same_name(name.as_str(), input.as_str()))
    }
}

/// Try each exact name another source states for `input`; the caller adds the evidence to
/// what `attempt` found. ChEMBL's record is read from MyChem's copy of ChEMBL, so a stalled
/// ChEMBL API never holds the search.
#[expect(
    clippy::disallowed_methods,
    reason = "Times nested commands, which charge their own command clocks; it opens no leg (1038)"
)]
fn with_aliases<T>(
    cx: &Ctx<'_>,
    input: &Text,
    selected: DrugSource,
    mut attempt: impl FnMut(&Stated, &Text, &mut Evidence) -> Option<T>,
) -> (Option<T>, Option<Stated>, Evidence) {
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
        let lookup = within(cx, deadline.remaining());
        let found = if source == DrugSource::Chembl {
            chembl(&lookup, input, &mut evidence)
        } else {
            card(&lookup, input, source, &mut evidence)
        };
        let Some(stated) = found else {
            continue;
        };
        let exact = !stated.ambiguous && stated.literal(input);
        if exact {
            for name in &stated.names {
                if deadline.remaining().is_zero() {
                    break;
                }
                if !tried.insert(name.as_str().to_lowercase()) {
                    continue;
                }
                if let Some(found) = attempt(&stated, name, &mut evidence) {
                    return (Some(found), None, evidence);
                }
            }
        }
        if alternatives.as_ref().is_none_or(|previous: &Stated| {
            exact && (previous.ambiguous || !previous.literal(input))
        }) {
            alternatives = Some(stated);
        }
    }
    (None, alternatives, evidence)
}

/// The record a source's own name resolution chooses for the name.
fn card(cx: &Ctx<'_>, input: &Text, source: DrugSource, evidence: &mut Evidence) -> Option<Stated> {
    let query = crate::next::checked(
        DrugGet::named(input.clone(), source),
        source.source(),
        &mut evidence.problems,
    )?;
    let out = super::resolve::get(cx, &query);
    evidence.sources.extend(out.sources.iter().cloned());
    evidence.problems.extend(out.problems.iter().cloned());
    evidence.local = evidence.local.or(out.local);
    if !usable(&out, source) || out.dropped {
        return None;
    }
    let ReplyData::DrugCard {
        body: Some(card), ..
    } = &out.body
    else {
        return None;
    };
    Some(Stated::card(card))
}

/// The ChEMBL molecule stating the name, from MyChem's copy of ChEMBL rather than ChEMBL's API.
fn chembl(cx: &Ctx<'_>, input: &Text, evidence: &mut Evidence) -> Option<Stated> {
    if cx.fetch.disabled(DrugSource::Chembl.source()) {
        evidence.sources.push(biomcp_adapters::registry::outcome(
            DrugSource::Chembl.source(),
            SourceStatus::Skipped(Some(Reason::Disabled)),
        ));
        return None;
    }
    // MyChem holds the copy, so with MyChem off ChEMBL's record is not read, as for search names.
    if cx.fetch.disabled(DrugSource::Mychem.source()) {
        return None;
    }
    let copy = super::search_names::chembl_copy(cx, input);
    evidence.sources.extend(copy.evidence);
    evidence.problems.extend(copy.problems);
    evidence.local = evidence.local.or(copy.error.and_then(status::local));
    let molecules = copy.molecules?;
    let molecule = molecules.first()?;
    // The record's ID becomes the follow-up command's input.
    let id = crate::next::checked(
        molecule.id.parse::<DrugId>(),
        DrugSource::Chembl.source(),
        &mut evidence.problems,
    )?;
    Some(Stated {
        source: DrugSource::Chembl,
        id,
        names: molecule.names.clone(),
        ambiguous: molecules.len() > 1,
    })
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

fn hint(out: &mut Outcome, stated: &Stated, input: &Text, selected: DrugSource, fresh: bool) {
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

#[expect(
    clippy::disallowed_methods,
    reason = "Times nested commands, which charge their own command clocks; it opens no leg (1038)"
)]
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
