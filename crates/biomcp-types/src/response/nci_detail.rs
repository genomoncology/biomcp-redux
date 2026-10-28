//! Native NCI protocol fields; no registry vocabulary mapping or text classification.
use crate::{FiniteNumber, Section, Text, TrialRecords};
use serde::{Deserialize, Serialize};
/// Independently paged native collections from one immutable source record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciTrialSections {
    /// Complete native summary with character continuations.
    pub summary: Option<Section<NciTextPage>>,
    /// Source criteria and structured eligibility.
    pub eligibility: Option<Section<NciEligibilityPage>>,
    /// Source outcomes without registry type coercion.
    pub outcomes: Option<Section<TrialRecords<NciOutcome>>>,
    /// Arm headers and their attached intervention records.
    pub arms: Option<Section<TrialRecords<NciArmRecord>>>,
    /// Every source disease concept in unchanged order.
    pub diseases: Option<Section<TrialRecords<crate::NciDisease>>>,
}
/// Native eligibility fields and whole unclassified criteria.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciEligibilityPage {
    /// Source structured bounds and sex, without interpretation.
    pub structured: Option<NciEligibilityStructured>,
    /// Whole source criteria in original order.
    pub unstructured: TrialRecords<NciCriterion>,
}
/// Complete native eligibility stored for immutable paging.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciEligibility {
    /// Source structured fields.
    pub structured: Option<NciEligibilityStructured>,
    /// Whole source criteria, with original metadata.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub unstructured: Box<[NciCriterion]>,
}
/// One intact source criterion; the publisher supplies its classification.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciCriterion {
    /// Original source text, preserving every character.
    pub description: Box<str>,
    /// Native source inclusion marker.
    pub inclusion_indicator: Option<bool>,
    /// Original publisher display order.
    pub display_order: Option<u32>,
}
/// Native NCI structured fields; sentinel ages remain native values.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciEligibilityStructured {
    /// Native min age field.
    pub min_age: Option<Text>,
    /// Native max age field.
    pub max_age: Option<Text>,
    /// Native min age unit field.
    pub min_age_unit: Option<Text>,
    /// Native max age unit field.
    pub max_age_unit: Option<Text>,
    /// Native min age number field.
    pub min_age_number: Option<FiniteNumber>,
    /// Native max age number field.
    pub max_age_number: Option<FiniteNumber>,
    /// Native min age in years field.
    pub min_age_in_years: Option<FiniteNumber>,
    /// Native max age in years field.
    pub max_age_in_years: Option<FiniteNumber>,
    /// Native sex field.
    pub sex: Option<Text>,
    /// Native accepts healthy volunteers field.
    pub accepts_healthy_volunteers: Option<bool>,
}
/// One complete NCI outcome with its original vocabulary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciOutcome {
    /// Original publisher name field.
    pub name: Option<Box<str>>,
    /// Original publisher description field.
    pub description: Option<Box<str>>,
    /// Original publisher timeframe field.
    pub timeframe: Option<Box<str>>,
    /// Original publisher type_code field.
    pub type_code: Option<Box<str>>,
}
/// Source arm header; interventions follow as indexed records.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciArm {
    /// Original arm name.
    pub name: Option<Box<str>>,
    /// Whole original description.
    pub description: Option<Box<str>>,
    /// Native arm type.
    pub r#type: Option<Text>,
}
/// One original NCI intervention attached to its source arm.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciIntervention {
    /// Native name field.
    pub name: Text,
    /// Native description field.
    pub description: Option<Box<str>>,
    /// Native nci thesaurus concept id field.
    pub nci_thesaurus_concept_id: biodata::NcitId,
    /// Native r#type field.
    pub r#type: Option<Text>,
    /// Native category field.
    pub category: Option<Text>,
    /// Native inclusion indicator field.
    pub inclusion_indicator: Option<Text>,
    /// Native parents field.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub parents: Box<[Text]>,
    /// Native synonyms field.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub synonyms: Box<[Text]>,
}
/// A record position preserves every arm/intervention relationship without invented identity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "record", rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum NciArmRecord {
    /// Original arm header, followed by its intervention rows.
    Arm {
        /// Zero-based original arm position.
        arm_index: u32,
        /// Source arm header.
        data: NciArm,
    },
    /// Original intervention and its unchanged parent position.
    Intervention {
        /// Zero-based parent arm position.
        arm_index: u32,
        /// Zero-based position inside that arm.
        index: u32,
        /// Source intervention fields.
        data: NciIntervention,
    },
}

/// Exact source text fragment with an honest original character count.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NciTextPage {
    /// Checked request for the next original characters, absent when complete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuation: Option<crate::TrialGet>,
    /// Original source fragment, including whitespace.
    pub text: Box<str>,
    /// Original Unicode scalar position.
    pub offset: u32,
    /// Complete source length in Unicode scalar values.
    pub total_characters: u32,
}
