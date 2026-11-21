//! Locate dotted prescribing-information headings in native label text.
use biomcp_types::DrugNameMatch;

#[derive(Debug)]
pub(super) struct Heading<'a> {
    pub part: usize,
    pub at: usize,
    pub number: &'a str,
    pub title: &'a str,
}

impl Heading<'_> {
    pub fn matches(&self, asked: &str) -> bool {
        self.number == asked
            || DrugNameMatch::same_name(self.title, asked)
            || DrugNameMatch::same_name(&self.printed(), asked)
    }

    pub fn printed(&self) -> String {
        format!("{} {}", self.number, self.title)
    }

    pub fn contains(&self, next: &Self) -> bool {
        next.number
            .strip_prefix(self.number)
            .is_some_and(|rest| rest.starts_with('.'))
    }
}

/// openFDA may join paragraphs with spaces. Accept a number after a sentence, closed
/// reference or line break, with a heading title following; ignore inline references.
pub(super) fn headings(parts: &[String]) -> Vec<Heading<'_>> {
    let mut found = Vec::new();
    for (part, text) in parts.iter().enumerate() {
        let mut previous = ' ';
        for (at, ch) in text.char_indices() {
            if ch.is_ascii_digit()
                && previous.is_whitespace()
                && let Some(heading) = heading_at(part, text, at)
            {
                found.push(heading);
            }
            previous = ch;
        }
    }
    found
}

fn heading_at(part: usize, text: &str, at: usize) -> Option<Heading<'_>> {
    let (before, rest) = text.split_at(at);
    let boundary = before.trim_end_matches(|c: char| c.is_whitespace() && c != '\n' && c != '\r');
    if !boundary.is_empty() && !boundary.ends_with(['\n', '\r', '.', ')', ']']) {
        return None;
    }
    let end = rest.find(char::is_whitespace)?;
    let (number, rest) = rest.split_at(end);
    if !number.contains('.')
        || !number
            .split('.')
            .all(|sub| !sub.is_empty() && sub.len() <= 2 && sub.bytes().all(|c| c.is_ascii_digit()))
    {
        return None;
    }
    let title = title(rest.trim_start())?;
    Some(Heading {
        part,
        at,
        number,
        title,
    })
}

/// Titles on flattened lines use title case. A capitalized word before lowercase
/// prose begins the sentence, rather than extending the title.
fn title(text: &str) -> Option<&str> {
    let line = text.lines().next()?;
    let mut rest = line;
    let mut end = 0;
    while let Some(word) = rest.split_whitespace().next() {
        let (_, after) = rest.trim_start().split_at(word.len());
        rest = after;
        if !word.starts_with(char::is_uppercase) {
            if end > 0 && connector(word) {
                continue;
            }
            break;
        }
        let next = rest.split_whitespace().find(|word| !connector(word));
        if end > 0 && next.is_some_and(|word| word.starts_with(char::is_lowercase)) {
            break;
        }
        if word.ends_with(['.', ':', ';', '!', '?']) {
            break;
        }
        end = line.len() - rest.len();
    }
    line.get(..end).filter(|title| !title.is_empty())
}

fn connector(word: &str) -> bool {
    matches!(
        word,
        "a" | "an"
            | "and"
            | "at"
            | "by"
            | "for"
            | "in"
            | "of"
            | "on"
            | "or"
            | "the"
            | "to"
            | "with"
            | "without"
    )
}
