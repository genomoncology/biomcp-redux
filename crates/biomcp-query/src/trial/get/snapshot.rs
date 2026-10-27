use biodata::Text;
use biomcp_adapters::ctgov::detail::Detail;
use biomcp_fetch::CacheError;
use biomcp_types::SourceOutcome;
use sha2::{Digest, Sha256};
pub(super) struct Snapshot {
    pub(super) detail: Detail,
    pub(super) fetched: u64,
    pub(super) version: Option<Text>,
    pub(super) sources: Vec<SourceOutcome>,
}
// Detail owns mandatory named fields; fixed query metadata follows it. The
// /3 key expires earlier named payloads before decode without changing the table.
type Saved = (Detail, u64, Option<Text>, Vec<SourceOutcome>);
impl Snapshot {
    pub(super) fn key(id: biodata::NctId) -> Vec<u8> {
        Sha256::digest(format!("biomcp.trial-snapshot/3\0{id}").as_bytes()).to_vec()
    }
    pub(super) fn encode(&self) -> Result<String, CacheError> {
        serde_json::to_string(&(&self.detail, self.fetched, &self.version, &self.sources))
            .map_err(|_| CacheError::Database)
    }
    pub(super) fn decode(payload: &str) -> Result<Self, CacheError> {
        let (detail, fetched, version, sources): Saved =
            serde_json::from_str(payload).map_err(|_| CacheError::Database)?;
        Ok(Self {
            detail,
            fetched,
            version,
            sources,
        })
    }
}
