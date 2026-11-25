use super::{Encoding, Outcome};
use biomcp_types::{Envelope, ErrorEnvelope, Status};
use std::io;

pub(super) fn clean(fetch: &biomcp_fetch::Fetch, encoding: Encoding) -> io::Result<Outcome> {
    clean_result(
        biomcp_query::store::clean(fetch, crate::config::BUDGET),
        encoding,
    )
}

pub(super) fn clean_without_active(
    path: &std::path::Path,
    encoding: Encoding,
) -> io::Result<Outcome> {
    clean_result(biomcp_query::store::clean_without_active(path), encoding)
}

fn clean_result(
    result: Result<biomcp_types::StoreClean, biomcp_fetch::CacheError>,
    encoding: Encoding,
) -> io::Result<Outcome> {
    match result {
        Ok(counts) => encoding.encode(
            Envelope::new(Status::Complete, biomcp_types::EmptyQuery {}, Some(counts)),
            crate::exit::Status::Success,
        ),
        Err(_) => encoding.encode(ErrorEnvelope::local(), crate::exit::Status::Local),
    }
}

pub(super) fn health(
    fetch: &biomcp_fetch::Fetch,
    budget: std::time::Duration,
    encoding: Encoding,
) -> io::Result<Outcome> {
    match biomcp_query::health::run(fetch, budget) {
        Ok(result) => encoding.encode(
            Envelope::new(Status::Complete, biomcp_types::EmptyQuery {}, Some(result)),
            crate::exit::Status::Success,
        ),
        Err(_) => encoding.encode(ErrorEnvelope::local(), crate::exit::Status::Local),
    }
}
