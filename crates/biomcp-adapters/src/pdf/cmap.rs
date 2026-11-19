//! Each font's decoding, read once per document, and the bound on `/ToUnicode` maps.
//!
//! A page that draws a form many times would otherwise read the form's fonts at every draw.
//! lopdf also builds a reverse table entry for every code a map covers, so one range over
//! every four-byte code would take billions of steps and gigabytes, inside lopdf where the
//! time limit cannot stop it. Which maps lopdf reads depends on its own encoding rules, which
//! two reviews got around (1068). So [`bound`] counts every `/ToUnicode` stream the loaded
//! document names, whatever refers to it, before any page is read, and removes each map over
//! [`MAP_CODES`] codes or not plainly written, and every map once the document's maps pass
//! [`DOCUMENT_CODES`] codes or [`MAP_BYTES`] bytes. lopdf can then read only bounded maps. A
//! font whose map was removed decodes as it would without one.
//!
//! A composite font decoded without a character map gives glyph IDs, not characters. Its text
//! is unsound, and only unsound text is judged for gibberish (1068).
use super::fonts::{Widths, widths};
use biomcp_types::Reason;
use lopdf::{Dictionary, Document, Encoding, Object, ObjectId};
use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

/// Most codes one map may cover; the largest map in 164 FDA labels and letters read on
/// 2026-11-19 covered 65,536.
pub(super) const MAP_CODES: u64 = 1 << 17;
/// Most codes the maps of one document may cover together; lopdf keeps about 80 bytes for
/// each. The most in one FDA document was 983,552.
pub(super) const DOCUMENT_CODES: u64 = 1 << 21;
/// Most bytes of maps one document may hold; the most in one FDA document was 1,402,222.
pub(super) const MAP_BYTES: usize = 4 << 20;
/// Objects or maps handled between checks of the time limit.
const CHECK_EVERY: usize = 1024;
/// Deepest reference chain followed to a map.
const CHAIN: usize = 8;
/// The bfrange section, whose entries span codes.
const RANGES: (&str, &str) = ("beginbfrange", "endbfrange");
const CHARS: (&str, &str) = ("beginbfchar", "endbfchar");

/// Remove every `/ToUnicode` map over the bounds from `document`, and how many were removed;
/// `Err` once `expired` says the time limit passed. Each map expands to at most the bytes the
/// document has left, and once the budget is spent every remaining map is removed without
/// being expanded, so the pass expands at most [`MAP_BYTES`] in all (1068 third review).
pub(super) fn bound(document: &mut Document, expired: &dyn Fn() -> bool) -> Result<usize, Reason> {
    let mut maps = BTreeSet::new();
    let mut pending: Vec<&Object> = document.objects.values().collect();
    let mut walked = 0usize;
    while let Some(object) = pending.pop() {
        walked += 1;
        if walked.is_multiple_of(CHECK_EVERY) && expired() {
            return Err(Reason::DeadlineExceeded);
        }
        match object {
            Object::Dictionary(dictionary) => named(dictionary, document, &mut maps, &mut pending),
            Object::Stream(stream) => named(&stream.dict, document, &mut maps, &mut pending),
            Object::Array(items) => pending.extend(items.iter()),
            _ => {}
        }
    }
    let (mut codes, mut bytes, mut removed, mut spent) = (0u64, 0usize, 0usize, false);
    for (count, id) in maps.into_iter().enumerate() {
        if count.is_multiple_of(CHECK_EVERY) && expired() {
            return Err(Reason::DeadlineExceeded);
        }
        let Some(Object::Stream(stream)) = document.objects.get(&id) else {
            continue;
        };
        let expanded =
            (!spent).then(|| stream.get_plain_content_with_limit(MAP_BYTES.saturating_sub(bytes)));
        let kept = match expanded {
            Some(Ok(raw)) => {
                let count = covered(&String::from_utf8_lossy(&raw)).unwrap_or(u64::MAX);
                codes = codes.saturating_add(count);
                bytes = bytes.saturating_add(raw.len());
                spent |= codes > DOCUMENT_CODES;
                count <= MAP_CODES && !spent
            }
            // A map that fails may have expanded up to the bytes left, so it spends them.
            _ => {
                spent = true;
                false
            }
        };
        if !kept {
            document.objects.insert(id, Object::Null);
            removed += 1;
        }
    }
    Ok(removed)
}

