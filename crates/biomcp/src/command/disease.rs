use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request};
use std::io;
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
        (Request::SearchDisease(q), QueryBody::DiseasePage(body)) => {
            let body = Some(body);
            meta.encode(q, body)
        }
        (Request::SearchDisease(q), QueryBody::DiseaseSites(body)) => {
            let body = Some(body);
            meta.encode(q, body)
        }
        (Request::GetDisease(q), QueryBody::DiseaseCard(body)) => meta.encode(q, body),
        (Request::GetDisease(q), QueryBody::DiseaseSynonyms(body)) => meta.encode(q, Some(body)),
        (Request::GetDisease(q), QueryBody::DiseaseXrefs(body)) => meta.encode(q, Some(body)),
        (Request::GetDisease(q), QueryBody::DiseaseHierarchy(body)) => meta.encode(q, Some(body)),
        (Request::GetDisease(q), QueryBody::DiseaseGenes(body)) => {
            let body = Some(body);
            meta.encode(q, body)
        }
        (Request::GetDisease(q), QueryBody::DiseasePhenotypes(body)) => {
            let body = Some(body);
            meta.encode(q, body)
        }
        (Request::GetDisease(q), QueryBody::DiseaseIncidence(body)) => {
            let body = Some(body);
            meta.encode(q, body)
        }
        (Request::GetDisease(q), QueryBody::DiseaseSurvival(body)) => {
            let body = Some(body);
            meta.encode(q, body)
        }
        _ => Err(io::Error::other("disease response mismatch")),
    }
}
