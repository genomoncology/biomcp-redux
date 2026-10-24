use crate::{Cytoband, GeneInput, Text};
use serde::{Deserialize, Serialize};

/// A query failed its structural bounds.
#[derive(Debug, Clone, Copy)]
pub struct QueryInvalid;
impl std::fmt::Display for QueryInvalid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("query.invalid")
    }
}
impl std::error::Error for QueryInvalid {}

/// Get one to twenty human gene inputs in their original order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "GetFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "GetFields"))]
pub struct GeneGet {
    input: Box<[GeneInput]>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct GetFields {
    /// Gene symbols or checked identifiers, in input order.
    #[cfg_attr(feature = "schema", schemars(length(min = 1, max = 20), extend("x-positional" = 0)))]
    input: Box<[GeneInput]>,
}
impl TryFrom<GetFields> for GeneGet {
    type Error = QueryInvalid;
    fn try_from(fields: GetFields) -> Result<Self, Self::Error> {
        Self::new(fields.input)
    }
}
impl GeneGet {
    /// Refuse an empty or oversized input batch.
    pub fn new(input: Box<[GeneInput]>) -> Result<Self, QueryInvalid> {
        if !(1..=20).contains(&input.len()) {
            return Err(QueryInvalid);
        }
        Ok(Self { input })
    }
    /// Inputs in caller order.
    pub fn input(&self) -> &[GeneInput] {
        &self.input
    }
}
/// Search genes by text, cytoband, or their intersection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SearchFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "SearchFields"))]
pub struct GeneSearch(SearchFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct SearchFields {
    /// Words matched against symbols, previous symbols, aliases and names.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    query: Option<Text>,
    /// A cytoband prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    band: Option<Cytoband>,
    /// Number of results per page.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Number of results to skip.
    #[serde(default)]
    offset: u32,
}
fn default_limit() -> u32 {
    10
}
impl TryFrom<SearchFields> for GeneSearch {
    type Error = QueryInvalid;
    fn try_from(fields: SearchFields) -> Result<Self, Self::Error> {
        Self::new(fields.query, fields.band, fields.limit, fields.offset)
    }
}
impl GeneSearch {
    /// Check the required search selector and page bounds.
    pub fn new(
        query: Option<Text>,
        band: Option<Cytoband>,
        limit: u32,
        offset: u32,
    ) -> Result<Self, QueryInvalid> {
        if query.is_none() && band.is_none() || !(1..=50).contains(&limit) {
            return Err(QueryInvalid);
        }
        Ok(Self(SearchFields {
            query,
            band,
            limit,
            offset,
        }))
    }
    /// Optional search text.
    pub fn query(&self) -> Option<&Text> {
        self.0.query.as_ref()
    }
    /// Optional band prefix.
    pub fn band(&self) -> Option<&Cytoband> {
        self.0.band.as_ref()
    }
    /// Page size.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Page start.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// The same selectors on another page.
    pub fn at_offset(&self, offset: u32) -> Self {
        let mut page = self.clone();
        page.0.offset = offset;
        page
    }
}
