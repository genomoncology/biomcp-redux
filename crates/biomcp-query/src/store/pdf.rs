//! Extracted PDF text only; raw publisher bytes never enter this table.
use super::{Store, db};
use biomcp_fetch::CacheError;
use rusqlite::{OptionalExtension, params};
const LIFETIME: i64 = 7 * 24 * 60 * 60;
impl Store<'_> {
    pub(crate) fn pdf_get(&self, article: &str) -> Result<Option<String>, CacheError> {
        self.read(|connection| connection.query_row(
            "SELECT document FROM pdf_parts WHERE article=?1 AND expires>?2 ORDER BY created DESC LIMIT 1",
            params![article, Self::now()], |row| row.get(0),
        ).optional().map_err(db))
    }
    pub(crate) fn pdf_put(
        &mut self,
        article: &str,
        url: &str,
        document: &str,
    ) -> Result<(), CacheError> {
        let now = Self::now();
        self.write(|tx| {
            tx.execute("DELETE FROM pdf_parts WHERE expires<=?1 OR article=?2", params![now, article]).map_err(db)?;
            tx.execute("INSERT INTO pdf_parts(article,url,created,expires,document) VALUES(?1,?2,?3,?4,?5)",
                params![article, url, now, now.saturating_add(LIFETIME), document]).map_err(db)?;
            Ok(())
        })
    }
}
