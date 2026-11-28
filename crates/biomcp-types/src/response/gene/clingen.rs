use crate::Text;
use serde::{Deserialize, Serialize};
/// ClinGen gene validity, dosage and actionability in published source order.
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
    /// Outcome of reading the detailed gene dosage file, when the gene has an NCBI ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dosage_evidence_status: Option<crate::SourceOutcome>,
    /// Published copy-loss score and the papers supporting it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub haploinsufficiency_evidence: Option<ClinGenGeneDosage>,
    /// Published copy-gain score and the papers supporting it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triplosensitivity_evidence: Option<ClinGenGeneDosage>,
    /// Clinical actionability summaries for single-gene requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actionability: Option<Box<ClinGenActionability>>,
}

/// ClinGen's independent adult and pediatric summaries.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ClinGenActionability {
    /// Publisher's index of clinical actionability reports.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub report_index: Text,
    /// Adult working group summary.
    pub adult: ClinGenActionabilityContext,
    /// Pediatric working group summary.
    pub pediatric: ClinGenActionabilityContext,
}

/// Result of looking up one gene in one actionability summary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ClinGenActionabilityContext {
    /// Read outcome and matching row count when available.
    pub status: crate::SourceOutcome,
    /// Matching rows in publisher order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub curations: Box<[ClinGenActionabilityRow]>,
}

/// One gene-condition row from the ClinGen actionability score summary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ClinGenActionabilityRow {
    /// Native condition label.
    pub disease: Text,
    /// Curation status as published, including retractions.
    pub status: Text,
    /// Published status of the early rule-out stage.
    pub early_rule_out: Text,
    /// Link to the evidence summary report.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub report_url: Text,
    /// The outcome and intervention when supplied by the working group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pair: Option<ClinGenActionabilityPair>,
}

/// A published outcome–intervention pair and its working group scores.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ClinGenActionabilityPair {
    /// Outcome described by ClinGen.
    pub outcome: Text,
    /// Intervention described by ClinGen.
    pub intervention: Text,
    /// Severity score as published.
    pub severity: Text,
    /// Likelihood score and evidence grade as published.
    pub likelihood: Text,
    /// Nature of intervention score as published.
    pub nature_of_intervention: Text,
    /// Effectiveness score and evidence grade as published.
    pub effectiveness: Text,
    /// Overall score and evidence grades as published.
    pub overall: Text,
}
/// Evidence from the detailed ClinGen gene dosage file for one direction.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ClinGenGeneDosage {
    /// Published score, when evaluated. Zero means no evidence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<u32>,
    /// Supporting PubMed identifiers in source order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub pmids: Box<[biodata::Pmid]>,
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
    pub score: u32,
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
