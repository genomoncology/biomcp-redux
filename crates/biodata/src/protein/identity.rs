use super::{ProteinCoordinateConvention, ProteinIsoformStatus, ProteinSequenceAvailability};
use crate::{
    AminoAcids, Attribution, Curie, Term, Text, UncertainSpan0, UniProtAcc, UniProtIsoform,
};
use serde::{Deserialize, Serialize};
/// A reviewed human protein listed by the authoritative gene record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "Protein_{A}"))]
pub struct Protein<A = UniProtAcc> {
    /// Checked primary accession.
    pub accession: A,
    /// Recommended protein name.
    pub name: Text,
    /// Sequence length in amino acids.
    pub length: u32,
    /// Whole source function paragraphs, omitted when absent.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub function: Box<[Text]>,
    /// Source subcellular location names.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub subcellular_locations: Box<[Text]>,
}

/// Native entry metadata wrapping the same Protein identity used by the gene slice.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinHeader {
    /// Shared gene/protein identity fields; never independently restated.
    pub protein: Protein<UniProtIsoform>,
    /// Native source entry name.
    pub entry_id: Text,
    /// Display names only; never identifier joins.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub gene_names: Box<[Text]>,
    /// Only identifiers explicitly supplied by the source.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub gene_ids: Box<[Curie]>,
    /// Native source taxonomy identifier.
    pub taxon: u32,
    /// Native reviewed status.
    pub reviewed: bool,
    /// Source entry URL.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Text,
}
/// Shared native feature contract without source label remapping.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UniProtFeature {
    /// Exact source sequence identity.
    pub protein: UniProtIsoform,
    /// Zero-based interbase coordinates retaining uncertain ranges.
    pub span: UncertainSpan0,
    /// Source feature label; coding only for documented exact feature types.
    pub kind: Term,
    /// Complete source description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Text>,
    /// Native feature identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Text>,
    /// Source original residues, when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original: Option<AminoAcids>,
    /// Every stated alternative, without translation.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub alternative: Box<[AminoAcids]>,
    /// All original source evidence references.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub evidence: Box<[Text]>,
}
/// UniProt feature with source attribution and coordinate provenance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinFeature {
    /// Shared feature, including native label and residues.
    pub feature: UniProtFeature,
    /// Source record and release facts.
    pub attribution: Attribution,
    /// Convention before mathematical conversion.
    pub source_coordinates: ProteinCoordinateConvention,
}
/// One source alternative-product row; all stated identifiers stay together.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinIsoform {
    /// All source-stated isoform identifiers, including External base identities.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub ids: Box<[UniProtIsoform]>,
    /// Source isoform name.
    pub name: Text,
    /// Every source synonym.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub synonyms: Box<[Text]>,
    /// Native sequence status; no sequence is inferred from it.
    pub sequence_status: ProteinIsoformStatus,
    /// Source VSP sequence identifiers.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub sequence_ids: Box<[Text]>,
    /// Explicit direct-sequence retrieval state, independent of descriptive status.
    pub direct_sequence: ProteinSequenceAvailability,
}