/// Note the stream `dictionary`'s `/ToUnicode` names, and queue its values to search.
fn named<'d>(
    dictionary: &'d Dictionary,
    document: &Document,
    maps: &mut BTreeSet<ObjectId>,
    pending: &mut Vec<&'d Object>,
) {
    for (key, value) in dictionary.iter() {
        if key.as_slice() == b"ToUnicode" {
            let mut target = value;
            for _ in 0..CHAIN {
                let Object::Reference(id) = target else { break };
                maps.insert(*id);
                match document.objects.get(id) {
                    Some(next) => target = next,
                    None => break,
                }
            }
        }
        pending.push(value);
    }
}

/// A font's decoding, glyph widths, and whether its decoding gives characters.
pub(super) struct Font<'a> {
    pub(super) encoding: Encoding<'a>,
    pub(super) widths: Option<Widths>,
    pub(super) sound: bool,
}

/// Fonts already read in one document, by the address of their dictionary, which the loaded
/// document keeps in place; a font shared by every page or form draw is read once.
#[derive(Default)]
pub(super) struct Cache<'a> {
    fonts: BTreeMap<usize, Rc<Font<'a>>>,
}

impl<'a> Cache<'a> {
    /// The decoding of `font`, its streams expanded to at most `limit` bytes.
    pub(super) fn font(
        &mut self,
        document: &'a Document,
        font: &'a Dictionary,
        limit: usize,
    ) -> lopdf::Result<Rc<Font<'a>>> {
        let key = std::ptr::from_ref(font).addr();
        if let Some(known) = self.fonts.get(&key) {
            return Ok(Rc::clone(known));
        }
        let composite = matches!(font.get(b"Subtype"), Ok(Object::Name(name)) if name == b"Type0");
        // [`bound`] already removed every map over the bounds.
        let encoding = font.get_font_encoding_with_limit(document, limit)?;
        // lopdf falls back to a one-byte table, and a withheld map to WinAnsi.
        let fallback = matches!(
            encoding,
            Encoding::OneByteEncoding(_) | Encoding::SimpleEncoding(b"WinAnsiEncoding")
        );
        let read = Rc::new(Font {
            sound: !(composite && fallback),
            widths: widths(document, font),
            encoding,
        });
        self.fonts.insert(key, Rc::clone(&read));
        Ok(read)
    }
}

/// How many codes a map's sections cover: each bfchar entry once, each bfrange every code it
/// spans; none when a section is not regular, so it is never read. One pass.
pub(super) fn covered(text: &str) -> Option<u64> {
    let mut codes = 0u64;
    for (section, ranges) in [(CHARS, false), (RANGES, true)] {
        let mut rest = text;
        while let Some(at) = rest.find(section.0) {
            let body = &rest[at + section.0.len()..];
            let end = body.find(section.1).unwrap_or(body.len());
            codes = codes.saturating_add(entries(&body[..end], ranges)?);
            rest = &body[end..];
        }
    }
    Some(codes)
}

fn entries(body: &str, ranges: bool) -> Option<u64> {
    let mut values = Vec::new();
    let mut array = false;
    for token in body.split(['<', '>']).map(str::trim) {
        if let Some(rest) = token.strip_prefix('[') {
            values.push(None);
            array = !rest.contains(']');
        } else if token.contains(']') {
            array = false;
        } else if !token.is_empty() && !array {
            values.push(hex(token));
        }
    }
    let width = if ranges { 3 } else { 2 };
    if !values.len().is_multiple_of(width) {
        return None;
    }
    let mut codes = 0u64;
    for entry in values.chunks(width) {
        codes = codes.saturating_add(match entry {
            [Some(low), Some(high), _] => high.checked_sub(*low)?.saturating_add(1),
            [Some(_), _] => 1,
            _ => return None,
        });
    }
    Some(codes)
}

/// A hexadecimal code, whitespace ignored; none when a character is not a hex digit.
fn hex(token: &str) -> Option<u64> {
    token
        .chars()
        .filter(|c| !c.is_whitespace())
        .try_fold(0u64, |value, c| {
            Some(
                value
                    .saturating_mul(16)
                    .saturating_add(u64::from(c.to_digit(16)?)),
            )
        })
}
