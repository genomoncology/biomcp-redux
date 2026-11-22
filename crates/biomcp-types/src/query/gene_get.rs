//! Checked gene batch reads and their gnomAD release selection.
use super::gene::{GeneSectionWord, present};
use super::validation::QueryInvalid;
use crate::GeneInput;
use serde::{Deserialize, Serialize};

const MIN_INPUTS: usize = 1;
const MAX_INPUTS: usize = 20;

/// Get one to twenty human gene inputs in their original order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "GetFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "GetFields"))]
pub struct GeneGet(GetFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct GetFields {
    /// Gene symbols or checked identifiers, in input order.
    #[cfg_attr(feature = "schema", schemars(length(min = MIN_INPUTS, max = MAX_INPUTS), extend("x-positional" = 0)))]
    input: Box<[GeneInput]>,
    /// Requested section words, preserving caller order and repetitions.
    /// Trailing section words select sections. Use --section WORD to keep inputs that equal a section word as inputs.
    /// A later word near a section name, or naming another record's section, is refused; give such an ID first or use --section.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    #[cfg_attr(feature = "schema", schemars(extend("x-trailing-section" = true)))]
    section: Box<[GeneSectionWord]>,
    /// Choose a gnomAD release for MANE Select.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "ConstraintRelease"))]
    constraint_release: Option<ConstraintRelease>,
    /// Keep resolved identity and constraint rows. Use with the constraint section.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    section_only: bool,
}
crate::query::validation::checked_query!(GeneGet, GetFields);
impl TryFrom<GetFields> for GeneGet {
    type Error = QueryInvalid;
    fn try_from(fields: GetFields) -> Result<Self, Self::Error> {
        Self::checked(fields)
    }
}
impl GeneGet {
    /// Refuse an empty or oversized input batch.
    pub fn new(input: Box<[GeneInput]>) -> Result<Self, QueryInvalid> {
        Self::with_sections(input, Box::default())
    }
    /// Check inputs and retain the requested section words.
    pub fn with_sections(
        input: Box<[GeneInput]>,
        section: Box<[GeneSectionWord]>,
    ) -> Result<Self, QueryInvalid> {
        Self::checked(GetFields {
            input,
            section,
            constraint_release: None,
            section_only: false,
        })
    }
    fn checked(fields: GetFields) -> Result<Self, QueryInvalid> {
        if !(MIN_INPUTS..=MAX_INPUTS).contains(&fields.input.len()) {
            return Err(QueryInvalid::new(&["input"]));
        }
        if fields.constraint_release.is_some()
            && !fields
                .section
                .iter()
                .any(|word| matches!(word, GeneSectionWord::Constraint | GeneSectionWord::All))
        {
            return Err(QueryInvalid::new(&["constraint_release", "section"]));
        }
        if fields.section_only && fields.section.as_ref() != [GeneSectionWord::Constraint] {
            return Err(QueryInvalid::new(&["section_only", "section"]));
        }
        Ok(Self(fields))
    }
    /// Explicit publisher release for the MANE Select constraint read.
    pub fn constraint_release(&self) -> Option<ConstraintRelease> {
        self.0.constraint_release
    }
    /// Whether to return only identity and the constraint section.
    pub fn section_only(&self) -> bool {
        self.0.section_only
    }
    /// Section words in caller order.
    pub fn sections(&self) -> &[GeneSectionWord] {
        &self.0.section
    }
    /// Inputs in caller order.
    pub fn input(&self) -> &[GeneInput] {
        &self.0.input
    }
}
biodata::wire_enum! {
    described
    /// gnomAD releases with supported constraint tables.
    ConstraintRelease {
        #[doc = " Version 4.0 table."] V4_0 => "v4.0",
        #[doc = " Version 4.1 table."] V4_1 => "v4.1"
    }
}
