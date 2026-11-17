//! g:Profiler over-representation results with the statistics the service states.
use super::ResponseBody;
use crate::{EnrichmentCorrection, EnrichmentTerms, GeneEnrichmentQuery};
use biodata::Text;
use serde::{Deserialize, Serialize};

/// Terms g:Profiler finds over-represented in a gene list, smallest adjusted p value first.
/// Over-representation is a statistical association, not evidence that the genes act together.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GeneEnrichment {
    /// g:Profiler data release, which names its Ensembl and Ensembl Genomes versions.
    pub release: Text,
    /// The test g:Profiler documents for g:GOSt.
    pub test: Box<str>,
    /// The multiple-testing correction g:Profiler states it applied.
    pub correction: EnrichmentCorrection,
    /// Adjusted p value threshold; g:Profiler returns only terms at or below it.
    pub threshold: f64,
    /// The background g:Profiler states it used; each term gives its size.
    pub background: Box<str>,
    /// Inputs g:Profiler could not map to a human gene; they are not tested.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub unmapped: Box<[Text]>,
    /// Inputs g:Profiler mapped to more than one gene; it tests its first choice.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub ambiguous: Box<[Text]>,
    /// Significant terms in g:Profiler's order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[EnrichedTerm]>,
}

/// One over-represented term as g:Profiler reports it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct EnrichedTerm {
    /// g:Profiler term ID, such as REAC:R-HSA-5693538 or GO:0006281.
    pub id: Text,
    /// Term name.
    pub name: Text,
    /// Term source.
    pub terms: EnrichmentTerms,
    /// Adjusted p value under the stated correction.
    pub p_value: f64,
    /// Input genes annotated to this term.
    pub overlap: u64,
    /// Input genes annotated in this term source.
    pub query_size: u64,
    /// Background genes annotated to this term.
    pub term_size: u64,
    /// Background size: genes annotated in this term source.
    pub background_size: u64,
    /// Inputs annotated to this term, in input order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub genes: Box<[Text]>,
}

impl ResponseBody for GeneEnrichment {
    const SCHEMA: &'static str = "biomcp.gene-enrichment/1";
    type Query = GeneEnrichmentQuery;
}
