//! When text first appears in a named section of a drug's US label, read from the label PDFs
//! Drugs@FDA archives for each approved supplement.
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

biodata::canonical_enum! {
    /// What reading one label version found.
    LabelVersionState, SourceUnknown {
        /// The section holds a searched text.
        Match => "match",
        /// The section was read and holds none of the searched texts.
        NoMatch => "no_match",
        /// The label has no such section.
        SectionAbsent => "section_absent",
        /// Unread: the PDF has no embedded text, such as a scan.
        NoText => "no_text",
        /// Unread: the document is not prescribing information, such as Instructions for Use alone.
        NotPrescribingInformation => "not_prescribing_information",
        /// Unread: a heading names the section but its end was not found.
        SectionNotBounded => "section_not_bounded",
        /// Unread: a label in the format used before 2006, which has no such heading.
        OlderFormat => "older_format",
        /// Unread: the document is not a PDF, such as an HTML page.
        NotPdf => "not_pdf",
        /// Unread: over BioMCP's size bounds.
        TooLarge => "too_large",
        /// Unread: FDA refused the download or it failed.
        NotFetched => "not_fetched",
        /// Unread: the PDF could not be parsed.
        Unreadable => "unreadable",
    }
}

impl LabelVersionState {
    /// Whether the section's text was read, with or without a match.
    pub fn read(self) -> bool {
        matches!(self, Self::Match | Self::NoMatch | Self::SectionAbsent)
    }
}

/// The versions of one section of an application's label in which searched text first appears.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugLabelHistory {
    /// The Drugs@FDA application.
    pub identity: DrugHeader,
    /// Label versions Drugs@FDA lists for the application: one per distinct label PDF.
    pub versions: u32,
    /// How versions were chosen and what the answer assumes.
    pub method: Text,
    /// The earliest version read whose section holds a searched text; absent when none does.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first: Option<LabelVersionMatch>,
    /// The latest version before `first` whose section was read and holds no searched text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<LabelVersion>,
    /// True when an unread version lies between `before` and `first`, so the text appeared
    /// on or before `first`'s date rather than on it.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub on_or_before: bool,
    /// Every version opened, oldest first, with what reading it found.
    pub read: Box<[LabelVersion]>,
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
    /// The label PDF, given for the matching version, the version before it and unread versions.
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
    /// The subsection heading above the first match, such as `6.2 Postmarketing Experience`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading: Option<Text>,
    /// At most two distinct sentences holding a searched text, each at most 400 characters.
    pub sentences: Box<[Text]>,
}

impl ResponseBody for DrugLabelHistory {
    const SCHEMA: &'static str = "biomcp.drug-label-history/1";
    type Query = DrugGet;
}
