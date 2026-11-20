//! Find one section of a label's prescribing information in the text BioMCP's PDF reader
//! gives, and the sentences in it that hold a searched text (1066).
//!
//! The reader runs headings into the text around them, so a heading is found inside a line: a
//! top-level section number and its title in capitals, such as `5 WARNINGS AND PRECAUTIONS`, or
//! its first subsection, such as `5.1 Myopathy`. A section ends at the next top-level heading of
//! another section. The last heading with at least 200 characters under it is the section
//! itself, which passes over the highlights and the contents. The boxed warning is the last
//! `WARNING:` block before section 1. Labels in the format used before 2006 have plain headings
//! (WARNINGS, PRECAUTIONS, ADVERSE REACTIONS) and no boxed-warning heading. FDA's stamp, page
//! numbers and reference IDs are taken out, and words hyphenated across lines are joined.
//! These rules were measured on 81 archived labels in the design note; new layouts may defeat
//! them, and a section they cannot bound is reported as unread, never as absent.
use biomcp_types::LabelHistorySection;

/// Top-level prescribing-information sections by number, their titles in capitals.
const TITLES: [(u8, &[&str]); 17] = [
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
/// Plain headings of the format used before 2006, as (key, title).
const OLD: [(u8, &str); 11] = [
    (1, "INDICATIONS"),
    (4, "CONTRAINDICATIONS"),
    (51, "WARNINGS"),
    (52, "PRECAUTIONS"),
    (6, "ADVERSE REACTIONS"),
    (90, "DRUG ABUSE AND DEPENDENCE"),
    (91, "OVERDOSAGE"),
    (92, "DOSAGE AND ADMINISTRATION"),
    (93, "HOW SUPPLIED"),
    (94, "CLINICAL PHARMACOLOGY"),
    (95, "DESCRIPTION"),
];
/// Shortest text under a heading that counts as the section rather than a mention of it.
const LEAST: usize = 200;
const STAMP: &str = "This label may not be the latest approved by FDA. For current labeling information, \
    please visit https://www.fda.gov/drugsatfda";

/// What looking for a section found.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Found {
    /// The section's text, whitespace collapsed.
    Section(String),
    /// The label has no such section.
    Absent,
    /// A heading names the section but its end was not found.
    Unbounded,
    /// The document is not prescribing information.
    NotPrescribing,
    /// A label in the older format, which has no heading for this section.
    Older,
}

/// The label's text cleaned and with whitespace collapsed to single spaces.
pub(super) fn clean(text: &str) -> String {
    let mut joined = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\u{ad}' => {}
            '\u{fb01}' => joined.push_str("fi"),
            '\u{fb02}' => joined.push_str("fl"),
            // A word hyphenated across a line break is one word.
            '-' if joined.chars().last().is_some_and(char::is_alphanumeric)
                && chars.peek() == Some(&'\n') =>
            {
                chars.next();
                if !chars.peek().is_some_and(|next| next.is_alphanumeric()) {
                    joined.push_str("-\n");
                }
            }
            c => joined.push(c),
        }
    }
    let flat = joined.split_whitespace().collect::<Vec<_>>().join(" ");
    let flat = flat.replace(&STAMP.split_whitespace().collect::<Vec<_>>().join(" "), " ");
    let flat = strip_numbered(&flat, "Reference ID:", None);
    let flat = strip_numbered(&flat, "Page ", Some(" of "));
    flat.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Remove `prefix` followed by digits (and `middle` and more digits when given).
