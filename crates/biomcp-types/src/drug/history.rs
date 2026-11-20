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
        /// The label's contents list was read and has no such section.
        SectionAbsent => "section-absent",
        /// The document names itself as another kind, such as Instructions for Use, and has no
        /// prescribing information; it is not a label version and is passed over.
        NotPrescribingInformation => "not-prescribing-information",
        /// Unread: the section's bounds could not be found from the label's contents list or its
        /// headings.
        SectionNotBounded => "section-not-bounded",
        /// Unread: a label in the format used before 2006 with no heading for the section.
        OlderFormat => "older-format",
        /// Unread: the PDF has no embedded text besides FDA's stamp, such as a scan.
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
fn valid(
    sections: &[super::DrugSection],
    drugsfda: bool,
    (find, every, section): (&[Text], bool, Option<&Text>),
) -> bool {
    if !sections.contains(&super::DrugSection::LabelHistory) {
        return find.is_empty() && !every;
    }
    sections.len() == 1 && drugsfda && !find.is_empty() && self::section(section).is_some()
}

/// Refuse label-history options that do not fit together. `--every-version` continues from
/// `--section-offset`; label history takes no other paging.
pub(super) fn checked(
    sections: &[super::DrugSection],
    drugsfda: bool,
    (limit, offset): (Option<u32>, Option<u32>),
    options: (&[Text], bool, Option<&Text>),
) -> Result<(), crate::QueryInvalid> {
    // A label history alone names every cause in one refusal: an offset without every
    // version, which also needs --find and a section, or a section it does not read (1081).
    if sections == [super::DrugSection::LabelHistory] {
        use crate::ErrorMessage::{
            LabelHistoryLimit, LabelHistoryOffset, LabelHistoryOffsetIncomplete,
            LabelHistoryOffsetSection, LabelHistorySectionUnknown,
        };
        if limit.is_some() {
            return Err(
                crate::QueryInvalid::new(&["section_limit"]).with_message(LabelHistoryLimit)
            );
        }
        let offset = offset.is_some() && !options.1;
        let unknown = options.2.is_some() && section(options.2).is_none();
        // Without --find, the refusal that names it also names the sections to choose from.
        let incomplete = options.0.is_empty() || options.2.is_none();
        let refused: Option<(&[&str], _)> = match (offset, unknown, incomplete) {
            (true, true, false) => Some((
                &["section_offset", "every_version", "label_section"],
                LabelHistoryOffsetSection,
            )),
            (true, _, true) => Some((
                &["section_offset", "every_version", "find", "label_section"],
                LabelHistoryOffsetIncomplete,
            )),
            (true, false, false) => {
                Some((&["section_offset", "every_version"], LabelHistoryOffset))
            }
            (false, true, _) => Some((&["label_section"], LabelHistorySectionUnknown)),
            (false, false, _) => None,
        };
        if let Some((names, message)) = refused {
            return Err(crate::QueryInvalid::new(names).with_message(message));
        }
    }
    let paged = limit.is_some() || offset.is_some() && !options.1;
    if !valid(sections, drugsfda && !paged, options) {
        return Err(crate::QueryInvalid::new(&[
            "section",
            "find",
            "label_section",
        ]));
    }
    // `--label-section` names an openFDA label field, or the section a label history reads.
    let named = sections.contains(&super::DrugSection::Label)
        || sections.contains(&super::DrugSection::LabelHistory);
    if options.2.is_some() && !named {
        return Err(crate::QueryInvalid::new(&["label_section", "section"]));
    }
    Ok(())
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
    /// What the versions read establish about when the text appeared.
    pub answer: Text,
    /// How versions were chosen, and the assumption the answer rests on when it rests on one.
    pub method: Text,
    /// Label versions Drugs@FDA lists for the application: one per distinct label PDF.
    pub versions: u32,
    /// The earliest confirmed match read, if the reply can establish one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first: Option<LabelVersionMatch>,
    /// The latest version before `first` whose section was read and holds no searched text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<LabelVersion>,
    /// True when any version older than `first` was unreadable or unopened.
    /// The first appearance could be earlier than `first`.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub on_or_before: bool,
    /// Versions this call never opened. An unopened version leaves the answer open if it
    /// precedes the earliest match or if no match was found.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub not_opened: u32,
    /// True when the call stopped at its read bound or time limit before the search finished;
    /// the reply's next step continues with `--every-version` and `--section-offset`.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub stopped: bool,
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
