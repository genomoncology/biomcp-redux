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
//! lopdf parses a map's PostScript header strictly. One PDF writer puts `def` inside the
//! `/CIDSystemInfo` dictionary, so lopdf drops the whole map and decodes each two-byte glyph ID
//! of an `Identity-H` font as a Latin-1 byte: "7KHUH" for "There". Sixteen FDA labels and
//! letters read so, and fifty more had such stretches (1069). The map's code ranges and
//! mappings are well formed, so they are copied in order under a plain header and read again;
//! the copy is used only if lopdf accepts it. It holds only sections [`bound`] already counted,
//! so it covers no more codes than the map it copies.
//!
//! A composite font decoded without a character map gives glyph IDs, not characters. Its text
//! is unsound, and only unsound text is judged for gibberish (1068).
use super::fonts::{Widths, entry, widths};
use lopdf::{Dictionary, Document, Encoding, Object, ObjectId, Stream};
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
/// Deepest reference chain followed to a map.
const CHAIN: usize = 8;
/// The bfrange section, whose entries span codes.
const RANGES: (&str, &str) = ("beginbfrange", "endbfrange");
const CHARS: (&str, &str) = ("beginbfchar", "endbfchar");
/// Sections a rebuilt map keeps, in the order the map gives them.
const KEPT: [(&str, &str); 3] = [("begincodespacerange", "endcodespacerange"), CHARS, RANGES];
const HEADER: &str =
    "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CMapName /Rebuilt def\n";
const FOOTER: &str = "endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n";

/// Remove every `/ToUnicode` map over the bounds from `document`; how many were removed.
pub(super) fn bound(document: &mut Document) -> usize {
    let mut maps = BTreeSet::new();
    let mut pending: Vec<&Object> = document.objects.values().collect();
    while let Some(object) = pending.pop() {
        match object {
            Object::Dictionary(dictionary) => named(dictionary, document, &mut maps, &mut pending),
            Object::Stream(stream) => named(&stream.dict, document, &mut maps, &mut pending),
            Object::Array(items) => pending.extend(items.iter()),
            _ => {}
        }
    }
    let (mut codes, mut bytes, mut removed) = (0u64, 0usize, 0usize);
    for id in maps {
        let Some(Object::Stream(stream)) = document.objects.get(&id) else {
            continue;
        };
        let size = stream
            .get_plain_content_with_limit(MAP_BYTES)
            .map(|raw| (raw.len(), covered(&String::from_utf8_lossy(&raw))));
        let kept = size.is_ok_and(|(length, count)| {
            let count = count.unwrap_or(u64::MAX);
            codes = codes.saturating_add(count);
            bytes = bytes.saturating_add(length);
            count <= MAP_CODES && codes <= DOCUMENT_CODES && bytes <= MAP_BYTES
        });
        if !kept {
            document.objects.insert(id, Object::Null);
            removed += 1;
        }
    }
    removed
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
        let encoding = match encoding {
            Encoding::UnicodeMapEncoding(_) => encoding,
            refused => rebuild(document, font, limit)?.unwrap_or(refused),
        };
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

/// The decoding of an Identity font whose map lopdf refused, read again from the map's own
/// sections under a plain header; none when there is no such map or lopdf refuses the copy.
fn rebuild<'a>(
    document: &'a Document,
    font: &'a Dictionary,
    limit: usize,
) -> lopdf::Result<Option<Encoding<'a>>> {
    let identity = matches!(entry(font, b"Encoding"), Some(Object::Name(name)) if name.starts_with(b"Identity-"));
    let map = match entry(font, b"ToUnicode") {
        Some(Object::Reference(id)) if identity => document.objects.get(id),
        _ => None,
    };
    let Some(Object::Stream(map)) = map else {
        return Ok(None);
    };
    let raw = map.get_plain_content_with_limit(limit.min(MAP_BYTES))?;
    let Some(text) = rebuilt(&String::from_utf8_lossy(&raw)) else {
        return Ok(None);
    };
    let mut copy = font.clone();
    copy.set(
        "ToUnicode",
        Object::Stream(Stream::new(Dictionary::new(), text.into_bytes())),
    );
    Ok(match copy.get_font_encoding_with_limit(document, limit)? {
        Encoding::UnicodeMapEncoding(map) => Some(Encoding::UnicodeMapEncoding(map)),
        _ => None,
    })
}

/// The map's code-space, bfchar and bfrange sections, in their order, under a header lopdf
/// accepts; none when there are none. One pass.
fn rebuilt(text: &str) -> Option<String> {
    let text = text.replace('\r', "\n");
    let next = |from: usize, open: &str| text[from..].find(open).map(|at| from + at);
    let mut found: Vec<Option<usize>> = KEPT.iter().map(|(open, _)| next(0, open)).collect();
    let mut out = String::from(HEADER);
    let mut from = 0;
    while let Some((which, at)) = found
        .iter()
        .enumerate()
        .filter_map(|(which, at)| at.map(|at| (which, at)))
        .min_by_key(|(_, at)| *at)
    {
        let Some((open, close)) = KEPT.get(which).copied() else {
            break;
        };
        let start = at + open.len();
        let Some(end) = next(start, close) else { break };
        let count = text[from..at]
            .split_whitespace()
            .next_back()
            .unwrap_or_default();
        out.push_str(&format!(
            "{count} {open}\n{}\n{close}\n",
            text[start..end].trim()
        ));
        from = end + close.len();
        for (slot, (open, _)) in found.iter_mut().zip(KEPT) {
            if slot.is_some_and(|at| at < from) {
                *slot = next(from, open);
            }
        }
    }
    (out.len() > HEADER.len()).then(|| out + FOOTER)
}
