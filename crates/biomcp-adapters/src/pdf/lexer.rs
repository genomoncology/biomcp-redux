//! The PDF object syntax (PDF 32000-1 §7.2 and §7.3) read straight from bytes, so a document
//! can be measured before lopdf loads it, and a content stream cut before lopdf decodes it.

use super::scan::Refusal::{self, Irregular};
use lopdf::{Dictionary, Object, StringFormat};

/// A reading that lopdf would refuse or read differently is irregular.
pub(super) type Read<T> = Result<T, Refusal>;

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
pub(super) fn integer(bytes: &[u8], from: usize) -> Read<(u64, usize)> {
    let end = skip_regular(bytes, from);
    let token = bytes.get(from..end).filter(|token| !token.is_empty());
    let value = token
        .ok_or(Irregular)?
        .iter()
        .try_fold(0u64, |value, byte| {
            let digit = char::from(*byte).to_digit(10)?;
            value.checked_mul(10)?.checked_add(u64::from(digit))
        });
    Ok((value.ok_or(Irregular)?, end))
}

/// Past the header `N G obj` of an indirect object at `from`, with its object number.
pub(super) fn header(bytes: &[u8], from: usize) -> Read<(u64, usize)> {
    let (number, at) = integer(bytes, skip_space(bytes, from))?;
    let (_, at) = integer(bytes, skip_space(bytes, at))?;
    let at = skip_space(bytes, at);
    let end = skip_regular(bytes, at);
    if bytes.get(at..end) == Some(b"obj".as_slice()) {
        Ok((number, end))
    } else {
        Err(Irregular)
    }
}

/// Where the object at `from` ends and how many objects it holds, counted without building
/// them; refused when lopdf would not read one object there.
pub(super) fn measure(bytes: &[u8], from: usize) -> Read<(usize, usize)> {
    measure_within(bytes, from, DEPTH)
}

fn measure_within(bytes: &[u8], from: usize, depth: usize) -> Read<(usize, usize)> {
    let at = skip_space(bytes, from);
    let rest = bytes.get(at..).unwrap_or_default();
    let close = match rest.first().ok_or(Irregular)? {
        _ if depth == 0 => return Err(Irregular),
        b'<' if rest.starts_with(b"<<") => b">>".as_slice(),
        b'[' => b"]".as_slice(),
        b'<' => return Ok((skip_past(bytes, at, b'>').ok_or(Irregular)?, 1)),
        b'(' => return Ok((skip_string(bytes, at).ok_or(Irregular)?, 1)),
        b'/' => return Ok((skip_regular(bytes, at + 1), 1)),
        byte if delimiter(*byte) => return Err(Irregular),
        _ => return scalar(bytes, at).map(|(_, end)| (end, 1)),
    };
    // Each opening delimiter is as long as its closing one.
    let (mut at, mut count) = (at + close.len(), 1usize);
    loop {
        at = skip_space(bytes, at);
        let rest = bytes
            .get(at..)
            .filter(|rest| !rest.is_empty())
            .ok_or(Irregular)?;
        if rest.starts_with(close) {
            return Ok((at + close.len(), count));
        }
        let (end, inner) = measure_within(bytes, at, depth - 1)?;
        at = end;
        count = count.saturating_add(inner);
    }
}

/// A number, keyword or reference at `from`, and where it ends.
fn scalar(bytes: &[u8], from: usize) -> Read<(Object, usize)> {
    let end = skip_regular(bytes, from);
    let token = bytes.get(from..end).filter(|token| !token.is_empty());
    match token.ok_or(Irregular)? {
        b"true" => return Ok((Object::Boolean(true), end)),
        b"false" => return Ok((Object::Boolean(false), end)),
        b"null" => return Ok((Object::Null, end)),
        token if token.iter().all(u8::is_ascii_digit) => {}
        token => {
            let text = std::str::from_utf8(token).map_err(|_| Irregular)?;
            let real = text.parse::<f32>().map_err(|_| Irregular)?;
            return Ok((Object::Real(real), end));
        }
    }
    let (number, after) = integer(bytes, from)?;
    let generation = skip_space(bytes, after);
    let generation_end = skip_regular(bytes, generation);
    let keyword = skip_space(bytes, generation_end);
    let digits = bytes.get(generation..generation_end).unwrap_or_default();
    if !digits.is_empty()
        && digits.iter().all(u8::is_ascii_digit)
        && bytes.get(keyword..skip_regular(bytes, keyword)) == Some(b"R".as_slice())
    {
        let (generation, _) = integer(bytes, generation)?;
        let id = (
            u32::try_from(number).map_err(|_| Irregular)?,
            u16::try_from(generation).map_err(|_| Irregular)?,
        );
        return Ok((Object::Reference(id), keyword + 1));
    }
    Ok((
        Object::Integer(i64::try_from(number).map_err(|_| Irregular)?),
        after,
    ))
}

/// The object at `from`, built, and where it ends; for the small dictionaries the measuring
/// reads, never for a whole document.
pub(super) fn object(bytes: &[u8], from: usize) -> Read<(Object, usize)> {
    object_within(bytes, from, DEPTH)
}

fn object_within(bytes: &[u8], from: usize, depth: usize) -> Read<(Object, usize)> {
    let at = skip_space(bytes, from);
    let rest = bytes.get(at..).unwrap_or_default();
    match rest.first().ok_or(Irregular)? {
        _ if depth == 0 => Err(Irregular),
        b'<' if rest.starts_with(b"<<") => {
            let (mut dictionary, mut at) = (Dictionary::new(), at + 2);
            loop {
                at = skip_space(bytes, at);
                if bytes.get(at..).unwrap_or_default().starts_with(b">>") {
                    return Ok((Object::Dictionary(dictionary), at + 2));
                }
                let (Object::Name(key), end) = object_within(bytes, at, depth - 1)? else {
                    return Err(Irregular);
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
                if bytes.get(at..).unwrap_or_default().starts_with(b"]") {
                    return Ok((Object::Array(items), at + 1));
                }
                let (item, end) = object_within(bytes, at, depth - 1)?;
                items.push(item);
                at = end;
            }
        }
        b'(' | b'<' => {
            let (end, _) = measure_within(bytes, at, 1)?;
            let raw = bytes.get(at + 1..end - 1).ok_or(Irregular)?.to_vec();
            Ok((Object::String(raw, StringFormat::Literal), end))
        }
        b'/' => {
            let end = skip_regular(bytes, at + 1);
            let name = bytes.get(at + 1..end).ok_or(Irregular)?;
            Ok((Object::Name(decoded(name)), end))
        }
        byte if delimiter(*byte) => Err(Irregular),
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
