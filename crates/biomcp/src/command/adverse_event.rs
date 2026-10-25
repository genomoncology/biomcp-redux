use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request, Status};
use std::io;
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
        (Request::SearchAdverseEvent(query), QueryBody::AdverseEventPage(body)) => {
            let body = (meta.status != Status::Failed).then_some(body);
            meta.encode(query, body)
        }
        (Request::GetAdverseEvent(query), QueryBody::AdverseEventMaudeCard(body)) => meta.encode(query, body),
        (Request::GetAdverseEvent(query), QueryBody::AdverseEventDevices(body)) => meta.encode(query, body),
        (Request::GetAdverseEvent(query), QueryBody::AdverseEventNarrative(body)) => meta.encode(query, body),
        _ => super::usage_envelope(biomcp_types::ErrorEnvelope::arguments()),
    }
}
