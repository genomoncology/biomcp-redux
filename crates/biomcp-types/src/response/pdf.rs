//! Mechanically extracted publisher text, with explicit reader bounds.
use serde::{Deserialize, Serialize};
/// One small contiguous source-text window, without summary or interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PdfText {
    /// Original FDA document URL.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: biodata::Text,
    /// Native PDF pages when readable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<u32>,
    /// Extraction outcome; no-text includes scanned PDFs and does not imply OCR.
    pub state: PdfTextState,
    /// Zero-based mechanical text-fragment offset.
    pub offset: u32,
    /// Number of fragments in the readable document.
    pub total: u32,
    /// Verbatim extracted text in document order; omitted when unavailable or when a find returns passages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<str>>,
}
/// One verbatim passage that contains the found text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PdfTextHit {
    /// Zero-based text fragment holding the passage start; read it with the fragment offset.
    pub fragment: u32,
    /// Extracted lines from the matching line through the following lines, in document order.
    pub text: Box<str>,
}
biodata::wire_enum! {
    /// Bounded PDF text reader outcome.
    PdfTextState {
        /// Embedded text was read.
        Text => "text",
        /// No embedded text was found; scans require OCR outside this reader.
        NoText => "no-text",
        /// The PDF exceeds the byte, page or expanded-text bound.
        TooLarge => "too-large"
    }
}
