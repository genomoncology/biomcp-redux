//! The cross-reference sections lopdf follows when it loads a document strictly, read from the
//! raw bytes so every object it will parse is known before it parses any (PDF 32000-1 §7.5.4,
//! §7.5.6 and §7.5.8).
//!
//! The sections are found as lopdf finds them: from the last `startxref` near the end, then
//! each trailer's `/Prev` and `/XRefStm`. Every in-use entry of every section is kept, so the
//! table read here holds every offset lopdf will parse and possibly more. A section that does
//! not read, or more than [`SECTIONS`] of them, refuses the document.

use super::lexer::{integer, object, skip_space};
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

/// Every section of the document, or `None` when one does not read or there are too many.
pub(super) fn read(bytes: &[u8], stream_limit: usize) -> Option<Table> {
    let tail = bytes.len().saturating_sub(1100);
    let keyword = bytes.get(tail..)?.windows(9).rposition(|w| w == b"startxref")? + tail;
    let (start, _) = integer(bytes, skip_space(bytes, keyword + 9))?;
    let mut table = Table::default();
    let (mut queue, mut seen) = (vec![start], BTreeSet::new());
    while let Some(offset) = queue.pop() {
        let offset = usize::try_from(offset).ok()?;
        if !seen.insert(offset) {
            continue;
        }
        if seen.len() > SECTIONS {
            return None;
        }
        let trailer = section(bytes, offset, stream_limit, &mut table)?;
        for key in [b"XRefStm".as_slice(), b"Prev"] {
            if let Ok(Object::Integer(next)) = trailer.get(key) {
                queue.push(u64::try_from(*next).ok()?);
            }
        }
        if table.entries.len() > ENTRIES {
            return None;
        }
    }
    Some(table)
}

/// The section at `offset`, its entries added to `table`, and its trailer dictionary.
fn section(bytes: &[u8], offset: usize, limit: usize, table: &mut Table) -> Option<Dictionary> {
    let at = skip_space(bytes, offset);
    if bytes.get(at..)?.starts_with(b"xref") {
        return classic(bytes, at + 4, table);
    }
    let (_, at) = super::lexer::header(bytes, at)?;
    let (dictionary, at) = small_dictionary(bytes, at)?;
    let Ok(Object::Integer(length)) = dictionary.get(b"Length") else {
        return None;
    };
    let start = payload(bytes, at)?;
    let end = start.checked_add(usize::try_from(*length).ok()?)?;
    let content = bytes.get(start..end)?.to_vec();
    let expanded = Stream::new(dictionary.clone(), content)
        .decompressed_content_with_limit(limit)
        .ok()?;
    table.expanded = table.expanded.saturating_add(expanded.len());
    stream_entries(&dictionary, &expanded, table)?;
    Some(dictionary)
}

/// A dictionary at `from` no longer than [`DICTIONARY`] bytes, and where it ends.
pub(super) fn small_dictionary(bytes: &[u8], from: usize) -> Option<(Dictionary, usize)> {
    let (end, _) = super::lexer::measure(bytes, from)?;
    if end.saturating_sub(from) > DICTIONARY {
        return None;
    }
    match object(bytes, from)? {
        (Object::Dictionary(dictionary), end) => Some((dictionary, end)),
        _ => None,
    }
}

/// Where a stream's data starts, after the `stream` keyword and its end of line that follow a
/// dictionary ending at `from`.
pub(super) fn payload(bytes: &[u8], from: usize) -> Option<usize> {
    let at = skip_space(bytes, from);
    let mut at = at + b"stream".len();
    if bytes.get(at - b"stream".len()..at)? != b"stream" {
        return None;
    }
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
fn classic(bytes: &[u8], from: usize, table: &mut Table) -> Option<Dictionary> {
    let mut at = from;
    loop {
        at = skip_space(bytes, at);
        if bytes.get(at..)?.starts_with(b"trailer") {
            return small_dictionary(bytes, at + b"trailer".len()).map(|(trailer, _)| trailer);
        }
        let (first, next) = integer(bytes, at)?;
        let (count, next) = integer(bytes, skip_space(bytes, next))?;
        at = next;
        for number in first..first.checked_add(count)? {
            let (offset, next) = integer(bytes, skip_space(bytes, at))?;
            let (_, next) = integer(bytes, skip_space(bytes, next))?;
            let kind = skip_space(bytes, next);
            match bytes.get(kind)? {
                b'n' => table.entries.push((number, usize::try_from(offset).ok()?)),
                b'f' => {}
                _ => return None,
            }
            at = kind + 1;
            if table.entries.len() > ENTRIES {
                return None;
            }
        }
    }
}

/// The entries of a cross-reference stream (PDF 32000-1 §7.5.8.3).
fn stream_entries(dictionary: &Dictionary, content: &[u8], table: &mut Table) -> Option<()> {
    let integers = |key: &[u8]| -> Option<Vec<u64>> {
        let Ok(Object::Array(items)) = dictionary.get(key) else {
            return None;
        };
        items
            .iter()
            .map(|item| item.as_i64().ok().and_then(|n| u64::try_from(n).ok()))
            .collect()
    };
    let widths = integers(b"W")?;
    let [kind, second, third] = widths.as_slice() else {
        return None;
    };
    let widths = [*kind, *second, *third].map(|width| usize::try_from(width).unwrap_or(9));
    if widths.iter().any(|width| *width > 8) || widths.iter().sum::<usize>() == 0 {
        return None;
    }
    let size = dictionary.get(b"Size").ok()?.as_i64().ok()?;
    let index = integers(b"Index").unwrap_or_else(|| vec![0, u64::try_from(size).unwrap_or(0)]);
    let mut rows = content.chunks_exact(widths.iter().sum());
    for pair in index.chunks_exact(2) {
        let [first, count] = pair else { return None };
        for number in *first..first.checked_add(*count)? {
            let Some(row) = rows.next() else {
                return Some(());
            };
            let field = |from: usize, width: usize| {
                row.get(from..from + width)
                    .unwrap_or_default()
                    .iter()
                    .fold(0u64, |value, byte| value << 8 | u64::from(*byte))
            };
            let kind = if widths[0] == 0 { 1 } else { field(0, widths[0]) };
            if kind == 1 {
                let offset = usize::try_from(field(widths[0], widths[1])).ok()?;
                table.entries.push((number, offset));
            }
            if table.entries.len() > ENTRIES {
                return None;
            }
        }
    }
    Some(())
}

