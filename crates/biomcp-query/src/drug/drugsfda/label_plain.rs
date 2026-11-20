//! Find a section from its headings alone, where a label gives no contents list to anchor on
//! (1082): labels in the format used before 2006, and 2006-format labels whose contents page the
//! reader lost, such as a page drawn as an image.
//!
//! A heading counts only where it stands alone and is no cross-reference (`see WARNINGS`, a
//! title inside parentheses, or one followed by a comma). The section starts at the one such
//! heading for it and ends at the next heading of any section, which must be the section that
//! always follows it (PRECAUTIONS after WARNINGS, `6 ADVERSE REACTIONS` after `5 WARNINGS AND
//! PRECAUTIONS`) and must appear once. Anything else leaves the section unbounded: a missed or
//! repeated heading can only make a section unread, never cut it short or run it into the next.
use super::label_sections::{Found, TITLES, heading_at};

/// The plain headings of the format used before 2006 that bound the sections searched, in the
/// order labels print them, longest first where one holds another.
const OLD: [&str; 10] = [
    "INDICATIONS AND USAGE",
    "INDICATIONS",
    "CONTRAINDICATIONS",
    "WARNINGS",
    "PRECAUTIONS",
    "ADVERSE REACTIONS",
    "DRUG ABUSE AND DEPENDENCE",
    "OVERDOSAGE",
    "DOSAGE AND ADMINISTRATION",
    "HOW SUPPLIED",
];
/// The sentence that closes a 2006-format contents list.
const OMITTED: &str = "omitted from the full prescribing information are not listed";

/// Whether the heading-like `needle` at `at` is a cross-reference: after `and`, `or`, `the` or
/// an upper-case `SEE`, inside an open parenthesis or bracket, or followed by punctuation or a
/// joining word.
fn reference(text: &str, at: usize, needle: &str) -> bool {
    let before = text.get(..at).unwrap_or_default();
    let after = text.get(at + needle.len()..).unwrap_or_default();
    let joined = ["and ", "or ", "the ", "SEE ", "in "]
        .iter()
        .any(|w| before.ends_with(w));
    let window: String = before.chars().rev().take(300).collect();
    let open = |o: char, c: char| {
        window.chars().filter(|x| *x == o).count() > window.chars().filter(|x| *x == c).count()
    };
    let punctuated = after.starts_with([',', ';', ')', ']', '.']);
    let joining = [" section", " and ", " or "]
        .iter()
        .any(|w| after.starts_with(w));
    joined || open('(', ')') || open('[', ']') || punctuated || joining
}

/// Every heading of `titles` from `from` on, as (position, index into `titles`), in text order;
/// where two titles overlap the longer, listed first, wins.
fn headings(flat: &str, titles: &[String], from: usize) -> Vec<(usize, usize)> {
    let mut found: Vec<(usize, usize, usize)> = Vec::new();
    for (index, title) in titles.iter().enumerate() {
        let rest = flat.get(from..).unwrap_or_default();
        for (at, _) in rest.match_indices(title.as_str()) {
            let at = from + at;
            let end = at + title.len();
            let overlaps = found.iter().any(|(a, e, _)| at < *e && *a < end);
            if !overlaps && heading_at(flat, at, title) && !reference(flat, at, title) {
                found.push((at, end, index));
            }
        }
    }
    found.sort_unstable();
    found
        .into_iter()
        .map(|(at, _, index)| (at, index))
        .collect()
}

/// The text from the heading of `chain[0]` through the headings of the rest of `chain`, each the
/// next heading after the one before, to the next heading, one of `ends`. Indices are into
/// `titles`; each heading in the chain and the end must appear once.
fn bounded(flat: &str, heads: &[(usize, usize)], chain: &[usize], ends: &[usize]) -> Found {
    let once = |index: usize| {
        let mut at = heads.iter().filter(|(_, i)| *i == index);
        match (at.next(), at.next()) {
            (Some((at, _)), None) => Some(*at),
            _ => None,
        }
    };
    let Some(begin) = chain.first().and_then(|first| once(*first)) else {
        return Found::Unbounded;
    };
    let mut cursor = begin;
    let mut wanted = chain.iter().skip(1);
    loop {
        let Some(&(at, index)) = heads.iter().find(|(at, _)| *at > cursor) else {
            return Found::Unbounded;
        };
        match wanted.next() {
            Some(next) if *next == index && once(index) == Some(at) => cursor = at,
            Some(_) => return Found::Unbounded,
            None if ends.contains(&index) && once(index) == Some(at) => {
                let text = flat.get(begin..at).unwrap_or_default().trim();
                // A stretch of a contents list has no sentence: no word ends with a full stop.
                let ended = format!("{text} ");
                let prose = ended.match_indices(". ").any(|(dot, _)| {
                    ended
                        .get(..dot)
                        .is_some_and(|b| b.ends_with(char::is_lowercase))
                });
                return if prose {
                    Found::Section(text.to_owned())
                } else {
                    Found::Unbounded
                };
            }
            None => return Found::Unbounded,
        }
    }
}

