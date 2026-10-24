//! Public iCite metadata; no abstracts or provider-specific scoring blend.
use crate::quotas;
use biodata::{Citations, FiniteNumber, Pmid, Source};
use biomcp_fetch::{
    Arg, BadReply, Endpoint, Freshness, KeyUse, Leg, Reply, RequestError, SourceSpec,
};
use serde::Deserialize;
use std::{collections::BTreeSet, time::Duration};
/// Maximum documented PMID request size.
pub const BATCH_LIMIT: usize = 200;
/// Conservative application quota; no provider rate is claimed.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::Icite,
    base: "https://icite.od.nih.gov/api",
    key: KeyUse::None,
    quota: &quotas::ICITE,
    deadline: Duration::from_secs(15),
};
const PUBS: Endpoint = SPEC.get("/pubs").fresh(Freshness::Days(1));
const FIELDS: &str = "pmid,pubYear,citedByPmidCount,citedByPmids,citedPmids,citedByClinicalArticle,citingClinicalPmids,rcr,rcrIsProvisional";
/// One checked, independently identified iCite record.
#[derive(Debug)]
pub struct Publication {
    /// Native PubMed identity.
    pub pmid: Pmid,
    /// Counts with absent lists distinguished from confirmed empty lists.
    pub counts: Citations,
    /// Distinct citing paper IDs when the source supplied the list.
    pub citing: Option<Vec<Pmid>>,
    /// Distinct referenced paper IDs when the source supplied the list.
    pub references: Option<Vec<Pmid>>,
}
/// Request one documented metadata batch, joining only requested identities.
pub fn publications(leg: &Leg<'_>, ids: &[Pmid]) -> Result<Vec<Publication>, RequestError> {
    if ids.is_empty() || ids.len() > BATCH_LIMIT {
        return Err(bad());
    }
    let names: Vec<_> = ids
        .iter()
        .map(|id| id.to_string().into_boxed_str())
        .collect();
    let raw = leg.json::<Raw>(
        &PUBS,
        &[
            ("pmids", Arg::Ids(&names)),
            ("legacy", Arg::Text("false")),
            ("fl", Arg::Text(FIELDS)),
        ],
    )?;
    if raw.data.iter().any(|r| !ids.contains(&r.pmid.0)) {
        return Err(bad());
    }
    Ok(raw.data.into_iter().map(Row::map).collect())
}
fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::Icite,
        reason: biodata::Reason::BadReply,
        status: Some(200),
        retry_after: None,
    }
    .into()
}
#[derive(Deserialize)]
#[serde(try_from = "u32")]
struct Id(Pmid);
impl TryFrom<u32> for Id {
    type Error = biodata::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Pmid::new(value).map(Self)
    }
}
#[derive(Deserialize)]
struct Raw {
    data: Vec<Row>,
}
impl Reply for Raw {
    fn check(&self) -> Result<(), BadReply> {
        let ids: BTreeSet<_> = self.data.iter().map(|r| r.pmid.0).collect();
        if self.data.len() > BATCH_LIMIT || ids.len() != self.data.len() {
            return Err(BadReply);
        }
        Ok(())
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Row {
    pmid: Id,
    cited_by_pmid_count: Option<u64>,
    cited_by_pmids: Option<Vec<Id>>,
    cited_pmids: Option<Vec<Id>>,
    citing_clinical_pmids: Option<Vec<Id>>,
    rcr: Option<FiniteNumber>,
    rcr_is_provisional: Option<bool>,
}
fn distinct(ids: Vec<Id>) -> Vec<Pmid> {
    let mut seen = BTreeSet::new();
    ids.into_iter()
        .map(|id| id.0)
        .filter(|id| seen.insert(*id))
        .collect()
}
impl Row {
    fn map(self) -> Publication {
        let citing = self.cited_by_pmids.map(distinct);
        let references = self.cited_pmids.map(distinct);
        let clinical = self.citing_clinical_pmids.map(distinct);
        Publication {
            pmid: self.pmid.0,
            counts: Citations {
                cited_by: citing
                    .as_ref()
                    .map(|ids| ids.len() as u64)
                    .or(self.cited_by_pmid_count),
                cited_by_clinical: clinical.map(|ids| ids.len() as u64),
                references: references.as_ref().map(|ids| ids.len() as u64),
                rcr: self.rcr.filter(|_| self.rcr_is_provisional == Some(false)),
            },
            citing,
            references,
        }
    }
}
