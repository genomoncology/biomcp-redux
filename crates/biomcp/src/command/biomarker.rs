//! Encode native biomarker concepts and bounded source refusals.
use super::{Outcome, ResponseMeta};
use biomcp_types::{BiomarkerGet, BiomarkerSearch, ErrorEnvelope, QueryBody};
use std::io;
pub(super) fn search(
    meta: ResponseMeta,
    q: BiomarkerSearch,
    body: QueryBody,
) -> io::Result<Outcome> {
    match body {
        QueryBody::BiomarkerPage(body) => {
            let body = Some(body);
            meta.encode(q, body)
        }
        QueryBody::BiomarkerTooBroad => super::usage_envelope(ErrorEnvelope::biomarker_too_broad()),
        QueryBody::PoolError { mismatch } => super::pool_error(mismatch, meta.next),
        _ => Err(io::Error::other("biomarker response mismatch")),
    }
}
pub(super) fn get(meta: ResponseMeta, q: BiomarkerGet, body: QueryBody) -> io::Result<Outcome> {
    match body {
        QueryBody::BiomarkerResult(body) => {
            let body = Some(body);
            meta.encode(q, body)
        }
        QueryBody::BiomarkerTooBroad => super::usage_envelope(ErrorEnvelope::biomarker_too_broad()),
        _ => Err(io::Error::other("biomarker response mismatch")),
    }
}
