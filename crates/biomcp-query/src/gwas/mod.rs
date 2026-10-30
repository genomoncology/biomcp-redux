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
    let mut out = empty();
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&gwas::SPEC, deadline).fresh(cx.fresh);
    let result = gwas::search(&leg, q);
    let error = result.as_ref().err().copied();
    let counts = result
        .as_ref()
        .ok()
        .map(|page| (page.rows.len() as u32, page.total));
    if let Ok(page) = result {
        out.include_refusals(page.refusals);
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
        {
            let unavailable = false;
            crate::status::item(&mut out, true, false, unavailable);
        };
    }
    let mut source = status::source(Source::Gwas, leg.finish(), error);
    if let Some((accepted, total)) = counts {
        source.retrieved_count = Some(accepted);
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
    if out.status() == Status::Complete && source.state == SourceState::Stale {
        crate::status::item(&mut out, true, false, true);
    }
    let QueryBody::GwasPage(body) = &out.body else {
        return out;
    };
    out.next = next::search(q, body.as_ref(), source.state, cx.fresh);
    out.sources.push(source);
    status::search(&mut out);
    out.local = error.and_then(status::local);
    command.finish(deadline.remaining());
    out
}

fn empty() -> Outcome {
    crate::status::empty(QueryBody::GwasPage(None))
}
