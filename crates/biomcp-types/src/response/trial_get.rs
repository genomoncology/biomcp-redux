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
    pub locations: Option<Section<TrialLocationsPage>>,
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
    /// Scope of the registry age fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age_scope: Option<TrialEligibilityAgeScope>,
    /// Study-wide source minimum age.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_age: Option<TrialAgeBound>,
    /// Study-wide source maximum age.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_age: Option<TrialAgeBound>,
    /// Source sex field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<TrialSex>,
    /// Source healthy-volunteer field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_volunteers: Option<bool>,
    /// Split text only: this page's registry text before its first "Inclusion Criteria" or "Exclusion Criteria" heading.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preamble: Option<Box<str>>,
    /// Split text only: this page's registry lines below "Inclusion Criteria" headings, each with its
    /// nested lines, word for word in source order. Absent when this page holds no inclusion heading
    /// or item; other pages may hold them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inclusion: Option<Vec<Box<str>>>,
    /// Split text only: this page's registry lines below "Exclusion Criteria" headings, each with its
    /// nested lines, word for word in source order. Absent when this page holds no exclusion heading
    /// or item; other pages may hold them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusion: Option<Vec<Box<str>>>,
    /// Named eligibility arms on this page, in source order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arms: Vec<TrialEligibilityArm>,
    /// Why the text is not split: it lacks one of the two headings or has another line ending in "criteria:", so `criteria` holds it whole.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_split: Option<Box<str>>,
    /// Exact original text fragment, including whitespace; absent when structured criteria replace it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "Box<str>"))]
    pub criteria: Option<Box<str>>,
    /// The page ends within a criterion whose lines exceed the page budget.
    #[serde(default)]
    pub criterion_continues: bool,
    /// Original character offset.
    pub offset: u32,
    /// Complete original text length in Unicode scalar values.
    pub total_characters: u32,
}
/// The registry age bounds apply to the study as a whole.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrialEligibilityAgeScope {
    /// Study-level age bounds, separate from the criteria text.
    StudyWide,
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for TrialEligibilityAgeScope {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "TrialEligibilityAgeScope".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "enum": [Self::StudyWide],
            "description": "The registry age bounds apply to the study as a whole."
        })
    }
}

/// This page's criteria under one registry arm heading.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialEligibilityArm {
    /// Registry arm title, repeated on continuation pages for this arm.
    pub heading: Box<str>,
    /// This arm's text outside an identifiable criteria list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preamble: Option<Box<str>>,
    /// The arm's inclusion items on this page, in source order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inclusion: Option<Vec<Box<str>>>,
    /// The arm's exclusion items on this page, in source order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusion: Option<Vec<Box<str>>>,
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
    /// The page every list states: offset, rows returned and total (1061).
    pub page: crate::PageInfo,
    /// Complete records in source order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[T]>,
}
impl<T> TrialRecords<T> {
    /// One page of a collection, from its offset, its whole size and the records it holds.
    pub fn new(offset: u32, total: u32, items: Box<[T]>) -> Self {
        Self {
            continuation: None,
            offset,
            total,
            page: crate::PageInfo {
                offset,
                returned: items.len() as u32,
                total: total.into(),
                pool: None,
                truncated: None,
            },
            items,
        }
    }
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

/// A page of sites after location filters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "TrialRecords_TrialLocation"))]
pub struct TrialLocationsPage {
    /// Count of sites without coordinates after state and status filters, before radius and paging.
    #[serde(default)]
    pub missing_coordinates: u32,
    /// Matching sites in page order.
    #[serde(flatten)]
    pub records: TrialRecords<TrialSite>,
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
    pub distance_mi: Option<f64>,
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
