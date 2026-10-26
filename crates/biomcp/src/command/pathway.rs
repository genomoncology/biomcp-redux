use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request, Status};
use std::io;
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
        (_, QueryBody::PoolError { mismatch }) => super::pool_error(mismatch, meta.next),
        (Request::SearchPathway(q), QueryBody::PathwayPage(body)) => {
            let body = (meta.status != Status::Failed).then_some(body);
            meta.encode(q, body)
        }
        (Request::GetPathway(q), QueryBody::PathwayCard(body)) => meta.encode(q, body),
        (Request::PathwayGenes(q), QueryBody::PathwayGenes(body)) => meta.encode(q, body),
        (Request::PathwayEvents(q), QueryBody::PathwayEvents(body)) => meta.encode(q, body),
        _ => Err(io::Error::other("pathway response mismatch")),
    }
}
pub(super) fn unsupported(action: &biomcp_query::Action) -> bool {
    match action {
        biomcp_query::Action::Run {
            request: Request::SearchPathway(_),
            ..
        } => false,
        biomcp_query::Action::Run {
            request: Request::GetPathway(q),
            ..
        } => {
            !matches!(q.section(), None | Some(biomcp_types::PathwaySection::Summary | biomcp_types::PathwaySection::All))
        }
        _ => false,
    }
}
