use crate::{Ctx, Outcome, store::Store};
use biomcp_fetch::{CacheError, Deadline, RequestError};
use biomcp_types::{
    PathwayPage, PathwaySearch, Problem, QueryBody, Request, SourceOutcome, StepKind,
};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub(super) struct Saved {
    pub body: PathwayPage,
    pub complete: bool,
    pub sources: Vec<SourceOutcome>,
    pub problems: Vec<Problem>,
}
fn key(q: &PathwaySearch) -> Result<Vec<u8>, RequestError> {
    serde_json::to_vec(&("biomcp.pathway_snapshot/1", q.restart()))
        .map_err(|_| CacheError::Database.into())
}
pub(super) fn load(
    cx: &Ctx<'_>,
    q: &PathwaySearch,
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Result<Option<Saved>, RequestError> {
    let Some(id) = q.pool().filter(|_| !(cx.fresh || q.fresh())) else {
        return Ok(None);
    };
    let store = Store::open(cx.fetch.cache_access()?, deadline)?;
    let payload = store.source_snapshot(id.as_str(), &key(q)?, |s| cx.fetch.disabled(s))?;
    let saved: Option<Saved> = payload
        .map(|p| serde_json::from_str(&p).map_err(|_| CacheError::Database))
        .transpose()?;
    if let Some(saved) = saved {
        if saved.complete || q.offset() == saved.body.page.offset {
            return Ok(Some(saved));
        }
        if saved.body.page.next_offset == Some(q.offset()) {
            return Ok(None);
        }
    }
    out.body = QueryBody::PoolError { mismatch: false };
    out.next.push((
        StepKind::Restart,
        super::run(Request::SearchPathway(q.restart())),
    ));
    Ok(None)
}
pub(super) fn save(
    cx: &Ctx<'_>,
    q: &PathwaySearch,
    deadline: Deadline<'_>,
    saved: &Saved,
) -> Result<biodata::Text, RequestError> {
    let payload = serde_json::to_string(saved).map_err(|_| CacheError::Database)?;
    let mut store = Store::open(cx.fetch.cache_access()?, deadline)?;
    Ok(store.save_source_snapshot(&key(q)?, &payload, &saved.sources)?)
}
