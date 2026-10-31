use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request};
use std::io;
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
        (Request::SearchAdverseEvent(query), QueryBody::VaersSummary(body)) => {
            meta.encode(query, body)
        }
        (Request::SearchAdverseEvent(query), QueryBody::AdverseEventPage(body)) => {
            let body = Some(body);
            meta.encode(query, body)
        }
        (Request::GetAdverseEvent(query), QueryBody::AdverseEventMaudeCard(body)) => {
            meta.encode(query, body)
        }
        (Request::GetAdverseEvent(query), QueryBody::AdverseEventDevices(body)) => {
            meta.encode(query, body)
        }
        (Request::GetAdverseEvent(query), QueryBody::AdverseEventNarrative(body)) => {
            meta.encode(query, body)
        }
        (Request::GetAdverseEvent(query), QueryBody::AdverseEventFaersCard(body)) => {
            meta.encode(query, body)
        }
        (Request::GetAdverseEvent(query), QueryBody::AdverseEventDrugs(body)) => {
            meta.encode(query, body)
        }
        (Request::GetAdverseEvent(query), QueryBody::AdverseEventReactions(body)) => {
            meta.encode(query, body)
        }
        (Request::GetAdverseEvent(query), QueryBody::AdverseEventIdentifiers(body)) => {
            meta.encode(query, body)
        }
        (_, QueryBody::PoolError { mismatch }) => super::pool_error(mismatch, meta.next),
        (_, QueryBody::TextPositionError) => {
            super::usage_envelope(biomcp_types::ErrorEnvelope::arguments())
        }
        _ => super::usage_envelope(biomcp_types::ErrorEnvelope::arguments()),
    }
}
