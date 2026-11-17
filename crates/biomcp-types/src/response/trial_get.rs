use crate::{
    NctId, Section, Status, TrialAgeBound, TrialArm, TrialIntervention, TrialOutcome,
    TrialPostedCard, TrialSection, TrialSex,
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
    pub source_updated: Option<crate::SourceUpdated>,
    /// Publisher attribution and scope.
    pub attribution: Box<str>,
}
impl super::ResponseBody for TrialCards {
    const SCHEMA: &'static str = "biomcp.trial-cards/11";
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
    pub card: Option<TrialPostedCard>,
    /// Original record retrieval timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetched: Option<u64>,
    /// Explicitly requested source sections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<TrialSections>,
    /// Available sections omitted from this request.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub more: Box<[TrialSection]>,
    /// Why a failed item carries no card, as a whole failed reply would say it; absent otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure: Option<TrialItemFailure>,
}
/// Why one trial in a batch was not read.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialItemFailure {
    /// The source that was asked.
    pub source: crate::Source,
    /// Typed reason; absent for a local storage error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<crate::Reason>,
    /// What went wrong and what to do, in plain words.
    pub problem: Box<str>,
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
    pub locations: Option<Section<TrialRecords<TrialSite>>>,
    /// Central contacts followed by overall officials, each in source order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Section<TrialRecords<TrialContactRecord>>>,
    /// Source eligibility fields and exact text fragment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligibility: Option<Section<TrialEligibilityPage>>,
    /// Whole registered outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<Section<TrialRecords<TrialOutcome>>>,
    /// Posted outcomes, then participant flow, baseline and adverse events; each collection in source order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Section<TrialRecords<crate::TrialResultRecord>>>,
    /// Participant-flow records from the posted results alone, in source order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<Section<TrialRecords<crate::TrialResultRecord>>>,
    /// Baseline-characteristics records from the posted results alone, in source order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline: Option<Section<TrialRecords<crate::TrialResultRecord>>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_age: Option<TrialAgeBound>,
    /// Source maximum age.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_age: Option<TrialAgeBound>,
    /// Source sex field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<TrialSex>,
    /// Source healthy-volunteer field.
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[cfg_attr(feature = "schema", schemars(rename = "TrialRecords_{T}"))]
pub struct TrialRecords<T> {
    /// Checked request for the next intact records, absent when complete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuation: Option<crate::TrialGet>,
    // The total comes before the rows (0942): a reader of only the start sees it.
    /// Offset in the unchanged source collection.
    pub offset: u32,
    /// Original collection size.
    pub total: u32,
    /// Complete records in source order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[T]>,
}
/// Source arm rows followed by source intervention rows, each in its native order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "record", content = "data", rename_all = "kebab-case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum TrialArmRecord {
    /// Original arm with its intervention names.
    Arm(TrialArm),
    /// Original intervention with aliases and arm labels.
    Intervention(TrialIntervention),
}

/// One source site, kept whole during pagination.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "TrialLocation"))]
pub struct TrialSite {
    /// The registry's site record.
    #[serde(flatten)]
    pub site: crate::TrialLocation,
    /// Straight-line miles from the requested --lat/--lon point, not travel distance; absent without a point or site coordinates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distance_mi: Option<biodata::FiniteNumber>,
}
impl From<crate::TrialLocation> for TrialSite {
    fn from(site: crate::TrialLocation) -> Self {
        Self {
            site,
            distance_mi: None,
        }
    }
}
/// Distinct registry-wide contact collections; site contacts stay on locations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "record", content = "data", rename_all = "kebab-case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum TrialContactRecord {
    /// A central contact for this trial.
    Central(crate::TrialContact),
    /// An overall official for this trial.
    Official(crate::TrialOfficial),
}
