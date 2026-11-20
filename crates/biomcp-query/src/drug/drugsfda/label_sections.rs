//! Find one section of a label in the text BioMCP's PDF reader gives (1066), anchored on the
//! label's own contents list, never on the shape of a line.
//!
//! Labels in the 2006 format print `FULL PRESCRIBING INFORMATION: CONTENTS`, which lists every
//! section by number and title in order, then the prescribing information itself. A heading in
//! the body counts only when it names a listed section: its number and title (`6 ADVERSE
//! REACTIONS` or `6. ADVERSE REACTIONS`), its title alone where the PDF draws the number apart,
//! or the listed title of its first subsection (`6.1 Clinical Studies Experience`); a listed
//! subsection implies its section when the reader lost the section's own line. Headings are
//! found in the list's order, each after the one before, so a table row such as `1.1 Rash 26 2`
//! is never a heading. A section runs from its heading to the heading of the next listed
//! section, with no numbered heading of another section between; otherwise it is unread. A
//! section is absent only when the list was read and lacks it and the body has no numbered
//! heading for it. The boxed warning is the text between the body's start and section 1 when
//! that text holds a warning; it is absent only when neither it, the highlights nor the list
//! holds one. When the box itself is unread, the list's title for it can still hold the text.
//!
//! Labels without a readable contents list, in the format used before 2006 or with the list's
//! page lost, are read from their headings alone in [`super::label_plain`] (1082). A document is
//! not a label only on positive evidence: no prescribing-information heading at all, and a
//! title such as Instructions for Use or Medication Guide.
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
const CONTENTS: [&str; 2] = [
    "FULL PRESCRIBING INFORMATION: CONTENTS",
    "FULL PRESCRIBING INFORMATION:CONTENTS",
];
pub(super) const FULL: &str = "FULL PRESCRIBING INFORMATION";

/// What looking for a section found.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Found {
    /// The section's text, whitespace collapsed.
    Section(String),
    /// The label's contents list was read and has no such section.
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

