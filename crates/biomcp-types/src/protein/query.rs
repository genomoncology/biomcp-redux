use crate::QueryInvalid;
use crate::query::present;
use biodata::{ComplexPortalId, GeneSymbol, Position1, Text, UniProtIsoform};
use serde::{Deserialize, Serialize};
fn limit() -> u32 {
    5
}
fn reviewed() -> bool {
    true
}
biodata::wire_enum! {
    /// Named protein sections; all expands to bounded first pages.
    ProteinSection {
        /// Native UniProt features.
        Features => "features",
        /// Native InterPro entry matches.
        Domains => "domains",
        /// Native alternative products.
        Isoforms => "isoforms",
        /// Exact participant-admitted complex headers.
        Complexes => "complexes",
        /// Native structure cross-reference links.
        Structures => "structures",
        /// Complete native function text and evidence.
        Function => "function",
        /// Direct exact-identity sequence and audit metadata.
        Sequence => "sequence",
        /// Bounded first page of every named section.
        All => "all"
    }
}
/// Exact source accession or a checked display gene symbol needing later resolution.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ProteinInput {
    /// Source accession, preserving an explicit isoform suffix.
    Accession(UniProtIsoform),
    /// A symbol requests candidates; no first-hit selection.
    Gene(GeneSymbol),
}
/// Human reviewed UniProt search; unreviewed positives remain unproved.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SearchFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "SearchFields", extend("oneOf" = [{"required":["query"]},{"required":["gene"]}])))]
pub struct ProteinSearch(SearchFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct SearchFields {
    /// Literal text, exclusive with gene.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text", extend("x-positional" = 0)))]
    query: Option<Text>,
    /// Exact native gene field, without identifier inference.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "GeneSymbol"))]
    gene: Option<GeneSymbol>,
    /// Reviewed human scope stays enabled; unreviewed search is deferred.
    #[serde(default = "reviewed")]
    #[cfg_attr(feature = "schema", schemars(extend("const" = true)))]
    reviewed: bool,
    /// Five rows by default; no more than 25.
    #[serde(default = "limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 25)))]
    limit: u32,
    /// Source/retained result offset.
    #[serde(default)]
    offset: u32,
    /// Exact retained collection identity.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pool: Option<Text>,
}
impl TryFrom<SearchFields> for ProteinSearch {
    type Error = QueryInvalid;
    fn try_from(v: SearchFields) -> Result<Self, QueryInvalid> {
        if v.query.is_some() == v.gene.is_some() || !v.reviewed || !(1..=25).contains(&v.limit) {
            return Err(QueryInvalid);
        }
        Ok(Self(v))
    }
}
impl ProteinSearch {
    /// Literal native text.
    pub fn query(&self) -> Option<&Text> {
        self.0.query.as_ref()
    }
    /// Exact native gene selector.
    pub fn gene(&self) -> Option<&GeneSymbol> {
        self.0.gene.as_ref()
    }
    /// Explicit reviewed scope.
    pub fn reviewed(&self) -> bool {
        self.0.reviewed
    }
    /// Bounded page size.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Native result offset.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Retained pool when supplied.
    pub fn pool(&self) -> Option<&Text> {
        self.0.pool.as_ref()
    }
}
/// One or more protein identities or gene symbols, with explicit named sections.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "GetFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "GetFields"))]
pub struct ProteinGet(GetFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct GetFields {
    /// Source accessions or gene symbols, preserving caller order.
    #[cfg_attr(feature = "schema", schemars(length(min = 1, max = 20), extend("x-positional" = 0)))]
    input: Box<[ProteinInput]>,
    /// Only requested sections disclose their full content.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    section: Box<[ProteinSection]>,
    /// Bounded first-page size for each requested section.
    #[serde(default = "limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 25)))]
    section_limit: u32,
}
impl TryFrom<GetFields> for ProteinGet {
    type Error = QueryInvalid;
    fn try_from(v: GetFields) -> Result<Self, QueryInvalid> {
        if !(1..=20).contains(&v.input.len()) || !(1..=25).contains(&v.section_limit) {
            return Err(QueryInvalid);
        }
        Ok(Self(v))
    }
}
impl ProteinGet {
    /// Caller identities in their original order.
    pub fn input(&self) -> &[ProteinInput] {
        &self.0.input
    }
    /// Explicit requested sections.
    pub fn sections(&self) -> &[ProteinSection] {
        &self.0.section
    }
    /// Bound for each first section page.
    pub fn section_limit(&self) -> u32 {
        self.0.section_limit
    }
}
mod relation;
pub use relation::*;
