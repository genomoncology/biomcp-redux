//! Controlled offline repair of a staged copy; the original is never opened for writes.
#![expect(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "Keep filesystem access and SQLite lock waits inside the offline store repair boundary"
)]
use super::{CacheError, OWNER, db};
use rusqlite::{Connection, OpenFlags};
use std::{fs, path::Path, time::Duration};

/// Prepare a scrubbed copy for deployment's file checks and study lookup.
/// The destination must not exist. This never replaces or modifies the source.
/// Hosting must stop writers before promoting the checked copy.
/// The budget bounds SQLite lock waits; copying and rebuilding may take longer.
pub fn scrub_copy(source: &Path, destination: &Path, budget: Duration) -> Result<(), CacheError> {
    let original =
        Connection::open_with_flags(source, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(db)?;
    original.busy_timeout(budget).map_err(db)?;
    validate(&original)?;
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let file = options
        .open(destination)
        .map_err(|_| CacheError::Filesystem)?;
    drop(file);
    let result = (|| {
        // VACUUM INTO copies a consistent SQLite snapshot, including committed WAL pages.
        original
            .execute(
                "VACUUM INTO ?1",
                [destination.to_str().ok_or(CacheError::Filesystem)?],
            )
            .map_err(db)?;
        let mut copy = Connection::open(destination).map_err(db)?;
        copy.busy_timeout(budget).map_err(db)?;
        validate(&copy)?;
        copy.pragma_update(None, "secure_delete", "ON")
            .map_err(db)?;
        let tx = copy.transaction().map_err(db)?;
        scrub_rows(&tx)?;
        tx.commit().map_err(db)?;
        copy.busy_timeout(budget).map_err(db)?;
        rebuild(&copy)?;
        let integrity: String = copy
            .query_row("PRAGMA integrity_check", [], |row| row.get(0))
            .map_err(db)?;
        if integrity != "ok" {
            return Err(CacheError::Database);
        }
        validate(&copy)?;
        copy.close().map_err(|(_, error)| db(error))?;
        fs::OpenOptions::new()
            .read(true)
            .open(destination)
            .and_then(|file| file.sync_all())
            .map_err(|_| CacheError::Filesystem)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(destination);
        for suffix in ["-wal", "-shm", "-journal"] {
            let mut sidecar = destination.as_os_str().to_os_string();
            sidecar.push(suffix);
            let _ = fs::remove_file(sidecar);
        }
    }
    result
}

fn validate(connection: &Connection) -> Result<(), CacheError> {
    biomcp_fetch::check_owners(
        connection,
        &[
            biomcp_fetch::SchemaOwner {
                name: "biomcp-fetch",
                version: biomcp_fetch::VERSION,
                migratable_versions: &[],
                tables: &["reply", "bundle"],
            },
            biomcp_fetch::SchemaOwner {
                migratable_versions: &[],
                ..OWNER
            },
        ],
    )
}

fn rebuild(connection: &Connection) -> Result<(), CacheError> {
    // Rebuild clears old free pages; truncation clears every previously committed WAL frame.
    checkpoint(connection)?;
    connection.execute_batch("VACUUM").map_err(db)?;
    checkpoint(connection)
}

fn checkpoint(connection: &Connection) -> Result<(), CacheError> {
    let busy: i64 = connection
        .query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |row| row.get(0))
        .map_err(db)?;
    if busy != 0 {
        return Err(CacheError::Busy);
    }
    Ok(())
}

fn scrub_rows(tx: &rusqlite::Transaction<'_>) -> Result<(), CacheError> {
    let rights = stored_rights(tx)?;
    scrub_replies(tx, &rights)?;
    scrub_items(tx, &rights)?;
    scrub_snapshots(tx, &rights)
}

fn stored_rights(
    tx: &rusqlite::Transaction<'_>,
) -> Result<biomcp_adapters::article_rights::StoredRights, CacheError> {
    let mut rights = biomcp_adapters::article_rights::StoredRights::default();
    {
        let mut statement = tx
            .prepare(
                "SELECT source,url,body FROM reply WHERE status BETWEEN 200 AND 299 ORDER BY key",
            )
            .map_err(db)?;
        let rows = statement
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Vec<u8>>(2)?,
                ))
            })
            .map_err(db)?;
        for row in rows {
            let (source, url, body) = row.map_err(db)?;
            rights
                .observe(
                    source.parse().map_err(|_| CacheError::Encoding)?,
                    &url,
                    &body,
                )
                .map_err(|_| CacheError::Encoding)?;
        }
    }
    Ok(rights.finish())
}

