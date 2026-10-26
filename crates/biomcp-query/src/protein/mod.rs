//! Exact reviewed-human protein discovery and cards; no clinical interpretation.
mod get;
mod next;
mod search;
use crate::Outcome;
use biomcp_types::{QueryBody, Status};
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
