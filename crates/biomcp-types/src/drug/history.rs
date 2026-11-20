//! When text first appears in a named section of a drug's US label, read from the label PDFs
//! Drugs@FDA archives for each approved supplement (1066).
use super::{DrugGet, DrugHeader};
use crate::ResponseBody;
use biodata::{PartialDate, Text};
use serde::{Deserialize, Serialize};

biodata::canonical_enum! {
    /// A prescribing-information section, named as openFDA names its label fields.
    LabelHistorySection, SourceUnknown {
        /// The boxed warning.
        BoxedWarning => "boxed_warning",
        /// Section 1, Indications and Usage.
        IndicationsAndUsage => "indications_and_usage",
        /// Section 4, Contraindications.
        Contraindications => "contraindications",
        /// Section 5, Warnings and Precautions; WARNINGS and PRECAUTIONS in older labels.
        WarningsAndCautions => "warnings_and_cautions",
        /// Section 6, Adverse Reactions, including any postmarketing subsection.
        AdverseReactions => "adverse_reactions",
        /// Section 7, Drug Interactions.
        DrugInteractions => "drug_interactions",
        /// Section 8, Use in Specific Populations.
        UseInSpecificPopulations => "use_in_specific_populations",
    }
}

biodata::wire_enum! {
    /// What reading one label version found.
    LabelVersionState {
        /// The section holds a searched text.
        Match => "match",
        /// The section was read and holds none of the searched texts.
        NoMatch => "no-match",
        /// The label was read and has no such section.
        SectionAbsent => "section-absent",
        /// The document was read and is not prescribing information, such as Instructions for Use
        /// alone; it is not a label version and is passed over.
        NotPrescribingInformation => "not-prescribing-information",
        /// Unread: a heading names the section but its end was not found.
        SectionNotBounded => "section-not-bounded",
        /// Unread: a label in the format used before 2006, which has no such heading.
        OlderFormat => "older-format",
        /// Unread: the PDF has no embedded text, such as a scan.
        NoText => "no-text",
        /// Unread: over BioMCP's PDF bounds.
        TooLarge => "too-large",
        /// Unread: the PDF's text reads as gibberish.
        Garbled => "garbled",
        /// Unread: a part of a page could not be read and the rest holds no searched text.
        Partial => "partial",
        /// Unread: the PDF is damaged.
        Damaged => "damaged",
        /// Unread: reading went over BioMCP's memory or time limit.
        Refused => "refused",
        /// Unread: FDA refused the download or it failed.
        NotFetched => "not-fetched",
        /// Unread: the section holds private-use characters, which stand for symbols, digits or
        /// letters the PDF's fonts do not name, and holds no searched text, so a miss proves nothing.
        PrivateUse => "private-use"
    }
}

impl LabelVersionState {
    /// Whether the version was read well enough to say whether it holds the text.
    pub fn read(self) -> bool {
        matches!(
            self,
            Self::Match | Self::NoMatch | Self::SectionAbsent | Self::NotPrescribingInformation
        )
    }
}

/// Whether a get's label-history options fit together: the section alone, from Drugs@FDA and
/// unpaged, with at least one text to find and one of its section names; `--every-version` only
/// with it.
pub(super) fn valid(
    sections: &[super::DrugSection],
    drugsfda: bool,
    (find, every, section): (&[Text], bool, Option<&Text>),
) -> bool {
    if !sections.contains(&super::DrugSection::LabelHistory) {
        return find.is_empty() && !every;
    }
    sections.len() == 1 && drugsfda && !find.is_empty() && self::section(section).is_some()
}

/// The label-history section a `--label-section` value names.
pub(super) fn section(value: Option<&Text>) -> Option<LabelHistorySection> {
    value.and_then(|s| s.as_str().parse().ok())
}

/// The version of one section of an application's label in which searched text first appears.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugLabelHistory {
    /// The Drugs@FDA application.
    pub identity: DrugHeader,
    /// The section searched.
    pub section: LabelHistorySection,
    /// The answer in one sentence: where the text first appears, or that no version read has it.
    pub answer: Text,
    /// How versions were chosen, and the assumption the answer rests on when it rests on one.
    pub method: Text,
    /// Label versions Drugs@FDA lists for the application: one per distinct label PDF.
    pub versions: u32,
    /// The earliest version read whose section holds a searched text; absent when none does.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first: Option<LabelVersionMatch>,
    /// The latest version before `first` whose section was read and holds no searched text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<LabelVersion>,
    /// True when a version between `before` and `first` was not read, so the text appeared on
    /// or before `first`'s date rather than on it.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub on_or_before: bool,
    /// Versions between `before` and `first` that were never opened, because the call's read
    /// bound or time limit ran out.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub not_opened: u32,
    /// Every version opened, oldest first, with what reading it found.
    pub read: Box<[LabelVersion]>,
}

fn is_zero(value: &u32) -> bool {
    *value == 0
}

/// One label version: the supplement whose approval it came with.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct LabelVersion {
    /// The supplement's Drugs@FDA submission status date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved: Option<PartialDate>,
    /// Submission type and number, such as `SUPPL 4`.
    pub submission: Text,
    /// What reading this version found.
    pub state: LabelVersionState,
    /// The label PDF: given on `first`, on `before` and on unread versions in `read`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub label: Option<Text>,
}

/// The first matching version and the sentences that hold the searched text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct LabelVersionMatch {
    /// The version.
    #[serde(flatten)]
    pub version: LabelVersion,
    /// The supplement's FDA letter, when Drugs@FDA lists one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub letter: Option<Text>,
    /// The subsection heading above the first match, as the label's contents name it, such as
    /// `6.2 Postmarketing Experience`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading: Option<Text>,
    /// At most two distinct sentences holding a searched text, each at most 300 characters,
    /// as the PDF's text gives them.
    pub sentences: Box<[Text]>,
    /// True when the section's text holds private-use characters, which stand for symbols,
    /// digits or letters the PDF's fonts do not name; a searched text with them may be missed.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub private_use: bool,
}

impl ResponseBody for DrugLabelHistory {
    const SCHEMA: &'static str = "biomcp.drug-label-history/1";
    type Query = DrugGet;
}
