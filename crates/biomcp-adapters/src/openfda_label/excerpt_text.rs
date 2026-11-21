//! Match normalized text while retaining source byte ranges for excerpts.
use caseless::Caseless;
use std::ops::Range;

const EXCERPT_CHARS: usize = 300;

pub(super) fn fold(
    text: &str,
    pipes: &[usize],
) -> impl Iterator<Item = (char, Range<usize>)> + Clone {
    let mut was_space = true;
    text.trim_end()
        .char_indices()
        .filter_map(move |(at, ch)| {
            let space = ch.is_whitespace() || (ch == '|' && pipes.binary_search(&at).is_ok());
            let repeated_space = std::mem::replace(&mut was_space, space) && space;
            if repeated_space {
                return None;
            }
            let folded = if space { ' ' } else { dash(ch) };
            Some((folded, at..at + ch.len_utf8()))
        })
        .flat_map(|(ch, span)| {
            case_fold(ch)
                .into_iter()
                .flatten()
                .map(move |folded| (folded, span.clone()))
        })
}

// A full Unicode fold expands one scalar into at most three. Store that expansion so
// the position iterator can be copied when locating the end of a matched phrase.
fn case_fold(ch: char) -> [Option<char>; 3] {
    if ch.is_ascii() {
        return [Some(ch.to_ascii_lowercase()), None, None];
    }
    let mut folded = std::iter::once(ch).default_case_fold();
    std::array::from_fn(|_| folded.next())
}

fn dash(ch: char) -> char {
    match ch {
        '\u{058a}'
        | '\u{05be}'
        | '\u{1400}'
        | '\u{1806}'
        | '\u{2010}'..='\u{2015}'
        | '\u{2053}'
        | '\u{207b}'
        | '\u{208b}'
        | '\u{2212}'
        | '\u{2e17}'
        | '\u{2e1a}'
        | '\u{2e3a}'
        | '\u{2e3b}'
        | '\u{2e40}'
        | '\u{2e5d}'
        | '\u{301c}'
        | '\u{3030}'
        | '\u{30a0}'
        | '\u{fe31}'
        | '\u{fe32}'
        | '\u{fe58}'
        | '\u{fe63}'
        | '\u{ff0d}'
        | '\u{10ead}' => '-',
        _ => ch,
    }
}

/// Do not end halfway through a source character expanded by Unicode case folding.
pub(super) fn match_end(
    mut rest: impl Iterator<Item = (char, Range<usize>)>,
    first_end: usize,
    characters: usize,
) -> Option<usize> {
    let end = match characters.checked_sub(2) {
        Some(skip) => rest.nth(skip)?.1.end,
        None => first_end,
    };
    rest.next()
        .is_none_or(|(_, span)| span.end != end)
        .then_some(end)
}

/// Give the surrounding paragraph, clipping at Unicode character boundaries when needed.
pub(super) fn around(text: &str, hit: Range<usize>) -> (Range<usize>, Box<str>) {
    let before = text.get(..hit.start).unwrap_or_default();
    let matched = text.get(hit.clone()).unwrap_or_default();
    let after = text.get(hit.end..).unwrap_or_default();
    let before = ["\n\n", "\r\n\r\n"]
        .iter()
        .filter_map(|breaks| before.rsplit_once(*breaks).map(|(_, tail)| tail))
        .min_by_key(|tail| tail.len())
        .unwrap_or(before);
    let after = ["\n\n", "\r\n\r\n"]
        .iter()
        .filter_map(|breaks| after.split_once(*breaks).map(|(head, _)| head))
        .min_by_key(|head| head.len())
        .unwrap_or(after);
    let room = EXCERPT_CHARS.saturating_sub(matched.chars().count());
    let left = before
        .char_indices()
        .rev()
        .take(room / 2)
        .last()
        .map_or("", |(at, _)| before.get(at..).unwrap_or_default());
    let remaining = room.saturating_sub(left.chars().count());
    let until = after
        .char_indices()
        .nth(remaining)
        .map_or(after.len(), |(at, _)| at);
    let span = hit.start.saturating_sub(left.len())..hit.end + until;
    let excerpt = words(text.get(span.clone()).unwrap_or_default());
    let prefix = if left.len() < before.len() { "…" } else { "" };
    let suffix = if until < after.len() { "…" } else { "" };
    (span, format!("{prefix}{excerpt}{suffix}").into())
}

pub(super) fn words(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
