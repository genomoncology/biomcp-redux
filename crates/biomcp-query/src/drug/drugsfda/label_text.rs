//! Clean the text BioMCP's PDF reader gives for a label, and find the sentences in a section
//! that hold a searched text (1066). FDA's stamp, page numbers and reference IDs are taken out,
//! and words hyphenated across lines are joined. Sections are found in [`super::label_sections`].

const STAMP: &str = "This label may not be the latest approved by FDA. For current labeling information, \
    please visit https://www.fda.gov/drugsatfda";

/// The label's text cleaned, each line's whitespace collapsed and empty lines dropped.
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
    joined
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Cleaned text on one line, without FDA's stamp, reference IDs or page numbers.
pub(super) fn flatten(cleaned: &str) -> String {
    let flat = cleaned.split_whitespace().collect::<Vec<_>>().join(" ");
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

/// At most two distinct sentences holding a hit, each cut to 300 characters.
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
        if sentence.chars().count() > 300 {
            sentence = sentence.chars().take(299).collect::<String>() + "…";
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
