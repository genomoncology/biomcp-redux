//! Bound label sections using the contents list and the corresponding body headings.
use super::label_plain;
use super::label_sections::{FULL, Found, TITLES, heading_at, word};

const CONTENTS: [&str; 2] = [
    "FULL PRESCRIBING INFORMATION: CONTENTS",
    "FULL PRESCRIBING INFORMATION:CONTENTS",
];

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

/// A top-level section named by the contents list.
struct Listed {
    number: u8,
    title: &'static str,
}

impl Listed {
    fn numbered(&self) -> [String; 2] {
        [
            format!("{} {}", self.number, self.title),
            format!("{}. {}", self.number, self.title),
        ]
    }
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
                    });
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
            let [plain, dotted] = l.numbered();
            let forms = [plain, dotted, l.title.to_owned()];
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

fn contents(flat: &str) -> Option<(usize, usize)> {
    CONTENTS
        .iter()
        .filter_map(|c| flat.find(c).map(|at| (at, at + c.len())))
        .min()
}

/// A section of a 2006-format label, or none when the label has no contents list.
pub(super) fn listed_section(flat: &str, number: Option<u8>) -> Option<Found> {
    let (start, entries) = contents(flat)?;
    let Some(body) = flat.get(entries..)?.find(FULL).map(|at| entries + at) else {
        // The body's own heading is missing, as when its first page is drawn as an image.
        let block = flat.get(entries..).unwrap_or_default();
        return Some(match number {
            None => titled(block),
            Some(number) => label_plain::numbered(flat, number),
        });
    };
    let block = flat.get(entries..body).unwrap_or_default();
    let listed = listed(block);
    let body = body + FULL.len();
    let places = placed(flat, &listed, body);
    let Some(number) = number else {
        return Some(boxed(flat, (start, block, body), &listed, &places));
    };
    let Some(i) = listed.iter().position(|l| l.number == number) else {
        return Some(
            if block.contains(label_plain::OMITTED)
                && bounded_omission(flat, number, &listed, &places)
            {
                Found::Absent
            } else {
                Found::Unbounded
            },
        );
    };
    let begin = places.get(i).copied().flatten();
    let end = places.get(i + 1).copied().flatten();
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

/// The omitted section has no body when the immediately preceding section explicitly says
/// "None." and the heading of the immediately following section starts right after it.
fn bounded_omission(flat: &str, number: u8, listed: &[Listed], places: &[Option<usize>]) -> bool {
    let Some(i) = listed.iter().position(|l| l.number + 1 == number) else {
        return false;
    };
    let (Some(before), Some(after)) = (listed.get(i), listed.get(i + 1)) else {
        return false;
    };
    if after.number != number + 1 {
        return false;
    }
    let (Some(Some(start)), Some(Some(end))) = (places.get(i), places.get(i + 1)) else {
        return false;
    };
    let Some(region) = flat.get(*start..*end) else {
        return false;
    };
    before
        .numbered()
        .iter()
        .any(|form| region.strip_prefix(form).map(str::trim) == Some("None."))
        && after.numbered().iter().any(|form| {
            flat.get(*end..).is_some_and(|tail| tail.starts_with(form))
                && heading_at(flat, *end, form)
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
    let Some(Some(one)) =
        (listed.first().is_some_and(|l| l.number == 1)).then(|| places.first().copied().flatten())
    else {
        return titled(block);
    };
    let region = flat.get(body..one).unwrap_or_default();
    if let Some(at) = region.find("WARNING") {
        return Found::Section(region.get(at..).unwrap_or_default().trim().to_owned());
    }
    let head = ["1 INDICATIONS", "1. INDICATIONS"]
        .into_iter()
        .filter_map(|title| block.find(title))
        .min()
        .and_then(|at| block.get(..at))
        .unwrap_or(block);
    if flat.get(..contents).is_some_and(|h| h.contains("WARNING:")) || head.contains("WARNING") {
        return titled(block);
    }
    Found::Absent
}

/// The nearest subsection before the hit whose full listed title also occurs in the section body.
pub(super) fn subheading(flat: &str, section: &str, hit: usize) -> Option<String> {
    let (_, start) = contents(flat)?;
    let (block, _) = flat.get(start..)?.split_once(FULL)?;
    let block = block.split('*').find(|part| !part.trim().is_empty())?;
    let entries: Vec<_> = block
        .char_indices()
        .filter_map(|(at, _)| entry(block, at).map(|e| (at, e)))
        .collect();
    let hit = section.char_indices().nth(hit)?.0;
    entries
        .iter()
        .enumerate()
        .filter_map(|(i, &(_, (major, minor, tail)))| {
            let minor = minor?;
            let end = entries.get(i + 1).map_or(block.len(), |(at, _)| *at);
            let start = block.len() - tail.len();
            let title = block.get(start..end)?.trim();
            if title.len() > 80 || !title.starts_with(char::is_uppercase) {
                return None;
            }
            let heading = format!("{major}.{minor} {title}");
            let dotted = format!("{major}.{minor}. {title}");
            let at = [heading.as_str(), dotted.as_str()]
                .into_iter()
                .filter_map(|form| heading_from(section, form, 0))
                .filter(|at| *at <= hit)
                .max()?;
            Some((at, heading))
        })
        .max_by_key(|(at, _)| *at)
        .map(|(_, heading)| heading)
}
