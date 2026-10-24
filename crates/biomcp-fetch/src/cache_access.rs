use crate::{CacheError, Fetch, ReplayCache};
use std::{path::PathBuf, sync::Arc};
/// Capability for another reviewed schema owner to open the active cache.
#[derive(Clone)]
pub struct CacheAccess {
    path: PathBuf,
    _cache: Arc<crate::cache::Cache>,
    _replay: Option<ReplayCache>,
}
impl std::fmt::Debug for CacheAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CacheAccess")
    }
}
impl CacheAccess {
    /// Open the selected owned file; callers validate their metadata before writes.
    pub fn connect(&self) -> Result<rusqlite::Connection, CacheError> {
        rusqlite::Connection::open(&self.path).map_err(|_| CacheError::Database)
    }
}
impl Fetch {
    /// Retain the active reply cache and its directory owner for a query connection.
    pub fn cache_access(&self) -> Result<CacheAccess, CacheError> {
        let cache = self.cache.as_ref().ok_or(CacheError::Database)?;
        Ok(CacheAccess {
            path: cache.2.clone(),
            _cache: cache.clone(),
            _replay: self.config.replay_cache.clone(),
        })
    }
}
