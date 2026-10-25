//! Mechanical Unicode-safe splitting measured in compact JSON-escaped bytes.
pub(crate) fn split(row: &str, budget: usize) -> Vec<String> {
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
