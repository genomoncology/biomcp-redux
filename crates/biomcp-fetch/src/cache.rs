//! Transport-owned SQLite schema. Other owners and tables remain untouched.
#![expect(
    clippy::disallowed_types,
    reason = "Serialize only SQL on one connection shared by transport legs"
)]
use crate::{CacheError, Deadline};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use std::{
    path::Path,
    sync::Mutex,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Transport SQLite schema version checked by other owners of the shared database.
pub const VERSION: u32 = 4;
/// Compatible cache namespace used by callers when naming shared cache files.
pub const CACHE_NAMESPACE_VERSION: u32 = 1;
const OWNER: &str = "biomcp-fetch";
const OPEN_WAIT: Duration = Duration::from_secs(5);

// One connection is shared by source legs. Its lock covers only SQLite work;
// callers decode and check owned bytes after the guard has been released.
#[derive(Debug)]
pub(crate) struct Cache(
    Mutex<Connection>,
    pub(crate) std::path::PathBuf,
    pub(crate) std::path::PathBuf,
);

#[derive(Debug, serde::Serialize)]
pub(crate) struct Entry {
    pub(crate) status: u16,
    #[serde(skip)] // Containment scans raw and decoded body bytes directly.
    pub(crate) body: Vec<u8>,
    pub(crate) content_type: Option<String>,
    pub(crate) content_range: Option<String>,
    pub(crate) pagination: Option<crate::Pagination>,
    pub(crate) fresh_until: i64,
    pub(crate) fetched_at: i64,
}

impl Entry {
    pub(crate) fn new(status: u16, body: Vec<u8>, seconds: u64, content_type: &str) -> Self {
        let fetched_at = now();
        let fresh_until = fetched_at.saturating_add(i64::try_from(seconds).unwrap_or(i64::MAX));
        Self {
            status,
            body,
            content_range: None,
            pagination: None,
            content_type: (status != 404).then(|| content_type.to_owned()),
            fetched_at,
            fresh_until,
        }
    }
}

/// Metadata owned by another schema sharing the active cache.
#[derive(Clone, Copy, Debug)]
pub struct SchemaOwner {
    /// Stable schema owner name.
    pub name: &'static str,
    /// Expected schema version.
    pub version: u32,
    /// Tables that must not exist without this owner's metadata.
    pub tables: &'static [&'static str],
}
/// Validate declared owners without changing journal mode or creating tables.
pub fn check_owners(connection: &Connection, owners: &[SchemaOwner]) -> Result<(), CacheError> {
    let has_meta: bool = connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE name='meta')",
            [],
            |r| r.get(0),
        )
        .map_err(db_error)?;
    for owner in owners {
        let version: Option<u32> = if has_meta {
            connection
                .query_row(
                    "SELECT version FROM meta WHERE owner=?1",
                    [owner.name],
                    |r| r.get(0),
                )
                .optional()
                .map_err(|_| CacheError::Version)?
        } else {
            None
        };
        if version.is_some_and(|v| v != owner.version) {
            return Err(CacheError::Version);
        }
        if version.is_none() {
            for table in owner.tables {
                let orphan: bool = connection
                    .query_row(
                        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE name=?1)",
                        [table],
                        |r| r.get(0),
                    )
                    .map_err(db_error)?;
                if orphan {
                    return Err(CacheError::Version);
                }
            }
        }
    }
    Ok(())
}
impl Cache {
    pub(crate) fn open_with_owners(
        path: &Path,
        owners: &[SchemaOwner],
    ) -> Result<Self, CacheError> {
        let mut connection = Connection::open(path).map_err(db_error)?;
        check_owners(&connection, owners)?;
        connection.busy_timeout(OPEN_WAIT).map_err(db_error)?;
        Self::initialize(&mut connection)?;
        // Ownership is validated before changing the file's journal mode.
        let journal: String = connection
            .pragma_query_value(None, "journal_mode", |row| row.get(0))
            .map_err(db_error)?;
        if !journal.eq_ignore_ascii_case("wal") {
            connection
                .pragma_update(None, "journal_mode", "WAL")
                .map_err(db_error)?;
        }
        Ok(Self(
            Mutex::new(connection),
            path.with_extension("parts"),
            path.to_owned(),
        ))
    }

