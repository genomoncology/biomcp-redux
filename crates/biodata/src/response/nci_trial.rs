//! NCI source fields keep their native names and vocabulary.
use crate::{NctId, Text, TrialGet, TrialSearch};
use serde::{Deserialize, Serialize};
/// NCI trial card without registry status or phase coercion.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub phase: Option<Text>,
    /// Native short summary, only on exact retrieval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brief_summary: Option<Text>,
    /// Publisher-provided disease concepts in source order.
    pub diseases: Box<[NciDisease]>,
}
/// One NCI disease concept; names never substitute for an NCIt identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciDisease {
    /// Source concept name.
    pub name: Text,
    /// Publisher NCIt identity.
    pub nci_thesaurus_concept_id: Text,
}
/// NCI search page in native order with native count.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciTrialPage {
    /// Source rows, never merged with another provider.
    pub items: Box<[NciTrial]>,
    /// Exact source parameters, excluding credentials.
    pub request: Box<[crate::TrialParameter]>,
    /// Publisher attribution.
    pub attribution: Box<str>,
}
impl super::ResponseBody for NciTrialPage {
    const SCHEMA: &'static str = "biomcp.nci_trial_page/1";
    type Query = TrialSearch;
}
/// Independent exact NCI answers in caller order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciTrialCards {
    /// One independent answer per requested identity.
    pub items: Box<[NciTrialItem]>,
    /// Publisher attribution.
    pub attribution: Box<str>,
}
impl super::ResponseBody for NciTrialCards {
    const SCHEMA: &'static str = "biomcp.nci_trial_cards/1";
    type Query = TrialGet;
}
/// An exact NCI card and its source state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    /// Requested sections without proven public-tier support.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub unsupported_sections: Box<[crate::TrialSection]>,
}
