use super::run;
use crate::Action;
use biodata::PathwayEventRef;
use biomcp_types::{GeneGet, GeneInput, PathwayEvents, PathwayEventsQuery, PathwayGenes,
    PathwayGenesQuery, PathwayGet, Request, StepKind};
type Steps = Vec<(StepKind, Action)>;
pub(super) fn genes(body: &PathwayGenes, limit: u32, next: &mut Steps) {
    let mut seen = Vec::new();
    for row in &body.items {
        if let Some(id) = row.gene && !seen.contains(&id) {
            seen.push(id);
            if let Ok(query) = GeneGet::new(vec![GeneInput::HgncId(id)].into()) {
                next.push((StepKind::Pivot, run(Request::GetGene(query))));
            }
        }
    }
    if let Some(offset) = body.page.next_offset
        && let Some(pool) = &body.page.pool
        && let Ok(query) = PathwayGenesQuery::new(body.header.id.clone(), limit) {
        next.push((StepKind::Page, run(Request::PathwayGenes(query.at_offset(offset)
            .with_pool(pool.id.clone())))));
    }
}
pub(super) fn events(body: &PathwayEvents, limit: u32, next: &mut Steps) {
    if let Some(offset) = body.page.next_offset
        && let Some(pool) = &body.page.pool
        && let Ok(query) = PathwayEventsQuery::new(body.header.id.clone(), limit) {
        next.push((StepKind::Page, run(Request::PathwayEvents(query.at_offset(offset)
            .with_pool(pool.id.clone())))));
    }
    for row in &body.items {
        if let PathwayEventRef::Object { id: Some(id), schema_class: Some(class), .. } = row
            && matches!(class.as_str(), "Pathway" | "TopLevelPathway") {
            next.push((StepKind::Disambiguate,
                run(Request::GetPathway(PathwayGet::new(biodata::PathwayId::Reactome(id.clone()))))));
            if next.iter().filter(|(kind, _)| *kind != StepKind::Page).count() >= 3 { break; }
        }
    }
}
