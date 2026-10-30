use super::page::{empty, page_info, run, slice};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::mydisease;
use biomcp_types::{
    DiseaseGet, DiseaseHierarchy, DiseaseSection, DiseaseSynonyms, DiseaseXrefs, QueryBody,
    Request, Source, StepKind,
};

/// Read only the selected exact annotation, then page its native component facts.
pub fn get(cx: &Ctx<'_>, query: &DiseaseGet) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&mydisease::SPEC, deadline).fresh(cx.fresh);
    let reply = query
        .id()
        .as_str()
        .parse()
        .map_err(|_| biomcp_fetch::RequestError::Input(biomcp_fetch::InputError::Path))
        .and_then(|id| mydisease::annotation(&leg, &id));
    let error = reply.as_ref().err().copied();
    let mut out = empty(QueryBody::DiseaseCard(None));
    out.sources
        .push(status::source(Source::MyDisease, leg.finish(), error));
    out.local = error.and_then(status::local);
    if let Ok(mapped) = reply {
        out.problems = mapped.problems;
        apply(&mut out, query, mapped.value);
    }
    super::page::record_problem(&mut out, query, Source::MyDisease);
    command.finish(deadline.remaining());
    out
}
fn apply(out: &mut Outcome, query: &DiseaseGet, record: mydisease::Annotation) {
    let components = record.card.components.clone();
    match query.section() {
        Some(DiseaseSection::Synonyms) => {
            let (items, total) = slice(record.synonyms, query);
            out.page = Some(page_info(query.section_offset(), items.len() as u32, total));
            out.body = QueryBody::DiseaseSynonyms(DiseaseSynonyms {
                items: items.into(),
                components,
            });
        }
        Some(DiseaseSection::Xrefs) => {
            let (items, total) = slice(record.xrefs, query);
            out.page = Some(page_info(query.section_offset(), items.len() as u32, total));
            out.body = QueryBody::DiseaseXrefs(DiseaseXrefs {
                items: items.into(),
                components,
            });
        }
        Some(DiseaseSection::Hierarchy) => {
            let (items, total) = slice(record.parents, query);
            out.page = Some(page_info(query.section_offset(), items.len() as u32, total));
            out.body = QueryBody::DiseaseHierarchy(DiseaseHierarchy {
                items: items.into(),
                components,
            });
        }
        _ => {
            for section in &record.card.sections {
                out.next.push((
                    StepKind::Section,
                    run(Request::GetDisease(query.with_section(*section))),
                ));
            }
            out.body = QueryBody::DiseaseCard(Some(record.card));
        }
    }
    if let Some(page) = &out.page {
        let offset = page.offset.saturating_add(page.returned);
        if page.returned > 0 && u64::from(offset) < page.total {
            out.next.push((
                StepKind::Page,
                run(Request::GetDisease(query.at_section_offset(offset))),
            ));
        }
    }
    {
        let absent = out.page.as_ref().is_some_and(|p| p.total == 0);
        crate::status::item(out, !absent, false, false);
    }
}
