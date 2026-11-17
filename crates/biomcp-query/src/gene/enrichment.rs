//! One g:Profiler analysis of a gene list, paged locally in the service's order.
use crate::{Ctx, Outcome, status};
use biomcp_adapters::gprofiler;
use biomcp_types::{
    GeneEnrichment, GeneEnrichmentQuery, ReplyBody, Request, Source, Status, StepKind,
};

/// Ask g:Profiler once and publish one page of its significant terms.
pub(crate) fn run(cx: &Ctx<'_>, query: &GeneEnrichmentQuery) -> Outcome {
    let command = cx
        .fetch
        .source_command_deadline(&gprofiler::SPEC, cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&gprofiler::SPEC, deadline).fresh(cx.fresh);
    let reply = gprofiler::enrichment(&leg, query);
    let error = reply.as_ref().err().copied();
    let mut out = status::empty(GeneEnrichment::into_reply(query.clone(), None));
    out.sources
        .push(status::source(Source::Gprofiler, leg.finish(), error));
    match reply {
        Ok(body) => publish(&mut out, query, body),
        Err(error) => out.local = status::local(error),
    }
    if out.status() == Status::Failed {
        out.next.push(crate::next::Step::run(
            StepKind::Fresh,
            &[Source::Gprofiler],
            Request::GeneEnrichment(query.clone()),
            true,
        ));
    }
    command.finish(deadline.remaining());
    out
}
/// Keep the requested page of every significant term and offer the next one.
fn publish(out: &mut Outcome, query: &GeneEnrichmentQuery, mut body: GeneEnrichment) {
    let total = body.items.len();
    let offset = usize::try_from(query.offset()).unwrap_or(usize::MAX);
    let limit = usize::try_from(query.limit()).unwrap_or(usize::MAX);
    let page: Box<[_]> = body
        .items
        .iter()
        .skip(offset)
        .take(limit)
        .cloned()
        .collect();
    let returned = u32::try_from(page.len()).unwrap_or(u32::MAX);
    let next = offset.saturating_add(page.len());
    body.items = page;
    out.page = Some(status::page_info(
        query.offset(),
        returned,
        u64::try_from(total).unwrap_or(u64::MAX),
    ));
    status::item(out, true, false, false);
    out.body = body.with_query(query.clone());
    if next < total {
        out.next.push(crate::next::Step::run(
            StepKind::Page,
            &[Source::Gprofiler],
            Request::GeneEnrichment(query.at_offset(u32::try_from(next).unwrap_or(u32::MAX))),
            false,
        ));
    }
}
