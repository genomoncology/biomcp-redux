//! Source-local association search, without clinical interpretation.
mod get;
pub(crate) mod map;
pub use get::get;
mod next;
mod threshold;
use crate::{Ctx, Outcome, status};
use biomcp_adapters::gwas;
use biomcp_types::{
    GwasPage, GwasPageQuery, GwasSearch, PageInfo, Problem, ProblemCode, QueryBody, Source,
    SourceState, Status,
};
/// One bounded native page; no per-hit metadata calls.
pub fn search(cx: &Ctx<'_>, q: &GwasSearch) -> Outcome {
    if q.p_value().is_some() {
        return threshold::search(cx, q);
    }
    let mut out = Outcome {
        status: Status::Failed,
        body: QueryBody::GwasPage(None),
        page: None,
        sources: vec![],
        problems: vec![],
        next: vec![],
        local: None,
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&gwas::SPEC, deadline).fresh(cx.fresh);
    let result = gwas::search(&leg, q);
    let error = result.as_ref().err().copied();
    let counts = result.as_ref().ok().map(|page| (page.received, page.total));
    if let Ok(page) = result {
        out.problems.extend(page.problems);
        let items: Box<[_]> = page
            .rows
            .into_iter()
            .map(|row| map::association(row, &mut out.problems))
            .collect();
        let returned = items.len() as u32;
        let continuation =
            (u64::from(q.offset() - q.offset() % q.limit()) + u64::from(page.received)
                < page.total)
                .then(|| {
                    GwasPageQuery::Search(q.at_offset(
                        (q.offset() - q.offset() % q.limit()).saturating_add(page.received),
                    ))
                });
        let page = PageInfo {
            offset: q.offset(),
            returned,
            total: page.total,
            pool: None,
            truncated: None,
        };
        out.page = Some(page.clone());
        out.body = QueryBody::GwasPage(Some(GwasPage {
            items,
            page,
            continuation,
            qualifying_total: None,
        }));
        out.status = if out.problems.is_empty() {
            Status::Complete
        } else {
            Status::Partial
        };
    }
    let mut source = status::source(Source::Gwas, leg.finish(), error);
    if let Some((received, total)) = counts {
        source.retrieved_count = Some(received);
        source.count = Some(total);
    }
    if source.reason == Some(biomcp_types::Reason::BadReply) {
        out.problems.push(Problem {
            source: Source::Gwas,
            record: None,
            field: "association_page".into(),
            code: ProblemCode::ValueUnknown,
        });
    }
    if out.status == Status::Complete && source.state == SourceState::Stale {
        out.status = Status::Partial;
    }
    let QueryBody::GwasPage(body) = &out.body else {
        return out;
    };
    out.next = next::search(q, body.as_ref(), source.state, cx.fresh);
    out.sources.push(source);
    out.local = error.and_then(status::local);
    command.finish(deadline.remaining());
    out
}
