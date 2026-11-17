//! g:Profiler g:GOSt over-representation analysis of a human gene list.
//!
//! Only openly licensed term sources are requested: Reactome, Gene Ontology and WikiPathways.
//! KEGG, TRANSFAC, CORUM, HPA, HP and miRTarBase carry their own terms and are never asked.
use crate::quotas;
use biodata::Text;
use biomcp_fetch::{BadReply, Endpoint, Freshness, Leg, Read, Reply, RequestError, SourceSpec};
use biomcp_types::{
    EnrichedTerm, EnrichmentCorrection, EnrichmentTerms, GeneEnrichment, GeneEnrichmentQuery,
    Source,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::Duration;

/// Public g:Profiler API policy.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "g:Profiler terms of service: free, including commercial use; cite g:Profiler and each term source",
        url: "https://biit.cs.ut.ee/gprofiler/page/terms-of-service",
        usage: biomcp_types::Use::Attribution,
    },
    source: Source::Gprofiler,
    base: "https://biit.cs.ut.ee/gprofiler",
    key: biomcp_fetch::KeyUse::None,
    quota: &quotas::GPROFILER,
    deadline: Duration::from_secs(30),
    command_budget: Some(Duration::from_secs(35)),
};
const PROFILE: Endpoint = SPEC
    .post_json("/api/gost/profile/")
    .idempotent()
    .fresh(Freshness::Days(7));

/// The test g:Profiler's documentation names for g:GOSt.
pub const TEST: &str = "Fisher's one-tailed test (cumulative hypergeometric), as g:Profiler documents for g:GOSt";
const THRESHOLD: f64 = 0.05;
const ORGANISM: &str = "hsapiens";
const DOMAIN: &str = "annotated";

#[derive(Serialize)]
struct Body<'a> {
    organism: &'static str,
    query: Vec<&'a str>,
    sources: Vec<&'static str>,
    user_threshold: f64,
    significance_threshold_method: &'static str,
    domain_scope: &'static str,
    all_results: bool,
    no_evidences: bool,
}

#[derive(Deserialize)]
struct Profile {
    result: Vec<Row>,
    meta: Meta,
}
#[derive(Deserialize)]
struct Row {
    native: Text,
    name: Text,
    source: String,
    p_value: f64,
    term_size: u64,
    query_size: u64,
    intersection_size: u64,
    effective_domain_size: u64,
    #[serde(default)]
    intersections: Vec<Vec<String>>,
    significant: bool,
    query: String,
}
#[derive(Deserialize)]
struct Meta {
    query_metadata: QueryMetadata,
    genes_metadata: GenesMetadata,
    version: Text,
}
#[derive(Deserialize)]
struct QueryMetadata {
    significance_threshold_method: String,
    user_threshold: f64,
    domain_scope: String,
    organism: String,
}
#[derive(Deserialize)]
struct GenesMetadata {
    #[serde(default)]
    failed: Vec<Text>,
    #[serde(default)]
    ambiguous: BTreeMap<String, serde_json::Value>,
    #[serde(default)]
    query: BTreeMap<String, QueryGenes>,
}
#[derive(Deserialize)]
struct QueryGenes {
    #[serde(default)]
    mapping: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    ensgs: Vec<String>,
}
impl Reply for Profile {
    fn check(&self) -> Result<(), BadReply> {
        let q = &self.meta.query_metadata;
        let known = EnrichmentCorrection::from_native(&q.significance_threshold_method).is_some();
        let rows = self.result.iter().all(|row| {
            row.significant
                && row.query == "query_1"
                && EnrichmentTerms::from_native(&row.source).is_some()
                && row.p_value.is_finite()
                && (0.0..=1.0).contains(&row.p_value)
        });
        if known && rows && q.organism == ORGANISM && q.domain_scope == DOMAIN {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}

/// Every significant term for the gene list, in g:Profiler's order; the caller pages them.
pub fn enrichment(
    leg: &Leg<'_>,
    query: &GeneEnrichmentQuery,
) -> Result<(GeneEnrichment, Option<i64>), RequestError> {
    let correction = query.correction();
    let body = Body {
        organism: ORGANISM,
        query: query.genes().iter().map(Text::as_str).collect(),
        sources: query.terms().into_iter().map(EnrichmentTerms::native).collect(),
        user_threshold: THRESHOLD,
        significance_threshold_method: correction.native(),
        domain_scope: DOMAIN,
        all_results: false,
        no_evidences: false,
    };
    let received = leg.call(&PROFILE, biomcp_fetch::Input::json(&[], &body), Read::json())?;
    let profile: Profile = received.value;
    let fetched = received.meta.fetched;
    let stated = EnrichmentCorrection::from_native(
        &profile.meta.query_metadata.significance_threshold_method,
    );
    if stated != Some(correction) {
        return Err(biomcp_fetch::bad_reply(Source::Gprofiler, Some(200)));
    }
    let genes = profile.meta.genes_metadata;
    // g:Profiler reports each term's input genes against its list of distinct mapped genes.
    let mapped = genes.query.get("query_1");
    let names: Vec<Option<Text>> = mapped
        .map(|q| {
            q.ensgs
                .iter()
                .map(|ensg| {
                    query
                        .genes()
                        .iter()
                        .find(|input| {
                            q.mapping
                                .get(input.as_str())
                                .is_some_and(|ids| ids.contains(ensg))
                        })
                        .cloned()
                        .or_else(|| Text::new(ensg).ok())
                })
                .collect()
        })
        .unwrap_or_default();
    let items = profile
        .result
        .into_iter()
        .filter_map(|row| {
            let terms = EnrichmentTerms::from_native(&row.source)?;
            let genes = row
                .intersections
                .iter()
                .zip(&names)
                .filter(|(evidence, _)| !evidence.is_empty())
                .filter_map(|(_, name)| name.clone())
                .collect();
            Some(EnrichedTerm {
                id: row.native,
                name: row.name,
                terms,
                p_value: row.p_value,
                overlap: row.intersection_size,
                query_size: row.query_size,
                term_size: row.term_size,
                background_size: row.effective_domain_size,
                genes,
            })
        })
        .collect();
    let ambiguous = genes
        .ambiguous
        .keys()
        .filter_map(|name| Text::new(name).ok())
        .collect();
    Ok((
        GeneEnrichment {
            release: profile.meta.version,
            test: TEST.into(),
            correction,
            threshold: profile.meta.query_metadata.user_threshold,
            background: "genes with at least one annotation in the term source (g:Profiler's annotated domain)".into(),
            unmapped: genes.failed.into(),
            ambiguous,
            items,
        },
        fetched,
    ))
}
/// Complete source policy and command declaration.
pub(crate) mod decl;
