use crate::Text;
use serde::{Deserialize, Serialize};
/// OncoKB's stated gene classification, without inference from other sources.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum OncokbGeneType {
    /// Both source-stated roles.
    #[serde(rename = "ONCOGENE_AND_TSG")]
    OncogeneAndTsg,
    /// Source-stated oncogene.
    #[serde(rename = "ONCOGENE")]
    Oncogene,
    /// Source-stated tumor suppressor.
    #[serde(rename = "TSG")]
    Tsg,
    /// Source states insufficient evidence.
    #[serde(rename = "INSUFFICIENT_EVIDENCE")]
    InsufficientEvidence,
    /// Source states neither role.
    #[serde(rename = "NEITHER")]
    Neither,
}
/// Short source-stated annotation on a gene card.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct OncokbCard {
    /// Source gene classification.
    pub gene_type: OncokbGeneType,
    /// Highest sensitive level, as stated by the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highest_sensitive_level: Option<Text>,
    /// Highest resistance level, as stated by the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highest_resistance_level: Option<Text>,
    /// Data release version, when verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Text>,
}
/// Source transcript annotation for one genome build.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct OncokbTranscript {
    /// Source-stated isoform, kept separate from HGNC MANE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isoform: Option<Text>,
    /// Source-stated RefSeq transcript.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refseq: Option<Text>,
}
/// Whole OncoKB gene prose and independent transcript annotations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Oncokb {
    /// Whole source summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Text>,
    /// Whole source background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Text>,
    /// GRCh37 source transcripts.
    pub grch37: OncokbTranscript,
    /// GRCh38 source transcripts.
    pub grch38: OncokbTranscript,
}
