//! Each font's decoding, read once per document, with a second reading of a `/ToUnicode` map
//! lopdf refuses (1069).
//!
//! lopdf parses a ToUnicode CMap's PostScript header strictly. One PDF writer puts `def` inside
//! the `/CIDSystemInfo` dictionary, so lopdf drops the whole map and decodes each two-byte glyph
//! ID of an `Identity-H` font as a Latin-1 byte: "7KHUH" for "There". Sixteen FDA labels and
//! letters read so, and fifty more had such stretches. The code ranges and mappings themselves
//! are well formed, so they are copied in one pass under a plain header and read again by lopdf;
//! the result is used only if lopdf accepts it. Rebuilt maps share one per-document budget of
//! [`REBUILT_BYTES`], so a document of many hostile maps costs at most that much more parsing.
use super::fonts::{Widths, widths};
use lopdf::{Dictionary, Document, Encoding, Object, Stream};
use std::{collections::BTreeMap, rc::Rc};

/// Most bytes of rebuilt maps one document may parse; a real FDA map is about 6 KiB.
pub(super) const REBUILT_BYTES: usize = 1 << 20;
/// Sections whose code ranges and mappings are copied.
const SECTIONS: [(&str, &str); 3] = [
    ("begincodespacerange", "endcodespacerange"),
    ("beginbfchar", "endbfchar"),
    ("beginbfrange", "endbfrange"),
];
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
    rebuilt: usize,
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
        let read = Rc::new((
            self.encoding(document, font, limit)?,
            widths(document, font),
        ));
        self.fonts.insert(key, Rc::clone(&read));
        Ok(read)
    }

    fn encoding(
        &mut self,
        document: &'a Document,
        font: &'a Dictionary,
        limit: usize,
    ) -> lopdf::Result<Encoding<'a>> {
        let encoding = font.get_font_encoding_with_limit(document, limit)?;
        let identity = matches!(font.get(b"Encoding"), Ok(Object::Name(name)) if name.starts_with(b"Identity-"));
        if !identity || matches!(encoding, Encoding::UnicodeMapEncoding(_)) {
            return Ok(encoding);
        }
        let Ok(map) = font
            .get_deref(b"ToUnicode", document)
            .and_then(Object::as_stream)
        else {
            return Ok(encoding);
        };
        let left = REBUILT_BYTES.saturating_sub(self.rebuilt);
        let Some(text) = rebuilt(&map.get_plain_content_with_limit(limit)?, left) else {
            return Ok(encoding);
        };
        self.rebuilt += text.len();
        let mut copy = font.clone();
        copy.set(
            "ToUnicode",
            Object::Stream(Stream::new(Dictionary::new(), text)),
        );
        match copy.get_font_encoding_with_limit(document, limit)? {
            Encoding::UnicodeMapEncoding(map) => Ok(Encoding::UnicodeMapEncoding(map)),
            _ => Ok(encoding),
        }
    }
}

/// The map's code-space, bfchar and bfrange sections, in their order, under a header lopdf
/// accepts; none when there are none or the copy would pass `budget` bytes. One pass.
pub(super) fn rebuilt(raw: &[u8], budget: usize) -> Option<Vec<u8>> {
    let text = String::from_utf8_lossy(raw).replace('\r', "\n");
    let next = |from: usize, open: &str| text[from..].find(open).map(|at| from + at);
    let mut found: Vec<Option<usize>> = SECTIONS.iter().map(|(open, _)| next(0, open)).collect();
    let mut out = String::from(HEADER);
    let mut from = 0;
    while let Some((which, at)) = found
        .iter()
        .enumerate()
        .filter_map(|(which, at)| at.map(|at| (which, at)))
        .min_by_key(|(_, at)| *at)
    {
        let (open, close) = SECTIONS[which];
        let start = at + open.len();
        let end = next(start, close)?;
        let before = text[from..at].trim_end();
        let count = &before[before
            .rfind(char::is_whitespace)
            .map_or(0, |space| space + 1)..];
        let body = text[start..end].trim();
        let grown = out.len() + count.len() + open.len() + body.len() + close.len() + 3;
        if grown + FOOTER.len() > budget {
            return None;
        }
        out.push_str(&format!("{count} {open}\n{body}\n{close}\n"));
        from = end + close.len();
        for (slot, (open, _)) in found.iter_mut().zip(SECTIONS) {
            if slot.is_some_and(|at| at < from) {
                *slot = next(from, open);
            }
        }
    }
    (out.len() > HEADER.len()).then(|| [out.as_str(), FOOTER].concat().into_bytes())
}