fn strip_numbered(text: &str, prefix: &str, middle: Option<&str>) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(prefix) {
        let (before, from) = rest.split_at(at);
        let after = from.get(prefix.len()..).unwrap_or_default().trim_start();
        let digits = after.len() - after.trim_start_matches(|c: char| c.is_ascii_digit()).len();
        let mut end = (digits > 0).then_some(from.len() - after.len() + digits);
        if let (Some(stop), Some(middle)) = (end, middle) {
            let tail = from.get(stop..).unwrap_or_default();
            end = tail.strip_prefix(middle).and_then(|more| {
                let n = more.len() - more.trim_start_matches(|c: char| c.is_ascii_digit()).len();
                (n > 0).then_some(stop + middle.len() + n)
            });
        }
        out.push_str(before);
        match end {
            Some(end) => {
                out.push(' ');
                rest = from.get(end..).unwrap_or_default();
            }
            None => {
                out.push_str(prefix);
                rest = from.get(prefix.len()..).unwrap_or_default();
            }
        }
    }
    out.push_str(rest);
    out
}

fn word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Every top-level heading in `flat`, as (start, end, section number), in order.
fn headings(flat: &str) -> Vec<(usize, usize, u8)> {
    let mut found = Vec::new();
    for (at, c) in flat.char_indices() {
        let before = flat.get(..at).and_then(|b| b.chars().last());
        if !c.is_ascii_digit() || before.is_some_and(|b| word(b) || matches!(b, '.' | '(' | '[')) {
            continue;
        }
        let rest = flat.get(at..).unwrap_or_default();
        let digits = rest.len() - rest.trim_start_matches(|c: char| c.is_ascii_digit()).len();
        let Some((number, titles)) = rest
            .get(..digits)
            .and_then(|n| TITLES.iter().find(|(k, _)| k.to_string() == n))
        else {
            continue;
        };
        let after = rest.get(digits..).unwrap_or_default();
        let titled = titles.iter().find_map(|title| {
            let tail = after.strip_prefix(' ')?.strip_prefix(title)?;
            (!tail.starts_with(word)).then_some(digits + 1 + title.len())
        });
        // The first subsection, such as `5.1 Myopathy`, also opens the section.
        let first = after.strip_prefix(".1 ").and_then(|tail| {
            let mut chars = tail.chars();
            (chars.next()?.is_ascii_uppercase() && chars.next()?.is_ascii_lowercase())
                .then_some(digits + 3)
        });
        if let Some(length) = titled.or(first) {
            found.push((at, at + length, *number));
        }
    }
    found
}

/// The text of section `number`, or why there is none.
fn numbered(flat: &str, marks: &[(usize, usize, u8)], number: u8) -> Found {
    let mut good: Vec<(usize, usize)> = Vec::new();
    let mut any = false;
    for (start, end, _) in marks.iter().filter(|m| m.2 == number) {
        any = true;
        let stop = marks
            .iter()
            .find(|m| m.2 != number && m.0 >= *end)
            .map_or(flat.len(), |m| m.0);
        if stop - start >= LEAST {
            good.push((*start, stop));
        }
    }
    let Some(&(_, stop)) = good.last() else {
        return if any { Found::Unbounded } else { Found::Absent };
    };
    let start = good
        .iter()
        .filter(|g| g.1 == stop)
        .map(|g| g.0)
        .min()
        .unwrap_or(stop);
    Found::Section(flat.get(start..stop).unwrap_or_default().to_owned())
}

/// The boxed warning: the last `WARNING:` block before section 1's body.
fn boxed(flat: &str, marks: &[(usize, usize, u8)]) -> Found {
    let has_box = flat.contains("WARNING: ") || flat.contains("WARNINGS: ");
    let Found::Section(indications) = numbered(flat, marks, 1) else {
        return if has_box {
            Found::Unbounded
        } else {
            Found::Absent
        };
    };
    let before = flat
        .get(..flat.find(&indications).unwrap_or(0))
        .unwrap_or_default();
    let last = [before.rfind("WARNING: "), before.rfind("WARNINGS: ")]
        .into_iter()
        .flatten()
        .max();
    let Some(last) = last else {
        return Found::Absent;
    };
    let block = before.get(last..).unwrap_or_default();
    let contents = block.contains("FULL PRESCRIBING INFORMATION: CONTENTS")
        || block.contains("FULL PRESCRIBING INFORMATION:CONTENTS")
        || subsection_two(block);
    if contents || block.len() < 100 {
        return Found::Unbounded;
    }
    Found::Section(block.trim_end().to_owned())
}

