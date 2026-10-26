//! Human source-owned pathway discovery and exact identity retrieval.
mod gene;
mod get;
mod get_sections;
mod gpml_membership;
mod gpml_relation;
mod membership;
mod relation;
mod relation_snapshot;
mod relation_steps;
mod search;
mod snapshot;
mod union;
mod wikipathways;
use crate::{Action, Outcome};
use biomcp_types::{QueryBody, Request, Status};
pub use get::get;
pub use relation::{events, genes};
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
