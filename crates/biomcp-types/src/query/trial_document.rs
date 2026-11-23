use crate::{NctId, Text};
use serde::{Deserialize, Serialize};
/// Read or save one listed trial PDF. Reads up to 25 MiB and 100 pages, one 6144-byte fragment at a time.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(extend("x-reply-query-titles" = ["TrialDocumentSaveQuery"])))]
pub struct TrialDocumentGet {
    /// Exact registry identity.
    #[cfg_attr(feature="schema", schemars(extend("x-positional"=0)))]
    pub input: NctId,
    /// Exact filename from the registry manifest; never a URL or inferred path.
    #[cfg_attr(feature="schema", schemars(extend("x-positional"=1)))]
    pub file: Text,
    /// Case-insensitive phrase to find, with PDF page numbers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pub find: Option<Text>,
    /// Zero-based text part or match offset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "u32"))]
    pub section_offset: Option<u32>,
    /// Save original bytes to a caller-owned file or folder instead of reading text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pub save: Option<Text>,
    /// Replace an existing file only when explicitly requested.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub replace: bool,
}

crate::query::validation::checked_query!(TrialDocumentGet);
