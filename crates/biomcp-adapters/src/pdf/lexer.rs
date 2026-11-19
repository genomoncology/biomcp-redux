//! The PDF object syntax (PDF 32000-1 §7.2 and §7.3) read straight from bytes, so a document
//! can be measured before lopdf loads it, and a content stream cut before lopdf decodes it.

use lopdf::{Dictionary, Object, StringFormat};

/// The nesting lopdf reads before it refuses an object.
const DEPTH: usize = 100;

pub(super) const fn whitespace(byte: u8) -> bool {
    matches!(byte, b'\0' | b'\t' | b'\n' | b'\x0c' | b'\r' | b' ')
}

pub(super) const fn delimiter(byte: u8) -> bool {
    matches!(
        byte,
        b'(' | b')' | b'<' | b'>' | b'[' | b']' | b'{' | b'}' | b'/' | b'%'
    )
}

/// The first `needle` in `bytes` at or after `from`.
pub(super) fn find(bytes: &[u8], from: usize, needle: &[u8]) -> Option<usize> {
    bytes
        .get(from..)?
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|at| from + at)
}

/// Past a run of regular characters.
pub(super) fn skip_regular(bytes: &[u8], from: usize) -> usize {
    bytes
        .get(from..)
        .and_then(|rest| rest.iter().position(|b| whitespace(*b) || delimiter(*b)))
        .map_or(bytes.len(), |length| from + length)
}

/// To the end of the line.
pub(super) fn skip_line(bytes: &[u8], from: usize) -> usize {
    bytes
        .get(from..)
        .and_then(|rest| rest.iter().position(|b| matches!(b, b'\r' | b'\n')))
        .map_or(bytes.len(), |length| from + length)
}

/// Past the first `end` at or after `from`.
pub(super) fn skip_past(bytes: &[u8], from: usize, end: u8) -> Option<usize> {
    bytes
        .get(from..)?
        .iter()
        .position(|b| *b == end)
        .map(|length| from + length + 1)
}

/// Past a literal string, its balanced parentheses and backslash escapes included.
pub(super) fn skip_string(bytes: &[u8], from: usize) -> Option<usize> {
    let (mut depth, mut at) = (0usize, from);
    while let Some(&byte) = bytes.get(at) {
        match byte {
            b'\\' => at += 1,
            b'(' => depth += 1,
            b')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(at + 1);
                }
            }
            _ => {}
        }
        at += 1;
    }
    None
}

/// Past whitespace and comments.
pub(super) fn skip_space(bytes: &[u8], from: usize) -> usize {
    let mut at = from;
    while let Some(&byte) = bytes.get(at) {
        if whitespace(byte) {
            at += 1;
        } else if byte == b'%' {
            at = skip_line(bytes, at);
        } else {
            break;
        }
    }
    at
}

/// The unsigned integer at `from`, and where it ends.
pub(super) fn integer(bytes: &[u8], from: usize) -> Option<(u64, usize)> {
    let end = skip_regular(bytes, from);
    let token = bytes.get(from..end).filter(|token| !token.is_empty())?;
    let value = token.iter().try_fold(0u64, |value, byte| {
        let digit = char::from(*byte).to_digit(10)?;
        value.checked_mul(10)?.checked_add(u64::from(digit))
    })?;
    Some((value, end))
}

/// Past the header `N G obj` of an indirect object at `from`, with its object number.
pub(super) fn header(bytes: &[u8], from: usize) -> Option<(u64, usize)> {
    let (number, at) = integer(bytes, skip_space(bytes, from))?;
    let (_, at) = integer(bytes, skip_space(bytes, at))?;
    let at = skip_space(bytes, at);
    let end = skip_regular(bytes, at);
    (bytes.get(at..end) == Some(b"obj".as_slice())).then_some((number, end))
}

/// Where the object at `from` ends and how many objects it holds, counted without building
/// them; `None` when lopdf would not read one object there.
pub(super) fn measure(bytes: &[u8], from: usize) -> Option<(usize, usize)> {
    measure_within(bytes, from, DEPTH)
}

