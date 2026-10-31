//! Protein cards and source windows share CLI and MCP envelope encoding.
use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request};
use std::io;
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
        (_, QueryBody::PoolError { mismatch }) => super::pool_error(mismatch, meta.next),
        (Request::SearchProtein(q), QueryBody::ProteinPage(body)) => meta.encode(q, Some(body)),
        (Request::ProteinDomains(q), QueryBody::ProteinDomains(body)) => meta.encode(q, body),
        (Request::ProteinComplexes(q), QueryBody::ProteinComplexes(body)) => meta.encode(q, body),
        (Request::ProteinComplexMembers(q), QueryBody::ProteinComplexMembers(body)) => {
            meta.encode(q, body)
        }
        (Request::ProteinFeatures(q), QueryBody::ProteinFeatures(body)) => meta.encode(q, body),
        (Request::ProteinIsoforms(q), QueryBody::ProteinIsoforms(body)) => meta.encode(q, body),
        (Request::ProteinStructures(q), QueryBody::ProteinStructures(body)) => meta.encode(q, body),
        (Request::ProteinSequence(q), QueryBody::ProteinSequence(body)) => meta.encode(q, body),
        (Request::GetProtein(q), QueryBody::ProteinCard(body)) => meta.encode(q, Some(body)),
        _ => super::usage_envelope(biomcp_types::ErrorEnvelope::arguments()),
    }
}
