//! NCI source fields keep their native names and vocabulary.
use crate::{NctId, Text, TrialGet, TrialSearch};
use serde::{Deserialize, Serialize};
/// NCI trial card without registry status or phase coercion.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciTrial {
    /// Publisher NCI identifier.
    pub nci_id: Text,
    /// Registry cross-reference supplied by NCI.
    pub nct_id: NctId,
    /// Native title.
    pub brief_title: Text,
    /// Native status, without a registry translation.
    pub current_trial_status: Text,
    /// Native phase.
    pub phase: Option<super::NciTrialPhase>,
    /// Native short summary, only on exact retrieval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brief_summary: Option<crate::NciTextPage>,
    /// Publisher-provided disease concepts in source order.
    pub diseases: crate::TrialRecords<NciDiseaseRef>,
}
/// One NCI disease concept; names never substitute for an NCIt identity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciDisease {
    /// Source concept name.
    pub name: Text,
    /// Publisher NCIt identity.
    pub nci_thesaurus_concept_id: biodata::NcitId,
    /// Publisher coding provenance, without local interpretation.
    pub inclusion_indicator: Option<Text>,
    /// Native lead-disease marker.
    pub is_lead_disease: Option<bool>,
    /// Original concept categories.
    #[serde(rename = "type", default)]
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub kind: Box<[Text]>,
    /// Original immediate ancestors.
    #[serde(default)]
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub parents: Box<[Text]>,
    /// Original alternative labels.
    #[serde(default)]
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub synonyms: Box<[Text]>,
}
/// NCI search page in native order with native count.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciTrialPage {
    /// Source rows, never merged with another provider.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[NciTrial]>,
    /// Exact source parameters, excluding credentials.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub request: Box<[crate::TrialParameter]>,
    /// Exact condition identity and source-provided mapping, absent for literal text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_resolution: Option<crate::TrialConditionResolution>,
    /// Publisher attribution.
    pub attribution: Box<str>,
}
impl super::ResponseBody for NciTrialPage {
    const SCHEMA: &'static str = "biomcp.nci-trial-page/5";
    type Query = TrialSearch;
}
/// Independent exact NCI answers in caller order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciTrialCards {
    /// One independent answer per requested identity.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[NciTrialItem]>,
    /// Publisher attribution.
    pub attribution: Box<str>,
}
impl super::ResponseBody for NciTrialCards {
    const SCHEMA: &'static str = "biomcp.nci-trial-cards/3";
    type Query = TrialGet;
}
/// An exact NCI card and its source state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciTrialItem {
    /// Exact caller identity.
    pub input: NctId,
    /// Independent retrieval state.
    pub status: crate::Status,
    /// Native source card, absent after failure or confirmed absence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<NciTrial>,
    /// Original reply retrieval time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetched: Option<u64>,
    /// Explicitly requested, source-native sections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<crate::NciTrialSections>,
    /// Proven sections omitted from this request.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub more: Box<[crate::TrialSection]>,
    /// Requested sections without proven public-tier support.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub unsupported_sections: Box<[crate::TrialSection]>,
}

/// Compact source disease reference; the diseases section retains every coding field.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciDiseaseRef {
    /// Native source name.
    pub name: Text,
    /// Source NCIt identity.
    pub nci_thesaurus_concept_id: biodata::NcitId,
}
