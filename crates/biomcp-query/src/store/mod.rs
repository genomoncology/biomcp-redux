//! Per-command SQLite owner for fixed search snapshots.
#![expect(
    clippy::disallowed_types,
    clippy::disallowed_methods,
    reason = "Keep wall time and SQLite state inside the reviewed store boundary"
)]
mod pool;
use biomcp_fetch::{CacheAccess, CacheError, Deadline, SchemaOwner};
pub(crate) use pool::{Pool, StoredItem};
use rusqlite::{Connection, params};
use std::time::{SystemTime, UNIX_EPOCH};
/// Query schema version used in the binary's shared cache filename.
pub const VERSION: u32 = 1;
/// Query-owned metadata checked by transport before any journal or schema mutation.
pub const OWNER: SchemaOwner = SchemaOwner {
    name: "biomcp-query",
    version: VERSION,
    tables: &["pool", "pool_current", "pool_item"],
};
/// One command's owned connection, retaining the replay directory capability.
#[derive(Debug)]
pub(crate) struct Store<'a> {
    connection: Connection,
    deadline: Deadline<'a>,
    _access: CacheAccess,
}
impl<'a> Store<'a> {
    pub(crate) fn open(access: CacheAccess, deadline: Deadline<'a>) -> Result<Self, CacheError> {
        if deadline.expired() {
            return Err(CacheError::Busy);
        }
        let mut connection = access.connect()?;
        connection
            .busy_timeout(if deadline.virtual_time() {
                std::time::Duration::ZERO
            } else {
                deadline.remaining()
            })
            .map_err(db)?;
        biomcp_fetch::check_owners(
            &connection,
            &[
                SchemaOwner {
                    name: "biomcp-fetch",
                    version: biomcp_fetch::VERSION,
                    tables: &["reply", "bundle"],
                },
                OWNER,
            ],
        )?;
        let tx = connection.transaction().map_err(db)?;
        tx.execute_batch(include_str!("schema.sql")).map_err(db)?;
        tx.execute(
            "INSERT INTO meta(owner,version) VALUES (?1,?2) ON CONFLICT(owner) DO NOTHING",
            params![OWNER.name, VERSION],
        )
        .map_err(db)?;
        tx.commit().map_err(db)?;
        connection
            .pragma_update(None, "journal_mode", "WAL")
            .map_err(db)?;
        Ok(Self {
            connection,
            deadline,
            _access: access,
        })
    }
    pub(super) fn prepare(&self)->Result<(),CacheError> {
        if self.deadline.expired(){return Err(CacheError::Busy);}
        self.connection.busy_timeout(if self.deadline.virtual_time(){std::time::Duration::ZERO}else{self.deadline.remaining()}).map_err(db)
    }
    pub(crate) fn now() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|t| i64::try_from(t.as_secs()).unwrap_or(i64::MAX))
            .unwrap_or(0)
    }
}
fn db(error: rusqlite::Error) -> CacheError {
    match error {
        rusqlite::Error::SqliteFailure(e, _)
            if matches!(
                e.code,
                rusqlite::ErrorCode::DatabaseBusy | rusqlite::ErrorCode::DatabaseLocked
            ) =>
        {
            CacheError::Busy
        }
        _ => CacheError::Database,
    }
}
