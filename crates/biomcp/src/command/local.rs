//! Bound local cache diagnostics once for every command surface.
use biomcp_fetch::RequestError;
use biomcp_types::{CacheProblem, ErrorEnvelope};

pub(super) fn kind(error: Option<RequestError>) -> Option<CacheProblem> {
    match error? {
        RequestError::Cache(error) => Some(error.into()),
        _ => None,
    }
}

pub(super) fn startup(error: crate::config::StartupError) -> std::io::Result<super::Outcome> {
    let mut envelope = ErrorEnvelope::local();
    envelope.local_error = error.0.map(Into::into);
    super::encode(envelope, crate::exit::Status::Local)
}
