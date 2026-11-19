//! Bounds on what lopdf expands while it loads a document, checked on the raw bytes first.
//!
//! lopdf decompresses every object stream and cross-reference stream while loading, each
//! against [`LOAD_STREAM_BYTES`], but sets no bound on how many there are, and the time limit
//! cannot interrupt the load. A document naming more than [`OBJECT_STREAMS`] object streams or
//! [`XREF_STREAMS`] cross-reference streams is refused before loading. Names are counted with
//! their `#xx` escapes decoded, as lopdf reads them, so an escaped `/Obj#53tm` still counts.
//! Over 223 FDA labels and letters read on 2026-11-19 the most were 390 object streams, 5
//! cross-reference streams, and 186,751 bytes in one object stream.

/// Most bytes one object or cross-reference stream may expand to while loading.
pub(super) const LOAD_STREAM_BYTES: usize = 1 << 20;
/// Most object streams a document may name.
pub(super) const OBJECT_STREAMS: usize = 1024;
/// Most cross-reference streams a document may name.
pub(super) const XREF_STREAMS: usize = 32;

/// Whether the raw document names no more streams that lopdf expands at load than the bounds.
pub(super) fn loadable(bytes: &[u8]) -> bool {
    let (mut objects, mut xrefs) = (0usize, 0usize);
    let mut at = 0;
    while let Some(slash) = bytes
        .get(at..)
        .and_then(|rest| rest.iter().position(|b| *b == b'/'))
    {
        let start = at + slash + 1;
        let end = bytes
            .get(start..)
            .and_then(|rest| rest.iter().position(|b| delimiter(*b)))
            .map_or(bytes.len(), |length| start + length);
        match decoded(bytes.get(start..end).unwrap_or_default()).as_slice() {
            b"ObjStm" => objects += 1,
            b"XRef" => xrefs += 1,
            _ => {}
        }
        at = end.max(start);
    }
    objects <= OBJECT_STREAMS && xrefs <= XREF_STREAMS
}

/// PDF 32000-1 §7.2.2: whitespace and delimiters end a name.
const fn delimiter(byte: u8) -> bool {
    matches!(
        byte,
        b'\0'
            | b'\t'
            | b'\n'
            | b'\x0c'
            | b'\r'
            | b' '
            | b'('
            | b')'
            | b'<'
            | b'>'
            | b'['
            | b']'
            | b'{'
            | b'}'
            | b'/'
            | b'%'
    )
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
