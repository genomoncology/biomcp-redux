use super::annotation::{PgxEvidenceTerm, PgxLocation, PgxObjectReference};
use crate::ClinPgxAccession;
use serde::{Deserialize, Serialize};
/// Source-native PgxAllelePhenotype fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAllelePhenotype {
    /// Native `id`; literal source value.
    pub id: crate::Identifier<i64>,
    /// Native `allele`; literal source value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allele: Option<Box<str>>,
    /// Native `limitedEvidence`; literal source value.
    #[serde(rename = "limitedEvidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_evidence: Option<bool>,
    /// Native `phenotype`; literal source value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phenotype: Option<Box<str>>,
}
/// Source-native PgxAnnotationHistory fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAnnotationHistory {
    /// Native `id`; literal source value.
    pub id: crate::Identifier<i64>,
    /// Native `date`; literal source value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Box<str>>,
    /// Native `type`; literal source value.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<Box<str>>,
    /// Native `version`; literal source value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    /// Native `description` when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
}
/// Source-native PgxAnnotationDetail fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAnnotationDetail {
    /// Native `id`; literal source value.
    pub id: crate::Identifier<i64>,
    /// Native `accessionId`; literal source value.
    #[serde(rename = "accessionId")]
    pub accession_id: ClinPgxAccession,
    /// Native `allelePhenotypes`; literal source value.
    #[serde(rename = "allelePhenotypes")]
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub allele_phenotypes: Box<[PgxAllelePhenotype]>,
    /// Native `conflictingVariantAnnotationIds`; literal source value.
    #[serde(rename = "conflictingVariantAnnotationIds")]
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub conflicting_variant_annotation_ids: Box<[i64]>,
    /// Native `history`; literal source value.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub history: Box<[PgxAnnotationHistory]>,
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
    /// Native `objCls`; literal source value.
    #[serde(rename = "objCls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obj_cls: Option<Box<str>>,
    /// Native `overrideLevel`; literal source value.
    #[serde(rename = "overrideLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_level: Option<bool>,
    /// Native `pediatric`; literal source value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pediatric: Option<bool>,
    /// Native `relatedAlleles`; literal source value.
    #[serde(rename = "relatedAlleles")]
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub related_alleles: Box<[PgxObjectReference]>,
    /// Native `relatedChemicals`; literal source value.
    #[serde(rename = "relatedChemicals")]
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub related_chemicals: Box<[PgxObjectReference]>,
    /// Native `relatedChemicalsLogic`; literal source value.
    #[serde(rename = "relatedChemicalsLogic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_chemicals_logic: Option<Box<str>>,
    /// Native `relatedDiseases`; literal source value.
    #[serde(rename = "relatedDiseases")]
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub related_diseases: Box<[PgxObjectReference]>,
    /// Native `relatedGuidelines`; literal source value.
    #[serde(rename = "relatedGuidelines")]
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub related_guidelines: Box<[PgxObjectReference]>,
    /// Native `relatedLabels`; literal source value.
    #[serde(rename = "relatedLabels")]
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub related_labels: Box<[PgxObjectReference]>,
    /// Native `score`; literal source value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<crate::FiniteNumber>,
    /// Native `scoreDetails`; literal source value.
    #[serde(rename = "scoreDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_details: Option<Box<str>>,
    /// Native `types`; literal source value.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub types: Box<[Box<str>]>,
    /// Native `overrideLevelDescription` when supplied.
    #[serde(rename = "overrideLevelDescription")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub override_level_description: Option<Box<str>>,
}
