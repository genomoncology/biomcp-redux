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
    let total = items.len() as u64;
    Ok(snapshot::Saved {
        body: PathwayPage {
            items: items.into(),
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
        problems: vec![],
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
    if let Ok(card) = reply {
        let absent = card.is_none();
        if card.is_some()
            && q.section().is_none()
            && let Ok(query) = q.with_section(PathwaySection::Summary)
        {
            out.next
                .push((StepKind::Section, run(Request::GetPathway(query))));
        }
        out.body = QueryBody::PathwayCard(card);
        out.sources
            .push(status::source(Source::WikiPathways, leg.finish(), error));
        out.status = overall(&out, absent);
    } else {
        out.sources
            .push(status::source(Source::WikiPathways, leg.finish(), error));
    }
    out.local = error.and_then(status::local);
    sync_step(&mut out);
    command.finish(deadline.remaining());
    out
}
