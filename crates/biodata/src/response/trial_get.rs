use crate::{NctId, Section, Status, TrialArm, TrialCard, TrialAgeBound, TrialSex, TrialIntervention, TrialOutcome, TrialSection};
use serde::{Deserialize, Serialize};
/// Trial answers in original input order, retaining independent states.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
pub struct TrialCards {
    /// One answer per requested input, including absence and failure.
    pub items: Box<[TrialCardItem]>,
    /// Registry processing timestamp, distinct from each study's update and fetch times.
    pub source_updated: Option<crate::Text>,
    /// Publisher attribution and scope.
    pub attribution: Box<str>,
}
impl super::ResponseBody for TrialCards {
    const SCHEMA: &'static str = "biomcp.trial_cards/1";
    type Query = crate::TrialGet;
}
/// Result for one exact requested NCT identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
pub struct TrialCardItem {
    /// Exact requested identity.
    pub input: NctId,
    /// Independent resolution state.
    pub status: Status,
    /// Source card, absent after confirmed 404 or failed lookup.
    #[serde(skip_serializing_if="Option::is_none")]
    pub card: Option<TrialCard>,
    /// Original record retrieval timestamp.
    #[serde(skip_serializing_if="Option::is_none")]
    pub fetched: Option<u64>,
    /// Explicitly requested source sections.
    #[serde(skip_serializing_if="Option::is_none")]
    pub sections: Option<TrialSections>,
    /// Available sections omitted from this request.
    #[serde(default,skip_serializing_if="<[_]>::is_empty")]
    pub more: Box<[TrialSection]>,
}
/// Independently bounded sections from the same source protocol snapshot.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
pub struct TrialSections {
    /// Source eligibility fields and exact text fragment.
    #[serde(skip_serializing_if="Option::is_none")]
    pub eligibility: Option<Section<TrialEligibilityPage>>,
    /// Whole registered outcomes.
    #[serde(skip_serializing_if="Option::is_none")]
    pub outcomes: Option<Section<TrialRecords<TrialOutcome>>>,
    /// Whole source arm and intervention records.
    #[serde(skip_serializing_if="Option::is_none")]
    pub arms: Option<Section<TrialRecords<TrialArmRecord>>>,
}
/// Character paging preserves source whitespace and counts Unicode scalar values.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
pub struct TrialEligibilityPage {
    /// Structured source fields; criteria contains only this page's unchanged fragment.
    pub eligibility: TrialAgeBound, TrialSex,
    /// Original character offset.
    pub offset: u32,
    /// Complete original text length in Unicode scalar values.
    pub total_characters: u32,
}
/// Whole-record page retaining collection order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
pub struct TrialRecords<T> {
    /// Complete records in source order.
    pub items: Box<[T]>,
    /// Offset in the unchanged source collection.
    pub offset: u32,
    /// Original collection size.
    pub total: u32,
}
/// Source arm rows followed by source intervention rows, each in its native order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag="record",content="data",rename_all="snake_case",deny_unknown_fields)]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
pub enum TrialArmRecord {
    /// Original arm with its intervention names.
    Arm(TrialArm),
    /// Original intervention with aliases and arm labels.
    Intervention(TrialIntervention),
}