    fn initialize(connection: &mut Connection) -> Result<(), CacheError> {
        let has_meta: bool = connection
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE name='meta')",
                [],
                |row| row.get(0),
            )
            .map_err(db_error)?;
        if has_meta
            && connection
                .query_row("SELECT version FROM meta WHERE owner=?1", [OWNER], |row| {
                    row.get::<_, u32>(0)
                })
                .optional()
                .map_err(|_| CacheError::Version)?
                == Some(VERSION)
        {
            return Ok(());
        }
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(db_error)?;
        let meta_exists: bool = tx
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE name='meta')",
                [],
                |r| r.get(0),
            )
            .map_err(db_error)?;
        let version: Option<u32> = if meta_exists {
            tx.query_row("SELECT version FROM meta WHERE owner=?1", [OWNER], |r| {
                r.get(0)
            })
            .optional()
            .map_err(|_| CacheError::Version)?
        } else {
            None
        };
        if version.is_some_and(|version| {
            version != 1 && version != 2 && version != 3 && version != VERSION
        }) {
            return Err(CacheError::Version);
        }
        if version.is_none() {
            let orphan: bool = tx.query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE name IN ('reply','reply_fresh','bundle','bundle_sync','bundle_row','bundle_key','bundle_key_find','bundle_words','bundle_content'))",
                [], |r| r.get(0),
            ).map_err(db_error)?;
            if orphan {
                return Err(CacheError::Version);
            }
        }
        tx.execute_batch(
            "CREATE TABLE IF NOT EXISTS meta (owner TEXT PRIMARY KEY, version INTEGER NOT NULL);
            CREATE TABLE IF NOT EXISTS reply (
                key BLOB PRIMARY KEY, source TEXT NOT NULL, url TEXT NOT NULL,
                status INTEGER NOT NULL, content_type TEXT, content_range TEXT, pagination TEXT, body BLOB NOT NULL,
                fetched_at INTEGER NOT NULL, fresh_until INTEGER NOT NULL);
            CREATE INDEX IF NOT EXISTS reply_fresh ON reply (fresh_until);",
        )
        .map_err(db_error)?;
        if version == Some(1) {
            tx.execute("ALTER TABLE reply ADD COLUMN content_range TEXT", [])
                .map_err(db_error)?;
        }
        if matches!(version, Some(1 | 2)) {
            tx.execute("ALTER TABLE reply ADD COLUMN pagination TEXT", [])
                .map_err(db_error)?;
        }
        crate::bundle::store::initialize(&tx)?;
        if matches!(version, Some(1..=3)) {
            crate::cache_hgnc::backfill(&tx)?;
        }
        tx.execute(
            "INSERT INTO meta(owner,version) VALUES (?1,?2) ON CONFLICT(owner) DO UPDATE SET version=excluded.version",
            (OWNER, VERSION),
        )
        .map_err(db_error)?;
        tx.commit().map_err(db_error)
    }

    // try_lock avoids an unbounded Rust mutex wait. SQLite's own busy handler
    // uses the remaining deadline after acquiring the connection.
    pub(crate) fn sql<T>(
        &self,
        deadline: Deadline<'_>,
        operation: impl FnOnce(&Connection) -> Result<T, CacheError>,
    ) -> Result<T, CacheError> {
        self.serial_sql(deadline, true, operation)
    }

    fn serial_sql<T>(
        &self,
        deadline: Deadline<'_>,
        wait_database: bool,
        operation: impl FnOnce(&Connection) -> Result<T, CacheError>,
    ) -> Result<T, CacheError> {
        let started = std::time::Instant::now();
        let remaining = deadline.remaining();
        loop {
            if deadline.expired() || started.elapsed() >= remaining {
                return Err(CacheError::Busy);
            }
            match self.0.try_lock() {
                Ok(connection) => {
                    connection
                        .busy_timeout(if deadline.virtual_time() || !wait_database {
                            Duration::ZERO
                        } else {
                            deadline.remaining()
                        })
                        .map_err(db_error)?;
                    return operation(&connection);
                }
                Err(std::sync::TryLockError::Poisoned(_)) => return Err(CacheError::Database),
                Err(std::sync::TryLockError::WouldBlock) => {
                    std::thread::park_timeout(Duration::from_millis(1).min(deadline.remaining()));
                }
            }
        }
    }

    pub(crate) fn read(
        &self,
        key: &[u8; 32],
        max_body: u64,
        deadline: Deadline<'_>,
        opportunistic: bool,
    ) -> Result<Option<Entry>, CacheError> {
        let limit = i64::try_from(max_body).unwrap_or(i64::MAX);
        let operation = |connection: &Connection| {
            let entry = connection.query_row(
                "SELECT status, CASE WHEN length(body)<=?2 THEN body ELSE NULL END, fresh_until, fetched_at, content_type, content_range, pagination FROM reply WHERE key=?1",
                params![key, limit], |row| Ok((row.get::<_, u16>(0)?, row.get::<_, Option<Vec<u8>>>(1)?, row.get::<_, i64>(2)?, row.get::<_, i64>(3)?, row.get::<_, Option<String>>(4)?, row.get::<_, Option<String>>(5)?, row.get::<_, Option<String>>(6)?)),
            ).optional().map_err(db_error)?;
            match entry {
                Some((
                    status,
                    Some(body),
                    fresh_until,
                    fetched_at,
                    content_type,
                    content_range,
                    pagination,
                )) if (200..300).contains(&status) || status == 404 && body.is_empty() => {
                    let pagination = pagination
                        .map(|p| serde_json::from_str::<crate::Pagination>(&p))
                        .transpose();
                    match pagination {
                        Ok(pagination)
                            if pagination.as_ref().is_none_or(crate::Pagination::valid) =>
                        {
                            Ok(Some(Entry {
                                status,
                                body,
                                fresh_until,
                                fetched_at,
                                content_type,
                                content_range,
                                pagination,
                            }))
                        }
                        _ => {
                            Self::delete(connection, key)?;
                            Ok(None)
                        }
                    }
                }
                Some(_) => {
                    Self::delete(connection, key)?;
                    Ok(None)
                }
                None => Ok(None),
            }
        };
        if opportunistic {
            self.try_sql(deadline, operation)
        } else {
            self.sql(deadline, operation)
        }
    }

    // Discretionary reply caching must leave the budget available to the source.
    fn try_sql<T>(
        &self,
        deadline: Deadline<'_>,
        operation: impl FnOnce(&Connection) -> Result<T, CacheError>,
    ) -> Result<T, CacheError> {
        if deadline.expired() {
            return Err(CacheError::Busy);
        }
        let connection = self.0.try_lock().map_err(|error| match error {
            std::sync::TryLockError::WouldBlock => CacheError::Busy,
            std::sync::TryLockError::Poisoned(_) => CacheError::Database,
        })?;
        connection.busy_timeout(Duration::ZERO).map_err(db_error)?;
        operation(&connection)
    }

    pub(crate) fn remove(&self, key: &[u8; 32], deadline: Deadline<'_>) -> Result<(), CacheError> {
        self.try_sql(deadline, |connection| Self::delete(connection, key))
    }

    fn delete(connection: &Connection, key: &[u8; 32]) -> Result<(), CacheError> {
        connection
            .execute("DELETE FROM reply WHERE key=?1", [key])
            .map_err(db_error)?;
        Ok(())
    }

    pub(crate) fn write(
        &self,
        key: &[u8; 32],
        source: biomcp_types::Source,
        url: &str,
        entry: &Entry,
        deadline: Deadline<'_>,
    ) -> Result<(), CacheError> {
        self.serial_sql(deadline, false, |connection| {
            connection.execute(
                "INSERT INTO reply(key,source,url,status,content_type,body,fetched_at,fresh_until,content_range,pagination) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)
                ON CONFLICT(key) DO UPDATE SET source=excluded.source,url=excluded.url,status=excluded.status,content_type=excluded.content_type,body=excluded.body,fetched_at=excluded.fetched_at,fresh_until=excluded.fresh_until,content_range=excluded.content_range,pagination=excluded.pagination",
                params![key, source.to_string(), url, entry.status, entry.content_type, entry.body, entry.fetched_at, entry.fresh_until, entry.content_range, entry.pagination.as_ref().map(serde_json::to_string).transpose().map_err(|_| CacheError::Database)?],
            ).map_err(db_error)?;
            Ok(())
        })
    }
}

pub(crate) fn now() -> i64 {
    i64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    )
    .unwrap_or(i64::MAX)
}
pub(crate) fn db_error(error: rusqlite::Error) -> CacheError {
    if matches!(
        error.sqlite_error_code(),
        Some(rusqlite::ErrorCode::DatabaseBusy | rusqlite::ErrorCode::DatabaseLocked)
    ) {
        CacheError::Busy
    } else {
        CacheError::Database
    }
}
