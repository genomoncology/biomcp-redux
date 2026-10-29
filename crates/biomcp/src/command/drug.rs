//! Encode source-local drug responses through the shared envelope contract.
use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request, Status};
use std::io;

pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
        (Request::SearchDrug(query), QueryBody::DrugRecallPage(body)) => {
            let body = (meta.status != Status::Failed).then_some(body);
            meta.encode(query, body)
        }
        (Request::GetDrug(query), QueryBody::DrugRecallCard(body)) => meta.encode(query, body),
        (Request::SearchDrug(query), QueryBody::DrugWhoPage(body)) => {
            let body = (meta.status != Status::Failed).then_some(body);
            meta.encode(query, body)
        }
        (Request::GetWhoMedicine(query), QueryBody::DrugWhoCard(body)) => meta.encode(query, body),
        (Request::SearchDrug(query), QueryBody::DrugFdaPage(body)) => {
            let body = (meta.status != Status::Failed).then_some(body);
            meta.encode(query, body)
        }
        (Request::SearchDrug(query), QueryBody::DrugPage(body)) => {
            let body = (meta.status != Status::Failed).then_some(body);
            meta.encode(query, body)
        }
        (Request::GetDrug(query), QueryBody::DrugCard(body)) => meta.encode(query, body),
        (Request::GetDrug(query), QueryBody::DrugXrefs(body)) => meta.encode(query, body),
        (Request::GetDrug(query), QueryBody::DrugTargets(body)) => meta.encode(query, body),
        (Request::GetDrug(query), QueryBody::DrugIndications(body)) => meta.encode(query, body),
        (Request::GetDrug(query), QueryBody::DrugLabel(body)) => meta.encode(query, body),
        (Request::GetDrug(query), QueryBody::DrugCivic(body)) => meta.encode(query, body),
        (Request::GetDrug(query), QueryBody::DrugRegulatory(body)) => meta.encode(query, body),
        (Request::GetDrug(query), QueryBody::DrugApprovals(body)) => meta.encode(query, body),
        (Request::GetDrug(query), QueryBody::DrugInteractions(body)) => meta.encode(query, body),

        _ => Err(io::Error::other("drug response mismatch")),
    }
}
