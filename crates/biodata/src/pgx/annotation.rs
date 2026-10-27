use crate::ClinPgxAccession;
use serde::{Deserialize, Serialize};
/// Source-native PgxObjectReference fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxCrossReference {
    /// Native `id`; literal source value.
    pub id: i64,
    /// Native `resource`; literal source value.
    pub resource: Box<str>,
    /// Native `resourceId`; literal source value.
    #[serde(rename = "resourceId")]
    pub resource_id: Box<str>,
    /// Native `_url`; literal source value.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub _url: Box<str>,
    /// Native `name` when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
}
/// Native location diplotype reference; its numeric ID is not a ClinPGx accession.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxDiplotypeReference {
    /// Native `id`; no expression, phase or phenotype is inferred.
    pub id: i64,
}
/// Source-native PgxLocation fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxLocation {
    /// Native `id`; literal source value.
    pub id: i64,
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
    pub copy_number: Box<str>,
    /// Native `diplotypes`; literal source value.
    pub diplotypes: Box<[PgxDiplotypeReference]>,
    /// Native `displayName`; literal source value.
    #[serde(rename = "displayName")]
    pub display_name: Box<str>,
    /// Native `genes`; literal source value.
    pub genes: Box<[PgxObjectReference]>,
    /// Native `gpPosition`; literal source value.
    #[serde(rename = "gpPosition")]
    pub gp_position: i64,
    /// Native `haplotypes`; literal source value.
    pub haplotypes: Box<[PgxObjectReference]>,
    /// Native `linkedObjects`; literal source value.
    #[serde(rename = "linkedObjects")]
    pub linked_objects: Box<[PgxObjectReference]>,
    /// Native `refSeqCrossReference` when supplied.
    #[serde(rename = "refSeqCrossReference")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_seq_cross_reference: Option<PgxCrossReference>,
    /// Native `refSeqPosition`; literal source value.
    #[serde(rename = "refSeqPosition")]
    pub ref_seq_position: i64,
    /// Native `rsid` when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsid: Option<Box<str>>,
    /// Native `tagGene`; literal source value.
    #[serde(rename = "tagGene")]
    pub tag_gene: bool,
    /// Native `type`; literal source value.
    #[serde(rename = "type")]
    pub kind: Box<str>,
    /// Native `variant` when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<PgxObjectReference>,
}
/// Source-native PgxEvidenceTerm fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxEvidenceTerm {
    /// Native `id`; literal source value.
    pub id: i64,
    /// Native `resource`; literal source value.
    pub resource: Box<str>,
    /// Native `term`; literal source value.
    pub term: Box<str>,
    /// Native `termId`; literal source value.
    #[serde(rename = "termId")]
    pub term_id: Box<str>,
    /// Native `synonyms` when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Box<[PgxUnobserved]>>,
    /// Native `valid` when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}
/// Source-native PgxAnnotationHeader fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAnnotationHeader {
    /// Native `id`; literal source value.
    pub id: i64,
    /// Native `accessionId`; literal source value.
    #[serde(rename = "accessionId")]
    pub accession_id: ClinPgxAccession,
    /// Native `levelOfEvidence`; literal source value.
    #[serde(rename = "levelOfEvidence")]
    pub level_of_evidence: PgxEvidenceTerm,
    /// Native `location`; literal source value.
    pub location: PgxLocation,
    /// Native `name`; literal source value.
    pub name: Box<str>,
    /// Native `relatedChemicals`; literal source value.
    #[serde(rename = "relatedChemicals")]
    pub related_chemicals: Box<[PgxObjectReference]>,
    /// Native `relatedChemicalsLogic`; literal source value.
    #[serde(rename = "relatedChemicalsLogic")]
    pub related_chemicals_logic: Box<str>,
    /// Native `relatedGuidelines`; literal source value.
    #[serde(rename = "relatedGuidelines")]
    pub related_guidelines: Box<[PgxObjectReference]>,
    /// Native `relatedLabels`; literal source value.
    #[serde(rename = "relatedLabels")]
    pub related_labels: Box<[PgxObjectReference]>,
    /// Native `types`; literal source value.
    pub types: Box<[Box<str>]>,
}
/// Source-native PgxAnnotationReference fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAnnotationReference {
    /// Native `objCls`; literal source value.
    #[serde(rename = "objCls")]
    pub obj_cls: Box<str>,
    /// Native `id`; literal source value.
    pub id: ClinPgxAccession,
    /// Native `name`; literal source value.
    pub name: Box<str>,
    /// Native `crossReferences`; literal source value.
    #[serde(rename = "crossReferences")]
    pub cross_references: Box<[PgxUnobserved]>,
}
/// No element shape was captured for these empty arrays. Populated values require new source proof.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PgxUnobserved {}
