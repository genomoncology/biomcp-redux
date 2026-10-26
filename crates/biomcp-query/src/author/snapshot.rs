use crate::{
    Ctx, Outcome,
    store::{Pool, Store},
};
use biomcp_adapters::semantic_scholar::author::Page;
use biomcp_fetch::{CacheError, Deadline, RequestError};
use biomcp_types::{AuthorSearch, QueryBody, Request, SourceOutcome, StepKind};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Saved {
    pub pages: Vec<Page>,
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
) -> Result<Option<(Saved, Pool)>, RequestError> {
    let Some(id) = q.pool().filter(|_| !cx.fresh) else {
        return Ok(None);
    };
    let store = Store::open(cx.fetch.cache_access()?, deadline)?;
    let expected = key(q)?;
    let pool = store.find(&expected, Some(id.as_str()))?;
    let mismatch = pool.as_ref().is_some_and(|pool| pool.key != expected);
    if let Some(pool) = pool.filter(|_| !mismatch) {
        let payload =
            store.source_snapshot(id.as_str(), &expected, |source| cx.fetch.disabled(source))?;
        if let Some(payload) = payload {
            let saved: Saved = serde_json::from_str(&payload).map_err(|_| CacheError::Database)?;
            if saved.pages.iter().any(|p| q.offset() == p.offset)
                || saved
                    .pages
                    .last()
                    .is_some_and(|p| p.next == Some(q.offset()))
            {
                return Ok(Some((saved, pool)));
            }
        }
    }
    out.body = QueryBody::PoolError { mismatch };
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
    pool: Option<Pool>,
) -> Result<Pool, RequestError> {
    let payload = serde_json::to_string(saved).map_err(|_| CacheError::Database)?;
    let mut store = Store::open(cx.fetch.cache_access()?, deadline)?;
    let size = saved.pages.iter().map(|p| p.data.len() as u32).sum();
    let sources = std::slice::from_ref(&saved.source);
    Ok(if let Some(pool) = pool {
        store.extend_source_pool(pool, &payload, size, sources)?
    } else {
        store.publish_source_pool(&key(q)?, &payload, size, sources)?
    })
}
