//! Bounds on what lopdf expands while it loads a document, checked on the raw bytes first.
//!
//! lopdf decompresses every object stream and cross-reference stream while loading, each
//! against [`LOAD_STREAM_BYTES`], and keeps every object they hold, but sets no bound on how
//! many there are, and the time limit cannot interrupt the load. Before loading, each such
//! stream is found in the raw bytes and expanded here: a plain Flate stream counts what it
//! expands to, and any other filter counts [`LOAD_STREAM_BYTES`]. A document whose streams
//! expand past [`LOAD_BYTES`] together, or that names more than [`OBJECT_STREAMS`] object
//! streams or [`XREF_STREAMS`] cross-reference streams, is refused. Names are read with their
//! `#xx` escapes decoded, as lopdf reads them, so an escaped `/Obj#53tm` still counts.
//! Over 223 FDA labels and letters read on 2026-11-19 the most were 390 object streams, 5
//! cross-reference streams, 186,751 bytes in one object stream and 3,807,424 in all.

/// Most bytes one object or cross-reference stream may expand to while loading.
pub(super) const LOAD_STREAM_BYTES: usize = 1 << 20;
/// Most object streams a document may name.
pub(super) const OBJECT_STREAMS: usize = 1024;
/// Most cross-reference streams a document may name.
pub(super) const XREF_STREAMS: usize = 32;
/// Most bytes all object and cross-reference streams may expand to together.
pub(super) const LOAD_BYTES: usize = 16 << 20;
/// Most bytes between a stream's type and its `stream` keyword.
const DICTIONARY: usize = 4096;
/// Filters other than Flate, whose streams count as [`LOAD_STREAM_BYTES`].
const OTHER_FILTERS: [&[u8]; 7] = [
    b"LZWDecode",
    b"RunLengthDecode",
    b"ASCII85Decode",
    b"ASCIIHexDecode",
    b"BrotliDecode",
    b"CCITTFaxDecode",
    b"Crypt",
];

/// Whether the streams lopdf expands at load stay within the bounds.
pub(super) fn loadable(bytes: &[u8]) -> bool {
    let (mut objects, mut xrefs, mut expanded) = (0usize, 0usize, 0usize);
    // Whether the current object names another filter, how far objects were sought, and how
    // far streams were measured, so each byte is searched about once.
    let (mut other, mut sought, mut measured) = (false, 0usize, 0usize);
    let mut at = 0;
    while let Some(slash) = find(bytes, at, b"/") {
        let start = slash + 1;
        let end = bytes
            .get(start..)
            .and_then(|rest| rest.iter().position(|b| delimiter(*b)))
            .map_or(bytes.len(), |length| start + length);
        while let Some(begun) = find(bytes.get(..slash).unwrap_or_default(), sought, b" obj") {
            other = false;
            sought = begun + 1;
        }
        sought = sought.max(slash.saturating_sub(3));
        let name = decoded(bytes.get(start..end).unwrap_or_default());
        other |= OTHER_FILTERS.contains(&name.as_slice());
        let stream = match name.as_slice() {
            b"ObjStm" => {
                objects += 1;
                true
            }
            b"XRef" => {
                xrefs += 1;
                true
            }
            _ => false,
        };
        if stream && end >= measured {
            let left = LOAD_BYTES.saturating_sub(expanded);
            let (size, until) = size(bytes, end, left);
            expanded = expanded.saturating_add(if other { LOAD_STREAM_BYTES } else { size });
            measured = until;
        }
        if objects > OBJECT_STREAMS || xrefs > XREF_STREAMS || expanded > LOAD_BYTES {
            return false;
        }
        at = end.max(start);
    }
    true
}

/// The first `needle` in `bytes` at or after `from`.
fn find(bytes: &[u8], from: usize, needle: &[u8]) -> Option<usize> {
    bytes
        .get(from..)?
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|at| from + at)
}

/// What the stream after `from` expands to as Flate, up to `left` bytes, and where it ends; a
/// stream that does not inflate, or whose keyword is not within [`DICTIONARY`] bytes, counts
/// [`LOAD_STREAM_BYTES`].
fn size(bytes: &[u8], from: usize, left: usize) -> (usize, usize) {
    let window = bytes
        .get(..from.saturating_add(DICTIONARY).min(bytes.len()))
        .unwrap_or_default();
    let Some(keyword) = find(window, from, b"stream") else {
        return (LOAD_STREAM_BYTES, from);
    };
    let mut start = keyword + b"stream".len();
    start += usize::from(bytes.get(start) == Some(&b'\r'));
    start += usize::from(bytes.get(start) == Some(&b'\n'));
    let end = find(bytes, start, b"endstream").unwrap_or(bytes.len());
    let content = bytes.get(start..end).unwrap_or_default().to_vec();
    let stream = lopdf::Stream::new(lopdf::dictionary! { "Filter" => "FlateDecode" }, content);
    let size = match stream.decompressed_content_with_limit(left.min(LOAD_STREAM_BYTES)) {
        Ok(expanded) if !expanded.is_empty() => expanded.len(),
        _ => LOAD_STREAM_BYTES,
    };
    (size, end)
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