fn scrub_replies(
    tx: &rusqlite::Transaction<'_>,
    rights: &biomcp_adapters::article_rights::StoredRights,
) -> Result<(), CacheError> {
    // Keyset iteration remains stable while rows are updated or deleted, and holds one body at a time.
    let mut last: Option<Vec<u8>> = None;
    loop {
        use rusqlite::OptionalExtension;
        let row = tx.query_row("SELECT key,source,url,body FROM reply WHERE status BETWEEN 200 AND 299 AND (?1 IS NULL OR key>?1) ORDER BY key LIMIT 1", [&last],
            |row| Ok((row.get::<_, Vec<u8>>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, Vec<u8>>(3)?)))
            .optional().map_err(db)?;
        let Some((key, source, url, body)) = row else {
            break;
        };
        match rights
            .scrub_reply(
                source.parse().map_err(|_| CacheError::Encoding)?,
                &url,
                &body,
            )
            .map_err(|_| CacheError::Encoding)?
        {
            Some(clean) if clean != body => {
                tx.execute(
                    "UPDATE reply SET body=?2 WHERE key=?1",
                    rusqlite::params![key, clean],
                )
                .map_err(db)?;
            }
            None => {
                tx.execute("DELETE FROM reply WHERE key=?1", [&key])
                    .map_err(db)?;
            }
            _ => {}
        }
        last = Some(key);
    }
    Ok(())
}

fn scrub_items(
    tx: &rusqlite::Transaction<'_>,
    rights: &biomcp_adapters::article_rights::StoredRights,
) -> Result<(), CacheError> {
    let mut last = 0_i64;
    loop {
        use rusqlite::OptionalExtension;
        let row = tx
            .query_row(
                "SELECT rowid,item FROM pool_item WHERE rowid>?1 ORDER BY rowid LIMIT 1",
                [last],
                |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?)),
            )
            .optional()
            .map_err(db)?;
        let Some((id, payload)) = row else { break };
        let mut item = serde_json::from_str(&payload).map_err(|_| CacheError::Encoding)?;
        if rights.scrub_item(&mut item) {
            let clean = serde_json::to_string(&item).map_err(|_| CacheError::Encoding)?;
            tx.execute(
                "UPDATE pool_item SET item=?2 WHERE rowid=?1",
                rusqlite::params![id, clean],
            )
            .map_err(db)?;
        }
        last = id;
    }
    Ok(())
}

fn scrub_snapshots(
    tx: &rusqlite::Transaction<'_>,
    rights: &biomcp_adapters::article_rights::StoredRights,
) -> Result<(), CacheError> {
    use rusqlite::OptionalExtension;
    for table in ["trial_snapshot", "citation_evidence"] {
        let mut last = 0_i64;
        loop {
            let row = tx
                .query_row(
                    &format!(
                        "SELECT rowid,payload FROM {table} WHERE rowid>?1 ORDER BY rowid LIMIT 1"
                    ),
                    [last],
                    |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?)),
                )
                .optional()
                .map_err(db)?;
            let Some((id, payload)) = row else { break };
            let clean = rights
                .scrub_snapshot(payload.as_bytes())
                .map_err(|_| CacheError::Encoding)?;
            if clean != payload.as_bytes() {
                let clean = std::str::from_utf8(&clean).map_err(|_| CacheError::Encoding)?;
                tx.execute(
                    &format!("UPDATE {table} SET payload=?2 WHERE rowid=?1"),
                    rusqlite::params![id, clean],
                )
                .map_err(db)?;
            }
            last = id;
        }
    }
    Ok(())
}
