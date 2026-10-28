//! Neutral study records preserve source metadata without I/O or clinical inference.
mod id;
use crate::{Pmid, Text};
pub use id::*;
use serde::{Deserialize, Serialize};
/// Compact source study identity; source codes and counts retain their own scope.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudyHeader {
    /// Exact case-sensitive source identity.
    pub study_id: StudyId,
    /// Native study name.
    pub name: Text,
    /// cBioPortal namespace only; no ontology inference.
    pub cancer_type_id: CbioportalCancerTypeId,
    /// Native allSampleCount, never a patient or profiled-gene denominator.
    pub all_sample_count: u64,
    /// Every validated source PMID in native order, including repetitions; absent remains absent.
    #[serde(default, skip_serializing_if = "crate::absent_or_empty")]
    pub pmids: Option<Box<[Pmid]>>,
    /// Native citation, absent for unpublished records.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation: Option<Box<str>>,
    /// Native study metadata label, never a coordinate assembly.
    pub reference_genome: Text,
}
/// Source-specific study card metadata; no inferred denominator or gene aggregate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Study {
    /// Compact study identity and publication references.
    pub header: StudyHeader,
    /// Native description.
    pub description: Box<str>,
    /// Native sequencedSampleCount, not gene coverage.
    pub sequenced_sample_count: u64,
    /// Native cnaSampleCount.
    pub cna_sample_count: u64,
    /// Native structuralVariantCount; its name does not imply sample units.
    pub structural_variant_count: u64,
    /// Native treatmentCount without a unit conversion.
    pub treatment_count: u64,
    /// Only an explicitly stated card patientCount; captured cards omit it, distinct from zero.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patient_count: Option<u64>,
    /// Independent resource-scoped counts, never summed into study totals.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub resource_counts: Box<[StudyResourceCount]>,
    /// Native timezone-unstated import time.
    pub import_date: StudyImportTime,
    /// Native access groups, possibly empty; not a reuse license.
    pub groups: Box<str>,
    /// Native access metadata; not a reuse license.
    pub read_permission: bool,
    /// Native public-study access metadata.
    pub public_study: bool,
}
/// One native resource scope; counts remain attached to its study and resource.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudyResourceCount {
    /// Native resource ID.
    pub resource_id: StudyResourceId,
    /// Native resource display name.
    pub display_name: Box<str>,
    /// Native resource description.
    pub description: Box<str>,
    /// Extensible native resource type, not a derived count unit.
    pub resource_type: Text,
    /// Native priority string, without numeric reinterpretation.
    pub priority: Text,
    /// Native access/display setting, not a reuse license.
    pub open_by_default: bool,
    /// Exact resource study scope.
    pub cancer_study_identifier: StudyId,
    /// Native count within this resource only.
    pub sample_count: u64,
    /// Native patient count within this resource only.
    pub patient_count: u64,
}
mod sections;
pub use sections::*;
