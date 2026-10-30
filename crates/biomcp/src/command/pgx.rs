use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request};
use std::io;
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
        (Request::SearchPgx(q), QueryBody::PgxPage(b)) => meta.encode(q, Some(b)),
        (Request::GetPgx(q), QueryBody::PgxCard(b)) => meta.encode(q, b),
        (Request::GetPgx(q), QueryBody::PgxRecommendations(b)) => {
            meta.encode(q, Some(b))
        }
        (Request::GetPgx(q), QueryBody::PgxGuidelines(b)) => meta.encode(q, Some(b)),
        (Request::GetPgx(q), QueryBody::PgxDiplotype(b)) => meta.encode(q, Some(b)),
        (Request::GetPgx(q), QueryBody::PgxAnnotations(b)) => meta.encode(q, Some(b)),
        (Request::GetPgxAnnotation(q), QueryBody::PgxAnnotation(b)) => meta.encode(q, b),
        (_, QueryBody::PoolError { mismatch }) => super::pool_error(mismatch, meta.next),
        _ => super::usage_envelope(biomcp_types::ErrorEnvelope::arguments()),
    }
}
