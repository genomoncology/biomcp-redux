use super::{Outcome, ResponseMeta};
use biomcp_query::Action;
use biomcp_types::{ErrorEnvelope, QueryBody, Request};
use std::io;

pub(super) fn unsupported(action: &Action) -> bool {
    matches!(action, Action::Run {request: Request::GetAuthor(q), ..}
        if q.ids().iter().any(|id| matches!(id, biodata::AuthorId::Orcid(_))))
}

pub(super) fn deferred() -> io::Result<Outcome> {
    super::encode(
        ErrorEnvelope::retired(biomcp_types::ErrorMessage::Deferred),
        crate::exit::Status::Usage,
    )
}

pub(super) fn response(
    meta: ResponseMeta,
    request: Request,
    body: QueryBody,
) -> io::Result<Outcome> {
    match (request, body) {
        (Request::SearchAuthor(q), QueryBody::AuthorPage(body)) => meta.encode(q, body),
        (Request::SearchAuthor(_), QueryBody::PoolError { mismatch }) => {
            super::pool_error(mismatch, meta.next)
        }
        (Request::GetAuthor(q), QueryBody::AuthorCards(body)) => meta.encode(q, Some(body)),
        _ => Err(io::Error::other("author response mismatch")),
    }
}
