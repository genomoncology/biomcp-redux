use super::{Outcome, encode};
use biomcp_types::{Envelope, ErrorEnvelope, Status};
use std::io;

pub(super) fn clean(fetch: &biomcp_fetch::Fetch) -> io::Result<Outcome> {
    match biomcp_query::store::clean(fetch, crate::config::BUDGET) {
        Ok(counts) => encode(
            Envelope::new(Status::Complete, biomcp_types::EmptyQuery {}, Some(counts)),
            crate::exit::Status::Success,
        ),
        Err(_) => encode(ErrorEnvelope::local(), crate::exit::Status::Local),
    }
}

pub(super) fn health(fetch: &biomcp_fetch::Fetch) -> io::Result<Outcome> {
    match biomcp_query::health::run(fetch, crate::config::BUDGET) {
        Ok(result) => encode(
            Envelope::new(Status::Complete, biomcp_types::EmptyQuery {}, Some(result)),
            crate::exit::Status::Success,
        ),
        Err(_) => encode(ErrorEnvelope::local(), crate::exit::Status::Local),
    }
}
