use crate::{PdfText, Section, Text, TrialDocument};
use serde::{Deserialize, Serialize};
/// Receipt for exactly the original source bytes written by the caller.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialDocumentSaved {
    /// Absolute caller-owned path.
    pub path: Text,
    /// Saved byte count.
    pub bytes: u64,
    /// SHA-256 of the complete saved bytes.
    pub sha256: Text,
    /// Original manifest metadata and publisher URL.
    pub document: TrialDocument,
}
/// The selected registry document and its bounded embedded text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialDocumentText {
    /// Original manifest metadata and publisher URL.
    pub document: TrialDocument,
    /// Version explicitly printed on the PDF cover, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<str>>,
    /// Text part, or extraction state and bounds for a find.
    pub text: PdfText,
    /// Match passages on a find call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hits: Option<Box<[TrialDocumentHit]>>,
}
/// One passage on a physical page of the posted PDF.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialDocumentHit {
    /// One-based PDF page, including the cover.
    pub page: u32,
    /// Verbatim embedded text around the match.
    pub text: Box<str>,
}
/// Read the embedded text of the selected trial PDF.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialDocumentReadResult {
    /// Text or a source failure for a read.
    pub content: Section<TrialDocumentText>,
}
impl super::ResponseBody for TrialDocumentReadResult {
    const SCHEMA: &'static str = "biomcp.trial-document-text/1";
    type Query = crate::TrialDocumentGet;
}
/// Original trial PDF download.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialDocumentResult {
    /// Saved receipt, confirmed absence, or a source failure.
    pub file: Section<TrialDocumentSaved>,
}
impl super::ResponseBody for TrialDocumentResult {
    const SCHEMA: &'static str = "biomcp.trial-document/8";
    type Query = TrialDocumentSaveQuery;
}
/// The exact trial document and destination echoed by an original-byte save.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialDocumentSaveQuery {
    /// Exact registry identity.
    #[cfg_attr(feature="schema", schemars(extend("x-positional"=0)))]
    pub input: crate::NctId,
    /// Exact filename from the registry manifest.
    #[cfg_attr(feature="schema", schemars(extend("x-positional"=1)))]
    pub file: Text,
    /// Caller-owned file or existing folder.
    pub save: Text,
    /// Replace an existing file only when explicitly requested.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub replace: bool,
}
