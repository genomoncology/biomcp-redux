//! Read and retain Reactome pathway relations and their source provenance.
use super::relation::{Saved, Selection};
use crate::{Ctx, Outcome, status};
use biodata::PathwayId;
use biomcp_adapters::reactome;
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{PathwayCard, PathwayHeader, PathwayPageInfo, PathwayTotalKind, Source};
pub(super) fn fetch(
    cx: &Ctx<'_>,
    q: &Selection<'_>,
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Result<Option<Saved>, RequestError> {
    let PathwayId::Reactome(id) = q.id else {
        return Err(biomcp_fetch::InputError::Serialization.into());
    };
    let leg = cx.fetch.leg(&reactome::SPEC, deadline).fresh(q.fresh);
    let mut release_error = None;
    let reply = (|| {
        let native_card;
        let card = if let Some(card) = q.card {
            card
        } else {
            let mapped = reactome::card(&leg, id, false)?;
            out.include_refusals(mapped.refusals);
            out.problems.extend(mapped.problems);
            let Some(card) = mapped.value else {
                return Ok(None);
            };
            native_card = card;
            &native_card
        };
        let header = relation_header(card);
        let mut saved = Saved {
            dropped: out.dropped,
            genes: None,
            events: None,
            sources: Vec::new(),
            problems: Vec::new(),
        };
        if q.genes {
            let rows = reactome::members(&leg, id)?;
            let release = if q.card.is_some() {
                card.release.clone()
            } else {
                reactome::release(&leg)
                    .inspect_err(|&error| release_error = Some(error))
                    .ok()
            };
            saved.dropped |= rows.refusals == biomcp_adapters::Refusal::Refused;
            saved.problems = rows.problems;
            saved.genes = Some(biomcp_types::PathwayGenes {
                header,
                release,
                gene_count: None,
                total: rows.value.items.len() as u64,
                page: full_page(rows.value.items.len(), PathwayTotalKind::DistinctReferences),
                left_out: None,
                items: rows.value.items.into(),
                reference_occurrences: rows.value.reference_occurrences,
                physical_entities: rows.value.physical_entities,
            });
        } else {
            let rows = reactome::events(&leg, id)?;
            saved.dropped |= rows.refusals == biomcp_adapters::Refusal::Refused;
            saved.problems = rows.problems;
            saved.events = Some(biomcp_types::PathwayEvents {
                header,
                total: rows.value.len() as u64,
                page: full_page(rows.value.len(), PathwayTotalKind::Observed),
                items: rows.value.into(),
            });
        }
        Ok::<_, RequestError>(Some(saved))
    })();
    out.sources.push(status::source(
        Source::Reactome,
        leg.finish(),
        reply.as_ref().err().copied().or(release_error),
    ));
    let Some(mut saved) = reply? else {
        return Ok(None);
    };
    saved.sources = out.sources.clone();
    saved.problems.extend(out.problems.clone());
    Ok(Some(saved))
}
fn relation_header(card: &PathwayCard) -> PathwayHeader {
    PathwayHeader {
        id: card.pathway.id.clone(),
        name: card.pathway.name.clone(),
        species: card.pathway.species,
        version: card.pathway.version.clone(),
        revision: card.pathway.revision.clone(),
    }
}

fn full_page(size: usize, kind: PathwayTotalKind) -> PathwayPageInfo {
    PathwayPageInfo {
        offset: 0,
        returned: size as u32,
        total: size as u64,
        total_kind: kind,
        next_offset: None,
        pool: None,
    }
}
