use serde::{Deserialize, Serialize};
/// Overall command outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Status {
    /// Every requested source answered.
    Complete,
    /// Some requested data failed or is stale.
    Partial,
    /// Every identity was confirmed absent.
    Absent,
    /// No requested identity could be answered.
    Failed,
}
/// Outcome for one input.
pub type ItemStatus = Status;
