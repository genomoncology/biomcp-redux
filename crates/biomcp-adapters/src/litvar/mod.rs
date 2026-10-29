//! Exact LitVar2 candidate selection and source-ordered publication identifiers.
use crate::quotas;
use biodata::{Pmid, Text};

use biomcp_fetch::{
    Arg, BadReply, Endpoint, Freshness, KeyUse, Leg, PathSegment, Reply, RequestError, SourceSpec,
};
use biomcp_types::{LitVarCandidate, Source};
use serde::Deserialize;
use std::time::Duration;
/// Public NCBI LitVar2 policy, independent of E-utilities credentials.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "NCBI public-domain metadata policy",
        url: "https://www.ncbi.nlm.nih.gov/home/about/policies/",
        usage: biomcp_types::Use::Open,
    },
    source: Source::LitVar,
    base: "https://www.ncbi.nlm.nih.gov/research/litvar2-api",
    key: KeyUse::None,
    quota: &quotas::LITVAR,
    deadline: Duration::from_secs(15),
};
pub(crate) const AUTOCOMPLETE: Endpoint =
    SPEC.get("/variant/autocomplete/").fresh(Freshness::Days(1));
const PUBLICATIONS: Endpoint = SPEC.get("/variant/get").fresh(Freshness::Days(1));
/// Exact source identity retained between autocomplete and publications.
#[derive(Debug)]
pub struct Variant {
    id: String,
    term: Text,
}
impl Variant {
    /// Source rsID, or its variant name when it has no rsID.
    pub fn term(&self) -> &Text {
        &self.term
    }
}
/// Result of exact autocomplete selection, before any publication search.
#[derive(Debug)]
pub enum Selection {
    /// Exactly one source variant matched.
    Exact(Variant),
    /// No publications call may be made.
    Candidates(Vec<LitVarCandidate>),
}
/// One selected variant's whole source count and bounded native sequence.
#[derive(Debug)]
pub struct Search {
    /// Selected source rsID or variant name.
    pub term: Text,
    /// Whole source publication count.
    pub count: u64,
    /// Native publication sequence capped at the pool limit.
    pub ids: Vec<Pmid>,
}
/// Select by exact case-insensitive gene/change or rsID, never by hit position.
pub fn select(leg: &Leg<'_>, gene: Option<&str>, variant: &str) -> Result<Selection, RequestError> {
    let rsid = is_rsid(variant);
    let change = variant
        .get(..2)
        .filter(|s| s.eq_ignore_ascii_case("p."))
        .map_or(variant, |_| variant.get(2..).unwrap_or(variant));
    let term = if rsid {
        variant.to_owned()
    } else {
        format!("{} {change}", gene.unwrap_or_default())
    };
    let candidates = leg.json::<Candidates>(&AUTOCOMPLETE, &[("query", Arg::Text(&term))])?;
    let mut matching = candidates.0.iter().filter(|row| {
        if rsid {
            row.rsid
                .as_ref()
                .is_some_and(|rsid| rsid.as_str().eq_ignore_ascii_case(variant))
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
        return Ok(Selection::Candidates(
            candidates
                .0
                .into_iter()
                .take(5)
                .map(Candidate::public)
                .collect(),
        ));
    };
    Ok(Selection::Exact(Variant {
        id: selected.id.clone(),
        term: selected.rsid.as_ref().unwrap_or(&selected.name).clone(),
    }))
}
/// Fetch publications only after exact source identity selection.
pub fn publications(leg: &Leg<'_>, selected: Variant) -> Result<Search, RequestError> {
    let segments = [&selected.id, "publications"]
        .into_iter()
        .map(PathSegment::new)
        .collect::<Result<Vec<_>, _>>()?;
    let reply = leg.json::<Publications>(&PUBLICATIONS.clone().with_segments(&segments), &[])?;
    Ok(Search {
        term: selected.term,
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
    rsid: Option<Text>,
    gene: Vec<Text>,
    #[serde(deserialize_with = "crate::native_text::deserialize")]
    name: biodata::Text,
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
        if self.0.len() > 10_000
            || self.0.iter().any(|c| {
                PathSegment::new(&c.id).is_err()
                    || c.rsid.as_ref().is_some_and(|rsid| !is_rsid(rsid.as_str()))
            })
        {
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
    fn discovery(&self) -> Option<biomcp_fetch::Identities> {
        biomcp_fetch::Identities::bounded(
            "pmid",
            self.pmids.iter().map(ToString::to_string).collect(),
        )
    }
    fn check(&self) -> Result<(), BadReply> {
        if self.pmids.len() as u64 > self.pmids_count || self.pmids.contains(&0) {
            return Err(BadReply);
        }
        Ok(())
    }
}

/// Recognize the exact rsID spelling accepted by variant lookup.
pub fn is_rsid(value: &str) -> bool {
    value
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("rs"))
        && value.get(2..).is_some_and(|digits| {
            !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
        })
}
