//! What lopdf builds while it loads a document, measured on the raw bytes before it loads.
//!
//! lopdf parses every object the cross-reference sections list, at the offset listed, and
//! expands every object stream with every object its index lists, keeping them all; nothing
//! interrupts a load. It does not check that entries point at distinct objects: 200 entries at
//! one 4 MB array took 17 s and 50 GB, and an object stream whose index lists one array 100
//! times took 3 GB from 1,271 bytes (1068). So before loading, [`loadable`] reads every section
//! (see [`super::xref`]) and measures every object listed: each must end before the next listed
//! offset, a stream's data must run exactly its `/Length`, and an object stream's index must
//! list its objects in order, each ending before the next. Then each byte is parsed once, and
//! the document is refused when it holds more than [`OBJECTS`] objects, counting those inside
//! arrays and dictionaries, or its object and cross-reference streams expand past
//! [`LOAD_BYTES`] together. The load is strict, so lopdf repairs nothing: a repair reads
//! offsets not measured here. All 164 FDA labels and letters read on 2026-11-19 load strictly
//! as they load leniently, and read the same text.

use super::fonts::entry;
use super::lexer::{Read, header, integer, measure, skip_space};
use super::xref::{Table, index, payload, small_dictionary, whole};
use Refusal::{Irregular, TooLarge};
use lopdf::{Dictionary, Object, Stream};

/// Most bytes one object or cross-reference stream may expand to while loading.
pub(super) const LOAD_STREAM_BYTES: usize = 1 << 20;
/// Most bytes all object and cross-reference streams may expand to together.
pub(super) const LOAD_BYTES: usize = 16 << 20;
/// Most objects a document may hold, those inside arrays and dictionaries included; lopdf
/// keeps about 120 bytes for each. The most in one FDA document was 588,311.
pub(super) const OBJECTS: usize = 1 << 20;
/// Longest object a stream's `/Length` may refer to.
const LENGTH_OBJECT: usize = 64;

/// Why a document is not loaded.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Refusal {
    /// It holds more than the bounds allow.
    TooLarge,
    /// Its objects overlap or its sections do not read, so lopdf's work cannot be measured.
    Irregular,
}

/// Whether lopdf may load `bytes` within the bounds.
pub(super) fn loadable(bytes: &[u8]) -> Read<()> {
    let table = super::xref::read(bytes, LOAD_STREAM_BYTES)?;
    let mut listed: Vec<(usize, u64)> = table.entries.iter().map(|(n, at)| (*at, *n)).collect();
    listed.sort_unstable();
    listed.dedup();
    let mut measured = Measured {
        objects: listed.len(),
        expanded: table.expanded,
    };
    for (index, (offset, _)) in listed.iter().enumerate() {
        let next = listed.get(index + 1).map_or(bytes.len(), |(at, _)| *at);
        let end = indirect(bytes, *offset, &table, &mut measured)?;
        if end > next {
            return Err(Irregular);
        }
        if measured.objects > OBJECTS || measured.expanded > LOAD_BYTES {
            return Err(TooLarge);
        }
    }
    Ok(())
}

/// What the objects measured so far hold.
struct Measured {
    objects: usize,
    expanded: usize,
}

/// Where the indirect object at `offset` ends, its stream data and object stream included.
fn indirect(bytes: &[u8], offset: usize, table: &Table, measured: &mut Measured) -> Read<usize> {
    let (_, body) = header(bytes, offset)?;
    let (end, count) = measure(bytes, body)?;
    measured.objects = measured.objects.saturating_add(count);
    let starts_dictionary = bytes
        .get(skip_space(bytes, body)..)
        .is_some_and(|rest| rest.starts_with(b"<<"));
    let Some(data) = payload(bytes, end).filter(|_| starts_dictionary) else {
        return Ok(end);
    };
    let (dictionary, _) = small_dictionary(bytes, body)?;
    let data_end = data
        .checked_add(length(bytes, &dictionary, table)?)
        .ok_or(Irregular)?;
    let mut after = data_end;
    match bytes.get(after..).unwrap_or_default() {
        [b'\r', b'\n', ..] => after += 2,
        [b'\n' | b'\r', ..] => after += 1,
        _ => {}
    }
    if !bytes
        .get(after..)
        .unwrap_or_default()
        .starts_with(b"endstream")
    {
        return Err(Irregular);
    }
    if dictionary.has_type(b"ObjStm") {
        let raw = bytes.get(data..data_end).ok_or(Irregular)?;
        object_stream(dictionary, raw.to_vec(), measured)?;
    }
    Ok(after + b"endstream".len())
}

/// A stream's `/Length`, direct or held in a short integer object that every section agrees on.
fn length(bytes: &[u8], dictionary: &Dictionary, table: &Table) -> Read<usize> {
    let number = match entry(dictionary, b"Length") {
        Some(Object::Reference((number, _))) => u64::from(*number),
        direct => return index(whole(direct)?),
    };
    let mut values = table
        .entries
        .iter()
        .filter(|(listed, _)| *listed == number)
        .map(|(_, offset)| {
            let (_, body) = header(bytes, *offset)?;
            let at = skip_space(bytes, body);
            let (value, end) = integer(bytes, at)?;
            if end - at <= LENGTH_OBJECT {
                Ok(value)
            } else {
                Err(Irregular)
            }
        });
    let first = values.next().ok_or(Irregular)??;
    for value in values {
        if value? != first {
            return Err(Irregular);
        }
    }
    index(first)
}

/// Check an object stream's index and count its objects (PDF 32000-1 §7.5.7).
fn object_stream(dictionary: Dictionary, raw: Vec<u8>, measured: &mut Measured) -> Read<()> {
    let first = index(whole(entry(&dictionary, b"First"))?)?;
    let content = Stream::new(dictionary, raw)
        .get_plain_content_with_limit(LOAD_STREAM_BYTES)
        .map_err(|issue| {
            if super::page::too_large(&issue) {
                TooLarge
            } else {
                Irregular
            }
        })?;
    measured.expanded = measured.expanded.saturating_add(content.len());
    let listing = content.get(..first).ok_or(Irregular)?;
    let mut starts = Vec::new();
    let (mut at, mut previous) = (skip_space(listing, 0), None);
    while at < listing.len() {
        let (_, next) = integer(listing, at)?;
        let (offset, next) = integer(listing, skip_space(listing, next))?;
        let offset = index(offset)?;
        if previous.is_some_and(|previous| offset <= previous) {
            return Err(Irregular);
        }
        previous = Some(offset);
        starts.push(first.checked_add(offset).ok_or(Irregular)?);
        at = skip_space(listing, next);
    }
    measured.objects = measured.objects.saturating_add(starts.len());
    for (position, start) in starts.iter().enumerate() {
        let next = starts.get(position + 1).copied().unwrap_or(content.len());
        let (end, count) = measure(&content, *start)?;
        if end > next {
            return Err(Irregular);
        }
        measured.objects = measured.objects.saturating_add(count);
    }
    Ok(())
}
