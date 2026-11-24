//! A complete GPML relation is mapped and retained before any page is selected.
use super::relation::{Saved, Selection};
use crate::{Ctx, Outcome, status};
use biodata::{PathwayId, PathwaySpecies};
use biomcp_adapters::wikipathways;
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{
    PathwayGeneCount, PathwayGeneCountScope, PathwayGenes, PathwayHeader, PathwayPageInfo,
    PathwayTotalKind, Source,
};

pub(super) fn fetch(
    cx: &Ctx<'_>,
    q: &Selection<'_>,
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Result<Option<Saved>, RequestError> {
    let PathwayId::Wikipathways(id) = q.id else {
        return Err(biomcp_fetch::InputError::Serialization.into());
    };
    let leg = cx
        .fetch
        .leg(
            &wikipathways::SPEC,
            deadline.child(std::time::Duration::from_secs(8)),
        )
        .fresh(q.fresh);
    let reply = (|| {
        // The stored human catalog admits the identity; its date is never a GPML revision.
        let card = wikipathways::card(&leg, id, false)?;
        if card.value.is_none() {
            return Ok(None);
        }
        let mapped = wikipathways::members(&leg, id)?;
        Ok::<_, RequestError>(Some((card, mapped)))
    })();
    out.sources.push(status::source(
        Source::WikiPathways,
        leg.finish(),
        reply.as_ref().err().copied(),
    ));
    let Some((card, mapped)) = reply? else {
        return Ok(None);
    };
    let rows = mapped.value;
    let reference_occurrences = rows.items.len() as u64;
    let mut problems: Vec<_> = card.problems.into_iter().chain(mapped.problems).collect();
    let mut sources = out.sources.clone();
    let (items, unresolved) = super::gpml_membership::resolve(
        cx,
        rows.items,
        deadline,
        q.fresh,
        &mut sources,
        &mut problems,
    );
    let total = items.len() as u64;
    Ok(Some(Saved {
        dropped: card.refusals == biomcp_adapters::Refusal::Refused
            || mapped.refusals == biomcp_adapters::Refusal::Refused,
        genes: Some(PathwayGenes {
            header: PathwayHeader {
                id: q.id.clone(),
                name: rows.name,
                species: PathwaySpecies::Human,
                version: None,
                revision: Some(rows.version),
            },
            release: None,
            gene_count: Some(PathwayGeneCount {
                scope: PathwayGeneCountScope::HgncGeneProducts,
                distinct: total,
                unmapped: rows.unmapped + unresolved,
            }),
            total,
            items: items.into(),
            page: PathwayPageInfo {
                offset: 0,
                returned: total as u32,
                total,
                total_kind: PathwayTotalKind::Observed,
                next_offset: None,
                pool: None,
            },
            left_out: None,
            reference_occurrences,
            physical_entities: rows.data_nodes,
        }),
        events: None,
        sources,
        problems,
    }))
}
