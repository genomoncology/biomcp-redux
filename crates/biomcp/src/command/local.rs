//! Bound local cache diagnostics once for every command surface.
use biomcp_fetch::{CacheError, RequestError};
use biomcp_types::{CacheProblem, ErrorEnvelope};

pub(super) fn kind(error: Option<RequestError>) -> Option<CacheProblem> {
    match error? {
        RequestError::Cache(error) => Some(cache(error)),
        _ => None,
    }
}

fn cache(error: CacheError) -> CacheProblem {
    match error {
        CacheError::Busy => CacheProblem::Busy,
        CacheError::Database => CacheProblem::Database,
        CacheError::Version => CacheProblem::Version,
    }
}

pub(super) fn startup(error: crate::config::StartupError) -> std::io::Result<super::Outcome> {
    let mut envelope = ErrorEnvelope::local();
    envelope.local_error = error.0.map(cache);
    super::encode(envelope, crate::exit::Status::Local)
}
