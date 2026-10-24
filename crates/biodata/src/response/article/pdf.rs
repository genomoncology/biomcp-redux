use crate::{License, Text};
use serde::{Deserialize, Serialize};
crate::macros::wire_enum! {
    /// Origin of extracted article text.
    TextOrigin {
        /// A publisher PDF rather than structured JATS.
        Pdf => "pdf",
    }
}
crate::source::canonical_enum! {
    /// Open-access status supplied by the paper-link provider.
    OpenAccessStatus, SourceUnknown {
        /// Publisher open access.
        Gold => "gold",
        /// Open article in a subscription journal.
        Hybrid => "hybrid",
        /// Free publisher copy without an open license claim.
        Bronze => "bronze",
        /// Repository copy.
        Green => "green",
        /// No open copy.
        Closed => "closed",
    }
}
/// One bounded PDF text part in original page order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PdfPart {
    /// One-based reader selector.
    pub number: u32,
    /// Original first page of this part.
    pub first_page: u32,
    /// Original last page of this part.
    pub last_page: u32,
    /// Compact serialized text bytes divided by four, rounded up.
    pub tokens: u32,
}
/// Publisher PDF identity and bounded plain-text outline.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PdfText {
    /// Distinguishes PDF extraction from the structured body reader.
    pub origin: TextOrigin,
    /// Checked publisher link.
    pub url: Text,
    /// Provider's open-access status.
    pub status: OpenAccessStatus,
    /// Provider's normalized license; other never grants reuse rights.
    pub license: License,
    /// Original document page count.
    pub pages: u32,
    /// Whether later pages were omitted by the extraction bound.
    pub truncated: bool,
    /// Bounded selectable parts.
    pub parts: Box<[PdfPart]>,
}
