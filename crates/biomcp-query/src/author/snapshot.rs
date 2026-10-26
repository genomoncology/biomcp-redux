use crate::{Ctx, Outcome, store::Store};
use biomcp_adapters::semantic_scholar::author::Page;
use biomcp_fetch::{CacheError, Deadline, RequestError};
use biomcp_types::{AuthorSearch, QueryBody, Request, SourceOutcome, StepKind};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Saved {
    pub page: Page,
    pub source: SourceOutcome,
}
fn key(q: &AuthorSearch) -> Result<Vec<u8>, RequestError> {
    serde_json::to_vec(&("biomcp.author_snapshot/1", q.query(), q.source(), q.limit()))
        .map_err(|_| CacheError::Database.into())
}
pub(super) fn load(
    cx: &Ctx<'_>,
    q: &AuthorSearch,
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Result<Option<Saved>, RequestError> {
    let Some(id) = q.pool().filter(|_| !cx.fresh) else {
        return Ok(None);
    };
    let store = Store::open(cx.fetch.cache_access()?, deadline)?;
    let payload =
        store.source_snapshot(id.as_str(), &key(q)?, |source| cx.fetch.disabled(source))?;
    if let Some(payload) = payload {
        let saved: Saved = serde_json::from_str(&payload).map_err(|_| CacheError::Database)?;
        if q.offset() == saved.page.offset {
            return Ok(Some(saved));
        }
        if Some(q.offset()) == saved.page.next {
            return Ok(None);
        }
    }
    out.body = QueryBody::PoolError { mismatch: false };
    out.next.push((
        StepKind::Restart,
        super::run(Request::SearchAuthor(q.restart())),
    ));
    Ok(None)
}
pub(super) fn save(
    cx: &Ctx<'_>,
    q: &AuthorSearch,
    deadline: Deadline<'_>,
    saved: &Saved,
) -> Result<biodata::Text, RequestError> {
    let payload = serde_json::to_string(saved).map_err(|_| CacheError::Database)?;
    let mut store = Store::open(cx.fetch.cache_access()?, deadline)?;
    Ok(store.save_source_snapshot(&key(q)?, &payload, std::slice::from_ref(&saved.source))?)
}
