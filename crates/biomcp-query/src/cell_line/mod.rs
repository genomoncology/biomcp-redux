//! Literal source discovery and exact cell identity; no name-based joins.
use biomcp_types::Paged;
use biomcp_types::ReplyBody;
mod all;
mod chembl;
mod get;
pub(crate) use chembl::assay_count;
mod provenance;
mod sections;
use crate::{Ctx, Outcome, status};
use biomcp_adapters::cellosaurus;
use biomcp_types::{
    CellLineFilteredPage, CellLineGet, CellLinePage, CellLineSearchCommand, ReplyData, Request,
    Source, Status, StepKind,
};
/// One native window; a full one also reads Cellosaurus's own count, never an estimate.
pub(crate) fn search(cx: &Ctx<'_>, query: &CellLineSearchCommand) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&cellosaurus::SPEC, deadline).fresh(cx.fresh);
    let reply = cellosaurus::search(&leg, query);
    let error = reply.as_ref().err().copied();
    // Only a full window can leave rows unread; a short one ends the source's matches.
    let full = reply
        .as_ref()
        .is_ok_and(|page| page.received == query.limit());
    let count = full.then(|| cellosaurus::count(&leg, query));
    let mut out = empty(search_body(
        CellLinePage {
            items: Box::default(),
            variation_match: None,
            release: None,
            fetched: None,
        },
        query,
    ));
    out.sources
        .push(status::source(Source::Cellosaurus, leg.finish(), error));
    out.local = error.and_then(status::local);
    if let Ok(page) = reply {
        out.include_refusals(page.refusals);
        out.problems.extend(page.problems);
        for row in &page.items {
            out.next.push(crate::next::Step::new(
                StepKind::Disambiguate,
                &[biomcp_types::Source::Cellosaurus],
                run(Request::GetCellLine(CellLineGet::accession(
                    row.accession.clone(),
                ))),
            ));
        }
        let read = u64::from(query.offset()) + u64::from(page.received);
        let total = report_total(&mut out, query, read, page.items.len(), count);
        if full
            && total.is_none_or(|total| read < total)
            && let Some(offset) = query.offset().checked_add(query.limit())
        {
            out.next.push(crate::next::Step::new(
                StepKind::Page,
                &[biomcp_types::Source::Cellosaurus],
                run(Request::SearchCellLine(query.at_offset(offset))),
            ));
        }
        status::search(&mut out);
        out.body = search_body(
            CellLinePage {
                items: page.items.into(),
                variation_match: query.variation_match(),
                release: None,
                fetched: page
                    .metadata
                    .fetched
                    .and_then(|value| u64::try_from(value).ok()),
            },
            query,
        );
        provenance::attach(cx, deadline, &mut out);
    }
    recovery(&mut out, Request::SearchCellLine(query.clone()));
    command.finish(deadline.remaining());
    out
}
/// Cellosaurus's own count on a full page, or the rows read to the end of a short one.
/// A failed count leaves no total and says so; the page step still follows a full page.
fn report_total(
    out: &mut Outcome,
    query: &CellLineSearchCommand,
    read: u64,
    returned: usize,
    count: Option<Result<u64, biomcp_fetch::RequestError>>,
) -> Option<u64> {
    let source = out
        .sources
        .iter_mut()
        .find(|s| s.source == Source::Cellosaurus);
    let total = match (count, source) {
        (None, _) => Some(read),
        (Some(Ok(found)), source) => {
            if let Some(source) = source {
                source.count = Some(found);
            }
            Some(found)
        }
        (Some(Err(_)), source) => {
            if let Some(source) = source {
                source
                    .notices
                    .push(biomcp_types::SourceNotice::RequestGuidance {
                        data_date: None,
                        message: "BioMCP could not read Cellosaurus's match count, so this \
                            page has no total; a full page has a next page step."
                            .into(),
                        retry_after_seconds: None,
                    });
            }
            None
        }
    };
    if let Some(total) = total {
        out.page = Some(biomcp_types::PageInfo {
            offset: query.offset(),
            returned: u32::try_from(returned).unwrap_or(u32::MAX),
            total,
            pool: None,
            truncated: None,
        });
    }
    total
}
/// Retrieve one verified identity, with unselected candidates on ambiguity.
pub(crate) fn get(cx: &Ctx<'_>, query: &CellLineGet) -> Outcome {
    get::get(cx, query)
}
use crate::next::run;
pub(super) use crate::status::empty;
fn recovery(out: &mut Outcome, request: Request) {
    if out.sources.iter().any(|s| {
        matches!(
            s.state,
            biomcp_types::SourceState::Failed | biomcp_types::SourceState::Stale
        )
    }) {
        out.next.push(crate::next::Step::run(
            StepKind::Fresh,
            &[biomcp_types::Source::Cellosaurus],
            request,
            true,
        ));
    }
}

fn search_body(page: CellLinePage, query: &CellLineSearchCommand) -> ReplyData {
    if let Some(literal) = query.unfiltered() {
        page.with_query(literal)
    } else {
        CellLineFilteredPage(page).with_query(query.clone())
    }
}
