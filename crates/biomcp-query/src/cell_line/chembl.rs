//! ChEMBL metadata and independent assay-count disclosure.
use super::{
    CellLineGet, Ctx, Outcome, QueryBody, Request, Source, Status, StepKind, empty, recovery, run,
    status,
};
use biodata::CellosaurusAccession;
use biomcp_adapters::chembl;
use biomcp_types::{
    Capped, CellLineAssayCount, CellLineAssayCountQuery, CellLineChembl, Problem, ProblemCode,
};
/// One native metadata page, never an assay request.
pub(super) fn get(cx: &Ctx<'_>, query: &CellLineGet, accession: &CellosaurusAccession) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let out = get_at(cx, query, accession, deadline);
    command.finish(deadline.remaining());
    out
}
pub(super) fn get_at(
    cx: &Ctx<'_>,
    query: &CellLineGet,
    accession: &CellosaurusAccession,
    deadline: biomcp_fetch::Deadline<'_>,
) -> Outcome {
    let leg = cx.fetch.leg(&chembl::SPEC, deadline).fresh(cx.fresh);
    let mut body = CellLineChembl {
        accession: accession.clone(),
        items: Capped {
            items: Box::default(),
            total: 0,
        },
        offset: query.section_offset(),
        release: None,
        fetched: None,
    };
    let limit = query.section_limit().unwrap_or(5);
    let reply = chembl::cells(&leg, accession, limit, body.offset);
    let error = reply.as_ref().err().copied();
    let mut out = empty(QueryBody::CellLineChembl(body.clone()));
    out.sources
        .push(status::source(Source::Chembl, leg.finish(), error));
    out.local = error.and_then(status::local);
    if let Ok((items, total, metadata)) = reply {
        let end = u64::from(body.offset) + items.len() as u64;
        out.status = if total == 0 {
            Status::Absent
        } else if end < total {
            Status::Partial
        } else {
            Status::Complete
        };
        if end < total
            && let Some(offset) = body.offset.checked_add(limit)
        {
            out.next.push((
                StepKind::Page,
                run(Request::GetCellLine(query.at_section_offset(offset))),
            ));
        }
        for row in &items {
            out.next.push((
                StepKind::Section,
                run(Request::CellLineAssayCount(CellLineAssayCountQuery {
                    id: row.id.clone(),
                })),
            ));
        }
        body.items = Capped {
            items: items.into(),
            total,
        };
        body.fetched = metadata.fetched.and_then(|value| u64::try_from(value).ok());
        out.body = QueryBody::CellLineChembl(body);
        attach(cx, deadline, &mut out);
    } else if error.is_some() {
        out.problems.push(problem("cellosaurus_id"));
    }
    recovery(&mut out, Request::GetCellLine(query.clone()));
    out
}
/// One exact assay request with its native total, including zero.
pub fn assay_count(cx: &Ctx<'_>, query: &CellLineAssayCountQuery) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&chembl::SPEC, deadline).fresh(cx.fresh);
    let reply = chembl::assay_count(&leg, &query.id);
    let error = reply.as_ref().err().copied();
    let mut out = empty(QueryBody::CellLineAssayCount(CellLineAssayCount {
        id: query.id.clone(),
        assay_count: 0,
        release: None,
        fetched: None,
    }));
    out.sources
        .push(status::source(Source::Chembl, leg.finish(), error));
    out.local = error.and_then(status::local);
    if let Ok((assay_count, metadata)) = reply {
        out.status = Status::Complete;
        out.body = QueryBody::CellLineAssayCount(CellLineAssayCount {
            id: query.id.clone(),
            assay_count,
            release: None,
            fetched: metadata.fetched.and_then(|value| u64::try_from(value).ok()),
        });
        attach(cx, deadline, &mut out);
    } else if error.is_some() {
        out.problems.push(problem("cell_chembl_id"));
    }
    recovery(&mut out, Request::CellLineAssayCount(query.clone()));
    command.finish(deadline.remaining());
    out
}
fn problem(field: &str) -> Problem {
    Problem {
        source: Source::Chembl,
        record: None,
        field: field.into(),
        code: ProblemCode::ValueUnknown,
    }
}
fn attach(cx: &Ctx<'_>, deadline: biomcp_fetch::Deadline<'_>, out: &mut Outcome) {
    let leg = cx.fetch.leg(&chembl::SPEC, deadline).fresh(cx.fresh);
    let release = chembl::release(&leg);
    let error = release.as_ref().err().copied();
    let incoming = status::source(Source::Chembl, leg.finish(), error);
    if let Some(source) = out.sources.last_mut() {
        status::retain_source_state(source, &incoming);
    }
    if error.is_some() {
        out.status = Status::Partial;
        out.local = error.and_then(status::local);
        out.problems.push(problem("release"));
    }
    match &mut out.body {
        QueryBody::CellLineChembl(body) => body.release = release.ok(),
        QueryBody::CellLineAssayCount(body) => body.release = release.ok(),
        _ => {}
    }
    if out
        .sources
        .iter()
        .any(|s| s.state == biomcp_types::SourceState::Stale)
    {
        out.status = Status::Partial;
    }
}
