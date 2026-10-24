use crate::{NctId, Text};
use serde::{Deserialize, Serialize};
/// Save one exact ClinicalTrials.gov manifest entry to the caller's destination.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialDocumentGet {
    /// Exact registry identity.
    #[cfg_attr(feature="schema", schemars(extend("x-positional"=0)))]
    pub input: NctId,
    /// Exact filename from the registry manifest; never a URL or inferred path.
    #[cfg_attr(feature="schema", schemars(extend("x-positional"=1)))]
    pub file: Text,
    /// Caller-owned file or existing folder.
    pub save: Text,
    /// Replace an existing file only when explicitly requested.
    #[serde(default)]
    pub replace: bool,
}
