use super::page::{empty, overall, page_info, run, slice};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::ols;
use biomcp_types::{
    DiseaseGet, DiseasePage, DiseaseSearch, DiseaseSection, DiseaseSynonyms, DiseaseXrefs,
    QueryBody, Request, Source, Status, StepKind,
};
/// Discover one source-ordered page from one explicit ontology.
pub fn search(cx: &Ctx<'_>, query: &DiseaseSearch) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&ols::SPEC, deadline).fresh(cx.fresh);
    let reply = ols::search(&leg, query);
    let error = reply.as_ref().err().copied();
    let mut out = empty(QueryBody::DiseasePage(DiseasePage {
        items: Box::default(),
        components: query
            .ontology()
            .and_then(|v| ols::components(v).ok())
            .unwrap_or_default(),
    }));
    out.sources
        .push(status::source(Source::Ols, leg.finish(), error));
    out.local = error.and_then(status::local);
    if let Ok(page) = reply {
        out.dropped = page.dropped;
        let returned = page.rows.value.len() as u32;
        for row in &page.rows.value {
            if let Some(id) = &row.concept.id {
                let value = serde_json::json!({"id":id,"source":"ols","ontology":row.ontology});
                if let Ok(get) = serde_json::from_value(value) {
                    out.next
                        .push((StepKind::Disambiguate, run(Request::GetDisease(get))));
                }
            }
        }
        if page.received > 0 && u64::from(query.offset()) + u64::from(page.received) < page.total {
            out.next.push((
                StepKind::Page,
                run(Request::SearchDisease(
                    query.at_offset(query.offset().saturating_add(page.received)),
                )),
            ));
        }
        out.page = Some(page_info(query.offset(), returned, page.total));
        out.problems = page.rows.problems;
        if let QueryBody::DiseasePage(body) = &mut out.body {
            body.items = page.rows.value.into();
        }
        out.status = status::search_status(&out.sources, out.dropped);
    }
    command.finish(deadline.remaining());
    out
}
/// Get an exact OLS record; section slicing reuses the checked original response.
pub fn get(cx: &Ctx<'_>, query: &DiseaseGet) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&ols::SPEC, deadline).fresh(cx.fresh);
    let reply = ols::get(&leg, query);
    let error = reply.as_ref().err().copied();
    let mut out = empty(QueryBody::DiseaseCard(None));
    out.sources
        .push(status::source(Source::Ols, leg.finish(), error));
    out.local = error.and_then(status::local);
    match reply {
        Ok(Some(record)) => apply(&mut out, query, record),
        Ok(None) => out.status = Status::Absent,
        _ => (),
    }
    super::page::record_problem(&mut out, query, Source::Ols);
    command.finish(deadline.remaining());
    out
}
fn apply(out: &mut Outcome, query: &DiseaseGet, record: ols::Record) {
    let components = record.card.components.clone();
    out.problems = record.problems;
    match query.section() {
        Some(DiseaseSection::Synonyms) => {
            out.problems.extend(record.synonyms.problems);
            let (items, total) = slice(record.synonyms.value, query);
            out.page = Some(page_info(query.section_offset(), items.len() as u32, total));
            out.body = QueryBody::DiseaseSynonyms(DiseaseSynonyms {
                items: items.into(),
                components,
            });
        }
        Some(DiseaseSection::Xrefs) => {
            out.problems.extend(record.xrefs.problems);
            let (items, total) = slice(record.xrefs.value, query);
            out.page = Some(page_info(query.section_offset(), items.len() as u32, total));
            out.body = QueryBody::DiseaseXrefs(DiseaseXrefs {
                items: items.into(),
                components,
            });
        }
        _ => {
            out.problems.extend(record.synonyms.problems);
            out.problems.extend(record.xrefs.problems);
            for section in record.card.sections.iter() {
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
    out.status = overall(out, out.page.as_ref().is_some_and(|p| p.total == 0));
}