/// Whether `block` holds a section-2 subsection number, which marks the contents.
fn subsection_two(block: &str) -> bool {
    block.match_indices("2.").any(|(at, _)| {
        let before = block.get(..at).and_then(|b| b.chars().last());
        let after = block.get(at + 2..).unwrap_or_default();
        before.is_none_or(|b| !word(b))
            && after.chars().next().is_some_and(|c| c.is_ascii_digit())
            && after.chars().nth(1) == Some(' ')
    })
}

/// Every plain older-format heading, as (start, end, key).
fn old_headings(flat: &str) -> Vec<(usize, usize, u8)> {
    let mut found = Vec::new();
    for (key, title) in OLD {
        for (at, _) in flat.match_indices(title) {
            let before = flat.get(..at).and_then(|b| b.chars().last());
            let after = flat.get(at + title.len()..).and_then(|a| a.chars().next());
            if before.is_none_or(|b| !b.is_alphabetic())
                && after.is_none_or(|a| !a.is_alphabetic() && a != ':')
            {
                found.push((at, at + title.len(), key));
            }
        }
    }
    found.sort_unstable();
    found
}

/// An older-format section: the last heading with enough text before the next heading.
fn old_section(flat: &str, marks: &[(usize, usize, u8)], key: u8) -> Option<String> {
    let mut best = None;
    for (start, end, _) in marks.iter().filter(|m| m.2 == key) {
        let stop = marks
            .iter()
            .find(|m| m.2 != key && m.0 >= *end)
            .map_or(flat.len(), |m| m.0);
        if stop - start >= LEAST {
            best = Some((*start, stop));
        }
    }
    best.and_then(|(start, stop)| flat.get(start..stop))
        .map(str::to_owned)
}

/// Find `section` in a label's cleaned text.
pub(super) fn find(flat: &str, section: LabelHistorySection) -> Found {
    use LabelHistorySection as S;
    let marks = headings(flat);
    let structured =
        flat.contains("FULL PRESCRIBING INFORMATION") || marks.iter().any(|m| m.2 == 1 || m.2 == 5);
    if structured {
        let number = match section {
            S::BoxedWarning => return boxed(flat, &marks),
            S::IndicationsAndUsage => 1,
            S::Contraindications => 4,
            S::WarningsAndCautions => 5,
            S::AdverseReactions => 6,
            S::DrugInteractions => 7,
            S::UseInSpecificPopulations => 8,
        };
        return numbered(flat, &marks, number);
    }
    if !flat.contains("INDICATIONS") && !flat.contains("CONTRAINDICATIONS") {
        return Found::NotPrescribing;
    }
    let old = old_headings(flat);
    let keys: &[u8] = match section {
        S::IndicationsAndUsage => &[1],
        S::Contraindications => &[4],
        S::WarningsAndCautions => &[51, 52],
        S::AdverseReactions => &[6],
        _ => return Found::Older,
    };
    let parts: Vec<String> = keys
        .iter()
        .filter_map(|key| old_section(flat, &old, *key))
        .collect();
    if parts.is_empty() {
        Found::Absent
    } else {
        Found::Section(parts.join(" "))
    }
}

/// Fold text for matching: ASCII letters lower-cased, typographic apostrophes and quotes plain.
/// Each character keeps its place, so a match's character index is the same in both.
fn fold(text: &str) -> Vec<char> {
    text.chars()
        .map(|c| match c {
            '\u{2019}' | '\u{2018}' => '\'',
            '\u{201c}' | '\u{201d}' => '"',
            '\u{2010}' | '\u{2011}' => '-',
            c => c.to_ascii_lowercase(),
        })
        .collect()
}

