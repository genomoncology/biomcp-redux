//! Native fields establishing complete label-name equality.
use serde::{Deserialize, Serialize};
/// Complete native name fields matching the requested label name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum DrugNameMatch {
    /// The complete brand name matches.
    Brand,
    /// The complete generic name matches.
    Generic,
    /// Both complete name fields match.
    Both,
}