fn measure_within(bytes: &[u8], from: usize, depth: usize) -> Option<(usize, usize)> {
    let at = skip_space(bytes, from);
    let rest = bytes.get(at..)?;
    let close = match rest.first()? {
        _ if depth == 0 => return None,
        b'<' if rest.starts_with(b"<<") => b">>".as_slice(),
        b'[' => b"]".as_slice(),
        b'<' => return Some((skip_past(bytes, at, b'>')?, 1)),
        b'(' => return Some((skip_string(bytes, at)?, 1)),
        b'/' => return Some((skip_regular(bytes, at + 1), 1)),
        byte if delimiter(*byte) => return None,
        _ => return scalar(bytes, at).map(|(_, end)| (end, 1)),
    };
    // Each opening delimiter is as long as its closing one.
    let (mut at, mut count) = (at + close.len(), 1usize);
    loop {
        at = skip_space(bytes, at);
        if bytes.get(at..)?.starts_with(close) {
            return Some((at + close.len(), count));
        }
        let (end, inner) = measure_within(bytes, at, depth - 1)?;
        at = end;
        count = count.checked_add(inner)?;
    }
}

/// A number, keyword or reference at `from`, and where it ends.
fn scalar(bytes: &[u8], from: usize) -> Option<(Object, usize)> {
    let end = skip_regular(bytes, from);
    let token = bytes.get(from..end).filter(|token| !token.is_empty())?;
    match token {
        b"true" => return Some((Object::Boolean(true), end)),
        b"false" => return Some((Object::Boolean(false), end)),
        b"null" => return Some((Object::Null, end)),
        _ => {}
    }
    if let Some((number, after)) = integer(bytes, from) {
        let reference = integer(bytes, skip_space(bytes, after)).and_then(|(generation, at)| {
            let at = skip_space(bytes, at);
            let id = (u32::try_from(number).ok()?, u16::try_from(generation).ok()?);
            (bytes.get(at..skip_regular(bytes, at)) == Some(b"R".as_slice())).then_some((id, at + 1))
        });
        if let Some((id, at)) = reference {
            return Some((Object::Reference(id), at));
        }
        return Some((Object::Integer(i64::try_from(number).ok()?), after));
    }
    let text = std::str::from_utf8(token).ok()?;
    text.parse::<f32>().ok().map(|real| (Object::Real(real), end))
}

/// The object at `from`, built, and where it ends; for the small dictionaries the measuring
/// reads, never for a whole document.
pub(super) fn object(bytes: &[u8], from: usize) -> Option<(Object, usize)> {
    object_within(bytes, from, DEPTH)
}

fn object_within(bytes: &[u8], from: usize, depth: usize) -> Option<(Object, usize)> {
    let at = skip_space(bytes, from);
    let rest = bytes.get(at..)?;
    match rest.first()? {
        _ if depth == 0 => None,
        b'<' if rest.starts_with(b"<<") => {
            let (mut dictionary, mut at) = (Dictionary::new(), at + 2);
            loop {
                at = skip_space(bytes, at);
                if bytes.get(at..)?.starts_with(b">>") {
                    return Some((Object::Dictionary(dictionary), at + 2));
                }
                let (Object::Name(key), end) = object_within(bytes, at, depth - 1)? else {
                    return None;
                };
                let (value, end) = object_within(bytes, end, depth - 1)?;
                dictionary.set(key, value);
                at = end;
            }
        }
        b'[' => {
            let (mut items, mut at) = (Vec::new(), at + 1);
            loop {
                at = skip_space(bytes, at);
                if bytes.get(at..)?.starts_with(b"]") {
                    return Some((Object::Array(items), at + 1));
                }
                let (item, end) = object_within(bytes, at, depth - 1)?;
                items.push(item);
                at = end;
            }
        }
        b'(' | b'<' => {
            let (end, _) = measure_within(bytes, at, 1)?;
            let raw = bytes.get(at + 1..end - 1)?.to_vec();
            Some((Object::String(raw, StringFormat::Literal), end))
        }
        b'/' => {
            let end = skip_regular(bytes, at + 1);
            Some((Object::Name(decoded(bytes.get(at + 1..end)?)), end))
        }
        byte if delimiter(*byte) => None,
        _ => scalar(bytes, at),
    }
}

/// A name's bytes with each `#xx` escape decoded (PDF 32000-1 §7.3.5).
fn decoded(name: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(name.len());
    let mut index = 0;
    while let Some(byte) = name.get(index) {
        let digit = |at: usize| name.get(at).copied().and_then(hex);
        let escape = (*byte == b'#')
            .then(|| Some(digit(index + 1)? * 16 + digit(index + 2)?))
            .flatten();
        if let Some(value) = escape {
            out.push(value);
            index += 3;
        } else {
            out.push(*byte);
            index += 1;
        }
    }
    out
}

const fn hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
