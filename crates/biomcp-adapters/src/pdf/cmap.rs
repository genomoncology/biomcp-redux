//! Each font's decoding, read once per document, with bounded `/ToUnicode` maps.
//!
//! A page that draws a form many times would otherwise read the form's fonts at every draw.
//! lopdf also builds a reverse table entry for every code a map covers, so one range over
//! every four-byte code would take billions of steps. Before lopdf reads any map, this module
//! counts the codes it covers; a map over [`MAP_CODES`], or past the document's
//! [`DOCUMENT_CODES`] or [`MAP_BYTES`], is not read, and its font decodes as if it had none.
//! A real FDA map covers at most 65,536 codes in about 6 KiB.
//!
//! lopdf parses a map's PostScript header strictly. One PDF writer puts `def` inside the
//! `/CIDSystemInfo` dictionary, so lopdf drops the whole map and decodes each two-byte glyph ID
//! of an `Identity-H` font as a Latin-1 byte: "7KHUH" for "There". Sixteen FDA labels and
//! letters read so, and fifty more had such stretches (1069). The map's code ranges and
//! mappings are well formed, so they are copied in order under a plain header, within the same
//! bounds, and read again; the copy is used only if lopdf accepts it.
//!
//! A composite font decoded without a character map gives glyph IDs, not characters. Its text
//! is unsound, and only unsound text is judged for gibberish (1068).
use super::fonts::{Widths, entry, widths};
use lopdf::{Dictionary, Document, Encoding, Object, Stream};
use std::{collections::BTreeMap, rc::Rc};

/// Most codes one map may cover.
pub(super) const MAP_CODES: u64 = 1 << 17;
/// Most codes the maps of one document may cover together.
pub(super) const DOCUMENT_CODES: u64 = 1 << 21;
/// Most bytes of maps one document may read.
pub(super) const MAP_BYTES: usize = 1 << 20;
/// The bfrange section, whose entries span codes.
const RANGES: (&str, &str) = ("beginbfrange", "endbfrange");
const CHARS: (&str, &str) = ("beginbfchar", "endbfchar");
/// Sections a rebuilt map keeps, in the order the map gives them.
const KEPT: [(&str, &str); 3] = [("begincodespacerange", "endcodespacerange"), CHARS, RANGES];
const HEADER: &str =
    "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CMapName /Rebuilt def\n";
const FOOTER: &str = "endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n";

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
    /// Map bytes read so far.
    pub(super) bytes: usize,
    codes: u64,
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
        let encoding = self.encoding(document, font, limit)?;
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

    /// The encoding lopdf gives, unless the character map it would read is over a bound.
    fn encoding(
        &mut self,
        document: &'a Document,
        font: &'a Dictionary,
        limit: usize,
    ) -> lopdf::Result<Encoding<'a>> {
        let named = match entry(font, b"Encoding") {
            Some(Object::Name(name)) => Some(name.as_slice()),
            Some(_) => Some(&b""[..]),
            None => None,
        };
        let identity = named.is_some_and(|name| name.starts_with(b"Identity-"));
        let map = font
            .get_deref(b"ToUnicode", document)
            .and_then(Object::as_stream);
        // lopdf reads the map only for an Identity font or one without an encoding.
        let (Ok(map), true) = (map, named.is_none() || identity) else {
            return font.get_font_encoding_with_limit(document, limit);
        };
        let raw = map.get_plain_content_with_limit(limit.min(MAP_BYTES))?;
        let codes = covered(&String::from_utf8_lossy(&raw));
        self.bytes = self.bytes.saturating_add(raw.len());
        self.codes = self.codes.saturating_add(codes);
        if codes > MAP_CODES || self.codes > DOCUMENT_CODES || self.bytes > MAP_BYTES {
            return Ok(Encoding::SimpleEncoding(b"WinAnsiEncoding"));
        }
        let encoding = font.get_font_encoding_with_limit(document, limit)?;
        if !identity || matches!(encoding, Encoding::UnicodeMapEncoding(_)) {
            return Ok(encoding);
        }
        let Some(text) = rebuilt(&String::from_utf8_lossy(&raw)) else {
            return Ok(encoding);
        };
        self.bytes = self.bytes.saturating_add(text.len());
        if self.bytes > MAP_BYTES {
            return Ok(encoding);
        }
        let mut copy = font.clone();
        copy.set(
            "ToUnicode",
            Object::Stream(Stream::new(Dictionary::new(), text.into_bytes())),
        );
        match copy.get_font_encoding_with_limit(document, limit)? {
            Encoding::UnicodeMapEncoding(map) => Ok(Encoding::UnicodeMapEncoding(map)),
            _ => Ok(encoding),
        }
    }
}

/// How many codes a map's sections cover: each bfchar entry once, each bfrange every code it
/// spans. One pass.
pub(super) fn covered(text: &str) -> u64 {
    let mut codes = 0u64;
    for (section, ranges) in [(CHARS, false), (RANGES, true)] {
        let mut rest = text;
        while let Some(at) = rest.find(section.0) {
            let body = &rest[at + section.0.len()..];
            let end = body.find(section.1).unwrap_or(body.len());
            codes = codes.saturating_add(entries(&body[..end], ranges));
            rest = &body[end..];
        }
    }
    codes
}

fn entries(body: &str, ranges: bool) -> u64 {
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
    if !ranges {
        return values.len() as u64 / 2;
    }
    values
        .chunks(3)
        .map(|entry| match entry {
            [Some(low), Some(high), _] => high.saturating_sub(*low).saturating_add(1),
            _ => 0,
        })
        .fold(0u64, u64::saturating_add)
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
        let (open, close) = KEPT[which];
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
