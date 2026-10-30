use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request};
use std::io;
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
        (Request::SearchDiagnostic(q), QueryBody::DiagnosticCompanion(body)) => {
            meta.encode(q, Some(body))
        }
        (Request::SearchDiagnostic(q), QueryBody::DiagnosticPage(body)) => {
            meta.encode(q, Some(body))
        }
        (Request::GetDiagnostic(q), QueryBody::DiagnosticCard(body)) => meta.encode(q, body),
        (Request::GetDiagnostic(q), QueryBody::DiagnosticGenes(body)) => meta.encode(q, Some(body)),
        (Request::GetDiagnostic(q), QueryBody::DiagnosticConditions(body)) => {
            meta.encode(q, Some(body))
        }
        (Request::GetDiagnostic(q), QueryBody::DiagnosticMethods(body)) => {
            meta.encode(q, Some(body))
        }
        (Request::GetDiagnostic(q), QueryBody::DiagnosticRegulatory(body)) => {
            meta.encode(q, Some(body))
        }
        (Request::GetDiagnostic(q), QueryBody::DiagnosticSupplements(body)) => {
            meta.encode(q, Some(body))
        }
        _ => Err(io::Error::other("diagnostic response mismatch")),
    }
}
