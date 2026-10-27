use crate::{NctId, Text};
use serde::{Deserialize, Serialize};
#[doc = " Download one exact ClinicalTrials.gov manifest PDF to an explicit caller path; never \
    read its text. CLI and stdio MCP support saves; HTTP refuses with the source URL."]
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
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub replace: bool,
}
