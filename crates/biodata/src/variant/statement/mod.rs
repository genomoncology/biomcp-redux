//! Source record provenance and checked subject references.
use crate::{PartialDate, Source, Text};
use serde::{Deserialize, Serialize};
mod reference;
pub use reference::VariantRef;
/// Provenance for one source record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Attribution {
    /// Source responsible for the claim.
    pub source: Source,
    /// Native assertion record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<Text>,
    /// Record version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Text>,
    /// Dataset release.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<Text>,
    /// Source submitter or panel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitter: Option<Text>,
    /// Source evaluation date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evaluated: Option<PartialDate>,
    /// Source record link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<Text>,
}
