//! Bounded, case-insensitive passages from embedded PDF text.

/// Escaped bytes of one found passage.
const PASSAGE_BYTES: usize = 3072;
/// Escaped bytes of text kept before a match.
const LEAD_BYTES: usize = 512;
/// Escaped bytes of one find page, matching article text.
pub(crate) const FIND_BYTES: usize = 12_000;

pub(crate) struct Passage<'a> {
    pub(crate) start: usize,
    pub(crate) text: &'a str,
}

/// A match starts a passage with a little lead text. Later matches inside it add no passage.
pub(crate) fn passages<'a>(
    text: &'a str,
    needles: &'a [String],
) -> impl Iterator<Item = Passage<'a>> + 'a {
    let mut covered = 0;
    std::iter::from_fn(move || {
        let at = next_match(text, covered, needles)?;
        let begin = lead(text, at).max(covered);
        let end = passage_end(text, begin, at);
        let past = text
            .get(at..)?
            .chars()
            .next()
            .map_or(text.len(), |c| at + c.len_utf8());
        covered = end.max(past);
        Some(Passage {
            start: begin,
            text: text.get(begin..end)?.trim(),
        })
    })
}
fn next_match(text: &str, from: usize, needles: &[String]) -> Option<usize> {
    let mut here = from;
    let mut lines = text.get(from..)?.split_inclusive('\n').peekable();
    while let Some(line) = lines.next() {
        let found = first_match(line, needles).or_else(|| {
            // A phrase may wrap onto the next extracted line; it then starts near this line's end.
            let next = lines.peek()?;
            let kept = line.trim_end();
            let joined = format!("{kept} {}", next.trim_start());
            first_match(&joined, needles).filter(|at| *at < kept.len())
        });
        if let Some(at) = found {
            return Some(here + at);
        }
        here += line.len();
    }
    None
}
/// Byte position of the first case-insensitive match in the original spelling.
fn first_match(line: &str, needles: &[String]) -> Option<usize> {
    let lower = line.to_lowercase();
    let start = needles
        .iter()
        .filter_map(|needle| lower.find(needle.as_str()))
        .min()?;
    let mut lower_at = 0;
    line.char_indices().find_map(|(byte, c)| {
        let width: usize = c.to_lowercase().map(char::len_utf8).sum();
        let found = (start < lower_at + width).then_some(byte);
        lower_at += width;
        found
    })
}
/// Keep the line start within the lead budget, or skip a clipped word.
fn lead(text: &str, at: usize) -> usize {
    let mut begin = at.saturating_sub(LEAD_BYTES);
    while !text.is_char_boundary(begin) {
        begin += 1;
    }
    let preceding = text.get(begin..at).unwrap_or_default();
    if let Some(gap) = preceding.rfind('\n') {
        return begin + gap + 1;
    }
    if begin == 0
        || text
            .get(..begin)
            .is_some_and(|before| before.ends_with('\n'))
    {
        return begin;
    }
    preceding.find(' ').map_or(begin, |gap| begin + gap + 1)
}
/// End within the passage size, at the last line break after the match when there is one.
fn passage_end(text: &str, begin: usize, at: usize) -> usize {
    let mut size = 0;
    let mut end = begin;
    let mut line_end = None;
    for (offset, c) in text.get(begin..).unwrap_or_default().char_indices() {
        size += biomcp_adapters::pdf::escaped_bytes(c.encode_utf8(&mut [0; 4]));
        if size > PASSAGE_BYTES {
            break;
        }
        end = begin + offset + c.len_utf8();
        if c == '\n' && end > at {
            line_end = Some(end);
        }
    }
    if end < text.len() {
        line_end.unwrap_or(end)
    } else {
        end
    }
}
