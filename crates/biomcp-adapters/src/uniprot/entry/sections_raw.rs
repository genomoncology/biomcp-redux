use biodata::{AminoAcids, ProteinIsoformStatus, ProteinStructureProperty, Text, UniProtIsoform};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Feature {
    #[serde(rename = "type")]
    pub kind: Text,
    pub location: Location,
    pub description: Option<String>,
    pub feature_id: Option<Text>,
    pub alternative_sequence: Option<Alternative>,
    #[serde(default)]
    pub evidences: Vec<Evidence>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Location {
    pub start: Boundary,
    pub end: Boundary,
    pub sequence: Option<UniProtIsoform>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Boundary {
    pub value: Option<u32>,
    pub modifier: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Alternative {
    pub original_sequence: Option<AminoAcids>,
    #[serde(default)]
    pub alternative_sequences: Vec<AminoAcids>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Evidence {
    pub evidence_code: Text,
    pub source: Option<Text>,
    pub id: Option<Text>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Comment {
    pub comment_type: String,
    pub molecule: Option<Text>,
    #[serde(default)]
    pub texts: Vec<Paragraph>,
    #[serde(default)]
    pub isoforms: Vec<Isoform>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Paragraph {
    pub value: Text,
    #[serde(default)]
    pub evidences: Vec<Evidence>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Isoform {
    pub name: super::raw::Value,
    #[serde(default)]
    pub synonyms: Vec<super::raw::Value>,
    #[serde(default)]
    pub isoform_ids: Vec<UniProtIsoform>,
    #[serde(default)]
    pub sequence_ids: Vec<Text>,
    pub isoform_sequence_status: ProteinIsoformStatus,
}
#[derive(Debug, Deserialize)]
pub(super) struct CrossReference {
    pub database: Text,
    pub id: Text,
    #[serde(default)]
    pub properties: Vec<ProteinStructureProperty>,
}
