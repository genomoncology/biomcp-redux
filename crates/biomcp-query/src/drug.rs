//! Literal native drug discovery and exact selected source identities.
mod civic;
mod ddinter;
mod drugsfda;
mod ema;
mod next;
mod openfda_label;
mod recall;
pub(crate) mod who;
use crate::{Action, Ctx, Outcome, status};
use biomcp_adapters::mychem;
use biomcp_types::{
    DrugGet, DrugPage, DrugSearch, DrugSource, PageInfo, QueryBody, Request, Source, Status,
    StepKind,
};
/// Search the one selected source, without fan-out or selecting an identity.
pub fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    if query.source() == DrugSource::FdaDrugRecall {
        return recall::search(cx, query);
    }
    if query.source() == DrugSource::WhoPq {
        return who::search(cx, query);
    }
    if query.source() == DrugSource::Drugsfda {
        return drugsfda::search(cx, query);
    }
    if matches!(query.source(), DrugSource::Chembl | DrugSource::Opentargets) {
        return native::search(cx, query);
    }
    if query.source() == DrugSource::OpenfdaLabel {
        return openfda_label::search(cx, query);
    }
    if query.source() == DrugSource::Civic {
        return civic::search(cx, query);
    }
    if query.source() == DrugSource::Ema {
        return ema::search(cx, query);
    }
    if query.source() == DrugSource::Ddinter {
        return ddinter::search(cx, query);
    }
    let mut out = empty(QueryBody::DrugPage(DrugPage {
        items: Box::default(),
        total: None,
    }));
    if query.source() != DrugSource::Mychem {
        return unsupported(query.source(), out);
    }
    let Some(name) = query.query() else {
        return unsupported(query.source(), out);
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&mychem::SPEC, deadline).fresh(cx.fresh);
    let result = mychem::search(&leg, name.as_str(), query.limit(), query.offset());
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::Mychem, leg.finish(), error));
    if let Ok((page, received)) = result {
        out.problems = page.problems;
        search_results(query, cx.fresh, page.value, &mut out, received);
    }
    component_outcomes(cx, &mut out, true);
    command.finish(deadline.remaining());
    out
}
/// Get one exact native document, without fallback or automatic section expansion.
pub fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let mut out = selected_get(cx, query);
    let source = query.source().source();
    if out.sources.iter().any(|outcome| {
        outcome.source == source && outcome.reason == Some(biomcp_types::Reason::BadReply)
    }) {
        out.problems.push(biomcp_types::Problem {
            source,
            record: None,
            field: "record".into(),
            code: biomcp_types::ProblemCode::ValueUnknown,
        });
    }
    out
}
fn selected_get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    if query.source() == DrugSource::FdaDrugRecall {
        return recall::get(cx, query);
    }
    if query.source() == DrugSource::Drugsfda {
        return drugsfda::get(cx, query);
    }
    if matches!(query.source(), DrugSource::Chembl | DrugSource::Opentargets) {
        return native::get(cx, query);
    }
    if query.source() == DrugSource::OpenfdaLabel {
        return openfda_label::get(cx, query);
    }
    if query.source() == DrugSource::Civic {
        return civic::get(cx, query);
    }
    if query.source() == DrugSource::Ema {
        return ema::get(cx, query);
    }
    if query.source() == DrugSource::Ddinter {
        return ddinter::get(cx, query);
    }
    let xrefs = matches!(query.sections(), [biomcp_types::DrugSection::Xrefs]);
    let mut out = empty(if xrefs {
        QueryBody::DrugXrefs(None)
    } else {
        QueryBody::DrugCard(None)
    });
    if query.source() != DrugSource::Mychem
        || !matches!(query.sections(), [] | [biomcp_types::DrugSection::Xrefs])
    {
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
    if let Ok(document) = result {
        out.status = if out
            .sources
            .iter()
            .any(|s| s.state == biomcp_types::SourceState::Stale)
        {
            Status::Partial
        } else {
            Status::Complete
        };
        if xrefs {
            link_page(query, cx.fresh, document, &mut out);
        } else {
            if document
                .value
                .card
                .available_sections
                .contains(&biomcp_types::DrugSection::Xrefs)
                && let Ok(request) = query.for_section(biomcp_types::DrugSection::Xrefs)
            {
                out.next.push((
                    StepKind::Section,
                    Action::Run {
                        request: Request::GetDrug(request),
                        fresh: cx.fresh,
                    },
                ));
            }
            if !document.problems.is_empty() {
                out.status = Status::Partial;
            }
            out.problems = document.problems;
            out.body = QueryBody::DrugCard(Some(document.value.card));
        }
    }
    component_outcomes(cx, &mut out, false);
    command.finish(deadline.remaining());
    out
}
fn link_page(
    query: &DrugGet,
    fresh: bool,
    document: biomcp_adapters::Mapped<mychem::DrugDocument>,
    out: &mut Outcome,
) {
    let card = document.value.card;
    let total = document.value.xrefs.len() as u64;
    let offset = query.section_offset().unwrap_or(0);
    let records: Box<[_]> = document
        .value
        .xrefs
        .iter()
        .skip(offset as usize)
        .take(query.section_limit().unwrap_or(10) as usize)
        .cloned()
        .collect();
    let returned = records.len() as u32;
    if let Some(offset) = offset
        .checked_add(returned)
        .filter(|next| returned > 0 && u64::from(*next) < total)
        && let Ok(request) = query.at_section_offset(offset)
    {
        out.next.push((
            StepKind::Page,
            Action::Run {
                request: Request::GetDrug(request),
                fresh,
            },
        ));
    }
    if !document.problems.is_empty() {
        out.status = Status::Partial;
    }
    out.problems = document.problems;
    out.page = Some(PageInfo {
        offset,
        returned,
        total,
        pool: None,
        truncated: None,
    });
    out.body = QueryBody::DrugXrefs(Some(biomcp_types::DrugXrefs {
        identity: card.header,
        xrefs: biomcp_types::Section::Data {
            source: Source::Mychem,
            data: biomcp_types::DrugRecords {
                records,
                offset,
                total: Some(total),
                truncated: None,
                version: card.version,
                retrieved: card.retrieved,
            },
        },
    }));
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

fn component_outcomes(cx: &Ctx<'_>, out: &mut Outcome, search: bool) {
    let Some(parent) = out.sources.first().cloned() else {
        return;
    };
    for component in mychem::requested_components(search) {
        let Some(source) = component.native_source else {
            continue;
        };
        let disabled = component.disabled(cx.fetch);
        out.sources.push(biomcp_adapters::registry::outcome(
            source,
            if disabled {
                biomcp_types::SourceState::Skipped
            } else {
                parent.state
            },
            if disabled {
                Some(biomcp_types::Reason::Disabled)
            } else {
                parent.reason
            },
        ));
    }
}

mod native;
fn searched(query: &DrugSearch, fresh: bool, page: DrugPage, out: &mut Outcome) {
    let received = page.items.len() as u32;
    search_results(query, fresh, page, out, received);
}
fn search_results(
    query: &DrugSearch,
    fresh: bool,
    page: DrugPage,
    out: &mut Outcome,
    received: u32,
) {
    out.status = if page.items.is_empty() && out.problems.is_empty() {
        Status::Absent
    } else if !out.problems.is_empty()
        || out
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
        s.fetched = Some(received);
    });
    out.page = page.total.map(|total| PageInfo {
        offset: query.offset(),
        returned,
        total,
        pool: None,
        truncated: None,
    });
    for item in &page.items {
        if let Ok(get) = DrugGet::new(item.id.clone(), query.source()) {
            out.next.push((
                StepKind::Disambiguate,
                Action::Run {
                    request: Request::GetDrug(get),
                    fresh,
                },
            ));
        }
    }
    if let Some(offset) = query
        .offset()
        .checked_add(received)
        .filter(|n| received > 0 && page.total.is_some_and(|t| u64::from(*n) < t))
    {
        out.next.push((
            StepKind::Page,
            Action::Run {
                request: Request::SearchDrug(query.at_offset(offset)),
                fresh,
            },
        ));
    }
    out.body = QueryBody::DrugPage(page);
}
