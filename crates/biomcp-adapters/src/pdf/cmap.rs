//! Each font's decoding, read once per document, with bounded `/ToUnicode` maps and a second
//! reading of a map lopdf refuses (1069).
//!
//! lopdf parses a ToUnicode CMap's PostScript header strictly. One PDF writer puts `def` inside
//! the `/CIDSystemInfo` dictionary, so lopdf drops the whole map and decodes each two-byte glyph
//! ID of an `Identity-H` font as a Latin-1 byte: "7KHUH" for "There". Sixteen FDA labels and
//! letters read so, and fifty more had such stretches. The code ranges and mappings themselves
//! are well formed, so they are copied in one pass under a plain header and read again by lopdf;
//! the result is used only if lopdf accepts it.
//!
//! lopdf also builds a reverse table entry for every code a map covers, so one range over every
//! four-byte code would take billions of steps. Before lopdf reads any map, this module counts
//! the codes it covers; a map over [`MAP_CODES`], or past the document's [`DOCUMENT_CODES`] or
//! [`MAP_BYTES`], is not read, and its font decodes as if it had none. A real FDA map covers at
//! most 65,536 codes in about 6 KiB.
use super::fonts::{Widths, widths};
use lopdf::{Dictionary, Document, Encoding, Object, Stream};
use std::{collections::BTreeMap, rc::Rc};

/// Most codes one map may cover.
pub(super) const MAP_CODES: u64 = 1 << 17;
/// Most codes the maps of one document may cover together.
pub(super) const DOCUMENT_CODES: u64 = 1 << 21;
/// Most bytes of maps, as found and as rebuilt, one document may read.
pub(super) const MAP_BYTES: usize = 1 << 20;
/// Sections whose code ranges and mappings are kept.
const SECTIONS: [(&str, &str); 3] = [
    ("begincodespacerange", "endcodespacerange"),
    ("beginbfchar", "endbfchar"),
    ("beginbfrange", "endbfrange"),
];
const RANGES: usize = 2;
const HEADER: &str =
    "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CMapName /Rebuilt def\n";
const FOOTER: &str = "endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n";

/// A font's decoding and glyph widths.
pub(super) type Font<'a> = Rc<(Encoding<'a>, Option<Widths>)>;

/// Fonts already read in one document, by the address of their dictionary, which the loaded
/// document keeps in place; a font shared by every page is read once.
#[derive(Default)]
pub(super) struct Cache<'a> {
    fonts: BTreeMap<usize, Font<'a>>,
    bytes: usize,
    codes: u64,
}

/// One kept section: which of [`SECTIONS`], its entry count as written, and its entries.
struct Section {
    which: usize,
    count: String,
    body: String,
}

impl<'a> Cache<'a> {
    /// The decoding of `font`, its streams expanded to at most `limit` bytes.
    pub(super) fn font(
        &mut self,
        document: &'a Document,
        font: &'a Dictionary,
        limit: usize,
    ) -> lopdf::Result<Font<'a>> {
        let key = std::ptr::from_ref(font).addr();
        if let Some(known) = self.fonts.get(&key) {
            return Ok(Rc::clone(known));
        }
        let read = Rc::new((self.encoding(document, font, limit)?, widths(document, font)));
        self.fonts.insert(key, Rc::clone(&read));
        Ok(read)
    }

    fn encoding(
        &mut self,
        document: &'a Document,
        font: &'a Dictionary,
        limit: usize,
    ) -> lopdf::Result<Encoding<'a>> {
        let named = match font.get(b"Encoding") {
            Ok(Object::Name(name)) => Some(name.as_slice()),
            Ok(_) => Some(&b""[..]),
            Err(_) => None,
        };
        let identity = named.is_some_and(|name| name.starts_with(b"Identity-"));
        let Ok(map) = font.get_deref(b"ToUnicode", document).and_then(Object::as_stream) else {
            return font.get_font_encoding_with_limit(document, limit);
        };
        // lopdf reads the map only for an Identity font or one without an encoding.
        if named.is_some() && !identity {
            return font.get_font_encoding_with_limit(document, limit);
        }
        let raw = map.get_plain_content_with_limit(limit)?;
        let parts = sections(&raw);
        let codes = covered(&parts);
        self.bytes = self.bytes.saturating_add(raw.len());
        self.codes = self.codes.saturating_add(codes);
        if codes > MAP_CODES || self.codes > DOCUMENT_CODES || self.bytes > MAP_BYTES {
            return Ok(Encoding::SimpleEncoding(b"WinAnsiEncoding"));
        }
        let encoding = font.get_font_encoding_with_limit(document, limit)?;
        if !identity || matches!(encoding, Encoding::UnicodeMapEncoding(_)) {
            return Ok(encoding);
        }
        let Some(text) = rebuilt(&parts) else {
            return Ok(encoding);
        };
        self.bytes = self.bytes.saturating_add(text.len());
        if self.bytes > MAP_BYTES {
            return Ok(encoding);
        }
        let mut copy = font.clone();
        copy.set("ToUnicode", Object::Stream(Stream::new(Dictionary::new(), text)));
        match copy.get_font_encoding_with_limit(document, limit)? {
            Encoding::UnicodeMapEncoding(map) => Ok(Encoding::UnicodeMapEncoding(map)),
            _ => Ok(encoding),
        }
    }
}

