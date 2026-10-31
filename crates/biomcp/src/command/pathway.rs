use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request};
use std::io;
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
        (_, QueryBody::Usage(biomcp_types::UsageError::PoolError { mismatch })) => super::pool_error(mismatch, meta.next),
        (Request::SearchPathway(q), QueryBody::PathwayPage(body)) => {
            let body = Some(body);
            meta.encode(q, body)
        }
        (Request::GetPathway(q), QueryBody::PathwayCard(body)) => meta.encode(q, body),
        (Request::PathwayGenes(q), QueryBody::PathwayGenes(body)) => meta.encode(q, body),
        (Request::PathwayEvents(q), QueryBody::PathwayEvents(body)) => meta.encode(q, body),
        _ => Err(io::Error::other("pathway response mismatch")),
    }
}
