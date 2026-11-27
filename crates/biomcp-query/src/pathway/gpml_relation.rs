//! A complete GPML relation is mapped and retained before any page is selected.
use super::relation::{Saved, Selection};
use crate::{Ctx, Outcome, status};
use biodata::{PathwayId, PathwayMember, PathwaySpecies};
use biomcp_adapters::wikipathways::{self, Members};
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{
    PathwayGeneCount, PathwayGeneCountScope, PathwayGenes, PathwayHeader, PathwayPageInfo,
    PathwayTotalKind, Source, SourceState,
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
    let (card, mapped) = match reply {
        Ok(Some(pair)) => pair,
        other => {
            out.sources.push(status::source(
                Source::WikiPathways,
                leg.finish(),
                other.as_ref().err().copied(),
            ));
            return other.map(|_| None);
        }
    };
    let mut rows = mapped.value;
    let reference_occurrences = rows.items.len() as u64;
    let mut problems: Vec<_> = card.problems.into_iter().chain(mapped.problems).collect();
    let mut sources = out.sources.clone();
    let mut xref_error = None;
    let (items, unresolved) = super::gpml_membership::resolve(
        cx,
        std::mem::take(&mut rows.items),
        deadline,
        q.fresh,
        &mut sources,
        &mut problems,
        || match wikipathways::gene_xrefs(&leg, id, rows.version.as_str()) {
            Ok(xrefs) => Some(xrefs),
            Err(error) => {
                xref_error = Some(error);
                None
            }
        },
    );
    sources.insert(
        0,
        status::source(Source::WikiPathways, leg.finish(), xref_error),
    );
    let unmapped = rows.unmapped + unresolved;
    let skipped_join = sources
        .iter()
        .any(|source| source.source == Source::Hgnc && source.state == SourceState::Skipped);
    Ok(Some(Saved {
        dropped: card.refusals == biomcp_adapters::Refusal::Refused
            || mapped.refusals == biomcp_adapters::Refusal::Refused
            || skipped_join
            || unmapped > 0,
        genes: Some(gene_body(
            q.id.clone(),
            rows,
            items,
            unmapped,
            reference_occurrences,
        )),
        events: None,
        sources,
        problems,
    }))
}

fn gene_body(
    id: PathwayId,
    rows: Members,
    items: Vec<PathwayMember>,
    unmapped: u64,
    reference_occurrences: u64,
) -> PathwayGenes {
    let total = items.len() as u64;
    PathwayGenes {
        header: PathwayHeader {
            id,
            name: rows.name,
            species: PathwaySpecies::Human,
            version: None,
            revision: Some(rows.version),
        },
        release: None,
        gene_count: Some(PathwayGeneCount {
            scope: PathwayGeneCountScope::HgncGeneProducts,
            distinct: total,
            unmapped,
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
    }
}
