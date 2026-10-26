use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request, Status};
use std::io;
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    let success = meta.status != Status::Failed;
    match (request, body) {
        (Request::SearchPhenotype(q), QueryBody::PhenotypePage(body)) => {
            meta.encode(q, success.then_some(body))
        }
        (Request::GetPhenotype(q), QueryBody::PhenotypeCard(body)) => meta.encode(q, body),
        (_, QueryBody::PoolError { mismatch }) => super::pool_error(mismatch, meta.next),
        _ => Err(io::Error::other("phenotype response mismatch")),
    }
}

pub(super) fn unsupported(action: &biomcp_query::Action) -> bool {
    matches!(action, biomcp_query::Action::Run { request: Request::GetPhenotype(query), .. } if query.section().is_some())
}
