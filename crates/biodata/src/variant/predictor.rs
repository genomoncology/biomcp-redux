//! Source-owned measurements and component provenance, without score interpretation.
use crate::{Attribution, FiniteNumber, Text, VariantRef};
use serde::{Deserialize, Serialize};
/// One unmodified source score, never a locally blended assessment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PredictorScore {
    /// Exact source-qualified subject.
    pub subject: VariantRef,
    /// Published component field path.
    pub method: Text,
    /// Source number, including zero.
    pub score: FiniteNumber,
    /// Source label without code expansion or interpretation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Text>,
    /// Source-owned record and dataset release.
    pub attribution: Attribution,
}
/// Metadata for a component carried by an aggregator.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantComponent {
    /// Component name from aggregator metadata.
    pub name: Text,
    /// Component version; absent when the source reports none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Text>,
    /// Component license URL, without any inferred grant.
    pub license: Text,
    /// Aggregator build version.
    pub aggregator_build: Text,
    /// Aggregator build timestamp, distinct from a component release.
    pub aggregator_built: Text,
    /// Component upload timestamp when the source states one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded: Option<Text>,
    /// Whole source snapshot warning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<Text>,
}
/// Source prediction scores and the terms of their individual components.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantPredictors {
    /// Every retained source score in declared field order.
    pub scores: Box<[PredictorScore]>,
    /// Independent component versions and licenses.
    pub components: Box<[VariantComponent]>,
}
/// Source conservation measurements; no local pathogenicity inference.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantConservation {
    /// Every retained source measurement in declared field order.
    pub scores: Box<[PredictorScore]>,
    /// Independent component versions and licenses.
    pub components: Box<[VariantComponent]>,
}
/// Historical COSMIC fields carried by MyVariant, without direct COSMIC access.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantCosmic {
    /// Native historical records in source order.
    pub records: Box<[CosmicRecord]>,
    /// Snapshot version, dates, warning and terms.
    pub components: Box<[VariantComponent]>,
}
/// One source record from MyVariant's historical COSMIC copy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CosmicRecord {
    /// Exact source-qualified subject.
    pub subject: VariantRef,
    /// Native COSMIC record identifier.
    pub cosmic_id: Text,
    /// Source nucleotide text, without repair or normalization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mut_nt: Option<Text>,
    /// Source frequency; never recomputed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mut_freq: Option<FiniteNumber>,
    /// Every source tumor-site label in native order.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub tumor_site: Box<[Text]>,
}
