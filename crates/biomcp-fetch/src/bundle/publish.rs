//! Stage checked rows in bounded transactions on an independently owned connection.
use super::{
    Bundle, Row,
    parse::Parsed,
    store::{Meta, kind},
};
use crate::{
    CacheError, Deadline,
    cache::{Cache, db_error},
};
use rusqlite::{Connection, params};
use std::time::Duration;

const BATCH_ROWS: usize = 256;
const TABLES: [&str; 3] = ["bundle_row", "bundle_key", "bundle_words"];

impl Cache {
    pub(super) fn publish<R: Row>(
        &self,
        bundle: &Bundle<R>,
        holder: Option<&str>,
        meta: &Meta,
        rows: Option<&[Parsed]>,
        deadline: Deadline<'_>,
    ) -> Result<bool, CacheError> {
        let connection = Connection::open(&self.2).map_err(db_error)?;
        let stage = tempfile::Builder::new()
            .prefix("bundle-stage-")
            .tempfile()
            .map_err(|_| CacheError::Database)?;
        let name = stage
            .path()
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or(CacheError::Database)?;
        let result = (|| {
            if let Some(rows) = rows {
                for (batch, chunk) in rows.chunks(BATCH_ROWS).enumerate() {
                    ready(&connection, deadline)?;
                    let tx = connection.unchecked_transaction().map_err(db_error)?;
                    insert(&tx, name, batch * BATCH_ROWS, chunk)?;
                    tx.commit().map_err(db_error)?;
                }
            }
            ready(&connection, deadline)?;
            let tx = connection.unchecked_transaction().map_err(db_error)?;
            if let Some(holder) = holder {
                let owned: bool = tx
                    .query_row(
                        "SELECT EXISTS(SELECT 1 FROM bundle_sync WHERE name=?1 AND holder=?2)",
                        params![bundle.name, holder],
                        |row| row.get(0),
                    )
                    .map_err(db_error)?;
                if !owned {
                    return Ok(false);
                }
            }
            if rows.is_some() {
                for table in TABLES {
                    tx.execute(
                        &format!("DELETE FROM {table} WHERE bundle=?1"),
                        [bundle.name],
                    )
                    .map_err(db_error)?;
                    tx.execute(
                        &format!("UPDATE {table} SET bundle=?1 WHERE bundle=?2"),
                        params![bundle.name, name],
                    )
                    .map_err(db_error)?;
                }
            }
            metadata(&tx, bundle.name, holder, meta)?;
            tx.commit().map_err(db_error)?;
            Ok(true)
        })();
        // A failed or fenced import must not expose or retain its staged rows.
        // Cleanup never waits behind another writer after the request expires.
        connection.busy_timeout(Duration::ZERO).map_err(db_error)?;
        for table in TABLES {
            let _ = connection.execute(&format!("DELETE FROM {table} WHERE bundle=?1"), [name]);
        }
        result
    }
}

fn ready(connection: &Connection, deadline: Deadline<'_>) -> Result<(), CacheError> {
    if deadline.expired() {
        return Err(CacheError::Busy);
    }
    connection
        .busy_timeout(if deadline.virtual_time() {
            Duration::ZERO
        } else {
            deadline.remaining()
        })
        .map_err(db_error)
}

fn insert(
    connection: &Connection,
    name: &str,
    start: usize,
    rows: &[Parsed],
) -> Result<(), CacheError> {
    for (index, row) in rows.iter().enumerate() {
        let index = (start + index) as i64;
        connection
            .execute(
                "INSERT INTO bundle_row VALUES(?1,?2,?3)",
                params![name, index, row.row],
            )
            .map_err(db_error)?;
        for (key, value) in &row.keys.exact {
            connection
                .execute(
                    "INSERT INTO bundle_key VALUES(?1,?2,?3,?4)",
                    params![name, kind(*key), value.to_ascii_lowercase(), index],
                )
                .map_err(db_error)?;
        }
        connection
            .execute(
                "INSERT INTO bundle_words(bundle,n,words) VALUES(?1,?2,?3)",
                params![name, index, row.keys.words.join(" ")],
            )
            .map_err(db_error)?;
    }
    Ok(())
}

fn metadata(
    connection: &Connection,
    name: &str,
    holder: Option<&str>,
    meta: &Meta,
) -> Result<(), CacheError> {
    connection.execute("INSERT INTO bundle VALUES(?1,?2,?3,?4,?5,?6) ON CONFLICT(name) DO UPDATE SET bytes=excluded.bytes,rows=excluded.rows,fresh_until=excluded.fresh_until,etag=excluded.etag,modified=excluded.modified",
        params![name,meta.bytes as i64,meta.rows as i64,meta.fresh_until,meta.etag,meta.modified]).map_err(db_error)?;
    if let (Some(revision), Some(retrieved)) = (&meta.content_revision, meta.retrieved_at) {
        connection
            .execute(
                "INSERT INTO bundle_content VALUES(?1,?2,?3)",
                params![name, revision, retrieved],
            )
            .map_err(db_error)?;
    }
    if let Some(holder) = holder {
        connection
            .execute(
                "DELETE FROM bundle_sync WHERE name=?1 AND holder=?2",
                params![name, holder],
            )
            .map_err(db_error)?;
    }
    Ok(())
}
