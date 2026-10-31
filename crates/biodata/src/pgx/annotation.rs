use crate::ClinPgxAccession;
use serde::{Deserialize, Serialize};
/// Source-native PgxObjectReference fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxObjectReference {
    /// Native `objCls` when supplied.
    #[serde(rename = "objCls")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obj_cls: Option<Box<str>>,
    /// Native `id`; literal source value.
    pub id: ClinPgxAccession,
    /// Native `name` when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
    /// Native `symbol` when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Box<str>>,
}
/// Source-native PgxCrossReference fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxCrossReference {
    /// Native `id`; literal source value.
    pub id: crate::Identifier<i64>,
    /// Native `resource`; literal source value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Box<str>>,
    /// Native `resourceId`; literal source value.
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Box<str>>,
    /// Native `_url`; literal source value.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _url: Option<Box<str>>,
    /// Native `name` when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
}
/// Native location diplotype reference; its numeric ID is not a ClinPGx accession.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxDiplotypeReference {
    /// Native `id`; no expression, phase or phenotype is inferred.
    pub id: crate::Identifier<i64>,
}
/// Source-native PgxLocation fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxLocation {
    /// Native `id`; literal source value.
    pub id: crate::Identifier<i64>,
    /// Native `buildVersion` when supplied.
    #[serde(rename = "buildVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_version: Option<Box<str>>,
    /// Native `chromosomeId` when supplied.
    #[serde(rename = "chromosomeId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chromosome_id: Option<Box<str>>,
    /// Native `chromosomeName` when supplied.
    #[serde(rename = "chromosomeName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chromosome_name: Option<Box<str>>,
    /// Native `copyNumber`; literal source value.
    #[serde(rename = "copyNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_number: Option<Box<str>>,
    /// Native `diplotypes`; literal source value.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub diplotypes: Box<[PgxDiplotypeReference]>,
    /// Native `displayName`; literal source value.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<Box<str>>,
    /// Native `genes`; literal source value.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub genes: Box<[PgxObjectReference]>,
    /// Native `gpPosition`; literal source value.
    #[serde(rename = "gpPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gp_position: Option<i64>,
    /// Native `haplotypes`; literal source value.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub haplotypes: Box<[PgxObjectReference]>,
    /// Native `linkedObjects`; literal source value.
    #[serde(rename = "linkedObjects")]
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub linked_objects: Box<[PgxObjectReference]>,
    /// Native `refSeqCrossReference` when supplied.
    #[serde(rename = "refSeqCrossReference")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_seq_cross_reference: Option<PgxCrossReference>,
    /// Native `refSeqPosition`; literal source value.
    #[serde(rename = "refSeqPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_seq_position: Option<i64>,
    /// Native `rsid` when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsid: Option<Box<str>>,
    /// Native `tagGene`; literal source value.
    #[serde(rename = "tagGene")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_gene: Option<bool>,
    /// Native `type`; literal source value.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<Box<str>>,
    /// Native `variant` when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<PgxObjectReference>,
}
/// Source-native PgxEvidenceTerm fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxEvidenceTerm {
    /// Native `id`; literal source value.
    pub id: crate::Identifier<i64>,
    /// Native `resource`; literal source value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Box<str>>,
    /// Native `term`; literal source value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<Box<str>>,
    /// Native `termId`; literal source value.
    #[serde(rename = "termId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_id: Option<Box<str>>,
    /// Native `synonyms` when supplied.
    #[serde(default, skip_serializing_if = "crate::absent_or_empty")]
    pub synonyms: Option<Box<[PgxUnobserved]>>,
    /// Native `valid` when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}
/// Source-native PgxAnnotationHeader fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAnnotationHeader {
    /// Native `id`; literal source value.
    pub id: crate::Identifier<i64>,
    /// Native `accessionId`; literal source value.
    #[serde(rename = "accessionId")]
    pub accession_id: ClinPgxAccession,
    /// Native `levelOfEvidence`; literal source value.
    #[serde(rename = "levelOfEvidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_of_evidence: Option<PgxEvidenceTerm>,
    /// Native `location`; literal source value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<PgxLocation>,
    /// Native `name`; literal source value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
    /// Native `relatedChemicals`; literal source value.
    #[serde(rename = "relatedChemicals")]
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub related_chemicals: Box<[PgxObjectReference]>,
    /// Native `relatedChemicalsLogic`; literal source value.
    #[serde(rename = "relatedChemicalsLogic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_chemicals_logic: Option<Box<str>>,
    /// Native `relatedGuidelines`; literal source value.
    #[serde(rename = "relatedGuidelines")]
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub related_guidelines: Box<[PgxObjectReference]>,
    /// Native `relatedLabels`; literal source value.
    #[serde(rename = "relatedLabels")]
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub related_labels: Box<[PgxObjectReference]>,
    /// Native `types`; literal source value.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub types: Box<[Box<str>]>,
}
/// Source-native PgxAnnotationReference fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAnnotationReference {
    /// Native `objCls`; literal source value.
    #[serde(rename = "objCls")]
    pub obj_cls: Box<str>,
    /// Native `id`; literal source value.
    pub id: ClinPgxAccession,
    /// Native `name`; literal source value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
    /// Native `crossReferences`; literal source value.
    #[serde(rename = "crossReferences")]
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub cross_references: Box<[PgxUnobserved]>,
}
/// No element shape was captured for these empty arrays. Populated values require new source proof.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PgxUnobserved {}