/// A section of a label in the format used before 2006, whose headings are plain titles.
pub(super) fn older(flat: &str, section: biomcp_types::LabelHistorySection) -> Found {
    use biomcp_types::LabelHistorySection as S;
    let titles: Vec<String> = OLD.iter().map(|t| (*t).to_owned()).collect();
    let index = |title: &&str| OLD.iter().position(|t| t == title).unwrap_or(usize::MAX);
    let (starts, chain, ends): (&[&str], &[&str], &[&str]) = match section {
        S::IndicationsAndUsage => (
            &["INDICATIONS AND USAGE", "INDICATIONS"],
            &[],
            &["CONTRAINDICATIONS"],
        ),
        S::Contraindications => (&["CONTRAINDICATIONS"], &[], &["WARNINGS"]),
        S::WarningsAndCautions => (&["WARNINGS"], &["PRECAUTIONS"], &["ADVERSE REACTIONS"]),
        S::AdverseReactions => (
            &["ADVERSE REACTIONS"],
            &[],
            &["DRUG ABUSE AND DEPENDENCE", "OVERDOSAGE"],
        ),
        _ => return Found::Older,
    };
    let heads = headings(flat, &titles, 0);
    let Some(start) = starts
        .iter()
        .map(index)
        .find(|i| heads.iter().any(|(_, h)| h == i))
    else {
        return Found::Older;
    };
    let chain: Vec<usize> = std::iter::once(start)
        .chain(chain.iter().map(index))
        .collect();
    let ends: Vec<usize> = ends.iter().map(index).collect();
    bounded(flat, &heads, &chain, &ends)
}

/// A numbered section of a 2006-format label whose contents list was not read, from its body's
/// numbered headings after the list's closing sentence or the body's own heading.
pub(super) fn numbered(flat: &str, number: u8) -> Found {
    let ends: &[u8] = match number {
        1 => &[2],
        4 => &[5],
        5 => &[6],
        6 => &[7],
        7 => &[8],
        8 => &[9, 10],
        _ => return Found::Unbounded,
    };
    let from = [OMITTED, super::label_sections::FULL]
        .iter()
        .filter_map(|marker| flat.rfind(marker).map(|at| at + marker.len()))
        .max()
        .unwrap_or(0);
    let mut titles = Vec::new();
    let mut numbers = Vec::new();
    for (n, names) in TITLES {
        for name in names {
            for form in [format!("{n} {name}"), format!("{n}. {name}")] {
                titles.push(form);
                numbers.push(n);
            }
        }
    }
    let heads: Vec<(usize, usize)> = headings(flat, &titles, from)
        .into_iter()
        .map(|(at, i)| (at, usize::from(numbers.get(i).copied().unwrap_or(0))))
        .collect();
    let ends: Vec<usize> = ends.iter().map(|n| usize::from(*n)).collect();
    bounded(flat, &heads, &[usize::from(number)], &ends)
}

/// Whether a numbered heading of a section other than `number` stands in `flat[begin..end]`.
pub(super) fn crossed(flat: &str, number: u8, (begin, end): (usize, usize)) -> bool {
    let inside = flat.get(..end).unwrap_or(flat);
    TITLES
        .iter()
        .filter(|(n, _)| *n != number)
        .flat_map(|(n, names)| names.iter().map(move |name| (n, name)))
        .flat_map(|(n, name)| [format!("{n} {name}"), format!("{n}. {name}")])
        .any(|form| {
            inside
                .get(begin..)
                .unwrap_or_default()
                .match_indices(form.as_str())
                .any(|(at, _)| heading_at(inside, begin + at, &form))
        })
}
