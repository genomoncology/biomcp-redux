mod constraint;
mod civic;
pub use civic::CivicGene;
use super::{ItemStatus, ResponseBody, Section};
use crate::GeneSection;
use crate::{GeneGet, GeneHeader, GeneInput, GeneResolution, GeneSearch};
pub use constraint::{Constraint, FiniteNumber, FiniteNumberError};
use serde::{Deserialize, Serialize};
/// Answer to one requested input.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GeneItem {
    /// Original checked input.
    pub input: GeneInput,
    /// This input's outcome.
    pub status: ItemStatus,
    /// Absent only when identity lookup failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<GeneResolution>,
    /// Requested sections, present only for a resolved identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<GeneSections>,
    /// Applicable sections left out of this request.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub more: Box<[GeneSection]>,
}
/// Gene answers in input order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GeneCards {
    /// Requested inputs and their answers.
    pub items: Box<[GeneItem]>,
}
/// One page of gene headers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GenePage {
    /// Headers after ranking and paging.
    pub items: Box<[GeneHeader]>,
}
impl ResponseBody for GeneCards {
    const SCHEMA: &'static str = "biomcp.gene_cards/1";
    type Query = GeneGet;
}
impl ResponseBody for GenePage {
    const SCHEMA: &'static str = "biomcp.gene_page/1";
    type Query = GeneSearch;
}
/// Typed dispatch result before the interface constructs an envelope.
#[derive(Clone, Debug)]
pub enum QueryBody {
    /// Gene get answers.
    GeneCards(GeneCards),
    /// Gene search page.
    GenePage(GenePage),
}

/// Explicit source synchronization selector.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SyncQuery {
    /// Source whose complete identity data is synchronized.
    pub source: crate::Source,
}
/// Published bundle metadata after synchronization.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SyncResult {
    /// Imported rows.
    pub rows: u64,
    /// Unencoded source bytes.
    pub bytes: u64,
}
impl ResponseBody for SyncResult {
    const SCHEMA: &'static str = "biomcp.sync/1";
    type Query = SyncQuery;
}

/// Landed gene sections, each retaining its source and explicit state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GeneSections {
    /// Population constraint reported by gnomAD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint: Option<Section<Constraint>>,
    /// CIViC gene-level curation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub civic: Option<Section<CivicGene>>,
}
