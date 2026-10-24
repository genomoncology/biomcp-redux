use biodata::{SourceOutcome, Text, TrialArm, TrialCard, TrialEligibility, TrialIntervention, TrialOutcome};
use biomcp_adapters::ctgov::detail::Detail;
use biomcp_fetch::CacheError;
use sha2::{Digest,Sha256};
pub(super) struct Snapshot {
    pub detail: Detail,
    pub fetched: u64,
    pub version: Option<Text>,
    pub sources: Vec<SourceOutcome>,
}
type Saved = (TrialCard, Option<TrialEligibility>, Box<[TrialOutcome]>, Box<[TrialArm]>, Box<[TrialIntervention]>, u64, Option<Text>, Vec<SourceOutcome>);
impl Snapshot {
    pub fn key(id: biodata::NctId) -> Vec<u8> {
        Sha256::digest(format!("biomcp.trial_snapshot/1\0{id}").as_bytes()).to_vec()
    }
    pub fn encode(&self) -> Result<String, CacheError> {
        let d=&self.detail;
        serde_json::to_string(&(&d.card,&d.eligibility,&d.outcomes,&d.arms,&d.interventions,self.fetched,&self.version,&self.sources)).map_err(|_|CacheError::Database)
    }
    pub fn decode(payload:&str) -> Result<Self,CacheError> {
        let(card,eligibility,outcomes,arms,interventions,fetched,version,sources):Saved=serde_json::from_str(payload).map_err(|_|CacheError::Database)?;
        Ok(Self{detail:Detail{card,eligibility,outcomes,arms,interventions},fetched,version,sources})
    }
}