fn word(c: char) -> bool {
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

/// The first place at or after `from` where `needle` stands alone as a heading.
fn heading_from(text: &str, needle: &str, from: usize) -> Option<usize> {
    let rest = text.get(from..)?;
    rest.match_indices(needle)
        .map(|(at, _)| from + at)
        .find(|at| heading_at(text, *at, needle))
}

/// The value of one or two ASCII digits.
fn number(digits: &str) -> u8 {
    digits
        .chars()
        .filter_map(|c| c.to_digit(10))
        .fold(0, |value, digit| value * 10 + digit as u8)
}

/// One numbered entry in the contents list: its number, minor number and the text after it.
fn entry(block: &str, at: usize) -> Option<(u8, Option<u8>, &str)> {
    let before = block.get(..at).and_then(|b| b.chars().last());
    if before.is_some_and(|b| word(b) || matches!(b, '.' | '(' | '[')) {
        return None;
    }
    let rest = block.get(at..)?;
    let digits = rest.len() - rest.trim_start_matches(|c: char| c.is_ascii_digit()).len();
    if !(1..=2).contains(&digits) {
        return None;
    }
    let major = number(rest.get(..digits)?);
    let after = rest.get(digits..)?;
    // Some labels print `5. WARNINGS AND PRECAUTIONS` and `5.1. Serious Infections`.
    if let Some(tail) = after.strip_prefix(' ').or_else(|| after.strip_prefix(". ")) {
        return Some((major, None, tail));
    }
    let minor_text = after.strip_prefix('.')?;
    let n = minor_text.len()
        - minor_text
            .trim_start_matches(|c: char| c.is_ascii_digit())
            .len();
    if !(1..=2).contains(&n) {
        return None;
    }
    let minor = number(minor_text.get(..n)?);
    let tail = minor_text.get(n..)?;
    let tail = tail.strip_prefix(' ').or_else(|| tail.strip_prefix(". "))?;
    Some((major, Some(minor), tail))
}

/// The first one to three words of a subsection title, as the contents list gives them.
fn title_words(tail: &str) -> Option<String> {
    let allowed = |c: char| c.is_alphanumeric() || ",'’-/&()".contains(c);
    let words: Vec<&str> = tail
        .split(' ')
        .take(3)
        .take_while(|w| !w.is_empty() && w.chars().all(allowed))
        .collect();
    let first = words.first()?;
    first
        .starts_with(|c: char| c.is_uppercase())
        .then(|| words.join(" "))
}

/// A listed top-level section: its number, title and first subsection's title.
struct Listed {
    number: u8,
    title: &'static str,
    first: Option<String>,
}

/// The top-level sections the contents list names, in order.
fn listed(block: &str) -> Vec<Listed> {
    let mut found: Vec<Listed> = Vec::new();
    for (at, c) in block.char_indices() {
        if !c.is_ascii_digit() {
            continue;
        }
        let Some((major, minor, tail)) = entry(block, at) else {
            continue;
        };
        let last = found.last().map_or(0, |l| l.number);
        // A subsection listed under a section whose own line the reader lost implies it.
        if minor.is_some() && major > last && major <= last + 3 {
            if let Some(title) = TITLES
                .iter()
                .find(|(n, _)| *n == major)
                .and_then(|(_, titles)| titles.first())
            {
                found.push(Listed {
                    number: major,
                    title,
                    first: (minor == Some(1)).then(|| title_words(tail)).flatten(),
                });
            }
            continue;
        }
        match minor {
            None if major > last => {
                let title = TITLES
                    .iter()
                    .find(|(n, _)| *n == major)
                    .and_then(|(_, titles)| {
                        titles.iter().find(|t| {
                            tail.strip_prefix(**t)
                                .is_some_and(|r| !r.starts_with(char::is_alphabetic))
                        })
                    });
                if let Some(title) = title {
                    found.push(Listed {
                        number: major,
                        title,
                        first: None,
                    });
                }
            }
            Some(1) if major == last => {
                if let Some(listed) = found.last_mut().filter(|l| l.first.is_none()) {
                    listed.first = title_words(tail);
                }
            }
            _ => {}
        }
    }
    found
}

/// Where each listed section's heading starts in the body, each after the one before.
fn placed(flat: &str, listed: &[Listed], body: usize) -> Vec<Option<usize>> {
    let mut cursor = body;
    listed
        .iter()
        .map(|l| {
            let mut forms = vec![
                format!("{} {}", l.number, l.title),
                format!("{}. {}", l.number, l.title),
                l.title.to_owned(),
            ];
            if let Some(first) = &l.first {
                forms.push(format!("{}.1 {first}", l.number));
                forms.push(format!("{}.1. {first}", l.number));
            }
            let at = forms
                .iter()
                .filter_map(|form| heading_from(flat, form, cursor))
                .min();
            if let Some(at) = at {
                cursor = at + 1;
            }
            at
        })
        .collect()
}

/// A section of a 2006-format label, or none when the label has no contents list.
fn listed_section(flat: &str, number: Option<u8>) -> Option<Found> {
    let (start, marker) = CONTENTS
        .iter()
        .filter_map(|c| flat.find(c).map(|at| (at, c.len())))
        .min()?;
    let Some(body) = flat
        .get(start + marker..)?
        .find(FULL)
        .map(|at| start + marker + at)
    else {
        // The body's own heading is missing, as when its first page is drawn as an image.
        let block = flat.get(start + marker..).unwrap_or_default();
        return Some(match number {
            None => titled(block),
            Some(number) => label_plain::numbered(flat, number),
        });
    };
    let block = flat.get(start + marker..body).unwrap_or_default();
    let listed = listed(block);
    let body = body + FULL.len();
    let places = placed(flat, &listed, body);
    let Some(number) = number else {
        return Some(boxed(flat, (start, block, body), &listed, &places));
    };
    let Some(i) = listed.iter().position(|l| l.number == number) else {
        // Absent only when the body has no heading for it either: the reader can lose a line.
        let titles = TITLES
            .iter()
            .find(|(n, _)| *n == number)
            .map_or(&[][..], |t| t.1);
        let headed = titles
            .iter()
            .any(|t| heading_from(flat, &format!("{number} {t}"), body).is_some());
        return Some(if listed.is_empty() || headed {
            Found::Unbounded
        } else {
            Found::Absent
        });
    };
    let begin = places.get(i).copied().flatten();
    let end = match listed.get(i + 1) {
        Some(_) => places.get(i + 1).copied().flatten(),
        None => Some(flat.len()),
    };
    Some(match (begin, end) {
        // A numbered heading of another section between them means the list lost that section.
        (Some(begin), Some(end))
            if end > begin && !label_plain::crossed(flat, number, (begin, end)) =>
        {
            Found::Section(flat.get(begin..end).unwrap_or_default().trim().to_owned())
        }
        _ => Found::Unbounded,
    })
}

/// The boxed warning's title as the contents list gives it, when the box itself is unread: the
/// list's text from `WARNING` to section 1, holding no list entry.
fn titled(block: &str) -> Found {
    let head = block.trim_start_matches(['*', ' ']);
    let end = ["1 INDICATIONS", "1. INDICATIONS"]
        .iter()
        .filter_map(|one| head.find(one))
        .min();
    let title = end
        .and_then(|end| head.get(..end))
        .unwrap_or_default()
        .trim();
    let clean = title.starts_with("WARNING") && title.len() <= 400;
    if clean && !title.contains(|c: char| c.is_ascii_digit()) {
        Found::Title(title.to_owned())
    } else {
        Found::Unbounded
    }
}

/// The boxed warning: the warning between the body's start and section 1.
fn boxed(
    flat: &str,
    (contents, block, body): (usize, &str, usize),
    listed: &[Listed],
    places: &[Option<usize>],
) -> Found {
    let Some(Some(one)) = (!listed.is_empty()).then(|| places.first().copied().flatten()) else {
        return titled(block);
    };
    let region = flat.get(body..one).unwrap_or_default();
    if let Some(at) = region.find("WARNING") {
        return Found::Section(region.get(at..).unwrap_or_default().trim().to_owned());
    }
    let head = block
        .find("1 INDICATIONS")
        .and_then(|at| block.get(..at))
        .unwrap_or(block);
    if flat.get(..contents).is_some_and(|h| h.contains("WARNING:")) || head.contains("WARNING") {
        return titled(block);
    }
    Found::Absent
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
        || flat.contains("1 INDICATIONS AND USAGE")
        || flat.contains("5 WARNINGS AND PRECAUTIONS");
    if structured {
        return number.map_or(Found::Unbounded, |n| label_plain::numbered(flat, n));
    }
    if not_label(flat) {
        return Found::NotPrescribing;
    }
    label_plain::older(flat, section)
}
