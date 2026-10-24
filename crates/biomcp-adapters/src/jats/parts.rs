use super::body::{BodyRow, Group, Rows};
use biodata::{Text, TextPart};
use biomcp_fetch::BadReply;
/// Serialized text capacity leaves room for reader envelope and advancing step.
pub const TEXT_BYTES: usize = 12_000;
/// A bounded body part with original row boundaries.
#[derive(Debug)]
pub struct Part {
    /// Public outline metadata.
    pub meta: TextPart,
    /// Source paragraphs or table rows, split only for the output bound.
    pub rows: Vec<Row>,
    /// Source heading ancestry.
    pub heading_path: Box<[Text]>,
    /// Caption and column headings for table find excerpts.
    pub context: Option<String>,
    /// Structural separator whose encoded size is included in the part bound.
    pub separator: &'static str,
    /// Following part is a continuation of this source section or table.
    pub continuation: bool,
}
/// Bounded fragment of one original source paragraph or table row.
#[derive(Debug)]
pub struct Row {
    /// Source row index in the document's original rows.
    pub source_row: usize,
    /// Whole fragment text; concatenating fragments preserves the original row.
    pub text: String,
}
pub(super) fn append(
    out: &mut Vec<Part>,
    group: Group,
    originals: &mut Rows,
) -> Result<(), BadReply> {
    let source = group;
    let incoming = source.rows.iter().try_fold(0usize, |n, r| {
        n.checked_add(r.markdown.len()).ok_or(BadReply)
    })?;
    originals.bytes = originals
        .bytes
        .checked_add(incoming)
        .filter(|n| *n <= super::MAX_MARKDOWN_BYTES)
        .ok_or(BadReply)?;
    let metadata = source
        .path
        .iter()
        .map(|v| escaped_bytes(v.as_str()) + 3)
        .sum::<usize>()
        + source.context.as_ref().map_or(0, |v| escaped_bytes(v));
    let budget = TEXT_BYTES
        .checked_sub(metadata + 256)
        .filter(|n| *n >= 256)
        .ok_or(BadReply)?;
    let start = out.len();
    let mut group = Vec::new();
    let mut bytes = 0;
    for row in source.rows.iter() {
        let source_row = originals.values.len();
        let pieces = split(&row.markdown, budget);
        originals.values.push(BodyRow {
            paragraphs: row.paragraphs.clone(),
            plain: row.plain.clone(),
            markdown: row.markdown.clone(),
            context: row.context,
            references: row.references.clone(),
        });
        for piece in pieces {
            let size = escaped_bytes(&piece);
            if !group.is_empty() && bytes + size + escaped_bytes(source.separator()) > budget {
                push(out, &source, std::mem::take(&mut group), bytes)?;
                bytes = 0;
            }
            if !group.is_empty() {
                bytes += escaped_bytes(source.separator());
            }
            bytes += size;
            group.push(Row {
                source_row,
                text: piece,
            });
        }
    }
    if !group.is_empty() {
        push(out, &source, group, bytes)?;
    }
    let end = out.len();
    for (i, part) in out.iter_mut().enumerate().skip(start) {
        part.continuation = i + 1 < end;
    }
    Ok(())
}
fn push(out: &mut Vec<Part>, source: &Group, rows: Vec<Row>, bytes: usize) -> Result<(), BadReply> {
    out.push(Part {
        meta: TextPart {
            number: u32::try_from(out.len() + 1).map_err(|_| BadReply)?,
            title: Text::new(&source.title).map_err(|_| BadReply)?,
            kind: source.kind,
            tokens: u32::try_from((bytes + 2).div_ceil(4)).map_err(|_| BadReply)?,
        },
        rows,
        continuation: false,
        heading_path: source.path.clone().into(),
        context: source.context.clone(),
        separator: source.separator(),
    });
    Ok(())
}
fn split(row: &str, budget: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut bytes = 0;
    for (i, ch) in row.char_indices() {
        let size = escaped_char(ch);
        if bytes + size > budget {
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
