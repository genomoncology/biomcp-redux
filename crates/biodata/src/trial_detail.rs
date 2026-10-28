//! Source-preserving registry protocol records; no clinical interpretation.
use crate::{Markdown, NctId, PartialDate, StudyType, TrialPhase, TrialSex, TrialStatus};
use serde::{Deserialize, Serialize};
/// Complete default registry card.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialCard {
    /// Exact registry identity.
    pub nct_id: NctId,
    /// Source brief title, absent when missing or rejected.
    pub title: Option<Box<str>>,
    /// Source overall status, absent when missing or rejected.
    pub status: Option<TrialStatus>,
    /// Source phases without inferred defaults.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub phases: Box<[TrialPhase]>,
    /// Native study type, absent when missing or rejected.
    pub study_type: Option<StudyType>,
    /// All registered condition names.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub conditions: Box<[Box<str>]>,
    /// All registered intervention names.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub interventions: Box<[Box<str>]>,
    /// Lead sponsor as registered.
    pub sponsor: Option<Box<str>>,
    /// Source start date, when present.
    pub start: Option<TrialDate>,
    /// Source primary completion date, when present.
    pub primary_completion: Option<TrialDate>,
    /// Source completion date, when present.
    pub completion: Option<TrialDate>,
    /// Last posted registry update, absent when missing or rejected.
    pub updated: Option<TrialDate>,
    /// Whole brief summary, preserving whitespace.
    pub brief_summary: Option<Markdown>,
    /// Canonical publisher record URL.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Box<str>,
}
/// Source age limit; absence is explicit and never a fabricated number.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum TrialAgeBound {
    /// Registry supplied no limit.
    NoLimit,
    /// Unchanged source value; no local age conversion.
    Source {
        /// Registry age expression.
        raw: Box<str>,
    },
}
/// Structured eligibility fields and complete original criteria.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialEligibility {
    /// Registry minimum age.
    pub minimum_age: Option<TrialAgeBound>,
    /// Registry maximum age.
    pub maximum_age: Option<TrialAgeBound>,
    /// Registry sex restriction.
    pub sex: Option<TrialSex>,
    /// Registry healthy volunteer flag.
    pub healthy_volunteers: Option<bool>,
    /// Original criteria text without heading interpretation.
    pub criteria: Option<Markdown>,
}
crate::wire_enum! {
    /// Registry outcome collection.
    TrialOutcomeKind {
        /// Primary registered outcome.
        Primary => "primary",
        /// Secondary registered outcome.
        Secondary => "secondary",
        /// Other registered outcome.
        Other => "other",
    }
}
/// One whole registered outcome with its attached time frame.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialOutcome {
    /// Source collection.
    pub kind: TrialOutcomeKind,
    /// Original measure text.
    pub measure: Option<Box<str>>,
    /// Original description.
    pub description: Option<Markdown>,
    /// Original time frame.
    pub time_frame: Option<Box<str>>,
}
/// Registry arm with source intervention relationships.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialArm {
    /// Exact arm label.
    pub label: Option<Box<str>>,
    /// Native arm type.
    pub kind: Option<TrialArmKind>,
    /// Original arm description.
    pub description: Option<Markdown>,
    /// Source intervention names, including their native prefixes.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub intervention_names: Box<[Box<str>]>,
}
/// Registry intervention, retaining aliases and arm labels.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialIntervention {
    /// Exact source name.
    pub name: Option<Box<str>>,
    /// Native intervention type.
    pub kind: Option<TrialInterventionKind>,
    /// Original description.
    pub description: Option<Markdown>,
    /// Source arm relationships.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub arm_group_labels: Box<[Box<str>]>,
    /// Source aliases, without invented expansion.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub other_names: Box<[Box<str>]>,
}

crate::wire_enum! {
    /// Registry date qualifier, without local interpretation.
    TrialDateKind {
        /// Source marks this date actual.
        Actual => "ACTUAL",
        /// Source marks this date estimated.
        Estimated => "ESTIMATED",
    }
}
/// A registry date retains its actual or estimated qualifier.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialDate {
    /// Checked source date at its original precision.
    pub date: PartialDate,
    /// Source qualifier, absent only if the registry omitted it.
    pub kind: Option<TrialDateKind>,
}
crate::wire_enum! {
    /// Native registry arm group type from the publisher enum metadata.
    #[non_exhaustive]
    #[cfg_attr(feature = "schema", schemars(extend("x-enum-open" = true)))]
    TrialArmKind {
        /// EXPERIMENTAL source value.
        Experimental => "EXPERIMENTAL",
        /// ACTIVE_COMPARATOR source value.
        ActiveComparator => "ACTIVE_COMPARATOR",
        /// PLACEBO_COMPARATOR source value.
        PlaceboComparator => "PLACEBO_COMPARATOR",
        /// SHAM_COMPARATOR source value.
        ShamComparator => "SHAM_COMPARATOR",
        /// NO_INTERVENTION source value.
        NoIntervention => "NO_INTERVENTION",
        /// OTHER source value.
        Other => "OTHER",
    }
}
crate::wire_enum! {
    /// Native registry intervention type from the publisher enum metadata.
    #[non_exhaustive]
    #[cfg_attr(feature = "schema", schemars(extend("x-enum-open" = true)))]
    TrialInterventionKind {
        /// BEHAVIORAL source value.
        Behavioral => "BEHAVIORAL",
        /// BIOLOGICAL source value.
        Biological => "BIOLOGICAL",
        /// COMBINATION_PRODUCT source value.
        CombinationProduct => "COMBINATION_PRODUCT",
        /// DEVICE source value.
        Device => "DEVICE",
        /// DIAGNOSTIC_TEST source value.
        DiagnosticTest => "DIAGNOSTIC_TEST",
        /// DIETARY_SUPPLEMENT source value.
        DietarySupplement => "DIETARY_SUPPLEMENT",
        /// DRUG source value.
        Drug => "DRUG",
        /// GENETIC source value.
        Genetic => "GENETIC",
        /// PROCEDURE source value.
        Procedure => "PROCEDURE",
        /// RADIATION source value.
        Radiation => "RADIATION",
        /// OTHER source value.
        Other => "OTHER",
    }
}
