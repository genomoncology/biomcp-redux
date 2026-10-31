//! Literal source discovery and exact cell identity; no name-based joins.
mod all;
mod chembl;
mod get;
pub use chembl::assay_count;
mod provenance;
mod sections;
use crate::{Action, Ctx, Outcome, status};
use biomcp_adapters::cellosaurus;
use biomcp_types::{
    CellLineGet, CellLinePage, CellLineSearch, QueryBody, Request, Source, Status, StepKind,
};
/// One native window, with no fabricated total or extra-page fetch.
pub fn search(cx: &Ctx<'_>, query: &CellLineSearch) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&cellosaurus::SPEC, deadline).fresh(cx.fresh);
    let reply = cellosaurus::search(&leg, query);
    let error = reply.as_ref().err().copied();
    let mut out = empty(QueryBody::CellLinePage(CellLinePage {
        items: Box::default(),
        release: None,
        fetched: None,
    }));
    out.sources
        .push(status::source(Source::Cellosaurus, leg.finish(), error));
    out.local = error.and_then(status::local);
    if let Ok(page) = reply {
        out.include_refusals(page.refusals);
        out.problems.extend(page.problems);
        let full = page.received == query.limit();
        for row in &page.items {
            out.next.push(crate::next::Step::new(
                StepKind::Disambiguate,
                &[biomcp_types::Source::Cellosaurus],
                run(Request::GetCellLine(CellLineGet::accession(
                    row.accession.clone(),
                ))),
            ));
        }
        if full && let Some(offset) = query.offset().checked_add(query.limit()) {
            out.next.push(crate::next::Step::new(
                StepKind::Page,
                &[biomcp_types::Source::Cellosaurus],
                run(Request::SearchCellLine(query.at_offset(offset))),
            ));
        }
        status::search(&mut out);
        out.body = QueryBody::CellLinePage(CellLinePage {
            items: page.items.into(),
            release: None,
            fetched: page
                .metadata
                .fetched
                .and_then(|value| u64::try_from(value).ok()),
        });
        provenance::attach(cx, deadline, &mut out);
    }
    recovery(&mut out, Request::SearchCellLine(query.clone()));
    command.finish(deadline.remaining());
    out
}
/// Retrieve one verified identity, with unselected candidates on ambiguity.
pub fn get(cx: &Ctx<'_>, query: &CellLineGet) -> Outcome {
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
        out.next.push(crate::next::Step::new(
            StepKind::Fresh,
            &[biomcp_types::Source::Cellosaurus],
            Action::Run {
                request,
                fresh: true,
            },
        ));
    }
}
