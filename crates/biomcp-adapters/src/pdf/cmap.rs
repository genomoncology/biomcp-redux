//! A font's encoding, with a second reading of a `/ToUnicode` map lopdf refuses (1069).
//!
//! Experimental: lopdf parses a ToUnicode CMap's PostScript header strictly. FDA labels made by
//! one PDF writer put `def` inside the `/CIDSystemInfo` dictionary, so lopdf drops the whole map
//! and decodes each two-byte glyph ID as a Latin-1 byte, giving "7KHUH" for "There". The code
//! ranges and mappings themselves are well formed, so they are copied under a plain header and
//! read again by lopdf.
use lopdf::{Dictionary, Document, Encoding, Object, Stream};

/// The encoding lopdf gives `font`, or the one its rebuilt ToUnicode map gives.
pub(super) fn encoding<'a>(
    document: &'a Document,
    font: &'a Dictionary,
    limit: usize,
) -> lopdf::Result<Encoding<'a>> {
    let encoding = font.get_font_encoding_with_limit(document, limit)?;
    let identity =
        matches!(font.get(b"Encoding"), Ok(Object::Name(name)) if name.starts_with(b"Identity-"));
    if !identity || matches!(encoding, Encoding::UnicodeMapEncoding(_)) {
        return Ok(encoding);
    }
    let Ok(map) = font
        .get_deref(b"ToUnicode", document)
        .and_then(Object::as_stream)
    else {
        return Ok(encoding);
    };
    let raw = map.get_plain_content_with_limit(limit)?;
    let mut copy = font.clone();
    copy.set(
        "ToUnicode",
        Object::Stream(Stream::new(Dictionary::new(), rebuilt(&raw))),
    );
    match copy.get_font_encoding_with_limit(document, limit)? {
        Encoding::UnicodeMapEncoding(map) => Ok(Encoding::UnicodeMapEncoding(map)),
        _ => Ok(encoding),
    }
}

/// The map's code-space, bfchar and bfrange sections under a header lopdf accepts.
fn rebuilt(raw: &[u8]) -> Vec<u8> {
    let text = String::from_utf8_lossy(raw).replace('\r', "\n");
    let mut body = String::new();
    for (open, close) in [
        ("begincodespacerange", "endcodespacerange"),
        ("beginbfchar", "endbfchar"),
        ("beginbfrange", "endbfrange"),
    ] {
        let mut rest = text.as_str();
        while let Some(at) = rest.find(open) {
            let count = rest[..at].split_whitespace().next_back().unwrap_or("0");
            let after = &rest[at + open.len()..];
            let Some(end) = after.find(close) else { break };
            body.push_str(&format!(
                "{count} {open}\n{}\n{close}\n",
                after[..end].trim()
            ));
            rest = &after[end + close.len()..];
        }
    }
    format!(
        "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CMapName /Rebuilt def\n{body}endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n"
    )
    .into_bytes()
}
