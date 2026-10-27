//! Measure owned store files without attributing shared SQLite pages to bundles.
use crate::{CacheError, Deadline, Fetch, cache::check_retained_owners};
use biomcp_types::StoreSize;
use rusqlite::{Connection, OpenFlags};
use std::{collections::BTreeSet, fs, io, path::Path};
impl Fetch {
    /// Read file lengths only; do not synchronize, checkpoint, vacuum or print paths.
    pub fn store_size(&self, deadline: Deadline<'_>) -> Result<StoreSize, CacheError> {
        let Some(cache) = self.cache.as_ref() else {
            return Ok(StoreSize::default());
        };
        let mut result = StoreSize::default();
        result.active_bytes = database(&cache.2)?;
        result.partial_download_bytes = parts(&cache.1)?;
        let mut seen = BTreeSet::from([cache.2.clone()]);
        // Replay and case-cache commands never inspect the ordinary user store.
        if self.config.replay_cache.is_none() {
            for path in &self.config.cache_predecessors {
                if deadline.expired() {
                    return Err(CacheError::Busy);
                }
                if !seen.insert(path.clone()) || regular_length(path)?.is_none() {
                    continue;
                }
                let connection =
                    Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
                        .map_err(|_| CacheError::Database)?;
                connection
                    .busy_timeout(deadline.remaining())
                    .map_err(|_| CacheError::Database)?;
                if check_retained_owners(&connection, &self.config.cache_owners).is_err() {
                    continue;
                }
                result.retained_bytes += database(path)?;
                result.partial_download_bytes += parts(&path.with_extension("parts"))?;
            }
        }
        result.total_bytes =
            result.active_bytes + result.retained_bytes + result.partial_download_bytes;
        Ok(result)
    }
}
fn database(path: &Path) -> Result<u64, CacheError> {
    let mut bytes = regular_length(path)?.ok_or(CacheError::Database)?;
    for suffix in ["-wal", "-shm"] {
        let mut name = path.as_os_str().to_os_string();
        name.push(suffix);
        bytes += regular_length(Path::new(&name))?.unwrap_or(0);
    }
    Ok(bytes)
}
fn regular_length(path: &Path) -> Result<Option<u64>, CacheError> {
    match fs::symlink_metadata(path) {
        Ok(meta) => Ok(meta.is_file().then_some(meta.len())),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err(CacheError::Database),
    }
}
fn parts(path: &Path) -> Result<u64, CacheError> {
    match fs::symlink_metadata(path) {
        Ok(meta) if !meta.is_dir() => return Err(CacheError::Database),
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(0),
        Err(_) => return Err(CacheError::Database),
        Ok(_) => {}
    }
    let mut bytes = 0;
    for entry in fs::read_dir(path).map_err(|_| CacheError::Database)? {
        let entry = entry.map_err(|_| CacheError::Database)?;
        if entry
            .file_name()
            .to_str()
            .is_some_and(crate::bundle::store::part_name)
        {
            bytes += regular_length(&entry.path())?.unwrap_or(0);
        }
    }
    Ok(bytes)
}
