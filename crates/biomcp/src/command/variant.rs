//! Encode the variant command responses through the shared envelope.
use super::{Outcome, ResponseMeta};
use biodata::{QueryBody, Request};
use std::io;

pub(super) fn response(
    meta: ResponseMeta,
    request: Request,
    body: QueryBody,
) -> io::Result<Outcome> {
    match (request, body) {
        (Request::VariantAssertions(query), QueryBody::VariantAssertions(body)) => {
            meta.encode(query, Some(body))
        }
        (Request::VariantNormalize(query), QueryBody::VariantNormalization(body)) => {
            meta.encode(query, Some(body))
        }
        (Request::SearchVariant(query), QueryBody::VariantPage(body)) => {
            meta.encode(query, Some(body))
        }
        (Request::VariantRecurrence(query), QueryBody::VariantRecurrence(body)) => {
            meta.encode(query, Some(body))
        }
        (Request::GetVariant(query), QueryBody::VariantCard(body)) => {
            meta.encode(query, Some(body))
        }
        _ => Err(io::Error::other("query response mismatch")),
    }
}
