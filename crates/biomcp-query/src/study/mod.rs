//! Literal study catalog reads with checked source admission and fixed continuation state.
mod page;
use crate::{Action, Bundle, Ctx, Outcome, status};
use biomcp_adapters::cbioportal::{SPEC, studies};
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{
    QueryBody, Reason, Source, SourceState, Status, StepKind, StudyCard, StudyGet, StudySearch,
};
fn empty(body: QueryBody) -> Outcome {
    Outcome {
        status: Status::Failed,
        body,
        page: None,
        sources: vec![],
        problems: vec![],
        next: vec![],
        local: None,
    }
}
fn finish(out: &mut Outcome, leg: Leg<'_>, error: Option<RequestError>) {
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::Cbioportal, leg.finish(), error));
    if error.is_some() {
        out.status = Status::Failed
    } else if out.sources.iter().any(|s| s.state == SourceState::Stale) {
        out.status = Status::Partial
    }
    if out
        .sources
        .iter()
        .any(|s| matches!(s.state, SourceState::Stale | SourceState::Syncing))
    {
        out.next.push((
            StepKind::Sync,
            Action::Sync {
                bundle: Bundle::CbioportalStudies,
            },
        ));
    }
}
/// Search only the last accepted metadata catalog; no implicit refresh.
pub fn search(cx: &Ctx<'_>, q: &StudySearch) -> Outcome {
    let mut out = empty(QueryBody::StudyPage(None));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&SPEC, deadline);
    let result = studies::search(&leg, q)
        .and_then(|(rows, info)| page::search(cx, q, rows, info, deadline, &mut out));
    let error = result.err();
    finish(&mut out, leg, error);
    command.finish(deadline.remaining());
    out
}
/// Registered exact study card; live sections belong to their following slice.
pub fn get(cx: &Ctx<'_>, q: &StudyGet) -> Outcome {
    let mut out = empty(QueryBody::StudyCard(None));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&SPEC, deadline);
    let result = if q.section().is_some() {
        Err(failure(Reason::Unsupported))
    } else {
        studies::card(&leg, q.id()).and_then(|(row, info)| {
            let snapshot = studies::snapshot(&info)?;
            out.status = if row.is_some() {
                Status::Complete
            } else {
                Status::Absent
            };
            out.body = QueryBody::StudyCard(row.map(|study| StudyCard {
                study,
                snapshot,
                profiles: None,
                sample_lists: None,
                files: None,
            }));
            Ok(())
        })
    };
    finish(&mut out, leg, result.err());
    command.finish(deadline.remaining());
    out
}
fn failure(reason: Reason) -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::Cbioportal,
        reason,
        status: None,
        retry_after: None,
    }
    .into()
}
