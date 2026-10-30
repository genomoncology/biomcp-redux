use super::{empty, overall, run, snapshot};
use crate::{Action, Bundle, Ctx, Outcome, status};
use biodata::PathwayId;
use biomcp_adapters::wikipathways;
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{
    PathwayGet, PathwayPage, PathwayPageInfo, PathwaySearch, PathwaySection, PathwayTotalKind,
    QueryBody, Request, Source, SourceState, StepKind,
};

pub(super) fn sync_step(out: &mut Outcome) {
    if out.sources.iter().any(|s| {
        s.source == Source::WikiPathways
            && matches!(s.state, SourceState::Stale | SourceState::Syncing)
    }) {
        out.next.push((
            StepKind::Sync,
            Action::Sync {
                bundle: Bundle::WikiPathways,
            },
        ));
    }
}
pub(super) fn headers(
    cx: &Ctx<'_>,
    q: &PathwaySearch,
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Result<snapshot::Saved, RequestError> {
    let leg = cx.fetch.leg(&wikipathways::SPEC, deadline);
    let reply = wikipathways::search(&leg, q.query().map_or("", |v| v.as_str()));
    let error = reply.as_ref().err().copied();
    out.sources
        .push(status::source(Source::WikiPathways, leg.finish(), error));
    let (items, info) = reply?;
    let total = items.value.len() as u64;
    Ok(snapshot::Saved {
        body: PathwayPage {
            items: items.value.into(),
            page: PathwayPageInfo {
                offset: 0,
                returned: total as u32,
                total,
                total_kind: PathwayTotalKind::Observed,
                next_offset: None,
                pool: None,
            },
        },
        complete: true,
        catalog_revision: info.content_revision.and_then(|s| s.parse().ok()),
        sources: out.sources.clone(),
        problems: items.problems,
    })
}
/// Exact stored human WikiPathways metadata, without an implicit refresh.
pub(super) fn get(cx: &Ctx<'_>, q: &PathwayGet) -> Outcome {
    let mut out = empty(QueryBody::PathwayCard(None));
    let PathwayId::Wikipathways(id) = q.id() else {
        return out;
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&wikipathways::SPEC, deadline);
    let reply = wikipathways::card(
        &leg,
        id,
        matches!(
            q.section(),
            Some(PathwaySection::Summary | PathwaySection::All)
        ),
    );
    let error = reply.as_ref().err().copied();
    out.sources
        .push(status::source(Source::WikiPathways, leg.finish(), error));
    if let Ok(mapped) = reply {
        out.problems.extend(mapped.problems);
        let mut card = mapped.value;
        let absent = card.is_none();
        if let Some(value) = &mut card {
            super::get_sections::attach(cx, q, value, deadline, &mut out);
            for section in &value.sections {
                if q.section() != Some(PathwaySection::All)
                    && Some(*section) != q.section()
                    && let Ok(query) = q.with_section(*section)
                {
                    out.next
                        .push((StepKind::Section, run(Request::GetPathway(query))));
                }
            }
        }
        if !matches!(out.body, QueryBody::PoolError { .. }) {
            out.body = QueryBody::PathwayCard(card);
            out.status = overall(&out, absent);
        }
    }
    out.local = out.local.or_else(|| error.and_then(status::local));
    sync_step(&mut out);
    command.finish(deadline.remaining());
    out
}
