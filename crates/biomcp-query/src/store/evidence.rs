//! Checked evidence continuation state belongs to the existing per-command store.
use super::{CacheError,Store,db};
use biodata::Text;
use rusqlite::{OptionalExtension,params};
impl Store<'_> {
    pub(crate) fn evidence(&self,id:&str,key:&[u8])->Result<Option<String>,CacheError> {
        self.read(|cx|cx.query_row("SELECT payload FROM citation_evidence WHERE id=?1 AND request_key=?2 AND expires>?3",params![id,key,Self::now()],|row|row.get(0)).optional().map_err(db))
    }
    pub(crate) fn save_evidence(&mut self,id:Option<&Text>,key:&[u8],payload:&str)->Result<Text,CacheError> {
        self.write(|tx| {
            let id=match id {Some(id)=>id.to_string(),None=>tx.query_row("SELECT lower(hex(randomblob(16)))",[],|r|r.get(0)).map_err(db)?};
            tx.execute("INSERT INTO citation_evidence(id,request_key,expires,payload) VALUES (?1,?2,?3,?4) ON CONFLICT(id) DO UPDATE SET payload=excluded.payload",params![id,key,Self::now().saturating_add(86400),payload]).map_err(db)?;
            Text::new(&id).map_err(|_|CacheError::Database)
        })
    }
}
