//! User text sent to openFDA as words, never as openFDA query syntax (0841).
use std::borrow::Cow;

/// openFDA answers HTTP 400 to these characters even inside a quoted phrase or after a backslash
/// (live, 2026-11-15). Its tokenizer splits words at them, so a space finds the same words.
const REFUSED: &[char] = &['!', '#', '$', ';', '?', '^', '|', '~', '`'];

/// Unicode's combining mark blocks; each mark follows the letter it marks.
const COMBINING: [std::ops::RangeInclusive<char>; 5] = [
    '\u{0300}'..='\u{036F}',
    '\u{1AB0}'..='\u{1AFF}',
    '\u{1DC0}'..='\u{1DFF}',
    '\u{20D0}'..='\u{20FF}',
    '\u{FE20}'..='\u{FE2F}',
];

fn sent(c: char) -> bool {
    c.is_ascii() && !REFUSED.contains(&c)
}

/// An accented Latin letter as its base ASCII letters, in its own letter case.
fn base(c: char) -> Option<String> {
    if let Some(base) = crate::latin_base(c) {
        return Some(base.into());
    }
    let mut lower = c.to_lowercase();
    match (lower.next(), lower.next()) {
        (Some(lower), None) if lower != c => crate::latin_base(lower).map(str::to_ascii_uppercase),
        _ => None,
    }
}

/// Text as openFDA accepts it. openFDA also answers HTTP 400 to every character outside ASCII
/// (live, 2026-11-15), and FDA records spell names in ASCII, so an accented Latin letter becomes
/// its base letter (Nestlé, NESTLE); a combining mark (one of the COMBINING blocks, as macOS
/// pastes a decomposed letter) is dropped, so its base letter keeps the word whole (0976); a refused
/// character or any other non-ASCII character becomes a space, and spaces collapse. Text openFDA
/// accepts is returned unchanged.
pub fn openfda_words(text: &str) -> Cow<'_, str> {
    if text.chars().all(sent) {
        return Cow::Borrowed(text);
    }
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        if sent(c) {
            out.push(c);
        } else if let Some(base) = base(c) {
            out.push_str(&base);
        } else if !COMBINING.iter().any(|range| range.contains(&c)) {
            out.push(' ');
        }
    }
    Cow::Owned(out.split_whitespace().collect::<Vec<_>>().join(" "))
}

/// Text as one quoted openFDA phrase of [`openfda_words`], with quotes and backslashes escaped,
/// so no operator, wildcard or field in user text reaches openFDA as syntax.
pub fn openfda_phrase(text: &str) -> String {
    format!(
        "\"{}\"",
        openfda_words(text)
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
    )
}
