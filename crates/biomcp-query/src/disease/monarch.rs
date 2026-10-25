use super::page::{empty, overall, page_info, run};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::monarch;
use biomcp_types::{
    DiseaseGet, DiseaseGenes, DiseasePhenotypes, DiseaseSection, GeneGet, GeneInput,
    QueryBody, Request, Source, StepKind,
};
/// Read one native association page without identity choice or cross-source joins.
pub fn get(cx: &Ctx<'_>, query: &DiseaseGet) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&monarch::SPEC, deadline).fresh(cx.fresh);
    let reply = monarch::associations(&leg, query);
    let error = reply.as_ref().err().copied();
    let mut out = empty(body(query, Vec::new(), Box::default(), None));
    out.sources.push(status::source(Source::Monarch, leg.finish(), error));
    out.local = error.and_then(status::local);
    if let Ok(page) = reply {
        let returned = page.items.len() as u32;
        out.page = Some(page_info(query.section_offset(), returned, page.total));
        if query.section() == Some(DiseaseSection::Genes) {
            gene_steps(&mut out, &page.items);
        }
        if returned > 0 && u64::from(query.section_offset()) + u64::from(returned) < page.total {
            out.next.push((StepKind::Page, run(Request::GetDisease(query.at_section_offset(query.section_offset().saturating_add(returned))))));
        }
        let components = page.items.first().map(|row| row.components.clone()).unwrap_or_default();
        out.body = body(query, page.items, components, u64::try_from(page.metadata.fetched).ok().filter(|v| *v > 0));
        out.status = overall(&out, page.total == 0);
    }
    command.finish(deadline.remaining());
    out
}
fn body(query: &DiseaseGet, items: Vec<biomcp_types::DiseaseAssociationItem>, components: Box<[biomcp_types::ComponentLabel]>, fetched: Option<u64>) -> QueryBody {
    if query.section() == Some(DiseaseSection::Genes) {
        QueryBody::DiseaseGenes(DiseaseGenes { items: items.into(), components, fetched })
    } else {
        QueryBody::DiseasePhenotypes(DiseasePhenotypes { items: items.into(), components, fetched })
    }
}
fn gene_steps(out: &mut Outcome, items: &[biomcp_types::DiseaseAssociationItem]) {
    for row in items {
        let Some(id) = &row.association.subject.id else { continue; };
        let id = id.to_string();
        let checked = match GeneInput::new(&id) {
            Ok(input @ (GeneInput::HgncId(_) | GeneInput::EnsemblId(_) | GeneInput::EntrezId(_))) => Some(input),
            _ => id.strip_prefix("NCBIGene:").and_then(|value| value.parse::<biodata::NcbiGeneId>().ok()).map(GeneInput::EntrezId),
        };
        if let Some(query) = checked.and_then(|input| GeneGet::new(vec![input].into()).ok()) {
            let step = (StepKind::Disambiguate, run(Request::GetGene(query)));
            if !out.next.contains(&step) { out.next.push(step); }
        }
    }
}
