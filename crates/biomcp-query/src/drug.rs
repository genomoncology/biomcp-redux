//! Literal native drug discovery and exact selected source identities.
use crate::{Action, Ctx, Outcome, status};
use biomcp_adapters::mychem;
use biomcp_types::{
    DrugGet, DrugPage, DrugSearch, DrugSource, PageInfo, QueryBody, Request, Source, Status,
    StepKind,
};
/// Search the one selected source, without fan-out or selecting an identity.
pub fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let mut out = empty(QueryBody::DrugPage(DrugPage {
        items: Box::default(),
        total: None,
    }));
    if query.source() != DrugSource::Mychem {
        return unsupported(query.source(), out);
    }
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&mychem::SPEC, deadline).fresh(cx.fresh);
    let result = mychem::search(&leg, query.query().as_str(), query.limit(), query.offset());
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::Mychem, leg.finish(), error));
    if let Ok(page) = result {
        out.status = if page.items.is_empty() {
            Status::Absent
        } else if out
            .sources
            .iter()
            .any(|s| s.state == biomcp_types::SourceState::Stale)
        {
            Status::Partial
        } else {
            Status::Complete
        };
        let returned = page.items.len() as u32;
        out.sources.iter_mut().for_each(|s| {
            s.count = page.total;
            s.fetched = Some(returned);
        });
        out.page = page.total.map(|total| PageInfo {
            offset: query.offset(),
            returned,
            total,
            pool: None,
            truncated: None,
        });
        for item in &page.items {
            if let Ok(get) = DrugGet::new(item.id.clone(), DrugSource::Mychem) {
                out.next.push((
                    StepKind::Disambiguate,
                    Action::Run {
                        request: Request::GetDrug(get),
                        fresh: cx.fresh,
                    },
                ));
            }
        }
        if let Some(offset) = query
            .offset()
            .checked_add(returned)
            .filter(|n| returned > 0 && page.total.is_some_and(|t| u64::from(*n) < t))
        {
            out.next.push((
                StepKind::Page,
                Action::Run {
                    request: Request::SearchDrug(query.at_offset(offset)),
                    fresh: cx.fresh,
                },
            ));
        }
        out.body = QueryBody::DrugPage(page);
    }
    component_outcomes(cx, &mut out, true);
    command.finish(deadline.remaining());
    out
}
/// Get one exact native document, without fallback or automatic section expansion.
pub fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let mut out = empty(QueryBody::DrugCard(None));
    if query.source() != DrugSource::Mychem || !query.sections().is_empty() {
        return unsupported(query.source(), out);
    }
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&mychem::SPEC, deadline).fresh(cx.fresh);
    let result = mychem::get(&leg, &query.input().to_string());
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::Mychem, leg.finish(), error));
    if let Ok(card) = result {
        out.status = if out
            .sources
            .iter()
            .any(|s| s.state == biomcp_types::SourceState::Stale)
        {
            Status::Partial
        } else {
            Status::Complete
        };
        out.body = QueryBody::DrugCard(Some(card));
    }
    component_outcomes(cx, &mut out, false);
    command.finish(deadline.remaining());
    out
}
fn empty(body: QueryBody) -> Outcome {
    Outcome {
        status: Status::Failed,
        body,
        page: None,
        sources: Vec::new(),
        problems: Vec::new(),
        next: Vec::new(),
        local: None,
    }
}
fn unsupported(source: DrugSource, mut out: Outcome) -> Outcome {
    out.sources.push(biomcp_adapters::registry::outcome(
        source.source(),
        biomcp_types::SourceState::Skipped,
        Some(biomcp_types::Reason::Unsupported),
    ));
    out
}

fn component_outcomes(cx:&Ctx<'_>,out:&mut Outcome,search:bool) {
    let Some(parent)=out.sources.first().cloned() else { return; };
    for component in biomcp_adapters::drug_sources::components::COMPONENTS {
        if search && matches!(component.name,"drugcentral"|"openfda") { continue; }
        let Some(source)=component.native_source else { continue; };
        let disabled=component.disabled(cx.fetch);
        out.sources.push(biomcp_adapters::registry::outcome(source,
            if disabled {biomcp_types::SourceState::Skipped}else{parent.state},
            if disabled {Some(biomcp_types::Reason::Disabled)}else{parent.reason}));
    }
}
