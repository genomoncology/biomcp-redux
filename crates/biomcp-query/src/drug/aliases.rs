//! Source-stated aliases may recover an empty source lookup, never substitute its records.
use super::resolve::within;
use crate::{Ctx, Outcome, status};
use biodata::Text;
use biomcp_types::{
    DrugCard, DrugGet, DrugNameSelector, DrugSearch, DrugSource, ReplyData, Request, SourceNotice,
    SourceOutcome, SourceState, StepKind,
};

pub(super) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let deadline = cx.fetch.deadline(cx.budget);
    let mut out = search_spelling(cx, query);
    if query.offset() != 0 || !zero(&out, query.source()) {
        return out;
    }
    let Some(input) = query.query() else {
        return out;
    };
    let (card, evidence) = authority(&within(cx, deadline.remaining()), input, query.source());
    evidence.append(&mut out);
    let Some(card) = card else {
        return out;
    };
    if ambiguous(&card) || query.explicit_name_selector() {
        hint(&mut out, &card, &evidence, input, query.source(), cx.fresh);
        return out;
    }
    for name in names(&card) {
        if biomcp_types::DrugNameMatch::same_name(input.as_str(), name.as_str()) {
            continue;
        }
        if deadline.remaining().is_zero() {
            break;
        }
        let retry = query.with_name(name.clone());
        let mut found = search_spelling(&within(cx, deadline.remaining()), &retry);
        if !zero(&found, query.source()) {
            evidence.append(&mut found);
            notice(
                &mut found,
                query.source(),
                format!(
                    "The search for {input:?} returned no rows. {} record {} supplies the name {name:?}; the selected source was searched with that name. Returned rows and totals belong to the selected source.",
                    card.header.source, card.header.id
                ),
            );
            return found;
        }
    }
    hint(&mut out, &card, &evidence, input, query.source(), cx.fresh);
    out
}

pub(super) fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let deadline = cx.fetch.deadline(cx.budget);
    let mut out = super::resolve::get(cx, query);
    if !matches!(out.body, ReplyData::Usage(_)) || !usable(&out, query.source()) {
        return out;
    }
    let input = query.input();
    let (card, evidence) = authority(&within(cx, deadline.remaining()), input, query.source());
    evidence.append(&mut out);
    let Some(card) = card else {
        return out;
    };
    if ambiguous(&card) {
        hint(&mut out, &card, &evidence, input, query.source(), cx.fresh);
        return out;
    }
    for name in names(&card) {
        if biomcp_types::DrugNameMatch::same_name(name.as_str(), input.as_str()) {
            continue;
        }
        if deadline.remaining().is_zero() {
            break;
        }
        let Ok(retry) = query.with_name(name.clone()) else {
            continue;
        };
        let mut found = super::resolve::get(&within(cx, deadline.remaining()), &retry);
        if !matches!(found.body, ReplyData::Usage(_)) {
            evidence.append(&mut found);
            notice(
                &mut found,
                query.source(),
                format!(
                    "The source did not resolve {input:?}. {} record {} supplies the name {name:?}; the selected source resolved that name independently.",
                    card.header.source, card.header.id
                ),
            );
            return found;
        }
    }
    hint(&mut out, &card, &evidence, input, query.source(), cx.fresh);
    out
}

#[derive(Default)]
struct Evidence {
    sources: Vec<SourceOutcome>,
    problems: Vec<biomcp_types::Problem>,
    local: Option<biomcp_fetch::RequestError>,
}
impl Evidence {
    fn append(&self, out: &mut Outcome) {
        for source in &self.sources {
            if !out.sources.iter().any(|s| s.source == source.source) {
                out.sources.push(source.clone())
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

fn authority(cx: &Ctx<'_>, input: &Text, selected: DrugSource) -> (Option<DrugCard>, Evidence) {
    let deadline = cx.fetch.deadline(cx.budget);
    let mut evidence = Evidence::default();
    let mut alternatives = None;
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
        let Ok(query) = DrugGet::named(input.clone(), source) else {
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
            if card
                .header
                .name_resolution
                .as_ref()
                .is_some_and(|r| r.total == 1)
            {
                return (Some(card), evidence);
            }
            if alternatives.is_none() {
                alternatives = Some(card)
            }
        }
    }
    (alternatives, evidence)
}

fn ambiguous(card: &DrugCard) -> bool {
    card.header
        .name_resolution
        .as_ref()
        .is_none_or(|r| r.total != 1)
}

fn names(card: &DrugCard) -> Vec<&Text> {
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
            .is_some_and(|page| page.total == 0 && page.returned == 0)
}

fn notice(out: &mut Outcome, selected: DrugSource, message: String) {
    if let Some(source) = out
        .sources
        .iter_mut()
        .find(|s| s.source == selected.source())
    {
        source.notices.push(SourceNotice::RequestGuidance {
            message: message.into(),
            retry_after_seconds: None,
        });
    }
}

fn hint(
    out: &mut Outcome,
    card: &DrugCard,
    evidence: &Evidence,
    input: &Text,
    selected: DrugSource,
    fresh: bool,
) {
    evidence.append(out);
    if ambiguous(card) {
        notice(
            out,
            selected,
            format!(
                "{} has several exact records for {input:?}; no cross-source alias was selected. Search that source to inspect its candidates.",
                card.header.source
            ),
        );
        if let Ok(query) = DrugSearch::named(input.clone(), card.header.source, 10) {
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
            "The completed queries in {selected} found no matching record for {input:?}. {} record {} matches the input; its presence does not establish a record in {selected}.",
            card.header.source, card.header.id
        ),
    );
    if let Ok(query) = DrugGet::new(card.header.id.clone(), card.header.source) {
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
    let out = super::search_source(cx, query);
    if query.source() != DrugSource::Chembl
        || query.explicit_name_selector()
        || query.offset() != 0
        || !zero(&out, query.source())
    {
        return out;
    }
    let Ok(synonym) = query.with_name_selector(DrugNameSelector::Synonym) else {
        return out;
    };
    let mut found = super::search_source(&within(cx, deadline.remaining()), &synonym);
    notice(
        &mut found,
        query.source(),
        format!(
            "The preferred-name search for {:?} returned no rows; tried the native exact synonym filter.",
            query.query()
        ),
    );
    found
}
