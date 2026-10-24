use crate::{FiniteNumber, Text};
use serde::{Deserialize, Serialize};
/// HPA tissue RNA abundance in normalized transcripts per million.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TissueNtpm {
    /// Exact source tissue name.
    pub tissue: Text,
    /// Finite nonnegative nTPM.
    pub ntpm: FiniteNumber,
}
/// HPA cancer RNA abundance in protein-coding transcripts per million.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CancerPtpm {
    /// Exact source cancer tissue name.
    pub tissue: Text,
    /// Finite nonnegative pTPM.
    pub ptpm: FiniteNumber,
}
/// Gene-level HPA expression and subcellular annotations.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Expression {
    /// Source RNA tissue specificity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rna_tissue_specificity: Option<Text>,
    /// Source RNA tissue distribution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rna_tissue_distribution: Option<Text>,
    /// Tissue-specific nTPM pairs.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub rna_tissue_ntpm: Box<[TissueNtpm]>,
    /// Source RNA cancer specificity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rna_cancer_specificity: Option<Text>,
    /// Cancer-specific pTPM pairs.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub rna_cancer_ptpm: Box<[CancerPtpm]>,
    /// Source main subcellular locations.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub subcellular_main: Box<[Text]>,
    /// Source additional subcellular locations.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub subcellular_additional: Box<[Text]>,
}
