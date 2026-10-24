use crate::{Reason, Source};
use serde::{Deserialize, Serialize};
/// State of a source actually asked by the command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum SourceState {
    /// Checked data is current.
    Ok,
    /// Checked stale data answered.
    Stale,
    /// Another process is synchronizing the bundle.
    Syncing,
    /// The source failed.
    Failed,
}
/// One source's outcome, without provider text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SourceOutcome {
    /// Asked source.
    pub source: Source,
    /// Retained state.
    pub state: SourceState,
    /// Failure or stale-fallback reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
}
