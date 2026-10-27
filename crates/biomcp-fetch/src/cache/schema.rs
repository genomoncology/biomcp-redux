//! Validate SQLite owners before initializing the transport schema.
use super::{Cache, CacheError, OWNER, VERSION};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior};
/// Metadata owned by another schema sharing the active cache.
#[derive(Clone, Copy, Debug)]
pub struct SchemaOwner {
    /// Stable schema owner name.
    pub name: &'static str,
    /// Expected schema version.
    pub version: u32,
    /// Older versions that this owner explicitly migrates in place.
    pub migratable_versions: &'static [u32],
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
        if version.is_some_and(|v| v != owner.version && !owner.migratable_versions.contains(&v)) {
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
    pub(super) fn initialize(connection: &mut Connection) -> Result<(), CacheError> {
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
            let orphan: bool = tx
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE name IN \
                    ('reply','reply_fresh','bundle','bundle_sync','bundle_row','bundle_key','bundle_key_find','bundle_wo\
                    rds','bundle_content'))",
                    [],
                    |r| r.get(0),
                )
                .map_err(db_error)?;
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
