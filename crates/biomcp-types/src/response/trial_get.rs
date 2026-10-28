use crate::{
    NctId, Section, Status, TrialAgeBound, TrialArm, TrialCard, TrialIntervention, TrialOutcome,
    TrialSection, TrialSex,
};
use serde::{Deserialize, Serialize};
/// Trial answers in original input order, retaining independent states.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialCards {
    /// One answer per requested input, including absence and failure.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[TrialCardItem]>,
    /// Registry processing timestamp, distinct from each study's update and fetch times.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_updated: Option<super::SourceTimestamp>,
    /// Publisher attribution and scope.
    pub attribution: Box<str>,
}
impl super::ResponseBody for TrialCards {
    const SCHEMA: &'static str = "biomcp.trial-cards/3";
    type Query = crate::TrialGet;
}
/// Result for one exact requested NCT identity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialCardItem {
    /// Exact requested identity.
    pub input: NctId,
    /// Independent resolution state.
    pub status: Status,
    /// Source card, absent after confirmed 404 or failed lookup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<TrialCard>,
    /// Original record retrieval timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetched: Option<u64>,
    /// Explicitly requested source sections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<TrialSections>,
    /// Available sections omitted from this request.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub more: Box<[TrialSection]>,
}
/// Independently bounded sections from the same source protocol snapshot.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialSections {
    /// Source publication records, without inferred roles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Section<TrialRecords<crate::TrialReference>>>,
    /// Posted document manifest, without downloading its files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Section<TrialRecords<crate::TrialDocument>>>,
    /// Whole sites with their original contacts attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Section<TrialRecords<crate::TrialLocation>>>,
    /// Central contacts followed by overall officials, each in source order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Section<TrialRecords<TrialContactRecord>>>,
    /// Source eligibility fields and exact text fragment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligibility: Option<Section<TrialEligibilityPage>>,
    /// Whole registered outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<Section<TrialRecords<TrialOutcome>>>,
    /// Whole source arm and intervention records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arms: Option<Section<TrialRecords<TrialArmRecord>>>,
}
/// Character paging preserves source whitespace and counts Unicode scalar values.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialEligibilityPage {
    /// Checked request for the next intact fragment, absent when complete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuation: Option<crate::TrialGet>,
    /// Source minimum age.
    pub minimum_age: TrialAgeBound,
    /// Source maximum age.
    pub maximum_age: TrialAgeBound,
    /// Source sex field.
    pub sex: Option<TrialSex>,
    /// Source healthy-volunteer field.
    pub healthy_volunteers: Option<bool>,
    /// Exact original text fragment, including whitespace.
    pub criteria: Box<str>,
    /// The page ends within a criterion whose lines exceed the page budget.
    #[serde(default)]
    pub criterion_continues: bool,
    /// Original character offset.
    pub offset: u32,
    /// Complete original text length in Unicode scalar values.
    pub total_characters: u32,
}
/// Whole-record page retaining collection order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialRecords<T> {
    /// Checked request for the next intact records, absent when complete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuation: Option<crate::TrialGet>,
    /// Complete records in source order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[T]>,
    /// Offset in the unchanged source collection.
    pub offset: u32,
    /// Original collection size.
    pub total: u32,
}
/// Source arm rows followed by source intervention rows, each in its native order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "record",
    content = "data",
    rename_all = "snake_case",
    deny_unknown_fields
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum TrialArmRecord {
    /// Original arm with its intervention names.
    Arm(TrialArm),
    /// Original intervention with aliases and arm labels.
    Intervention(TrialIntervention),
}

/// Distinct registry-wide contact collections; site contacts stay on locations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "record",
    content = "data",
    rename_all = "snake_case",
    deny_unknown_fields
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum TrialContactRecord {
    /// A central contact for this trial.
    Central(crate::TrialContact),
    /// An overall official for this trial.
    Official(crate::TrialOfficial),
}
