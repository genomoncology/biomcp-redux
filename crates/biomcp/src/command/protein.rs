//! Protein cards and source windows share CLI and MCP envelope encoding.
use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request, Status};
use std::io;
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
        (_, QueryBody::PoolError { mismatch }) => super::pool_error(mismatch, meta.next),
        (Request::SearchProtein(q), QueryBody::ProteinPage(body)) => {
            let present = meta.status != Status::Failed;
            meta.encode(q, present.then_some(body))
        }
        (Request::GetProtein(q), QueryBody::ProteinCard(body)) => meta.encode(q, Some(body)),
        _ => super::usage_envelope(biomcp_types::ErrorEnvelope::arguments()),
    }
}
