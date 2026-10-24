//! Explicit keyed NCI trial access, without registry vocabulary translation.
use biodata::{NciDisease, NciTrial, NctId, Reason, Text, TrialSearch};
use biomcp_fetch::{Arg, BadReply, Credential, Endpoint, Freshness, KeyPlace, KeyUse, Leg, Reply, RequestError, SecretName, SourceSpec};
use serde::Deserialize;
use std::time::Duration;
/// NCI requires the user's own API key for every request, including health probes.
pub static SPEC: SourceSpec = SourceSpec {
    source: biodata::Source::Nci,
    base: "https://clinicaltrialsapi.cancer.gov/api/v2",
    key: KeyUse::Required(Credential { secret: SecretName::NciApiKey, place: KeyPlace::Header("X-API-KEY") }),
    quota: &crate::quotas::NCI,
    deadline: Duration::from_secs(15),
    terms: biomcp_fetch::Terms {
        name: "NCI Clinical Trials Search publisher terms; user's API key required",
        url: "https://www.cancer.gov/syndication/api",
        usage: biodata::Use::Restricted,
    },
};
/// Native trial search, also used for one bounded health request.
pub const SEARCH: Endpoint = SPEC.get("/trials").fresh(Freshness::Days(1));
const DETAIL: Endpoint = SPEC.get("/trials").fresh(Freshness::Days(1));
/// Native page, preserving provider counts and order.
#[derive(Debug, Deserialize)]
pub struct Page {
    /// Publisher total, never a local count.
    pub total: u64,
    /// Checked publisher trial rows.
    pub data: Vec<Trial>,
}
impl Reply for Page {
    fn check(&self) -> Result<(), BadReply> {
        if self.data.len() > 50 || self.data.len() as u64 > self.total { return Err(BadReply); }
        self.data.iter().try_for_each(Reply::check)
    }
}
/// Source-native identity and card fields. Extra source fields remain adapter-owned.
#[derive(Debug, Deserialize)]
pub struct Trial {
    nci_id: Text,
    nct_id: NctId,
    brief_title: Text,
    current_trial_status: Text,
    phase: Option<Text>,
    brief_summary: Option<Text>,
    #[serde(default)]
    diseases: Vec<Disease>,
}
#[derive(Debug, Deserialize)]
struct Disease { name: Text, nci_thesaurus_concept_id: Text }
impl Reply for Trial {
    fn check(&self) -> Result<(), BadReply> {
        if !self.nci_id.as_str().starts_with("NCI-") { return Err(BadReply); }
        Ok(())
    }
}
impl Trial {
    /// Native publisher card, with no clinical interpretation or inferred fields.
    pub fn card(self, summary: bool) -> NciTrial {
        NciTrial { nci_id: self.nci_id, nct_id: self.nct_id, brief_title: self.brief_title,
            current_trial_status: self.current_trial_status, phase: self.phase,
            brief_summary: if summary {self.brief_summary} else {None},
            diseases: self.diseases.into_iter().map(|d| NciDisease {name:d.name,nci_thesaurus_concept_id:d.nci_thesaurus_concept_id}).collect() }
    }
}
/// Exact native parameters. Repeated includes remain separate parameters.
pub fn parameters(query: &TrialSearch) -> Vec<(String, String)> {
    let mut params = vec![("size".into(),query.limit().min(50).to_string()),("from".into(),query.next_page().map_or("0",Text::as_str).into())];
    for (name,value) in [("diseases.nci_thesaurus_concept_id",query.condition()),("keyword",query.keyword())] {
        if let Some(value)=value {params.push((name.into(),value.as_str().into()));}
    }
    for field in ["nci_id","nct_id","brief_title","current_trial_status","phase","diseases"] {
        params.push(("include".into(),field.into()));
    }
    params
}
/// Retrieve and check one source-native search page before cache admission.
pub fn search(leg: &Leg<'_>, params: &[(String,String)]) -> Result<Page, RequestError> {
    let args: Vec<_> = params.iter().map(|(k,v)|(k.as_str(),Arg::Text(v))).collect();
    leg.json(&SEARCH,&args)
}
/// Exact identity check runs before cache admission and on replay or cache hits.
pub fn get(leg: &Leg<'_>, id: NctId) -> Result<(NciTrial,biomcp_fetch::ReplyMetadata),RequestError> {
    let endpoint = DETAIL.clone().with_path(format!("/trials/{id}"))?;
    let (raw,metadata) = leg.json_received_checked::<Trial>(&endpoint,&[],|r| {
        if r.nct_id == id {Ok(())} else {Err(BadReply)}
    })?;
    Ok((raw.card(true),metadata))
}
/// Source-native missing-data failures stay distinct from confirmed empty results.
pub fn failure(reason: Reason) -> RequestError {
    biomcp_fetch::SourceError {source:biodata::Source::Nci,reason,status:None,retry_after:None}.into()
}
