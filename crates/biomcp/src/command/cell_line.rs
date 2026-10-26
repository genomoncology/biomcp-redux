use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request, Status};
use std::io;
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    let success = meta.status != Status::Failed;
    match (request, body) {
        (Request::SearchCellLine(q), QueryBody::CellLinePage(body)) => {
            meta.encode(q, success.then_some(body))
        }
        (Request::GetCellLine(q), QueryBody::CellLineCard(body)) => meta.encode(q, body),
        _ => Err(io::Error::other("cell line response mismatch")),
    }
}
