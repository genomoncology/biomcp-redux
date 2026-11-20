//! Read candidate names from other sources and try each unique, literal alias.
use super::name_evidence::Evidence;
use crate::drug::resolve::{lookup_deadline, within};
use crate::{Ctx, Outcome, status};
use biodata::{DrugId, Text};
use biomcp_types::{DrugCard, DrugGet, DrugSource, Reason, ReplyData, SourceState, SourceStatus};

/// One other source's record for a name, with the names it states.
pub(super) struct Stated {
    pub(super) source: DrugSource,
    pub(super) id: DrugId,
    names: Vec<Text>,
    /// The source holds more than one record for the name.
    pub(super) ambiguous: bool,
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
    pub(super) fn literal(&self, input: &Text) -> bool {
        self.names
            .iter()
            .any(|name| biomcp_types::DrugNameMatch::same_name(name.as_str(), input.as_str()))
    }
}

/// Try each exact name another source states for `input`; the caller adds the evidence to
/// what `attempt` found. ChEMBL's record is read from MyChem's copy of ChEMBL, so a stalled
/// ChEMBL API never holds the search.
pub(super) fn with_aliases<T>(
    cx: &Ctx<'_>,
    input: &Text,
    selected: DrugSource,
    mut attempt: impl FnMut(&Stated, &Text, &mut Evidence) -> Option<T>,
) -> (Option<T>, Option<Stated>, Evidence) {
    let deadline = lookup_deadline(cx);
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
            let provider = if source == DrugSource::Chembl {
                DrugSource::Mychem
            } else {
                source
            };
            if !cx.fetch.disabled(source.source()) && !cx.fetch.disabled(provider.source()) {
                evidence.deadline(provider.source());
            }
            continue;
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
                if !tried.insert(name.as_str().to_lowercase()) {
                    continue;
                }
                if deadline.remaining().is_zero() {
                    evidence.deadline(selected.source());
                    break;
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
    let out = super::super::resolve::get(cx, &query);
    evidence.observe(&out, source);
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
    let copy = super::super::search_names::chembl_copy(cx, input);
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

pub(in crate::drug) fn names(card: &DrugCard) -> Vec<&Text> {
    let mut seen = std::collections::BTreeSet::new();
    card.header
        .name
        .iter()
        .chain(card.alternate_names.iter())
        .filter(|name| seen.insert(name.as_str().to_lowercase()))
        .collect()
}

pub(super) fn usable(out: &Outcome, source: DrugSource) -> bool {
    out.local.is_none()
        && out
            .sources
            .iter()
            .any(|s| s.source == source.source() && s.state == SourceState::Ok)
}
