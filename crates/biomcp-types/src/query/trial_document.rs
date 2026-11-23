use crate::{NctId, Text};
use serde::{Deserialize, Serialize};
/// Read or save a listed trial PDF (25 MiB, 100 pages per read).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(extend("x-reply-query-titles" = ["TrialDocumentSaveQuery"])))]
pub struct TrialDocumentGet {
    /// ClinicalTrials.gov ID.
    #[cfg_attr(feature="schema", schemars(extend("x-positional"=0)))]
    pub input: NctId,
    /// Exact listed PDF filename.
    #[cfg_attr(feature="schema", schemars(extend("x-positional"=1)))]
    pub file: Text,
    /// Case-insensitive phrase search with pages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pub find: Option<Text>,
    /// First physical page to read or search (1-based, default 1).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "std::num::NonZeroU32"))]
    pub page_start: Option<std::num::NonZeroU32>,
    /// Zero-based part or hit offset within the selected pages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "u32"))]
    pub section_offset: Option<u32>,
    /// Save the original PDF to a file or folder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pub save: Option<Text>,
    /// Replace an existing file.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub replace: bool,
}

crate::query::validation::checked_query!(TrialDocumentGet);
