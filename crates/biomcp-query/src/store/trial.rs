//! Source snapshots use the existing store owner, deadlines and source admission.
use super::{CacheError, Store, db};
use biodata::Text;
use biomcp_types::{Source, SourceOutcome};
use rusqlite::{OptionalExtension, params};
use sha2::{Digest, Sha256};
const SNAPSHOT_SECONDS: i64 = 86400;
impl Store<'_> {
    /// Read source dependencies before any saved protocol content.
    pub(crate) fn source_snapshot(
        &self,
        id: &str,
        key: &[u8],
        disabled: impl Fn(Source) -> bool,
    ) -> Result<Option<String>, CacheError> {
        self.read(|cx| {
            let outcomes:Option<String>=cx.query_row("SELECT outcomes FROM trial_snapshot WHERE id=?1 AND request_key=?2 AND expires>?3",params![id,key,Self::now()],|r|r.get(0)).optional().map_err(db)?;
            let Some(outcomes)=outcomes else {return Ok(None)};
            let sources:Vec<SourceOutcome>=serde_json::from_str(&outcomes).map_err(|_|CacheError::Database)?;
            if sources.iter().any(|s|disabled(s.source)) {return Ok(None)}
            cx.query_row("SELECT payload FROM trial_snapshot WHERE id=?1",params![id],|r|r.get(0)).map(Some).map_err(db)
        })
    }
    /// Check a retained source pool's binding without reading its saved content.
    pub(crate) fn source_snapshot_matches(
        &self,
        id: &str,
        key: &[u8],
    ) -> Result<Option<bool>, CacheError> {
        self.read(|cx| {
            cx.query_row(
                "SELECT request_key FROM trial_snapshot WHERE id=?1 AND expires>?2",
                params![id, Self::now()],
                |r| r.get::<_, Vec<u8>>(0),
            )
            .optional()
            .map(|stored| stored.map(|stored| stored == key))
            .map_err(db)
        })
    }
    /// Metadata of a snapshot already admitted through source_snapshot.
    pub(crate) fn source_snapshot_info(
        &self,
        id: Text,
        size: u32,
    ) -> Result<biomcp_types::PoolInfo, CacheError> {
        self.read(|cx| {
            let expires: i64 = cx
                .query_row(
                    "SELECT expires FROM trial_snapshot WHERE id=?1",
                    [id.as_str()],
                    |r| r.get(0),
                )
                .map_err(db)?;
            Ok(biomcp_types::PoolInfo {
                id,
                size,
                created: u64::try_from(expires.saturating_sub(SNAPSHOT_SECONDS))
                    .map_err(|_| CacheError::Database)?,
                expires: u64::try_from(expires).map_err(|_| CacheError::Database)?,
            })
        })
    }
    pub(crate) fn save_source_snapshot(
        &mut self,
        key: &[u8],
        payload: &str,
        sources: &[SourceOutcome],
    ) -> Result<Text, CacheError> {
        self.save_snapshot(key, payload, sources, false)
    }
    /// Publish a distinct immutable source pool with a fixed lifetime.
    pub(crate) fn publish_source_pool(
        &mut self,
        key: &[u8],
        payload: &str,
        sources: &[SourceOutcome],
    ) -> Result<Text, CacheError> {
        self.save_snapshot(key, payload, sources, true)
    }
    fn save_snapshot(
        &mut self,
        key: &[u8],
        payload: &str,
        sources: &[SourceOutcome],
        distinct: bool,
    ) -> Result<Text, CacheError> {
        self.write(|tx| {
            Self::collect_expired(tx)?;
            let id = if distinct { tx.query_row("SELECT lower(hex(randomblob(32)))", [], |r| r.get::<_, String>(0)).map_err(db)? } else { Sha256::digest([key,payload.as_bytes()].concat()).iter().map(|b|format!("{b:02x}")).collect::<String>() };
            let outcomes=serde_json::to_string(sources).map_err(|_|CacheError::Database)?;
            tx.execute("INSERT INTO trial_snapshot(id,request_key,expires,payload,outcomes) VALUES (?1,?2,?3,?4,?5) ON CONFLICT(id) DO UPDATE SET expires=excluded.expires",params![id,key,Self::now().saturating_add(SNAPSHOT_SECONDS),payload,outcomes]).map_err(db)?;
            Text::new(&id).map_err(|_|CacheError::Database)
        })
    }
}