/// Character positions in `section` where any of `finds` starts, in order.
pub(super) fn hits(section: &str, finds: &[biodata::Text]) -> Vec<usize> {
    let hay = fold(section);
    let mut found = Vec::new();
    for find in finds {
        let needle = fold(
            &find
                .as_str()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" "),
        );
        if needle.is_empty() || needle.len() > hay.len() {
            continue;
        }
        found.extend(
            (0..=hay.len() - needle.len())
                .filter(|at| hay.get(*at..*at + needle.len()) == Some(&needle[..])),
        );
    }
    found.sort_unstable();
    found.dedup();
    found
}

/// At most two distinct sentences holding a hit, each cut to 400 characters.
pub(super) fn sentences(section: &str, hits: &[usize]) -> Vec<String> {
    let chars: Vec<char> = section.chars().collect();
    let mut out: Vec<String> = Vec::new();
    for &at in hits {
        let start = (0..at)
            .rev()
            .find(|i| matches!(chars.get(*i..*i + 2), Some(['.', ' '] | ['•', ' '])))
            .map_or(0, |i| i + 2);
        let end = (at..chars.len())
            .find(|i| chars.get(*i..*i + 2) == Some(&['.', ' ']))
            .map_or(chars.len(), |i| i + 1);
        let mut sentence: String = chars.get(start..end).unwrap_or_default().iter().collect();
        if sentence.chars().count() > 400 {
            sentence = sentence.chars().take(399).collect::<String>() + "…";
        }
        if !out.contains(&sentence) {
            out.push(sentence);
        }
        if out.len() == 2 {
            break;
        }
    }
    out
}

/// The subsection heading above character `at` of `section`, as the label's contents name it.
pub(super) fn heading(flat: &str, section: &str, at: usize) -> Option<String> {
    let before: String = section.chars().take(at).collect();
    let number = before.match_indices('.').rev().find_map(|(dot, _)| {
        let major = before.get(..dot)?;
        let digits = major.len() - major.trim_end_matches(|c: char| c.is_ascii_digit()).len();
        let start = dot - digits;
        let minor = before.get(dot + 1..)?;
        let n = minor.len() - minor.trim_start_matches(|c: char| c.is_ascii_digit()).len();
        let lead = before.get(..start).and_then(|b| b.chars().last());
        let opens = digits > 0
            && n > 0
            && minor.get(n..n + 1) == Some(" ")
            && lead.is_none_or(|l| !word(l) && l != '.' && l != '(');
        opens
            .then(|| before.get(start..dot + 1 + n).map(str::to_owned))
            .flatten()
    })?;
    // The contents list each subsection once: `6.2 Postmarketing Experience 7 DRUG ...`.
    let entry = flat
        .match_indices(&format!("{number} "))
        .find_map(|(at, _)| {
            let rest = flat.get(at + number.len() + 1..)?;
            let stop = rest.char_indices().find(|(i, c)| {
                c.is_ascii_digit() && rest.get(..*i).is_some_and(|b| b.ends_with(' '))
            })?;
            let title = rest.get(..stop.0)?.trim();
            (title.len() <= 80 && !title.is_empty()).then(|| format!("{number} {title}"))
        });
    Some(entry.unwrap_or(number))
}

#[cfg(test)]
mod probe {
    #[test]
    fn probe_livalo() {
        let t = std::fs::read_to_string("/tmp/biomcp-1066-probe.txt").unwrap();
        let flat = super::clean(&t);
        let marks = super::headings(&flat);
        eprintln!("PROBE marks {:?}", marks.iter().filter(|m| m.2 >= 5 && m.2 <= 8).collect::<Vec<_>>());
        eprintln!("PROBE myas {:?} len {}", flat.find("myasthenia"), flat.len());
        match super::find(&flat, biomcp_types::LabelHistorySection::AdverseReactions) {
            super::Found::Section(s) => eprintln!("PROBE section {} {:?} hits {:?}", s.len(), &s[..80], super::hits(&s, &[biodata::Text::from_source("myasthenia gravis").unwrap()])),
            other => eprintln!("PROBE {other:?}"),
        }
    }
}
