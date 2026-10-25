use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request, Status};
use std::io;
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    let usable = meta.status != Status::Failed;
    match (request, body) {
        (Request::SearchDiagnostic(q), QueryBody::DiagnosticCompanion(body)) => {
            meta.encode(q, usable.then_some(body))
        }
        (Request::SearchDiagnostic(q), QueryBody::DiagnosticPage(body)) => {
            meta.encode(q, usable.then_some(body))
        }
        (Request::GetDiagnostic(q), QueryBody::DiagnosticCard(body)) => meta.encode(q, body),
        (Request::GetDiagnostic(q), QueryBody::DiagnosticGenes(body)) => {
            meta.encode(q, usable.then_some(body))
        }
        (Request::GetDiagnostic(q), QueryBody::DiagnosticConditions(body)) => {
            meta.encode(q, usable.then_some(body))
        }
        (Request::GetDiagnostic(q), QueryBody::DiagnosticMethods(body)) => {
            meta.encode(q, usable.then_some(body))
        }
        _ => Err(io::Error::other("diagnostic response mismatch")),
    }
}
