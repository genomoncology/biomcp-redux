use serde::{Deserialize, Serialize};
/// Purpose of an available next command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum StepKind {
    /// Read another page.
    Page,
    /// Retry without a cached reply.
    Fresh,
    /// Select a canonical identity.
    Disambiguate,
    /// Synchronize a missing or stale identity file.
    Sync,
}
/// A command argument list spelled from a typed action.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Step {
    /// Purpose of the action.
    pub kind: StepKind,
    /// CLI and MCP arguments.
    pub args: Box<[Box<str>]>,
}
