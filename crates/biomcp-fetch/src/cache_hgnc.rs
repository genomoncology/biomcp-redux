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
    let mut query = connection.prepare("SELECT n,row FROM bundle_row WHERE bundle='hgnc'")
        .map_err(db_error)?;
    let rows = query.query_map([], |row| Ok((row.get::<_, u64>(0)?, row.get::<_, String>(1)?)))
        .map_err(db_error)?;
    connection.execute("DELETE FROM bundle_key WHERE bundle='hgnc' AND kind=4 AND key LIKE 'uniprot:%'", [])
        .map_err(db_error)?;
    let mut insert = connection.prepare("INSERT INTO bundle_key(bundle,kind,key,n) VALUES ('hgnc',4,?1,?2)")
        .map_err(db_error)?;
    for row in rows {
        let (n, payload) = row.map_err(db_error)?;
        let gene: Gene = serde_json::from_str(&payload).map_err(|_| CacheError::Database)?;
        for value in gene.uniprot_ids.as_deref().unwrap_or("").split('|') {
            if let Ok(id) = UniProtAcc::new(value) {
                insert.execute(params![format!("uniprot:{id}"), n]).map_err(db_error)?;
            }
        }
    }
    Ok(())
}
