//! Exact reviewed-human protein discovery and cards; no clinical interpretation.
mod get;
mod next;
mod relations;
mod search;
mod sections;
mod sequence;
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

/// Page checked native feature rows on their own sequence identity.
pub fn features(cx: &crate::Ctx<'_>, q: &biomcp_types::ProteinFeaturesQuery) -> Outcome {
    relations::run(cx, relations::Selection::Features(q.clone()))
}
/// Page native alternative products without constructing their sequences.
pub fn isoforms(cx: &crate::Ctx<'_>, q: &biomcp_types::ProteinRelationQuery) -> Outcome {
    relations::run(cx, relations::Selection::Isoforms(q.clone()))
}
/// Page native structure references and source properties.
pub fn structures(cx: &crate::Ctx<'_>, q: &biomcp_types::ProteinRelationQuery) -> Outcome {
    relations::run(cx, relations::Selection::Structures(q.clone()))
}
/// Read a whole direct sequence under its exact source identity.
pub fn sequence(cx: &crate::Ctx<'_>, q: &biomcp_types::ProteinSequenceQuery) -> Outcome {
    sequence::run(cx, q)
}
