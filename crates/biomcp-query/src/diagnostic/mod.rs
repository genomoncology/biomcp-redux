//! Explicit diagnostic source ownership; reads consume only complete local snapshots.
mod clearance;
mod companion;
mod get;
mod next;
mod pma;
mod recall;
mod who;
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
    finish_bundle(out, leg, error, Source::Gtr, Bundle::Gtr);
}
fn finish_bundle(
    out: &mut Outcome,
    leg: Leg<'_>,
    error: Option<RequestError>,
    source: Source,
    bundle: Bundle,
) {
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(source, leg.finish(), error));
    if error.is_some() {
        out.status = Status::Failed;
    } else if (!out.problems.is_empty()
        || out.sources.iter().any(|s| s.state == SourceState::Stale))
        && out.status == Status::Complete
    {
        out.status = Status::Partial;
    }
    if out
        .sources
        .iter()
        .any(|s| matches!(s.state, SourceState::Syncing | SourceState::Stale))
        && !out
            .next
            .iter()
            .any(|(_, action)| action == &Action::Sync { bundle })
    {
        out.next.push((StepKind::Sync, Action::Sync { bundle }));
    }
}
/// Search only stored source-native GTR relations and retain matched selectors.
pub fn search(cx: &Ctx<'_>, q: &DiagnosticSearch) -> Outcome {
    if q.source() == DiagnosticSearchSource::FdaDeviceRecall {
        return recall::search(cx, q);
    }
    if q.source() == DiagnosticSearchSource::FdaCdx {
        return companion::search(cx, q);
    }
    let (spec, source, bundle, count) = search_source(q.source());
    let who = q.source() == DiagnosticSearchSource::WhoIvd;
    let mut out = empty(QueryBody::DiagnosticPage(DiagnosticPage {
        items: Box::default(),
        count_unit: Some(count),
        distinct_id_total: None,
    }));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(spec, deadline);
    let result = if who {
        biomcp_adapters::who_ivd::search(&leg, q).map(|p| (p.items, p.total, p.info))
    } else {
        gtr::search(&leg, q).map(|p| {
            status::append(&mut out.problems, p.problems);
            (p.items, p.total, p.info)
        })
    };
    let error = result.as_ref().err().copied();
    if let Ok((items, total, info)) = result {
        out.status = Status::Complete;
        let q = info
            .content_revision
            .as_deref()
            .and_then(|v| v.parse().ok())
            .map_or_else(|| q.clone(), |v| q.with_bundle_revision(v));
        next::candidates(&mut out, &q, &items, cx.fresh);
        let returned = items.len() as u32;
        if u64::from(q.offset()) + u64::from(returned) < total {
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
            total,
            pool: None,
            truncated: None,
        });
        out.body = QueryBody::DiagnosticPage(DiagnosticPage {
            items,
            count_unit: Some(count),
            distinct_id_total: None,
        });
    }
    finish_bundle(&mut out, leg, error, source, bundle);
    command.finish(deadline.remaining());
    out
}
fn search_source(
    source: DiagnosticSearchSource,
) -> (
    &'static biomcp_fetch::SourceSpec,
    Source,
    Bundle,
    biomcp_types::DiagnosticCountUnit,
) {
    if source == DiagnosticSearchSource::WhoIvd {
        (
            &biomcp_adapters::who_ivd::SPEC,
            Source::WhoIvd,
            Bundle::WhoIvd,
            biomcp_types::DiagnosticCountUnit::ProductIdentities,
        )
    } else {
        (
            &gtr::SPEC,
            Source::Gtr,
            Bundle::Gtr,
            biomcp_types::DiagnosticCountUnit::TestIdentities,
        )
    }
}
fn unsupported(source: Source, mut out: Outcome) -> Outcome {
    out.sources.push(biomcp_adapters::registry::outcome(
        source,
        SourceState::Skipped,
        Some(biomcp_types::Reason::Unsupported),
    ));
    out
}
