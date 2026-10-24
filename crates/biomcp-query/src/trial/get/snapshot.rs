use biodata::{
    SourceOutcome, Text, TrialArm, TrialCard, TrialContact, TrialEligibility, TrialIntervention,
    TrialLocation, TrialOfficial, TrialOutcome,
};
use biomcp_adapters::ctgov::detail::Detail;
use biomcp_fetch::CacheError;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
pub(super) struct Snapshot {
    pub(super) detail: Detail,
    pub(super) fetched: u64,
    pub(super) version: Option<Text>,
    pub(super) sources: Vec<SourceOutcome>,
}
// This named shape supersedes /1's positional tuple. The owner key changes so old
// capabilities expire before decode; the VERSION4 database tables remain valid.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Saved {
    card: TrialCard,
    eligibility: Option<TrialEligibility>,
    outcomes: Box<[TrialOutcome]>,
    arms: Box<[TrialArm]>,
    interventions: Box<[TrialIntervention]>,
    locations: Box<[TrialLocation]>,
    contacts: Box<[TrialContact]>,
    officials: Box<[TrialOfficial]>,
    fetched: u64,
    version: Option<Text>,
    sources: Vec<SourceOutcome>,
}
impl Snapshot {
    pub(super) fn key(id: biodata::NctId) -> Vec<u8> {
        Sha256::digest(format!("biomcp.trial_snapshot/2\0{id}").as_bytes()).to_vec()
    }
    pub(super) fn encode(&self) -> Result<String, CacheError> {
        let d = &self.detail;
        serde_json::to_string(&Saved {
            card:d.card.clone(), eligibility:d.eligibility.clone(), outcomes:d.outcomes.clone(), arms:d.arms.clone(),
            interventions:d.interventions.clone(), locations:d.locations.clone(), contacts:d.contacts.clone(), officials:d.officials.clone(),
            fetched:self.fetched, version:self.version.clone(), sources:self.sources.clone(),
        }).map_err(|_| CacheError::Database)
    }
    pub(super) fn decode(payload: &str) -> Result<Self, CacheError> {
        let s: Saved = serde_json::from_str(payload).map_err(|_| CacheError::Database)?;
        Ok(Self {
            detail: Detail {
                card:s.card, eligibility:s.eligibility, outcomes:s.outcomes, arms:s.arms, interventions:s.interventions,
                locations:s.locations, contacts:s.contacts, officials:s.officials,
            },
            fetched:s.fetched, version:s.version, sources:s.sources,
        })
    }
}
