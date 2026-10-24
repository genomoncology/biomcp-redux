use super::{ItemStatus, ResponseBody};
use crate::{GeneGet, GeneHeader, GeneInput, GeneResolution, GeneSearch};
use serde::{Deserialize, Serialize};
/// Answer to one requested input.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
}
/// Gene answers in input order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GeneCards {
    /// Requested inputs and their answers.
    pub items: Box<[GeneItem]>,
}
/// One page of gene headers.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

/// Explicit synchronization selector.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SyncQuery {
    /// Declared bundle name.
    pub bundle: String,
}
/// Published bundle metadata after synchronization.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
