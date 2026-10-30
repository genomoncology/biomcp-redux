use crate::{Action, Ctx, Outcome, status};
use biomcp_adapters::seer;
use biomcp_types::{
    DiseaseGet, DiseaseIncidence, DiseaseSearch, DiseaseSection, DiseaseSites, DiseaseSurvival,
    QueryBody, Request, Source, StepKind,
};

/// Page the native site catalog without selecting an ontology identity.
pub fn search(cx: &Ctx<'_>, query: &DiseaseSearch) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&seer::SPEC, deadline).fresh(cx.fresh);
    let reply = seer::catalog(&leg, query.limit(), query.offset());
    let error = reply.as_ref().err().copied();
    let mut out = empty(QueryBody::DiseaseSites(DiseaseSites {
        items: Box::default(),
        components: Box::default(),
        fetched: None,
    }));
    out.sources
        .push(status::source(Source::Seer, leg.finish(), error));
    out.local = error.and_then(status::local);
    if let Ok(page) = reply {
        out.dropped = page.dropped;
        let returned = page.rows.len() as u32;
        out.page = Some(page_info(query.offset(), returned, page.total));
        if page.received > 0 && u64::from(query.offset()) + u64::from(page.received) < page.total {
            out.next.push((
                StepKind::Page,
                run(Request::SearchDisease(
                    query.at_offset(query.offset().saturating_add(page.received)),
                )),
            ));
        }
        out.problems = page.problems;
        out.body = QueryBody::DiseaseSites(DiseaseSites {
            items: page.rows.into(),
            components: Box::default(),
            fetched: fetched(page.metadata.fetched),
        });
        {
            let absent = page.total == 0;
            crate::status::item(&mut out, !absent, false, false);
        }
    }
    command.finish(deadline.remaining());
    out
}
/// Preserve source stage or annual rows behind an explicit native site selection.
pub fn get(cx: &Ctx<'_>, query: &DiseaseGet) -> Outcome {
    let survival = query.section() == Some(DiseaseSection::Survival);
    let body = if survival {
        QueryBody::DiseaseSurvival(DiseaseSurvival {
            items: Box::default(),
            components: Box::default(),
            fetched: None,
        })
    } else {
        QueryBody::DiseaseIncidence(DiseaseIncidence {
            items: Box::default(),
            components: Box::default(),
            fetched: None,
        })
    };
    let mut out = empty(body);
    let Ok(selection) = seer::selection(query) else {
        return out;
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&seer::SPEC, deadline).fresh(cx.fresh);
    let reply = seer::statistics(
        &leg,
        &selection,
        query.section_limit(),
        query.section_offset(),
    );
    let error = reply.as_ref().err().copied();
    out.sources
        .push(status::source(Source::Seer, leg.finish(), error));
    out.local = error.and_then(status::local);
    if let Ok(page) = reply {
        out.dropped = page.dropped;
        let returned = page.rows.len() as u32;
        out.page = Some(page_info(query.section_offset(), returned, page.total));
        if page.received > 0
            && u64::from(query.section_offset()) + u64::from(page.received) < page.total
        {
            out.next.push((
                StepKind::Page,
                run(Request::GetDisease(query.at_section_offset(
                    query.section_offset().saturating_add(page.received),
                ))),
            ));
        }
        if let Ok(catalog) = serde_json::from_value(serde_json::json!({"source":"seer"})) {
            out.next
                .push((StepKind::Disambiguate, run(Request::SearchDisease(catalog))));
        }
        out.problems = page.problems;
        out.body = if survival {
            QueryBody::DiseaseSurvival(DiseaseSurvival {
                items: page.rows.into(),
                components: Box::default(),
                fetched: fetched(page.metadata.fetched),
            })
        } else {
            QueryBody::DiseaseIncidence(DiseaseIncidence {
                items: page.rows.into(),
                components: Box::default(),
                fetched: fetched(page.metadata.fetched),
            })
        };
        {
            let absent = page.total == 0;
            crate::status::item(&mut out, !absent, false, false);
        }
    }
    super::page::record_problem(&mut out, query, Source::Seer);
    command.finish(deadline.remaining());
    out
}
fn fetched(value: Option<i64>) -> Option<u64> {
    value.and_then(|value| u64::try_from(value).ok())
}
pub(super) use crate::status::empty;
pub(super) use crate::status::page_info;
fn run(request: Request) -> Action {
    Action::Run {
        request,
        fresh: false,
    }
}
