use crate::{ArticleHeader, ArticleSearch, ArticleSource, Doi, EuropePmcId, Pmcid, Pmid};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// Stable identity independent of whether citation metadata was accepted.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "namespace", content = "id", rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ArticleKey {
    /// PubMed identity.
    Pubmed(Pmid),
    /// PMC identity when PubMed is absent.
    Pmc(Pmcid),
    /// DOI when both indexed numeric identities are absent.
    Doi(Doi),
    /// Europe PMC local source identity.
    Europepmc(EuropePmcId),
}
/// One original pool position, including a summary hole.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ArticleItem {
    /// Stable checked pool key.
    pub key: ArticleKey,
    /// The selected PMID, retained when summary mapping fails.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "Pmid"))]
    pub pmid: Option<Pmid>,
    /// Native one-based source positions, disclosed only for multiple sources.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub found_by: BTreeMap<ArticleSource, u32>,
    /// iCite citing-paper count, only under citation ordering.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cited_by: Option<u64>,
    /// Citation metadata when accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<ArticleHeader>,
}
/// A slice of the stored article source order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ArticlePage {
    /// Original ordered items.
    pub items: Box<[ArticleItem]>,
    /// Nearby source candidates when an exact variant was not uniquely selected.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub candidates: Vec<LitVarCandidate>,
}
impl super::ResponseBody for ArticlePage {
    const SCHEMA: &'static str = "biomcp.article_page/1";
    type Query = ArticleSearch;
}

/// A page of citing or referenced papers in citation order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ArticleEdgePage {
    /// Papers in the fixed edge pool.
    pub items: Box<[ArticleItem]>,
}
impl super::ResponseBody for ArticleEdgePage {
    const SCHEMA: &'static str = "biomcp.article_edges/1";
    type Query = crate::ArticleEdges;
}

/// Public LitVar2 autocomplete candidate, without a clinical ranking claim.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct LitVarCandidate {
    /// Source rsID, available for exact follow-up.
    pub rsid: crate::Text,
    /// Source gene symbols.
    pub gene: Vec<crate::Text>,
    /// Source variant name.
    pub name: crate::Text,
    /// Number of associated publications reported by LitVar2.
    pub count: u64,
}
