use super::{Encoding, Outcome};
use biomcp_types::{Envelope, ErrorEnvelope, Status};
use std::io;

pub(super) fn clean(fetch: &biomcp_fetch::Fetch, encoding: Encoding) -> io::Result<Outcome> {
    match biomcp_query::store::clean(fetch, crate::config::BUDGET) {
        Ok(counts) => encoding.encode(
            Envelope::new(Status::Complete, biomcp_types::EmptyQuery {}, Some(counts)),
            crate::exit::Status::Success,
        ),
        Err(_) => encoding.encode(ErrorEnvelope::local(), crate::exit::Status::Local),
    }
}

pub(super) fn health(fetch: &biomcp_fetch::Fetch, encoding: Encoding) -> io::Result<Outcome> {
    match biomcp_query::health::run(fetch, crate::config::BUDGET) {
        Ok(result) => encoding.encode(
            Envelope::new(Status::Complete, biomcp_types::EmptyQuery {}, Some(result)),
            crate::exit::Status::Success,
        ),
        Err(_) => encoding.encode(ErrorEnvelope::local(), crate::exit::Status::Local),
    }
}

/// Resolve the source without opening the ordinary writable transport cache.
pub(super) fn scrub(destination: &std::path::Path, encoding: Encoding) -> io::Result<Outcome> {
    let source = crate::config::cache_path().ok().flatten();
    match source.ok_or(biomcp_fetch::CacheError::Filesystem).and_then(|source| {
        biomcp_query::store::scrub_copy(&source, destination, crate::config::BUDGET)
    }) {
        Ok(()) => encoding.encode(
            Envelope::new(Status::Complete, biomcp_types::EmptyQuery {}, Some(biomcp_types::StoreClean::default())),
            crate::exit::Status::Success,
        ),
        Err(_) => encoding.encode(ErrorEnvelope::local(), crate::exit::Status::Local),
    }
}
