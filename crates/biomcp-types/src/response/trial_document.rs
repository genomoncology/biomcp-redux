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
/// Read or save the selected trial PDF.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialDocumentResult {
    /// Text or a source failure for a read.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Section<TrialDocumentText>>,
    /// Saved receipt or a source failure for a save.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<Section<TrialDocumentSaved>>,
}
impl super::ResponseBody for TrialDocumentResult {
    const SCHEMA: &'static str = "biomcp.trial-document/8";
    type Query = crate::TrialDocumentGet;
}
