use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request};
use std::io;

pub(super) fn response(
    meta: ResponseMeta,
    request: Request,
    body: QueryBody,
) -> io::Result<Outcome> {
    match (request, body) {
        (Request::AuthorPapers(q), QueryBody::AuthorPapers(body)) => meta.encode(q, body),
        (Request::AuthorPapers(q), QueryBody::AuthorPaperDetails(body)) => meta.encode(q, body),
        (Request::AuthorPapers(q), QueryBody::OrcidWorks(body)) => meta.encode(q, body),
        (Request::AuthorPapers(_), QueryBody::Usage(biomcp_types::UsageError::PoolError { mismatch })) => {
            super::pool_error(mismatch, meta.next)
        }
        (Request::SearchAuthor(q), QueryBody::AuthorPage(body)) => meta.encode(q, body),
        (Request::SearchAuthor(_), QueryBody::Usage(biomcp_types::UsageError::PoolError { mismatch })) => {
            super::pool_error(mismatch, meta.next)
        }
        (Request::GetAuthor(q), QueryBody::AuthorCards(body)) => meta.encode(q, Some(body)),
        _ => Err(io::Error::other("author response mismatch")),
    }
}
