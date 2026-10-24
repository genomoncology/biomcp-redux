//! Transport-owned SQLite schema. Other owners and tables remain untouched.
use crate::{CacheError, Deadline};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use std::{
    path::Path,
    sync::Mutex,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Transport schema version used by callers when naming shared cache files.
pub const VERSION: u32 = 1;
const OWNER: &str = "biomcp-fetch";
const OPEN_WAIT: Duration = Duration::from_secs(5);

// One connection is shared by source legs. Its lock covers only SQLite work;
// callers decode and check owned bytes after the guard has been released.
#[expect(
    clippy::disallowed_types,
    reason = "Serialize only SQL on one connection shared by transport legs"
)]
#[derive(Debug)]
pub(crate) struct Cache(Mutex<Connection>);

#[derive(Debug)]
pub(crate) struct Entry {
    pub(crate) status: u16,
    pub(crate) body: Vec<u8>,
    pub(crate) fresh_until: i64,
}

impl Cache {
    pub(crate) fn open(path: &Path) -> Result<Self, CacheError> {
        let mut connection = Connection::open(path).map_err(db_error)?;
        connection.busy_timeout(OPEN_WAIT).map_err(db_error)?;
        Self::initialize(&mut connection)?;
        // Ownership is validated before changing the file's journal mode.
        connection
            .pragma_update(None, "journal_mode", "WAL")
            .map_err(db_error)?;
        Ok(Self(Mutex::new(connection)))
    }

    fn initialize(connection: &mut Connection) -> Result<(), CacheError> {
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
        if version.is_some_and(|version| version != VERSION) {
            return Err(CacheError::Version);
        }
        if version.is_none() {
            let orphan: bool = tx.query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE name IN ('reply','reply_fresh','bundle','bundle_sync','bundle_row','bundle_key','bundle_key_find','bundle_words'))",
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
                status INTEGER NOT NULL, content_type TEXT, body BLOB NOT NULL,
                fetched_at INTEGER NOT NULL, fresh_until INTEGER NOT NULL);
            CREATE INDEX IF NOT EXISTS reply_fresh ON reply (fresh_until);",
        )
        .map_err(db_error)?;
        tx.execute(
            "INSERT INTO meta(owner,version) VALUES (?1,?2) ON CONFLICT(owner) DO NOTHING",
            (OWNER, VERSION),
        )
        .map_err(db_error)?;
        tx.commit().map_err(db_error)
    }

    // try_lock avoids an unbounded Rust mutex wait. SQLite's own busy handler
    // uses the remaining deadline after acquiring the connection.
    fn sql<T>(
        &self,
        deadline: Deadline,
        operation: impl FnOnce(&Connection) -> Result<T, CacheError>,
    ) -> Result<T, CacheError> {
        loop {
            if deadline.expired() {
                return Err(CacheError::Busy);
            }
            match self.0.try_lock() {
                Ok(connection) => {
                    connection
                        .busy_timeout(deadline.remaining())
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
        deadline: Deadline,
    ) -> Result<Option<Entry>, CacheError> {
        let limit = i64::try_from(max_body).unwrap_or(i64::MAX);
        self.sql(deadline, |connection| {
            let entry = connection.query_row(
                "SELECT status, CASE WHEN length(body)<=?2 THEN body ELSE NULL END, fresh_until FROM reply WHERE key=?1",
                params![key, limit], |row| Ok((row.get::<_, u16>(0)?, row.get::<_, Option<Vec<u8>>>(1)?, row.get::<_, i64>(2)?)),
            ).optional().map_err(db_error)?;
            match entry {
                Some((status, Some(body), fresh_until)) if (200..300).contains(&status) || status == 404 && body.is_empty() => {
                    Ok(Some(Entry {status, body, fresh_until}))
                }
                Some(_) => { Self::delete(connection, key)?; Ok(None) }
                None => Ok(None),
            }
        })
    }

    pub(crate) fn remove(&self, key: &[u8; 32], deadline: Deadline) -> Result<(), CacheError> {
        self.sql(deadline, |connection| Self::delete(connection, key))
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
        source: biodata::Source,
        url: &str,
        entry: &Entry,
        deadline: Deadline,
    ) -> Result<(), CacheError> {
        self.sql(deadline, |connection| {
            connection.execute(
                "INSERT INTO reply(key,source,url,status,content_type,body,fetched_at,fresh_until) VALUES (?1,?2,?3,?4,?5,?6,?7,?8)
                ON CONFLICT(key) DO UPDATE SET source=excluded.source,url=excluded.url,status=excluded.status,content_type=excluded.content_type,body=excluded.body,fetched_at=excluded.fetched_at,fresh_until=excluded.fresh_until",
                params![key, source.to_string(), url, entry.status, if entry.status == 404 {None} else {Some("application/json")}, entry.body, now(), entry.fresh_until],
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
fn db_error(error: rusqlite::Error) -> CacheError {
    if matches!(
        error.sqlite_error_code(),
        Some(rusqlite::ErrorCode::DatabaseBusy | rusqlite::ErrorCode::DatabaseLocked)
    ) {
        CacheError::Busy
    } else {
        CacheError::Database
    }
}
