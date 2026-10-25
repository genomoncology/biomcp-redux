//! Explicit diagnostic source ownership; reads consume only complete local snapshots.
mod get;
mod next;
use crate::{Action, Bundle, Ctx, Outcome, status};
use biomcp_adapters::gtr;
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{
    DiagnosticPage, DiagnosticSearch, DiagnosticSearchSource, PageInfo, QueryBody, Request, Source,
    SourceState, Status, StepKind,
};
pub use get::get;
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
fn finish(out: &mut Outcome, leg: Leg<'_>, error: Option<RequestError>) {
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::Gtr, leg.finish(), error));
    if error.is_some() {
        out.status = Status::Failed;
    } else if out.sources.iter().any(|s| s.state == SourceState::Stale)
        && out.status == Status::Complete
    {
        out.status = Status::Partial;
    }
    if out
        .sources
        .iter()
        .any(|s| matches!(s.state, SourceState::Syncing | SourceState::Stale))
    {
        out.next.push((
            StepKind::Sync,
            Action::Sync {
                bundle: Bundle::Gtr,
            },
        ));
    }
}
/// Search only stored source-native GTR relations and retain matched selectors.
pub fn search(cx: &Ctx<'_>, q: &DiagnosticSearch) -> Outcome {
    let mut out = empty(QueryBody::DiagnosticPage(DiagnosticPage {
        items: Box::default(),
        count_unit: Some(biomcp_types::DiagnosticCountUnit::TestIdentities),
        distinct_id_total: None,
    }));
    if q.source() != DiagnosticSearchSource::Gtr {
        return unsupported(Source::FdaCdx, out);
    }
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&gtr::SPEC, deadline);
    let result = gtr::search(&leg, q);
    let error = result.as_ref().err().copied();
    if let Ok(page) = result {
        out.status = Status::Complete;
        let q = page
            .info
            .content_revision
            .as_deref()
            .and_then(|v| v.parse().ok())
            .map_or_else(|| q.clone(), |v| q.with_bundle_revision(v));
        next::candidates(&mut out, &q, &page.items, cx.fresh);
        let returned = page.items.len() as u32;
        if u64::from(q.offset()) + u64::from(returned) < page.total {
            next::push(
                &mut out,
                StepKind::Page,
                Request::SearchDiagnostic(q.at_offset(q.offset() + returned)),
                cx.fresh,
            );
        }
        out.page = Some(PageInfo {
            offset: q.offset(),
            returned,
            total: page.total,
            pool: None,
            truncated: None,
        });
        out.body = QueryBody::DiagnosticPage(DiagnosticPage {
            items: page.items,
            count_unit: Some(biomcp_types::DiagnosticCountUnit::TestIdentities),
            distinct_id_total: None,
        });
    }
    finish(&mut out, leg, error);
    command.finish(deadline.remaining());
    out
}
fn unsupported(source: Source, mut out: Outcome) -> Outcome {
    out.sources.push(biomcp_adapters::registry::outcome(
        source,
        SourceState::Skipped,
        Some(biomcp_types::Reason::Unsupported),
    ));
    out
}
