//! Encode organization observations and typed narrowing refusals.
use super::{Outcome, ResponseMeta};
use biomcp_types::{ErrorEnvelope, OrganizationSearch, QueryBody, Status};
use std::io;
pub(super) fn encode(
    meta: ResponseMeta,
    q: OrganizationSearch,
    body: QueryBody,
) -> io::Result<Outcome> {
    match body {
        QueryBody::OrganizationPage(body) => {
            let body = (meta.status != Status::Failed).then_some(body);
            meta.encode(q, body)
        }
        QueryBody::OrganizationTooBroad => {
            super::usage_envelope(ErrorEnvelope::organization_too_broad())
        }
        QueryBody::PoolError { mismatch } => super::pool_error(mismatch, meta.next),
        _ => Err(io::Error::other("organization response mismatch")),
    }
}
