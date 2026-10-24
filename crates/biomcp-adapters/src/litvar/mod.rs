//! Exact LitVar2 candidate selection and source-ordered publication identifiers.
use crate::quotas;
use biodata::{LitVarCandidate, Pmid, Source, Text};
use biomcp_fetch::{
    Arg, BadReply, Endpoint, Freshness, KeyUse, Leg, PathSegment, Reply, RequestError, SourceSpec,
};
use serde::Deserialize;
use std::time::Duration;
/// Public NCBI LitVar2 policy, independent of E-utilities credentials.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::LitVar,
    base: "https://www.ncbi.nlm.nih.gov/research/litvar2-api",
    key: KeyUse::None,
    quota: &quotas::LITVAR,
    deadline: Duration::from_secs(15),
};
const AUTOCOMPLETE: Endpoint = SPEC.get("/variant/autocomplete/").fresh(Freshness::Days(1));
const PUBLICATIONS: Endpoint = SPEC.get("/variant/get").fresh(Freshness::Days(1));
/// One checked result, or nearby candidates when exact selection was impossible.
#[derive(Debug)]
pub enum Search {
    /// Exactly one matched variant's native publication order.
    Selected {
        /// Selected exact rsID.
        rsid: Text,
        /// Whole source publication count.
        count: u64,
        /// Native publication sequence capped at the pool limit.
        ids: Vec<Pmid>,
    },
    /// No publications call was made.
    Candidates(Vec<LitVarCandidate>),
}
/// Select by exact case-insensitive gene/change or rsID, never by hit position.
pub fn search(leg: &Leg<'_>, gene: Option<&str>, variant: &str) -> Result<Search, RequestError> {
    let is_rsid = variant
        .get(..2)
        .is_some_and(|s| s.eq_ignore_ascii_case("rs"))
        && variant
            .get(2..)
            .is_some_and(|s| !s.is_empty() && s.bytes().all(|c| c.is_ascii_digit()));
    let change = variant
        .get(..2)
        .filter(|s| s.eq_ignore_ascii_case("p."))
        .map_or(variant, |_| variant.get(2..).unwrap_or(variant));
    let term = if is_rsid {
        variant.to_owned()
    } else {
        format!("{} {change}", gene.unwrap_or_default())
    };
    let candidates = leg.json::<Candidates>(&AUTOCOMPLETE, &[("query", Arg::Text(&term))])?;
    let mut matching = candidates.0.iter().filter(|row| {
        if is_rsid {
            row.rsid.as_str().eq_ignore_ascii_case(variant)
        } else {
            gene.is_some_and(|gene| {
                row.gene
                    .iter()
                    .any(|g| g.as_str().eq_ignore_ascii_case(gene))
            }) && row
                .name
                .as_str()
                .eq_ignore_ascii_case(&format!("p.{change}"))
        }
    });
    let Some(selected) = matching.next().filter(|_| matching.next().is_none()) else {
        return Ok(Search::Candidates(
            candidates
                .0
                .into_iter()
                .take(5)
                .map(Candidate::public)
                .collect(),
        ));
    };
    let segments = [&selected.id, "publications"]
        .into_iter()
        .map(PathSegment::new)
        .collect::<Result<Vec<_>, _>>()?;
    let reply = leg.json::<Publications>(&PUBLICATIONS.clone().with_segments(&segments), &[])?;
    Ok(Search::Selected {
        rsid: selected.rsid.clone(),
        count: reply.pmids_count,
        ids: reply
            .pmids
            .into_iter()
            .take(9_999)
            .filter_map(|id| Pmid::new(id).ok())
            .collect(),
    })
}
#[derive(Deserialize)]
struct Candidates(Vec<Candidate>);
#[derive(Deserialize)]
struct Candidate {
    #[serde(rename = "_id", alias = "id")]
    id: String,
    rsid: Text,
    gene: Vec<Text>,
    name: Text,
    pmids_count: u64,
}
impl Candidate {
    fn public(self) -> LitVarCandidate {
        LitVarCandidate {
            rsid: self.rsid,
            gene: self.gene,
            name: self.name,
            count: self.pmids_count,
        }
    }
}
impl Reply for Candidates {
    fn check(&self) -> Result<(), BadReply> {
        if self.0.len() > 10_000 || self.0.iter().any(|c| PathSegment::new(&c.id).is_err()) {
            return Err(BadReply);
        }
        Ok(())
    }
}
#[derive(Deserialize)]
struct Publications {
    pmids: Vec<u32>,
    pmids_count: u64,
}
impl Reply for Publications {
    fn check(&self) -> Result<(), BadReply> {
        if self.pmids.len() as u64 > self.pmids_count || self.pmids.contains(&0) {
            return Err(BadReply);
        }
        Ok(())
    }
}
