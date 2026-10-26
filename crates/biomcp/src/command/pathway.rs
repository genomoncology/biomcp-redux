use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request, Status};
use std::io;
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
        (Request::SearchPathway(q), QueryBody::PathwayPage(body)) => {
            let body = (meta.status != Status::Failed).then_some(body);
            meta.encode(q, body)
        }
        (Request::GetPathway(q), QueryBody::PathwayCard(body)) => meta.encode(q, body),
        _ => Err(io::Error::other("pathway response mismatch")),
    }
}
