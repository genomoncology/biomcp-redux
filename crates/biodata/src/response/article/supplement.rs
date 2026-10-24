use crate::{Text, License, Pmcid, Section};
use serde::{Serialize, Deserialize};
crate::source::canonical_enum! {
    /// Supplement format determined by the exact source filename suffix.
    FileKind, SourceUnknown {
        /// Excel workbook.
        Xlsx => "xlsx",
        /// Comma-separated text.
        Csv => "csv",
        /// Tab-separated text.
        Tsv => "tsv",
        /// Plain text lines.
        Txt => "txt",
        /// PDF download offered without parsing.
        Pdf => "pdf",
        /// Other source format.
        Other => "other",
    }
}
impl FileKind {
    /// Whether the local reader supports this format.
    pub fn readable(self) -> bool { matches!(self,Self::Xlsx|Self::Csv|Self::Tsv|Self::Txt) }
}
/// A JATS supplement confirmed by the same processing record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
pub struct SupplementFile {
    /// Exact source filename.
    pub name: Text,
    /// Checked format.
    pub kind: FileKind,
    /// JATS label when present.
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<Text>,
    /// JATS caption when present.
    #[serde(skip_serializing_if="Option::is_none")]
    pub caption: Option<crate::Markdown>,
}
/// Available supplements with their owning article license.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
pub struct Supplements {
    /// Article license from the record.
    pub license: License,
    /// Exact JATS/record intersection.
    pub files: Box<[SupplementFile]>,
}
/// One original sparse row, with its first source row as header.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
pub struct SheetRow {
    /// Original sheet name.
    pub sheet: Text,
    /// Original one-based row number.
    pub row: u32,
    /// First original row, including empty cells.
    pub header: Box<[String]>,
    /// This row's cells, including empty cells before populated columns.
    pub cells: Box<[String]>,
    /// Character offset of this row fragment in flattened cell text.
    #[serde(skip_serializing_if="Option::is_none")]
    pub fragment: Option<u32>,
}
/// Source sheet with its maximum original row number.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
pub struct SheetInfo {
    /// Sheet name.
    pub name: Text,
    /// Last original row number, preserving gaps.
    pub rows: u32,
}
/// Downloaded bounded file contents or an unsupported format's original URL.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
pub struct SupplementContent {
    /// Article license.
    pub license: License,
    /// Hash of the original downloaded bytes; absent when no download occurred.
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha256: Option<Text>,
    /// Unsupported file URL, returned without downloading it.
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<Text>,
    /// Typed unsupported reason.
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<crate::Reason>,
    /// Original sheet dimensions.
    #[serde(skip_serializing_if="Option::is_none")]
    pub sheets: Option<Box<[SheetInfo]>>,
    /// Selected original rows.
    #[serde(skip_serializing_if="Option::is_none")]
    pub rows: Option<Box<[SheetRow]>>,
    /// Explicit variant spellings searched.
    #[serde(skip_serializing_if="Option::is_none")]
    pub forms: Option<[String;4]>,
}
/// One supplement reader answer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
pub struct ArticleSupplementResult {
    /// Resolved PMC identity.
    #[serde(skip_serializing_if="Option::is_none")]
    pub pmcid: Option<Pmcid>,
    /// Independently licensed supplement content.
    pub content: Section<SupplementContent>,
}
impl super::super::ResponseBody for ArticleSupplementResult {
    const SCHEMA: &'static str = "biomcp.article_supplement/1";
    type Query = crate::ArticleSupplement;
}