/// The map's code-space, bfchar and bfrange sections, in their order, in one pass.
fn sections(raw: &[u8]) -> Vec<Section> {
    let text = String::from_utf8_lossy(raw).replace('\r', "\n");
    let next = |from: usize, open: &str| text[from..].find(open).map(|at| from + at);
    let mut found: Vec<Option<usize>> = SECTIONS.iter().map(|(open, _)| next(0, open)).collect();
    let mut kept = Vec::new();
    let mut from = 0;
    while let Some((which, at)) = found
        .iter()
        .enumerate()
        .filter_map(|(which, at)| at.map(|at| (which, at)))
        .min_by_key(|(_, at)| *at)
    {
        let (open, close) = SECTIONS[which];
        let start = at + open.len();
        let Some(end) = next(start, close) else { break };
        let before = text[from..at].trim_end();
        let count = &before[before.rfind(char::is_whitespace).map_or(0, |space| space + 1)..];
        kept.push(Section {
            which,
            count: count.to_owned(),
            body: text[start..end].trim().to_owned(),
        });
        from = end + close.len();
        for (slot, (open, _)) in found.iter_mut().zip(SECTIONS) {
            if slot.is_some_and(|at| at < from) {
                *slot = next(from, open);
            }
        }
    }
    kept
}

/// How many codes the sections map: each bfchar entry once, each bfrange every code it spans.
fn covered(parts: &[Section]) -> u64 {
    let mut codes = 0u64;
    for part in parts {
        let mut values = Vec::new();
        let mut array = false;
        for token in part.body.split(['<', '>']).map(str::trim) {
            if let Some(rest) = token.strip_prefix('[') {
                array = true;
                values.push(None);
                if rest.contains(']') {
                    array = false;
                }
            } else if token.contains(']') {
                array = false;
            } else if !token.is_empty() && !array {
                values.push(u64::from_str_radix(&token.replace(char::is_whitespace, ""), 16).ok());
            }
        }
        codes = codes.saturating_add(if part.which == RANGES {
            values
                .chunks(3)
                .map(|entry| match entry {
                    [Some(low), Some(high), _] => high.saturating_sub(*low).saturating_add(1),
                    _ => 0,
                })
                .fold(0u64, u64::saturating_add)
        } else {
            values.len() as u64 / 2
        });
    }
    codes
}

/// The sections under a header lopdf accepts; none when there are none.
fn rebuilt(parts: &[Section]) -> Option<Vec<u8>> {
    let mut out = String::from(HEADER);
    for part in parts {
        let (open, close) = SECTIONS[part.which];
        out.push_str(&format!("{} {open}\n{}\n{close}\n", part.count, part.body));
    }
    out.push_str(FOOTER);
    (!parts.is_empty()).then(|| out.into_bytes())
}
