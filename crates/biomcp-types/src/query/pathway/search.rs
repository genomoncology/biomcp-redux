use super::{PathwaySource, QueryInvalid, default_limit, valid_limit};
use crate::GeneInput;
use biodata::Text;
use serde::{Deserialize, Serialize};
/// Search exactly one source through text, an exact gene join or top-level discovery.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SearchFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "SearchFields"))]
pub struct PathwaySearch(SearchFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct SearchFields {
    /// Literal source query; exclusive with gene and top-level discovery.
    #[serde(
        default,
        deserialize_with = "super::super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text", extend("x-positional" = 0)))]
    query: Option<Text>,
    /// Human gene input for exact resolution and UniProt mapping; Reactome only.
    #[serde(
        default,
        deserialize_with = "super::super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "GeneInput"))]
    gene: Option<GeneInput>,
    /// Native human Reactome top-level discovery, exclusive with text and gene.
    #[serde(default)]
    top_level: bool,
    /// One source; Reactome by default.
    #[serde(default)]
    source: PathwaySource,
    /// Five rows by default, one to fifty. Reactome rows equals this limit without refill.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Native Reactome Start row or retained WikiPathways catalog offset.
    #[serde(default)]
    offset: u32,
    /// Retained snapshot identity for deterministic continuation.
    #[serde(
        default,
        deserialize_with = "super::super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pool: Option<Text>,
    /// Request a new retained observation.
    #[serde(default)]
    fresh: bool,
}
impl TryFrom<SearchFields> for PathwaySearch {
    type Error = QueryInvalid;
    fn try_from(v: SearchFields) -> Result<Self, Self::Error> {
        let selectors = usize::from(v.query.is_some())
            + usize::from(v.gene.is_some())
            + usize::from(v.top_level);
        if selectors != 1
            || !valid_limit(v.limit)
            || (v.source == PathwaySource::Wikipathways && (v.gene.is_some() || v.top_level))
        {
            return Err(QueryInvalid);
        }
        Ok(Self(v))
    }
}
impl PathwaySearch {
    /// Literal positional text.
    pub fn query(&self) -> Option<&Text> {
        self.0.query.as_ref()
    }
    /// Exact-resolution gene input.
    pub fn gene(&self) -> Option<&GeneInput> {
        self.0.gene.as_ref()
    }
    /// Whether native top-level discovery was selected.
    pub fn top_level(&self) -> bool {
        self.0.top_level
    }
    /// Explicit source, without fallback.
    pub fn source(&self) -> PathwaySource {
        self.0.source
    }
    /// Native requested page size, which may exceed admitted rows.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Native result offset, independent of admitted row count.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Retained page snapshot.
    pub fn pool(&self) -> Option<&Text> {
        self.0.pool.as_ref()
    }
    /// Whether to refresh the observation.
    pub fn fresh(&self) -> bool {
        self.0.fresh
    }
    /// Advance the native position without changing selectors.
    pub fn at_offset(&self, offset: u32) -> Self {
        let mut value = self.clone();
        value.0.offset = offset;
        value
    }
}
