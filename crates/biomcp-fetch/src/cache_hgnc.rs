//! Backfill the exact HGNC UniProt index from already retained native rows.
use crate::{CacheError, cache::db_error};
use biodata::UniProtAcc;
use rusqlite::{Connection, params};
use serde::Deserialize;
#[derive(Deserialize)]
struct Gene {
    uniprot_ids: Option<String>,
}
pub(crate) fn backfill(connection: &Connection) -> Result<(), CacheError> {
    backfill_tables(connection, "main.bundle_row", "main.bundle_key", None)
}
pub(crate) fn backfill_staged(
    connection: &Connection,
    deadline: crate::Deadline<'_>,
) -> Result<(), CacheError> {
    backfill_tables(
        connection,
        "temp.stage_bundle_row",
        "temp.stage_bundle_key",
        Some(deadline),
    )
}
fn backfill_tables(
    connection: &Connection,
    row_table: &str,
    key_table: &str,
    deadline: Option<crate::Deadline<'_>>,
) -> Result<(), CacheError> {
    let mut query = connection
        .prepare(&format!(
            "SELECT n,row FROM {row_table} WHERE bundle='hgnc'"
        ))
        .map_err(db_error)?;
    let rows = query
        .query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(db_error)?;
    connection
        .execute(
            &format!(
                "DELETE FROM {key_table} WHERE bundle='hgnc' AND kind=4 AND key LIKE 'uniprot:%'"
            ),
            [],
        )
        .map_err(db_error)?;
    let mut insert = connection
        .prepare(&format!(
            "INSERT INTO {key_table}(bundle,kind,key,n) VALUES ('hgnc',4,?1,?2)"
        ))
        .map_err(db_error)?;
    for row in rows {
        if deadline.is_some_and(|d| d.expired()) {
            return Err(CacheError::Busy);
        }
        let (n, payload) = row.map_err(db_error)?;
        let gene: Gene = serde_json::from_str(&payload).map_err(|_| CacheError::Database)?;
        for value in gene.uniprot_ids.as_deref().unwrap_or("").split('|') {
            if let Ok(id) = UniProtAcc::new(value) {
                insert
                    .execute(params![format!("uniprot:{id}").to_ascii_lowercase(), n])
                    .map_err(db_error)?;
            }
        }
    }
    Ok(())
}
