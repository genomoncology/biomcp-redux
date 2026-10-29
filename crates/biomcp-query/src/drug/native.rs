//! Source-local native drug requests; selected identities never fan out.
mod identity;
mod sections;
use crate::{Action, Ctx, Outcome, status};
use biomcp_adapters::{chembl, opentargets};
use biomcp_fetch::{Leg, RequestError, SourceError};
use biomcp_types::{
    DrugGet, DrugPage, DrugSearch, DrugSection, DrugSource, PageInfo, QueryBody, Reason, Request,
    Source, Status, StepKind,
};
fn failure(source: Source) -> RequestError {
    RequestError::Source(SourceError {
        source,
        reason: Reason::BadReply,
        status: None,
        retry_after: None,
    })
}
fn empty(body: QueryBody) -> Outcome {
    super::empty(body)
}
fn spec(source: DrugSource) -> &'static biomcp_fetch::SourceSpec {
    if source == DrugSource::Chembl {
        &chembl::SPEC
    } else {
        &opentargets::SPEC
    }
}
pub(super) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let mut out = empty(QueryBody::DrugPage(DrugPage {
        items: Box::default(),
        total: None,
    }));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(spec(query.source()), deadline).fresh(cx.fresh);
    let result = search_page(&leg, query, &mut out.problems);
    let error = result.as_ref().err().copied();
    if let Ok((page, received)) = result {
        let returned = page.items.len() as u32;
        out.status = if !out.problems.is_empty() {
            Status::Partial
        } else if returned == 0 {
            Status::Absent
        } else {
            Status::Complete
        };
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
                        fresh: cx.fresh,
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
                    fresh: cx.fresh,
                },
            ));
        }
        out.body = QueryBody::DrugPage(page);
    }
    finish(&mut out, leg, query.source().source(), error);
    command.finish(deadline.remaining());
    out
}
fn search_page(
    leg: &Leg<'_>,
    query: &DrugSearch,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Result<(DrugPage, u32), RequestError> {
    let name = query
        .query()
        .ok_or(biomcp_fetch::InputError::Serialization)?;
    let source = query.source().source();
    let (items, total, received) = if query.source() == DrugSource::Chembl {
        let (page, _) = chembl::search(
            leg,
            name.as_str(),
            query.name_selector(),
            query.limit(),
            query.offset(),
        )?;
        problems.extend(page.problems);
        let received = page.received;
        (
            page.molecules
                .iter()
                .map(|row| identity::chembl_header(row, problems))
                .collect::<Result<Vec<_>, _>>(),
            page.page_meta.total_count,
            received,
        )
    } else {
        let (page, _) = opentargets::search(
            leg,
            name.as_str(),
            query.offset() / query.limit(),
            query.limit(),
        )?;
        let mapped = opentargets::search_headers(page, (query.offset() % query.limit()) as usize);
        problems.extend(mapped.problems);
        let received = mapped.value.items.len() as u32;
        return Ok((mapped.value, received));
    };
    Ok((
        DrugPage {
            items: items.map_err(|_| failure(source))?.into(),
            total: Some(total),
        },
        received,
    ))
}
pub(super) fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let mut out = empty(match query.sections() {
        [DrugSection::Targets] => QueryBody::DrugTargets(None),
        [DrugSection::Indications] => QueryBody::DrugIndications(None),
        [DrugSection::Xrefs] => QueryBody::DrugXrefs(None),
        _ => QueryBody::DrugCard(None),
    });
    if !matches!(
        query.sections(),
        [] | [DrugSection::Targets] | [DrugSection::Indications] | [DrugSection::Xrefs]
    ) || query.source() == DrugSource::Opentargets && query.sections() == [DrugSection::Xrefs]
    {
        return super::unsupported(query.source(), out);
    }
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(spec(query.source()), deadline).fresh(cx.fresh);
    let result = if query.source() == DrugSource::Chembl {
        sections::chembl(&leg, query, cx.fresh, &mut out)
    } else {
        sections::opentargets(&leg, query, cx.fresh, &mut out)
    };
    let error = result.err();
    if error.is_some() {
        out.status = Status::Failed;
    }
    finish(&mut out, leg, query.source().source(), error);
    command.finish(deadline.remaining());
    out
}
fn finish(out: &mut Outcome, leg: Leg<'_>, source: Source, error: Option<RequestError>) {
    out.local = error.and_then(status::local);
    let mut outcome = status::source(source, leg.finish(), error);
    if let Some(page) = &out.page {
        outcome.count = Some(page.total);
        outcome.fetched = Some(page.returned);
    }
    if (!out.problems.is_empty() || outcome.state == biomcp_types::SourceState::Stale)
        && out.status == Status::Complete
    {
        out.status = Status::Partial;
    }
    out.sources.push(outcome);
}
