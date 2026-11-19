//! A content stream cut into pieces of whole operations, so the reader never holds more than
//! one piece's parsed operations at a time.
//!
//! lopdf's `Content::decode` builds every operation of a stream at once, at about 600 bytes
//! each: 4 MiB of one-byte `n` operations took 1.2 GB (1068). The stream is cut after an
//! operator at the top level once a piece holds [`PIECE`] bytes, so each piece decodes alone
//! to the same operations. An operation longer than [`OPERATION`] bytes, such as a `TJ` array
//! of two million numbers, is left out and reported; every inline image (`BI` … `EI`) is left
//! out silently, since it draws no text.

use super::lexer::{delimiter, skip_line, skip_past, skip_regular, skip_string, whitespace};

/// Bytes after which a piece ends at the next operator.
pub(super) const PIECE: usize = 64 << 10;
/// Longest operation kept.
pub(super) const OPERATION: usize = 64 << 10;

/// The pieces of `content` to decode in order, and whether an operation too long to keep was
/// left out.
pub(super) fn pieces(content: &[u8]) -> (Vec<&[u8]>, bool) {
    let mut pieces = Vec::new();
    let (mut piece, mut operation, mut at, mut depth) = (0usize, 0usize, 0usize, 0usize);
    let mut dropped = false;
    while let Some(&byte) = content.get(at) {
        match byte {
            b'%' => at = skip_line(content, at),
            b'(' => at = skip_string(content, at).unwrap_or(content.len()),
            b'<' if content.get(at + 1) == Some(&b'<') => {
                depth += 1;
                at += 2;
            }
            b'<' => at = skip_past(content, at, b'>').unwrap_or(content.len()),
            b'>' if content.get(at + 1) == Some(&b'>') => {
                depth = depth.saturating_sub(1);
                at += 2;
            }
            b'[' | b'{' => {
                depth += 1;
                at += 1;
            }
            b']' | b'}' => {
                depth = depth.saturating_sub(1);
                at += 1;
            }
            b'/' => at = skip_regular(content, at + 1),
            _ if whitespace(byte) || byte == b')' || byte == b'>' => at += 1,
            _ => {
                let end = skip_regular(content, at);
                let token = content.get(at..end).unwrap_or_default();
                at = end.max(at + 1);
                if depth > 0 || operand(token) {
                    continue;
                }
                // An operator ends an operation at the top level.
                let inline = token == b"BI";
                if inline {
                    at = skip_inline_image(content, at);
                }
                if inline || at - operation > OPERATION {
                    if operation > piece {
                        pieces.push(content.get(piece..operation).unwrap_or_default());
                    }
                    dropped |= !inline;
                    piece = at;
                } else if at - piece >= PIECE {
                    pieces.push(content.get(piece..at).unwrap_or_default());
                    piece = at;
                }
                operation = at;
            }
        }
    }
    if piece < content.len() {
        pieces.push(content.get(piece..).unwrap_or_default());
    }
    (pieces, dropped)
}

/// Numbers and the keywords `true`, `false` and `null` are operands, not operators.
fn operand(token: &[u8]) -> bool {
    matches!(token, b"true" | b"false" | b"null")
        || token
            .iter()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'+' | b'-' | b'.'))
}

/// Past an inline image's `EI` keyword, which follows whitespace and ends at a delimiter.
fn skip_inline_image(content: &[u8], from: usize) -> usize {
    let mut at = from;
    while let Some(found) = content
        .get(at..)
        .and_then(|rest| rest.windows(2).position(|pair| pair == b"EI"))
    {
        let start = at + found;
        let before = start
            .checked_sub(1)
            .and_then(|index| content.get(index))
            .is_some_and(|byte| whitespace(*byte));
        let after = content
            .get(start + 2)
            .is_none_or(|byte| whitespace(*byte) || delimiter(*byte));
        if before && after {
            return start + 2;
        }
        at = start + 2;
    }
    content.len()
}
