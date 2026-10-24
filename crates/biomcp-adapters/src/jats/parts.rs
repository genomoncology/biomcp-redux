use biodata::{Text, TextPart, TextPartKind};
use biomcp_fetch::BadReply;
/// Serialized text capacity leaves room for reader envelope and advancing step.
pub const TEXT_BYTES: usize = 12_000;
/// A bounded body part with original row boundaries.
#[derive(Debug)]
pub struct Part {
    /// Public outline metadata.
    pub meta: TextPart,
    /// Source paragraphs or table rows, split only for the output bound.
    pub rows: Vec<String>,
    /// Following part is a continuation of this source section or table.
    pub continuation: bool,
}
pub(super) fn append(
    out: &mut Vec<Part>,
    title: &str,
    kind: TextPartKind,
    rows: Vec<String>,
) -> Result<(), BadReply> {
    let start = out.len();
    let mut group = Vec::new();
    let mut bytes = 0;
    for row in rows {
        for piece in split(&row) {
            let size = escaped_bytes(&piece);
            if !group.is_empty() && bytes + size + 2 > TEXT_BYTES {
                push(out, title, kind, std::mem::take(&mut group), bytes)?;
                bytes = 0;
            }
            if !group.is_empty() {
                bytes += 2;
            }
            bytes += size;
            group.push(piece);
        }
    }
    if !group.is_empty() {
        push(out, title, kind, group, bytes)?;
    }
    let end = out.len();
    for (i, part) in out.iter_mut().enumerate().skip(start) {
        part.continuation = i + 1 < end;
    }
    Ok(())
}
fn push(
    out: &mut Vec<Part>,
    title: &str,
    kind: TextPartKind,
    rows: Vec<String>,
    bytes: usize,
) -> Result<(), BadReply> {
    out.push(Part {
        meta: TextPart {
            number: u32::try_from(out.len() + 1).map_err(|_| BadReply)?,
            title: Text::new(title).map_err(|_| BadReply)?,
            kind,
            tokens: u32::try_from((bytes + 2).div_ceil(4)).map_err(|_| BadReply)?,
        },
        rows,
        continuation: false,
    });
    Ok(())
}
fn split(row: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut bytes = 0;
    for (i, ch) in row.char_indices() {
        let size = escaped_char(ch);
        if bytes + size > TEXT_BYTES {
            out.push(row[start..i].to_owned());
            start = i;
            bytes = 0;
        }
        bytes += size;
    }
    if start < row.len() {
        out.push(row[start..].to_owned());
    }
    out
}
/// Compact JSON escaped bytes for a string's contents, excluding surrounding quotes.
pub fn escaped_bytes(text: &str) -> usize {
    text.chars().map(escaped_char).sum()
}
fn escaped_char(ch: char) -> usize {
    match ch {
        '"' | '\\' | '\n' | '\r' | '\t' | '\u{8}' | '\u{c}' => 2,
        c if c <= '\u{1f}' => 6,
        c => c.len_utf8(),
    }
}
