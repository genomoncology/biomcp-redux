//! A gene list sent to g:Profiler's over-representation analysis.
use super::gene::QueryInvalid;
use biodata::Text;
use serde::{Deserialize, Serialize};

biodata::wire_enum! {
    described
    /// One openly licensed g:Profiler term source; licensed and noncommercial sources are not offered.
    EnrichmentTerms {
        #[doc = "Reactome pathways."] Reactome => "reactome",
        #[doc = "Gene Ontology biological processes."] GoBp => "go-bp",
        #[doc = "Gene Ontology molecular functions."] GoMf => "go-mf",
        #[doc = "Gene Ontology cellular components."] GoCc => "go-cc",
        #[doc = "WikiPathways pathways."] Wikipathways => "wikipathways"
    }
}
biodata::wire_enum! {
    #[derive(Default)]
    /// g:Profiler's multiple-testing correction.
    EnrichmentCorrection {
        #[default]
        #[doc = "g:SCS, g:Profiler's default."] GScs => "g-scs",
        #[doc = "Benjamini-Hochberg false discovery rate."] Fdr => "fdr",
        #[doc = "Bonferroni."] Bonferroni => "bonferroni"
    }
}
impl EnrichmentTerms {
    /// g:Profiler's source code for this term source.
    pub const fn native(self) -> &'static str {
        match self {
            Self::Reactome => "REAC",
            Self::GoBp => "GO:BP",
            Self::GoMf => "GO:MF",
            Self::GoCc => "GO:CC",
            Self::Wikipathways => "WP",
        }
    }
    /// The term source for a g:Profiler source code, when it is one BioMCP offers.
    pub fn from_native(code: &str) -> Option<Self> {
        [
            Self::Reactome,
            Self::GoBp,
            Self::GoMf,
            Self::GoCc,
            Self::Wikipathways,
        ]
        .into_iter()
        .find(|terms| terms.native() == code)
    }
}
impl EnrichmentCorrection {
    /// g:Profiler's name for this correction.
    pub const fn native(self) -> &'static str {
        match self {
            Self::GScs => "g_SCS",
            Self::Fdr => "fdr",
            Self::Bonferroni => "bonferroni",
        }
    }
    /// The correction g:Profiler states it applied.
    pub fn from_native(code: &str) -> Option<Self> {
        [Self::GScs, Self::Fdr, Self::Bonferroni]
            .into_iter()
            .find(|correction| correction.native() == code)
    }
}

/// Over-represented terms for a human gene list from g:Profiler, with its own statistics.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct GeneEnrichmentQuery(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// One to 500 human gene symbols or Ensembl, Entrez or UniProt IDs, as g:Profiler reads them.
    #[cfg_attr(feature = "schema", schemars(length(min = 1, max = 500), extend("x-positional" = 0)))]
    genes: Box<[Text]>,
    /// Term sources to test; all five by default.
    #[serde(default = "all_terms")]
    #[cfg_attr(feature = "schema", schemars(length(min = 1, max = 5), extend("x-comma-separated" = true)))]
    terms: Box<[EnrichmentTerms]>,
    /// Multiple-testing correction; g:SCS by default.
    #[serde(default)]
    correction: EnrichmentCorrection,
    /// Ten terms by default, one to fifty, smallest adjusted p value first.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Terms to skip in that order.
    #[serde(default)]
    offset: u32,
}
fn all_terms() -> Box<[EnrichmentTerms]> {
    Box::new([
        EnrichmentTerms::Reactome,
        EnrichmentTerms::GoBp,
        EnrichmentTerms::GoMf,
        EnrichmentTerms::GoCc,
        EnrichmentTerms::Wikipathways,
    ])
}
fn default_limit() -> u32 {
    10
}
crate::query::validation::checked_query!(GeneEnrichmentQuery, Fields);
impl TryFrom<Fields> for GeneEnrichmentQuery {
    type Error = QueryInvalid;
    fn try_from(v: Fields) -> Result<Self, QueryInvalid> {
        if v.genes.is_empty()
            || v.genes.len() > 500
            || v.genes.iter().any(|gene| {
                let gene = gene.as_str();
                gene.is_empty() || gene.len() > 64 || gene.chars().any(char::is_whitespace)
            })
        {
            return Err(QueryInvalid::new(&["genes"]));
        }
        if v.terms.is_empty() || v.terms.len() > 5 {
            return Err(QueryInvalid::new(&["terms"]));
        }
        if !(1..=50).contains(&v.limit) {
            return Err(QueryInvalid::new(&["limit"]));
        }
        Ok(Self(v))
    }
}
impl GeneEnrichmentQuery {
    /// Gene inputs in caller order.
    pub fn genes(&self) -> &[Text] {
        &self.0.genes
    }
    /// Term sources to test, each once, in caller order.
    pub fn terms(&self) -> Vec<EnrichmentTerms> {
        let mut terms = Vec::new();
        for term in self.0.terms.iter() {
            if !terms.contains(term) {
                terms.push(*term);
            }
        }
        terms
    }
    /// Multiple-testing correction.
    pub fn correction(&self) -> EnrichmentCorrection {
        self.0.correction
    }
    /// Terms per page.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Terms skipped.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// The same analysis at another offset.
    pub fn at_offset(&self, offset: u32) -> Self {
        let mut query = self.clone();
        query.0.offset = offset;
        query
    }
}
