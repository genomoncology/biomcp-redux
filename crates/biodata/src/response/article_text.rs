use crate::{ArticleText, Pmcid, Text};
use serde::{Deserialize, Serialize};
crate::source::canonical_enum! {
    /// License reported by the bucket metadata record.
    License, SourceUnknown {
        /// Creative Commons Attribution.
        CcBy => "cc_by",
        /// Creative Commons Attribution NonCommercial.
        CcByNc => "cc_by_nc",
        /// Creative Commons Attribution ShareAlike.
        CcBySa => "cc_by_sa",
        /// Creative Commons Attribution NonCommercial ShareAlike.
        CcByNcSa => "cc_by_nc_sa",
        /// Creative Commons Attribution NoDerivatives.
        CcByNd => "cc_by_nd",
        /// Creative Commons Attribution NonCommercial NoDerivatives.
        CcByNcNd => "cc_by_nc_nd",
        /// Creative Commons public-domain dedication.
        Cc0 => "cc0",
        /// Public domain.
        PublicDomain => "public_domain",
        /// Text and data mining manuscript terms.
        Tdm => "tdm",
        /// The bucket reports another license.
        Other => "other",
    }
}
crate::source::canonical_enum! {
    /// Body part kind.
    TextPartKind, SourceUnknown {
        /// Top-level body section.
        Section => "section",
        /// Table, including caption and rows.
        Table => "table",
    }
}
/// One independently readable bounded part.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TextPart {
    /// One-based reader selector.
    pub number: u32,
    /// Source section title or table label.
    pub title: Text,
    /// Source structure kind.
    pub kind: TextPartKind,
    /// Compact serialized text bytes divided by four, rounded up.
    pub tokens: u32,
}
/// Figure text in document order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FigureCaption {
    /// Source label.
    pub label: Text,
    /// Source caption.
    pub caption: Text,
}
/// Bucket license and body outline; processing version is not publication chronology.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FullText {
    /// Selected bucket license.
    pub license: License,
    /// Highest numeric processing version.
    pub processing_version: u32,
    /// Whether this selected version is an author manuscript.
    pub manuscript: bool,
    /// Bounded body parts in reading order.
    pub parts: Box<[TextPart]>,
    /// Source figure captions in document order.
    pub figures: Box<[FigureCaption]>,
}
/// One matching paragraph or table row, split when necessary to fit the text budget.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TextHit {
    /// Outline part containing this text.
    pub part: u32,
    /// Whole bounded source row.
    pub markdown: crate::Markdown,
    /// Section ancestry in source order.
    pub heading_path: Box<[Text]>,
}
/// Body text response; the envelope supplies advancing reader steps.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ArticleTextResult {
    /// Requested PMC record when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmcid: Option<Pmcid>,
    /// Explicit section outcome including not applicable and not open.
    pub content: crate::Section<TextContent>,
}
/// One bounded part or matching rows with the bucket license.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TextContent {
    /// Bucket license.
    pub license: License,
    /// Selected processing version.
    pub processing_version: u32,
    /// Author manuscript flag.
    pub manuscript: bool,
    /// Scope of returned source text.
    pub scope: Text,
    /// Part number for a direct part selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part: Option<u32>,
    /// Bounded body text for a direct part selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<crate::Markdown>,
    /// Section ancestry for a selected part.
    #[serde(skip_serializing_if="Option::is_none")]
    pub heading_path: Option<Box<[Text]>>,
    /// Matching source rows for a find selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hits: Option<Box<[TextHit]>>,
}
impl super::ResponseBody for ArticleTextResult {
    const SCHEMA: &'static str = "biomcp.article_text/1";
    type Query = ArticleText;
}
