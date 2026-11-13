use crate::Text;
use serde::{Deserialize, Serialize};
/// ClinGen gene validity and dosage assessments in published source order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ClinGenGene {
    /// Outcome of the gene–disease validity source read.
    pub validity_status: crate::SourceOutcome,
    /// Outcome of the dosage sensitivity source read.
    pub dosage_status: crate::SourceOutcome,
    /// Whole gene–disease validity records in file order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub validity: Box<[ClinGenValidity]>,
    /// Source haploinsufficiency assessment, when a dosage record exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub haploinsufficiency: Option<Text>,
    /// Source triplosensitivity assessment, when a dosage record exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triplosensitivity: Option<Text>,
    /// Source dosage assessment date, preserving its original spelling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dosage_date: Option<Text>,
}
/// One published ClinGen gene–disease validity record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ClinGenValidity {
    /// Source disease label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disease: Option<Text>,
    /// Source MONDO identifier.
    pub mondo_id: biodata::MondoId,
    /// Source mode of inheritance.
    pub mode_of_inheritance: Text,
    /// Source classification without inference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<Text>,
    /// Source classification date, preserving its original spelling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_date: Option<Text>,
    /// Source expert panel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expert_panel: Option<Text>,
    /// Source report URL.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_url: Option<Text>,
}
/// One ClinGen region dosage curation (an ISCA region) from the GRCh38 region file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ClinGenRegion {
    /// ClinGen ISCA region identifier, such as ISCA-37400.
    pub id: Text,
    /// Source region name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Text>,
    /// Source cytoband, such as 16p11.2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cytoband: Option<Text>,
    /// GRCh38 location as published, such as chr16:29638676-30188531.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Text>,
    /// Haploinsufficiency (copy loss) assessment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub haploinsufficiency: Option<ClinGenRegionDosage>,
    /// Triplosensitivity (copy gain) assessment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triplosensitivity: Option<ClinGenRegionDosage>,
    /// Source date last evaluated, preserving its original spelling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Text>,
}
/// One ClinGen region dosage assessment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ClinGenRegionDosage {
    /// ClinGen score: 0 no evidence, 1 little, 2 some, 3 sufficient evidence for dosage pathogenicity, 30 autosomal recessive, 40 dosage sensitivity unlikely.
    pub score: u8,
    /// Source description of the score.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Text>,
    /// Supporting PubMed identifiers in source order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub pmids: Box<[biodata::Pmid]>,
    /// Source disease identifier, usually MONDO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disease_id: Option<Text>,
}
