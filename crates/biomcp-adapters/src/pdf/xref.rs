//! The cross-reference sections lopdf follows when it loads a document strictly, read from the
//! raw bytes so every object it will parse is known before it parses any (PDF 32000-1 §7.5.4,
//! §7.5.6 and §7.5.8).
//!
//! The sections are found as lopdf finds them: from the last `startxref` near the end, then
//! each trailer's `/Prev` and `/XRefStm`. Every in-use entry of every section is kept, so the
//! table read here holds every offset lopdf will parse and possibly more. A section that does
//! not read refuses the document as irregular; more than [`SECTIONS`] sections or [`ENTRIES`]
//! entries, or a cross-reference stream over its limit, as too large.

use super::fonts::entry;
use super::lexer::{Read, integer, object, skip_space};
use super::scan::Refusal::{Irregular, TooLarge};
use lopdf::{Dictionary, Object, Stream};
use std::collections::BTreeSet;

/// Most cross-reference sections a document may have; the FDA corpus has at most five.
pub(super) const SECTIONS: usize = 32;
/// Most entries all sections may hold together.
pub(super) const ENTRIES: usize = 1 << 20;
/// Most bytes a section dictionary may take.
const DICTIONARY: usize = 64 << 10;

/// The in-use entries of every section, as object number and byte offset, and the bytes their
/// cross-reference streams expanded to.
#[derive(Default)]
pub(super) struct Table {
    pub(super) entries: Vec<(u64, usize)>,
    pub(super) expanded: usize,
}

/// A non-negative integer object.
pub(super) fn whole(object: Option<&Object>) -> Read<u64> {
    match object {
        Some(Object::Integer(value)) => u64::try_from(*value).map_err(|_| Irregular),
        _ => Err(Irregular),
    }
}

/// A count or offset as an index into memory.
pub(super) fn index(value: u64) -> Read<usize> {
    usize::try_from(value).map_err(|_| Irregular)
}

/// Where the first section starts, found as lopdf 0.47 finds it (`Reader::get_xref_start`):
/// the last `%%EOF` in the final 1,024 bytes once trailing whitespace is trimmed, and the last
/// `startxref` in the 25 bytes before it. A file with anything but whitespace after that
/// `%%EOF`, or whose `startxref` line is not exactly `startxref`, its number and `%%EOF`, is
/// refused, so this reading and lopdf's cannot pick different sections (1068 third review).
fn start(bytes: &[u8]) -> Read<u64> {
    let end = bytes
        .iter()
        .rposition(|byte| !matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | b'\0' | b'\x0c'))
        .map_or(0, |at| at + 1);
    let seek = end.saturating_sub(1024);
    let eof = bytes
        .get(seek..end)
        .and_then(|tail| tail.windows(5).rposition(|w| w == b"%%EOF"))
        .map(|at| seek + at)
        .filter(|at| *at > 25 && at + 5 == end)
        .ok_or(Irregular)?;
    let keyword = bytes
        .get(eof - 25..eof)
        .and_then(|window| window.windows(9).rposition(|w| w == b"startxref"))
        .ok_or(Irregular)?
        + eof
        - 25;
    let mut at = keyword + b"startxref".len();
    at += usize::from(bytes.get(at) == Some(&b' '));
    at = line_end(bytes, at)?;
    while bytes.get(at) == Some(&b' ') {
        at += 1;
    }
    let (start, mut at) = integer(bytes, at)?;
    while bytes.get(at) == Some(&b' ') {
        at += 1;
    }
    if line_end(bytes, at)? == eof {
        Ok(start)
    } else {
        Err(Irregular)
    }
}

/// Past the end of line at `at`.
fn line_end(bytes: &[u8], at: usize) -> Read<usize> {
    match bytes.get(at..).unwrap_or_default() {
        [b'\r', b'\n', ..] => Ok(at + 2),
        [b'\n' | b'\r', ..] => Ok(at + 1),
        _ => Err(Irregular),
    }
}

/// Every section of the document.
pub(super) fn read(bytes: &[u8], stream_limit: usize) -> Read<Table> {
    let mut table = Table::default();
    let start = start(bytes)?;
    let (mut queue, mut seen) = (vec![start], BTreeSet::new());
    while let Some(offset) = queue.pop() {
        let offset = index(offset)?;
        if !seen.insert(offset) {
            continue;
        }
        if seen.len() > SECTIONS {
            return Err(TooLarge);
        }
        let trailer = section(bytes, offset, stream_limit, &mut table)?;
        for key in [b"XRefStm".as_slice(), b"Prev"] {
            if let Some(next) = entry(&trailer, key) {
                queue.push(whole(Some(next))?);
            }
        }
    }
    Ok(table)
}

