//! Encode source-specific drug bodies through the shared envelope.
use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request, Status};
use std::io;

pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
        (Request::SearchDrug(query), QueryBody::DrugPage(body)) => {
            let body = (meta.status != Status::Failed).then_some(body);
            meta.encode(query, body)
        }
        (Request::GetDrug(query), QueryBody::DrugCard(body)) => meta.encode(query, body),
        (Request::GetDrug(query), QueryBody::DrugRegulatory(body)) => meta.encode(query, body),
        (Request::GetDrug(query), QueryBody::DrugXrefs(body)) => meta.encode(query, body),
        (Request::GetDrug(query), QueryBody::DrugCivic(body)) => meta.encode(query, body),
        (request, body) => super::variant::response(meta, request, body),
    }
}
