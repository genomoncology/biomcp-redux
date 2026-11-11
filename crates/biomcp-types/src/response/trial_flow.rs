//! Posted participant flow and baseline characteristics, retaining source labels and numeric strings.
use crate::{TrialResultClass, TrialResultDenominator};
use serde::{Deserialize, Serialize};

/// Native participant-flow notes; arms and periods follow as their own records.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialParticipantFlow {
    /// Native `pre_assignment_details` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_assignment_details: Option<Box<str>>,
    /// Native `recruitment_details` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recruitment_details: Option<Box<str>>,
    /// Native `type_units_analyzed` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_units_analyzed: Option<Box<str>>,
}

/// One native flow period; counts are never summed or subtracted.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialFlowPeriod {
    /// Native `title` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
    /// Native `milestones` field.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub milestones: Box<[TrialFlowMilestone]>,
    /// Native `drop_withdraws` field.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub drop_withdraws: Box<[TrialFlowDrop]>,
}

/// Native milestone such as STARTED, COMPLETED or NOT COMPLETED.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialFlowMilestone {
    /// Native `type` field.
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Box<str>>,
    /// Native `comment` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<Box<str>>,
    /// Native `achievements` field.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub achievements: Box<[TrialFlowAchievement]>,
}

/// Reported participants reaching a milestone in one flow group.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialFlowAchievement {
    /// Native `group_id` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Box<str>>,
    /// Native `comment` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<Box<str>>,
    /// Native `num_subjects` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_subjects: Option<Box<str>>,
    /// Native `num_units` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_units: Option<Box<str>>,
}

/// Native reason participants did not complete a period.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialFlowDrop {
    /// Native `type` field.
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Box<str>>,
    /// Native `comment` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<Box<str>>,
    /// Native `reasons` field.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub reasons: Box<[TrialFlowReason]>,
}

/// Reported participants leaving for one reason in one flow group.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialFlowReason {
    /// Native `group_id` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Box<str>>,
    /// Native `comment` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<Box<str>>,
    /// Native `num_subjects` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_subjects: Option<Box<str>>,
}

/// Native baseline population; groups and measures follow as their own records.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialBaseline {
    /// Native `population_description` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub population_description: Option<Box<str>>,
    /// Native `type_units_analyzed` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_units_analyzed: Option<Box<str>>,
    /// Native `denoms` field.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub denoms: Box<[TrialResultDenominator]>,
}

/// One native baseline characteristic; values are never recomputed.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialBaselineMeasure {
    /// Native `title` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
    /// Native `description` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    /// Native `population_description` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub population_description: Option<Box<str>>,
    /// Native `param_type` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param_type: Option<Box<str>>,
    /// Native `dispersion_type` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dispersion_type: Option<Box<str>>,
    /// Native `unit_of_measure` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<Box<str>>,
    /// Native `calculate_pct` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calculate_pct: Option<bool>,
    /// Native `denom_units_selected` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denom_units_selected: Option<Box<str>>,
    /// Native `denoms` field.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub denoms: Box<[TrialResultDenominator]>,
    /// Native `classes` field.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub classes: Box<[TrialResultClass]>,
}
