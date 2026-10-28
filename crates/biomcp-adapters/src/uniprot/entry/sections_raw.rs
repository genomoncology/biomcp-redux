use biodata::{AminoAcids, ProteinIsoformStatus, ProteinStructureProperty, Text, UniProtIsoform};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::uniprot) struct Feature {
    #[serde(rename = "type")]
    pub kind: Text,
    pub location: Location,
    pub description: Option<String>,
    pub feature_id: Option<String>,
    pub alternative_sequence: Option<Alternative>,
    pub evidences: Option<Vec<Evidence>>,
}
#[derive(Debug, Deserialize)]
pub(in crate::uniprot) struct Location {
    pub start: Boundary,
    pub end: Boundary,
    pub sequence: Option<UniProtIsoform>,
}
#[derive(Debug, Deserialize)]
pub(in crate::uniprot) struct Boundary {
    pub value: Option<u32>,
    pub modifier: Modifier,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub(in crate::uniprot) enum Modifier {
    Exact,
    Outside,
    Unknown,
    Unsure,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::uniprot) struct Alternative {
    pub original_sequence: Option<AminoAcids>,
    pub alternative_sequences: Option<Vec<AminoAcids>>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::uniprot) struct Evidence {
    pub evidence_code: Text,
    pub source: Option<Text>,
    pub id: Option<Text>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::uniprot) struct Comment {
    pub comment_type: String,
    pub molecule: Option<Text>,
    pub texts: Option<Vec<Paragraph>>,
    pub isoforms: Option<Vec<Isoform>>,
}
#[derive(Debug, Deserialize)]
pub(in crate::uniprot) struct Paragraph {
    pub value: Text,
    pub evidences: Option<Vec<Evidence>>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::uniprot) struct Isoform {
    pub name: super::raw::Value,
    pub synonyms: Option<Vec<super::raw::Value>>,
    pub isoform_ids: Option<Vec<UniProtIsoform>>,
    pub sequence_ids: Option<Vec<Text>>,
    pub isoform_sequence_status: ProteinIsoformStatus,
}
#[derive(Debug, Deserialize)]
pub(in crate::uniprot) struct CrossReference {
    pub database: Text,
    pub id: Text,
    #[serde(default, deserialize_with = "crate::strict_input::deserialize")]
    pub properties: Option<Vec<ProteinStructureProperty>>,
}
