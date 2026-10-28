use super::annotation::{PgxEvidenceTerm, PgxLocation, PgxObjectReference};
use crate::ClinPgxAccession;
use serde::{Deserialize, Serialize};
/// Source-native PgxAllelePhenotype fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAllelePhenotype {
    /// Native `id`; literal source value.
    pub id: i64,
    /// Native `allele`; literal source value.
    pub allele: Box<str>,
    /// Native `limitedEvidence`; literal source value.
    #[serde(rename = "limitedEvidence")]
    pub limited_evidence: bool,
    /// Native `phenotype`; literal source value.
    pub phenotype: Box<str>,
}
/// Source-native PgxAnnotationHistory fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAnnotationHistory {
    /// Native `id`; literal source value.
    pub id: i64,
    /// Native `date`; literal source value.
    pub date: Box<str>,
    /// Native `type`; literal source value.
    #[serde(rename = "type")]
    pub kind: Box<str>,
    /// Native `version`; literal source value.
    pub version: i64,
    /// Native `description` when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
}
/// Source-native PgxAnnotationDetail fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAnnotationDetail {
    /// Native `id`; literal source value.
    pub id: i64,
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
    pub level_of_evidence: PgxEvidenceTerm,
    /// Native `location`; literal source value.
    pub location: PgxLocation,
    /// Native `name`; literal source value.
    pub name: Box<str>,
    /// Native `objCls`; literal source value.
    #[serde(rename = "objCls")]
    pub obj_cls: Box<str>,
    /// Native `overrideLevel`; literal source value.
    #[serde(rename = "overrideLevel")]
    pub override_level: bool,
    /// Native `pediatric`; literal source value.
    pub pediatric: bool,
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
    pub related_chemicals_logic: Box<str>,
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
    pub score: crate::FiniteNumber,
    /// Native `scoreDetails`; literal source value.
    #[serde(rename = "scoreDetails")]
    pub score_details: Box<str>,
    /// Native `types`; literal source value.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub types: Box<[Box<str>]>,
    /// Native `overrideLevelDescription` when supplied.
    #[serde(rename = "overrideLevelDescription")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub override_level_description: Option<Box<str>>,
}
