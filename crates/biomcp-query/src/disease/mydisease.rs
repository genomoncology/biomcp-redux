use super::page::{empty, page_info, run, slice};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::mydisease;
use biomcp_types::ReplyBody;
use biomcp_types::{
    DiseaseGet, DiseaseHierarchy, DiseaseSection, DiseaseSynonyms, DiseaseXrefs, Request, Source,
    StepKind,
};

/// Read only the selected exact annotation, then page its native component facts.
pub(crate) fn get(cx: &Ctx<'_>, query: &DiseaseGet, id: &biodata::Curie) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&mydisease::SPEC, deadline).fresh(cx.fresh);
    let mut reply = mydisease::annotation(&leg, id);
    let mut label_problems = Vec::new();
    let mut label_error = None;
    let hierarchy = matches!(query.section(), Some(DiseaseSection::Hierarchy));
    for mapped in reply.iter_mut().filter(|_| hierarchy) {
        let page = mapped
            .value
            .parents
            .iter_mut()
            .skip(query.section_offset() as usize)
            .take(query.section_limit() as usize);
        label_error = label(&leg, page.collect(), &mut label_problems);
    }
    let error = reply.as_ref().err().copied();
    let mut out = empty(biomcp_types::DiseaseCard::into_reply(query.clone(), None));
    // A failed parent-name read leaves the rows unnamed, so the source and reply are not complete.
    out.sources.push(status::source(
        Source::MyDisease,
        leg.finish(),
        error.or(label_error),
    ));
    out.local = error.and_then(status::local);
    if let Ok(mapped) = reply {
        out.include_refusals(mapped.refusals);
        out.problems = mapped.problems;
        out.problems.extend(label_problems);
        apply(&mut out, query, mapped.value);
    }
    if error.is_some() || label_error.is_none() {
        super::page::record_problem(&mut out, query, Source::MyDisease);
    }
    command.finish(deadline.remaining());
    out
}
/// Name each direct parent on the page from its own record in one batch; an unnamed parent keeps its ID.
/// Returns the lookup failure, which the caller reports on the source.
fn label(
    leg: &biomcp_fetch::Leg<'_>,
    parents: Vec<&mut biodata::DiseaseRelation>,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Option<biomcp_fetch::RequestError> {
    let ids: Vec<_> = parents.iter().map(|row| row.object.id.clone()).collect();
    match mydisease::parent_labels(leg, &ids) {
        Ok(labels) => {
            problems.extend(labels.problems);
            for row in parents {
                if let Some(label) = labels.value.get(&row.object.id.to_string()) {
                    row.object.label = Some(label.clone());
                }
            }
            None
        }
        Err(error) => {
            problems.push(biomcp_types::Problem {
                source: Source::MyDisease,
                record: None,
                field: "parents.label: parent label lookup failed".into(),
                code: biomcp_types::ProblemCode::ValueUnknown,
            });
            Some(error)
        }
    }
}
fn apply(out: &mut Outcome, query: &DiseaseGet, record: mydisease::Annotation) {
    let components = record.card.components.clone();
    match query.section() {
        Some(DiseaseSection::Synonyms) => {
            let (items, total) = slice(record.synonyms, query);
            out.page = Some(page_info(query.section_offset(), items.len() as u32, total));
            out.body = (DiseaseSynonyms {
                items: items.into(),
                components,
            })
            .with_query(query.clone());
        }
        Some(DiseaseSection::Xrefs) => {
            let (items, total) = slice(record.xrefs, query);
            out.page = Some(page_info(query.section_offset(), items.len() as u32, total));
            out.body = (DiseaseXrefs {
                items: items.into(),
                components,
            })
            .with_query(query.clone());
        }
        Some(DiseaseSection::Hierarchy) => {
            let (items, total) = slice(record.parents, query);
            out.page = Some(page_info(query.section_offset(), items.len() as u32, total));
            out.body = (DiseaseHierarchy {
                items: items.into(),
                components,
            })
            .with_query(query.clone());
        }
        _ => {
            for section in &record.card.sections {
                out.next.push(crate::next::Step::new(
                    StepKind::Section,
                    &[biomcp_types::Source::MyDisease],
                    run(Request::GetDisease(query.with_section(*section))),
                ));
            }
            out.body = (record.card).with_query(query.clone());
        }
    }
    if let Some(page) = &out.page {
        let offset = page.offset.saturating_add(page.returned);
        if page.returned > 0 && u64::from(offset) < page.total {
            out.next.push(crate::next::Step::new(
                StepKind::Page,
                &[biomcp_types::Source::MyDisease],
                run(Request::GetDisease(query.at_section_offset(offset))),
            ));
        }
    }
    crate::status::item(out, true, false, false);
}
