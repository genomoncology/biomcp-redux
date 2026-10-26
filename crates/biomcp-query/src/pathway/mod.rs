//! Human source-owned pathway discovery and exact identity retrieval.
mod gene;
mod get;
mod search;
mod snapshot;
use crate::{Action, Outcome};
use biomcp_types::{QueryBody, Request, Status};
pub use get::get;
pub use search::search;
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
fn run(request: Request) -> Action {
    Action::Run {
        request,
        fresh: false,
    }
}
fn overall(out: &Outcome, absent: bool) -> Status {
    if !out.problems.is_empty()
        || out.sources.iter().any(|s| {
            matches!(
                s.state,
                biomcp_types::SourceState::Stale | biomcp_types::SourceState::Failed
            )
        })
    {
        Status::Partial
    } else if absent {
        Status::Absent
    } else {
        Status::Complete
    }
}

/// Read retained native pathway membership.
pub fn genes(_: &crate::Ctx<'_>, _: &biomcp_types::PathwayGenesQuery) -> Outcome {
    empty(QueryBody::PathwayGenes(None))
}
/// Read retained native contained events.
pub fn events(_: &crate::Ctx<'_>, _: &biomcp_types::PathwayEventsQuery) -> Outcome {
    empty(QueryBody::PathwayEvents(None))
}