/// The section at `offset`, its entries added to `table`, and its trailer dictionary.
fn section(bytes: &[u8], offset: usize, limit: usize, table: &mut Table) -> Read<Dictionary> {
    let at = skip_space(bytes, offset);
    if bytes.get(at..).unwrap_or_default().starts_with(b"xref") {
        return classic(bytes, at + 4, table);
    }
    let (_, at) = super::lexer::header(bytes, at)?;
    let (dictionary, at) = small_dictionary(bytes, at)?;
    let length = index(whole(entry(&dictionary, b"Length"))?)?;
    let start = payload(bytes, at).ok_or(Irregular)?;
    let end = start.checked_add(length).ok_or(Irregular)?;
    let content = bytes.get(start..end).ok_or(Irregular)?.to_vec();
    let expanded = Stream::new(dictionary.clone(), content)
        .get_plain_content_with_limit(limit)
        .map_err(|issue| {
            if super::page::too_large(&issue) {
                TooLarge
            } else {
                Irregular
            }
        })?;
    table.expanded = table.expanded.saturating_add(expanded.len());
    stream_entries(&dictionary, &expanded, table)?;
    Ok(dictionary)
}

/// A dictionary at `from` no longer than [`DICTIONARY`] bytes, and where it ends.
pub(super) fn small_dictionary(bytes: &[u8], from: usize) -> Read<(Dictionary, usize)> {
    let (end, _) = super::lexer::measure(bytes, from)?;
    if end.saturating_sub(from) > DICTIONARY {
        return Err(Irregular);
    }
    match object(bytes, from)? {
        (Object::Dictionary(dictionary), end) => Ok((dictionary, end)),
        _ => Err(Irregular),
    }
}

/// Where a stream's data starts, after the `stream` keyword and its end of line that follow a
/// dictionary ending at `from`; `None` when no stream follows.
pub(super) fn payload(bytes: &[u8], from: usize) -> Option<usize> {
    let at = skip_space(bytes, from);
    if !bytes.get(at..)?.starts_with(b"stream") {
        return None;
    }
    let mut at = at + b"stream".len();
    while matches!(bytes.get(at), Some(b' ' | b'\t')) {
        at += 1;
    }
    match bytes.get(at..)? {
        [b'\r', b'\n', ..] => Some(at + 2),
        [b'\n' | b'\r', ..] => Some(at + 1),
        _ => None,
    }
}

/// A classic table's subsections from `from` up to its trailer.
fn classic(bytes: &[u8], from: usize, table: &mut Table) -> Read<Dictionary> {
    let mut at = from;
    loop {
        at = skip_space(bytes, at);
        if bytes.get(at..).unwrap_or_default().starts_with(b"trailer") {
            return small_dictionary(bytes, at + b"trailer".len()).map(|(trailer, _)| trailer);
        }
        let (first, next) = integer(bytes, at)?;
        let (count, next) = integer(bytes, skip_space(bytes, next))?;
        at = next;
        for number in first..first.checked_add(count).ok_or(Irregular)? {
            let (offset, next) = integer(bytes, skip_space(bytes, at))?;
            let (_, next) = integer(bytes, skip_space(bytes, next))?;
            let kind = skip_space(bytes, next);
            match bytes.get(kind) {
                Some(b'n') => table.entries.push((number, index(offset)?)),
                Some(b'f') => {}
                _ => return Err(Irregular),
            }
            at = kind + 1;
            if table.entries.len() > ENTRIES {
                return Err(TooLarge);
            }
        }
    }
}

/// The entries of a cross-reference stream (PDF 32000-1 §7.5.8.3).
fn stream_entries(dictionary: &Dictionary, content: &[u8], table: &mut Table) -> Read<()> {
    let integers = |key: &[u8]| -> Option<Read<Vec<u64>>> {
        match entry(dictionary, key)? {
            Object::Array(items) => Some(items.iter().map(|item| whole(Some(item))).collect()),
            _ => Some(Err(Irregular)),
        }
    };
    let widths = integers(b"W").ok_or(Irregular)??;
    let [kind, second, third] = widths.as_slice() else {
        return Err(Irregular);
    };
    let widths = [index(*kind)?, index(*second)?, index(*third)?];
    if widths.iter().any(|width| *width > 8) || widths.iter().sum::<usize>() == 0 {
        return Err(Irregular);
    }
    let size = whole(entry(dictionary, b"Size"))?;
    let ranges = integers(b"Index").unwrap_or_else(|| Ok(vec![0, size]))?;
    let mut rows = content.chunks_exact(widths.iter().sum());
    for pair in ranges.chunks_exact(2) {
        let [first, count] = pair else {
            return Err(Irregular);
        };
        for number in *first..first.checked_add(*count).ok_or(Irregular)? {
            let Some(row) = rows.next() else {
                return Ok(());
            };
            let field = |from: usize, width: usize| {
                row.get(from..from + width)
                    .unwrap_or_default()
                    .iter()
                    .fold(0u64, |value, byte| value << 8 | u64::from(*byte))
            };
            let kind = if widths[0] == 0 {
                1
            } else {
                field(0, widths[0])
            };
            if kind == 1 {
                table
                    .entries
                    .push((number, index(field(widths[0], widths[1]))?));
            }
            if table.entries.len() > ENTRIES {
                return Err(TooLarge);
            }
        }
    }
    Ok(())
}
