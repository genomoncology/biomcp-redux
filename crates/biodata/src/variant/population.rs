//! Source population observations, without computed frequency or classification.
use crate::{Attribution, Build, FiniteNumber, Text, VariantRef};
use serde::{Deserialize, Serialize};
/// One source cohort allele-frequency result under the shared variant model.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
pub struct PopulationFrequency {
    /// Exact source-qualified allele subject.
    pub subject: VariantRef,
    /// Source dataset identifier.
    pub dataset: Text,
    /// Source dataset assembly.
    pub build: Build,
    /// Source group identifier, absent for the complete sequencing subset.
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub group: Option<Text>,
    /// Source allele count; zero remains an observation.
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub allele_count: Option<u64>,
    /// Source allele denominator.
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub allele_number: Option<u64>,
    /// Source homozygote count, never derived.
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub homozygotes: Option<u64>,
    /// Source reported frequency, never calculated from counts.
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub frequency: Option<FiniteNumber>,
    /// Every source filter in native order.
    #[serde(default,skip_serializing_if="<[ _ ]>::is_empty")]
    pub filters: Box<[Text]>,
    /// Native source record and dataset provenance.
    pub attribution: Attribution,
}
