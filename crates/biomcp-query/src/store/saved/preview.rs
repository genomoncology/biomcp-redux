//! Count every delete target without reading saved names, search words or note text.
use super::SavedError;
use rusqlite::{Connection, OpenFlags};
use sha2::{Digest, Sha256};
use std::{path::Path, time::Duration};

/// The counts shown to the user and a digest of the corresponding target IDs.
#[derive(Debug, PartialEq, Eq)]
pub struct DeletePreview {
    /// Searches, lists and notes, in that order.
    pub counts: [usize; 3],
    /// An internal digest used to reject an approval after the targets change.
    pub digest: [u8; 32],
}

impl DeletePreview {
    /// Preview a file without creating it or applying migrations.
    pub fn open(path: &Path) -> Result<Self, SavedError> {
        match path.try_exists() {
            Ok(false) => return Self::counts_and_digest(None),
            Err(_) => return Err(SavedError::Database),
            Ok(true) => {}
        }
        let mut connection = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        connection.busy_timeout(Duration::from_secs(5))?;
        let tx = connection.transaction()?;
        super::store::check_compatibility(&tx)?;
        Self::read(&tx)
    }

    pub(super) fn read(connection: &Connection) -> Result<Self, SavedError> {
        Self::counts_and_digest(Some(connection))
    }

    fn counts_and_digest(connection: Option<&Connection>) -> Result<Self, SavedError> {
        let mut counts = [0; 3];
        let mut digest = Sha256::new();
        for (count, table) in counts
            .iter_mut()
            .zip(["saved_search", "saved_list", "saved_note"])
        {
            digest.update(table.as_bytes());
            let Some(connection) = connection else {
                continue;
            };
            // An earlier schema may not have a notes table yet.
            let present: bool = connection.query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_schema WHERE type='table' AND name=?1)",
                [table],
                |row| row.get(0),
            )?;
            if present {
                let mut statement =
                    connection.prepare(&format!("SELECT id FROM {table} ORDER BY id"))?;
                let mut rows = statement.query([])?;
                while let Some(row) = rows.next()? {
                    let id = row.get_ref(0)?.as_str().map_err(|_| SavedError::Database)?;
                    digest.update((id.len() as u64).to_be_bytes());
                    digest.update(id.as_bytes());
                    *count += 1;
                }
            }
        }
        Ok(Self {
            counts,
            digest: digest.finalize().into(),
        })
    }
}
