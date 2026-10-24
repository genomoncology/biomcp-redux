use crate::{Build, FiniteNumber, PopulationFrequency, Text, VariantId};
use serde::{Deserialize, Serialize};
/// Bounded population summary with independent optional sequencing datasets.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantPopulation {
    /// Source allele identifier.
    pub variant: Text,
    /// Source dataset selector.
    pub dataset: Text,
    /// Source dataset assembly.
    pub build: Build,
    /// Exome observation; omitted when the source dataset is null.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exome: Option<Box<PopulationFrequency>>,
    /// Genome observation; omitted when the source dataset is null.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub genome: Option<Box<PopulationFrequency>>,
}
/// One native ancestry row qualified by its sequencing subset.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PopulationGroup {
    /// Sequencing subset; overlapping groups are never summed.
    pub subset: PopulationSubset,
    /// Unchanged source counts and group identifier.
    pub observation: PopulationFrequency,
}
crate::macros::wire_enum! {
    /// Independent source sequencing subset.
    PopulationSubset {
        /// Exome observations.
        Exome => "exome",
        /// Genome observations.
        Genome => "genome",
    }
}
/// One bounded page of source ancestry rows in source order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantPopulationDetail {
    /// Source allele identifier.
    pub variant: Text,
    /// Source dataset selector.
    pub dataset: Text,
    /// Source assembly.
    pub build: Build,
    /// Selected groups in source order, each qualified by its subset.
    pub groups: Box<[PopulationGroup]>,
}
/// One page of exact rsID catalog associations.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantGwas {
    /// Exact source rsID.
    pub rs_id: VariantId,
    /// Native source associations, without inferred clinical meaning.
    pub associations: Box<[GwasAssociation]>,
    /// Exact source page bounds and total.
    pub page: crate::PageInfo,
    /// Continue this section using the same rsID and source selectors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuation: Option<crate::VariantGet>,
}
/// Source association with unchanged trait and effect labels.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GwasAssociation {
    /// Native association identity.
    pub id: u64,
    /// Study accession.
    pub study: Text,
    /// Source reported traits.
    pub traits: Box<[Text]>,
    /// Every source effect allele paired with its rsID.
    pub alleles: Box<[GwasEffectAllele]>,
    /// Reported p value.
    pub p_value: FiniteNumber,
    /// Source beta text, including units and direction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beta: Option<Text>,
    /// Source odds ratio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub odds_ratio: Option<FiniteNumber>,
    /// Source allele frequency text, without interpreting NR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_frequency: Option<Text>,
    /// Source effect interval.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<Text>,
    /// Source publication identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publication: Option<Text>,
}
/// Catalog effect allele paired with its exact rsID.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GwasEffectAllele {
    /// Source rsID.
    pub rs_id: Text,
    /// Source allele text.
    pub allele: Text,
}
