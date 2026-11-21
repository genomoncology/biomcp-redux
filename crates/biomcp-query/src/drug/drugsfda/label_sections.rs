//! Find one section of a label in the text BioMCP's PDF reader gives (1066), anchored on the
//! label's own contents list, never on the shape of a line.
//!
//! Labels in the 2006 format print `FULL PRESCRIBING INFORMATION: CONTENTS`, which lists every
//! section by number and title in order, then the prescribing information itself. A heading in
//! the body counts only when it names a listed section: its number and title (`6 ADVERSE
//! REACTIONS` or `6. ADVERSE REACTIONS`), or its title alone where the PDF draws the number apart.
//! A listed subsection can imply a missing contents entry. Headings are
//! found in the list's order, each after the one before, so a table row such as `1.1 Rash 26 2`
//! is never a heading. A section runs from its heading to the heading of the next listed
//! section, with no numbered heading of another section between; otherwise it is unread.
//! The boxed warning is the text between the body's start and section 1 when
//! that text holds a warning; it is absent only when neither it, the highlights nor the list
//! holds one. When the box itself is unread, the list's title for it can still hold the text.
//!
//! Labels without a readable contents list, in the format used before 2006 or with the list's
//! page lost, are read from their headings alone in [`super::label_plain`] (1082). A document is
//! not a label only on positive evidence: no prescribing-information heading at all, and a
//! title such as Instructions for Use or Medication Guide.
use super::label_contents::listed_section;
use super::label_plain;
use biomcp_types::LabelHistorySection;

/// Top-level prescribing-information sections by number, their titles in capitals.
pub(super) const TITLES: [(u8, &[&str]); 17] = [
    (1, &["INDICATIONS AND USAGE", "INDICATIONS & USAGE"]),
    (2, &["DOSAGE AND ADMINISTRATION"]),
    (3, &["DOSAGE FORMS AND STRENGTHS"]),
    (4, &["CONTRAINDICATIONS"]),
    (5, &["WARNINGS AND PRECAUTIONS"]),
    (6, &["ADVERSE REACTIONS"]),
    (7, &["DRUG INTERACTIONS"]),
    (8, &["USE IN SPECIFIC POPULATIONS"]),
    (9, &["DRUG ABUSE AND DEPENDENCE"]),
    (10, &["OVERDOSAGE"]),
    (11, &["DESCRIPTION"]),
    (12, &["CLINICAL PHARMACOLOGY"]),
    (13, &["NONCLINICAL TOXICOLOGY"]),
    (14, &["CLINICAL STUDIES"]),
    (15, &["REFERENCES"]),
    (16, &["HOW SUPPLIED"]),
    (17, &["PATIENT COUNSELING"]),
];
/// Titles that show a document is not prescribing information, in capitals.
const NOT_LABEL: [&str; 6] = [
    "INSTRUCTIONS FOR USE",
    "MEDICATION GUIDE",
    "PATIENT INFORMATION",
    "PATIENT PACKAGE INSERT",
    "CARTON",
    "CONTAINER LABEL",
];
pub(super) const FULL: &str = "FULL PRESCRIBING INFORMATION";

/// What looking for a section found.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Found {
    /// The section's text, whitespace collapsed.
    Section(String),
    /// The bounded portion of the label shows the section is absent.
    Absent,
    /// The section's bounds could not be found, so it is unread.
    Unbounded,
    /// Only the boxed warning's title, as the contents list gives it, was read: a match in it
    /// stands, a miss proves nothing.
    Title(String),
    /// The document is not prescribing information.
    NotPrescribing,
    /// A label in the older format whose headings could not be found, so it is unread.
    Older,
}

pub(super) fn word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Whether `needle` at `at` in `text` stands alone as a heading: nothing word-like touches it,
/// it is not a number's tail, and it is not a cross-reference such as `ADVERSE REACTIONS (6.1)`.
pub(super) fn heading_at(text: &str, at: usize, needle: &str) -> bool {
    let before = text.get(..at).and_then(|b| b.chars().last());
    let rest = text.get(at + needle.len()..).unwrap_or_default();
    let reference = rest
        .strip_prefix(" (")
        .is_some_and(|r| r.starts_with(|c: char| c.is_ascii_digit()));
    let see = text
        .get(..at)
        .is_some_and(|b| b.ends_with("see ") || b.ends_with("See "));
    before.is_none_or(|b| !word(b) && !matches!(b, '.' | '(' | '['))
        && !rest.starts_with(word)
        && !reference
        && !see
}

/// Whether a document without any prescribing-information heading names itself as another kind.
fn not_label(flat: &str) -> bool {
    let head: String = flat.chars().take(400).collect::<String>().to_uppercase();
    NOT_LABEL.iter().any(|title| head.contains(title))
}

/// Find `section` in a label's cleaned text on one line.
pub(super) fn find(flat: &str, section: LabelHistorySection) -> Found {
    use LabelHistorySection as S;
    let number = match section {
        S::BoxedWarning => None,
        S::IndicationsAndUsage => Some(1),
        S::Contraindications => Some(4),
        S::WarningsAndCautions => Some(5),
        S::AdverseReactions => Some(6),
        S::DrugInteractions => Some(7),
        S::UseInSpecificPopulations => Some(8),
    };
    if let Some(found) = listed_section(flat, number) {
        return found;
    }
    let structured = flat.contains(FULL)
        || flat.contains("HIGHLIGHTS OF PRESCRIBING INFORMATION")
        || [
            "1 INDICATIONS AND USAGE",
            "1. INDICATIONS AND USAGE",
            "5 WARNINGS AND PRECAUTIONS",
            "5. WARNINGS AND PRECAUTIONS",
        ]
        .iter()
        .any(|heading| flat.contains(heading));
    if structured {
        return number.map_or(Found::Unbounded, |n| label_plain::numbered(flat, n));
    }
    if not_label(flat) {
        return Found::NotPrescribing;
    }
    label_plain::older(flat, section)
}
