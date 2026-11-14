use serde::{Deserialize, Serialize};
biodata::wire_enum! {
/// Purpose of an available next command.
StepKind {
    /// Start a replacement snapshot.
    Restart => "restart",
    /// Request sections omitted from this card.
    Section => "section",
    /// Read the same search in another supported order.
    Sort => "sort",
    /// Read another page.
    Page => "page",
    /// Retry without a cached reply.
    Fresh => "fresh",
    /// Select a canonical identity.
    Disambiguate => "disambiguate",
    /// Follow an exact linked record in another entity.
    Pivot => "pivot",
    /// Synchronize a missing or stale identity file.
    Sync => "sync",
    /// Explain how to configure a source key.
    Key => "key",
    /// Run a corrected or adjusted call; its `options` say what may be added.
    Configure => "configure",
}
}
/// A command argument list spelled from a typed action.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Step {
    /// Purpose of the action.
    pub kind: StepKind,
    /// CLI and MCP arguments.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub args: Box<[Box<str>]>,
    /// Options this call also takes, each with a placeholder to fill in, such as
    /// `"--decision-from": "<YYYY-MM-DD>"`; add any of them to `args`. MCP names them as tool
    /// arguments. Absent when the step is only runnable as it is.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub options: std::collections::BTreeMap<Box<str>, Box<str>>,
}
