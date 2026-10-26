//! Check ZIP layout before zip allocates, then actual XML before calamine allocates.
use super::{LIMITS, check};
use biomcp_fetch::LocalDeadline;
use biomcp_types::Reason;
use std::io::{Cursor, Read};
fn number(bytes: &[u8], at: usize, len: usize) -> Result<usize, Reason> {
    let mut n = 0usize;
    for (shift, b) in bytes
        .get(at..at + len)
        .ok_or(Reason::BadReply)?
        .iter()
        .enumerate()
    {
        n |= usize::from(*b) << (shift * 8);
    }
    Ok(n)
}
fn signature(bytes: &[u8], at: usize, expected: &[u8]) -> bool {
    bytes.get(at..at + expected.len()) == Some(expected)
}
pub(super) fn check_zip(bytes: &[u8], end: &LocalDeadline) -> Result<(), Reason> {
    let eocd = (bytes.len().saturating_sub(65557)..bytes.len().saturating_sub(21))
        .rev()
        .find(|at| {
            signature(bytes, *at, b"PK\x05\x06")
                && number(bytes, *at + 20, 2).is_ok_and(|n| *at + 22 + n == bytes.len())
        })
        .ok_or(Reason::BadReply)?;
    if number(bytes, eocd + 4, 2)? != 0 || number(bytes, eocd + 6, 2)? != 0 {
        return Err(Reason::Unsupported);
    }
    let count = number(bytes, eocd + 10, 2)?;
    let size = number(bytes, eocd + 12, 4)?;
    let offset = number(bytes, eocd + 16, 4)?;
    if count == 65535 || size == u32::MAX as usize || offset == u32::MAX as usize {
        return Err(Reason::Unsupported);
    }
    if count > LIMITS.entries || size > LIMITS.directory {
        return Err(Reason::TooLarge);
    }
    if count != number(bytes, eocd + 8, 2)? || offset.checked_add(size) != Some(eocd) {
        return Err(Reason::Unsupported);
    }
    let mut pos = offset;
    for _ in 0..count {
        check(end)?;
        if !signature(bytes, pos, b"PK\x01\x02") {
            return Err(Reason::BadReply);
        }
        let flags = number(bytes, pos + 8, 2)?;
        let method = number(bytes, pos + 10, 2)?;
        let compressed = number(bytes, pos + 20, 4)?;
        let expanded = number(bytes, pos + 24, 4)?;
        let local = number(bytes, pos + 42, 4)?;
        let name = number(bytes, pos + 28, 2)?;
        let extra = number(bytes, pos + 30, 2)?;
        let comment = number(bytes, pos + 32, 2)?;
        if flags & 0x41 != 0
            || !matches!(method, 0 | 8)
            || number(bytes, pos + 34, 2)? != 0
            || [compressed, expanded, local].contains(&(u32::MAX as usize))
        {
            return Err(Reason::Unsupported);
        }
        if !signature(bytes, local, b"PK\x03\x04")
            || number(bytes, local + 6, 2)? != flags
            || number(bytes, local + 8, 2)? != method
        {
            return Err(Reason::BadReply);
        }
        let data = local
            .checked_add(30 + number(bytes, local + 26, 2)? + number(bytes, local + 28, 2)?)
            .ok_or(Reason::BadReply)?;
        if data.checked_add(compressed).is_none_or(|n| n > offset) {
            return Err(Reason::BadReply);
        }
        let mut x = pos + 46 + name;
        let extra_end = x.checked_add(extra).ok_or(Reason::BadReply)?;
        while x < extra_end {
            if number(bytes, x, 2)? == 1 {
                return Err(Reason::Unsupported);
            }
            x = x
                .checked_add(4 + number(bytes, x + 2, 2)?)
                .ok_or(Reason::BadReply)?;
        }
        if x != extra_end {
            return Err(Reason::BadReply);
        }
        pos = extra_end.checked_add(comment).ok_or(Reason::BadReply)?;
        if pos > eocd {
            return Err(Reason::BadReply);
        }
    }
    if pos != eocd {
        return Err(Reason::BadReply);
    }
    Ok(())
}
pub(super) fn xml(bytes: &[u8], end: &LocalDeadline) -> Result<(), Reason> {
    check_zip(bytes, end)?;
    let mut zip = zip::ZipArchive::new(Cursor::new(bytes)).map_err(|_| Reason::BadReply)?;
    let mut total = 0usize;
    for index in 0..zip.len() {
        check(end)?;
        let mut entry = zip.by_index(index).map_err(|_| Reason::BadReply)?;
        let name = entry
            .name()
            .map_err(|_| Reason::BadReply)?
            .to_ascii_lowercase();
        if !name.ends_with(".xml") && !name.ends_with(".rels") {
            continue;
        }
        let mut expanded = Vec::new();
        let mut chunk = [0u8; 8192];
        loop {
            check(end)?;
            let n = entry.read(&mut chunk).map_err(|_| Reason::BadReply)?;
            if n == 0 {
                break;
            }
            total = total.checked_add(n).ok_or(Reason::TooLarge)?;
            if expanded.len() + n > LIMITS.xml || total > LIMITS.expanded {
                return Err(Reason::TooLarge);
            }
            expanded.extend_from_slice(chunk.get(..n).ok_or(Reason::BadReply)?);
        }
        validate_xml(&expanded, end)?;
    }
    Ok(())
}
fn validate_xml(bytes: &[u8], end: &LocalDeadline) -> Result<(), Reason> {
    use quick_xml::events::Event;
    let text = std::str::from_utf8(bytes).map_err(|_| Reason::BadReply)?;
    let mut reader = quick_xml::Reader::from_str(text);
    reader.config_mut().enable_all_checks(true);
    let (mut depth, mut strings, mut cells, mut text_size) = (0usize, 0usize, 0usize, 0usize);
    loop {
        check(end)?;
        let event = reader.read_event().map_err(|_| Reason::BadReply)?;
        let empty = matches!(event, Event::Empty(_));
        match event {
            Event::Start(tag) | Event::Empty(tag) => {
                if depth >= 64 {
                    return Err(Reason::TooLarge);
                }
                if tag.local_name().as_ref() == "Relationship" {
                    worksheet_layout(&tag)?;
                }
                match tag.local_name().as_ref() {
                    "si" => {
                        strings += 1;
                        text_size = 0
                    }
                    "c" => {
                        cells += 1;
                        text_size = 0
                    }
                    _ => {}
                }
                if strings > LIMITS.strings || cells > LIMITS.cells {
                    return Err(Reason::TooLarge);
                }
                for attr in tag.attributes() {
                    let attr = attr.map_err(|_| Reason::BadReply)?;
                    if tag.local_name().as_ref() == "sst"
                        && matches!(attr.key.as_ref(), "count" | "uniqueCount")
                    {
                        let n: usize = attr.value.parse().map_err(|_| Reason::BadReply)?;
                        let cap = if attr.key.as_ref() == "uniqueCount" {
                            LIMITS.strings
                        } else {
                            LIMITS.cells
                        };
                        if n > cap {
                            return Err(Reason::TooLarge);
                        }
                    }
                    if tag.local_name().as_ref() == "c" && attr.key.as_ref() == "r" {
                        coordinate(attr.value.as_ref())?;
                    }
                }
                if !empty {
                    depth += 1;
                }
            }
            Event::End(_) => depth = depth.checked_sub(1).ok_or(Reason::BadReply)?,
            Event::Text(t) => {
                text_size += t.len();
                if text_size > LIMITS.text {
                    return Err(Reason::TooLarge);
                }
            }
            Event::CData(t) => {
                text_size += t.len();
                if text_size > LIMITS.text {
                    return Err(Reason::TooLarge);
                }
            }
            Event::DocType(_) => return Err(Reason::Unsupported),
            Event::Eof => {
                return if depth == 0 {
                    Ok(())
                } else {
                    Err(Reason::BadReply)
                };
            }
            _ => {}
        }
    }
}
fn coordinate(s: &str) -> Result<(), Reason> {
    let first = s
        .find(|c: char| c.is_ascii_digit())
        .ok_or(Reason::BadReply)?;
    let col = s.get(..first).ok_or(Reason::BadReply)?;
    let row: u32 = s
        .get(first..)
        .ok_or(Reason::BadReply)?
        .parse()
        .map_err(|_| Reason::BadReply)?;
    if col.is_empty() || !col.bytes().all(|c| c.is_ascii_uppercase()) || row == 0 {
        return Err(Reason::BadReply);
    }
    if col.len() > 3 || col.len() == 3 && col > "XFD" || row > 1048576 {
        return Err(Reason::TooLarge);
    }
    Ok(())
}

fn worksheet_layout(tag: &quick_xml::events::BytesStart<'_>) -> Result<(), Reason> {
    let attribute = |name: &str| {
        tag.try_get_attribute(name)
            .map_err(|_| Reason::BadReply)?
            .map(|a| {
                a.normalized_value(quick_xml::XmlVersion::Implicit1_0)
                    .map(|v| v.into_owned())
                    .map_err(|_| Reason::BadReply)
            })
            .transpose()
    };
    if attribute("Type")?.is_some_and(|v| {
        matches!(
            v.rsplit('/').next(),
            Some("worksheet" | "chartsheet" | "dialogsheet")
        )
    }) && (attribute("TargetMode")?.is_some_and(|v| v.eq_ignore_ascii_case("External"))
        || attribute("Target")?.is_none_or(|v| !v.to_ascii_lowercase().ends_with(".xml")))
    {
        return Err(Reason::Unsupported);
    }
    Ok(())
}
