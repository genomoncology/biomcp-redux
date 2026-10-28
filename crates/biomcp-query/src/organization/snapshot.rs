//! Retain complete organization observations with their source outcomes.
use crate::{Ctx, store::Store};
use biomcp_fetch::{CacheError, Deadline, RequestError};
use biomcp_types::{Organization, OrganizationSearch, Problem, SourceOutcome};
pub(super) struct Saved {
    pub items: Vec<Organization>,
    pub sources: Vec<SourceOutcome>,
    pub problems: Vec<Problem>,
}
type Wire = (Vec<Organization>, Vec<SourceOutcome>, Vec<Problem>);
fn key(q: &OrganizationSearch) -> Result<Vec<u8>, RequestError> {
    serde_json::to_vec(&("biomcp.organization-snapshot/1", q.snapshot_query()))
        .map_err(|_| CacheError::Database.into())
}
pub(super) fn load(
    cx: &Ctx<'_>,
    q: &OrganizationSearch,
    deadline: Deadline<'_>,
) -> Result<Option<Saved>, RequestError> {
    let Some(id) = q.pool().filter(|_| !cx.fresh) else {
        return Ok(None);
    };
    let store = Store::open(cx.fetch.cache_access()?, deadline)?;
    store
        .source_snapshot(id.as_str(), &key(q)?, |s| cx.fetch.disabled(s))?
        .map(|p| {
            serde_json::from_str::<Wire>(&p)
                .map(|(items, sources, problems)| Saved {
                    items,
                    sources,
                    problems,
                })
                .map_err(|_| CacheError::Database.into())
        })
        .transpose()
}
pub(super) fn save(
    cx: &Ctx<'_>,
    q: &OrganizationSearch,
    deadline: Deadline<'_>,
    saved: &Saved,
) -> Result<biodata::Text, RequestError> {
    let payload = serde_json::to_string(&(&saved.items, &saved.sources, &saved.problems))
        .map_err(|_| CacheError::Database)?;
    let mut store = Store::open(cx.fetch.cache_access()?, deadline)?;
    Ok(store.save_source_snapshot(&key(q)?, &payload, &saved.sources)?)
}
