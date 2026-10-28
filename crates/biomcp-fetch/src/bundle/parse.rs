use super::{Keys, Row, RowFormat};
use crate::BadReply;
use std::collections::BTreeSet;
pub(super) struct Parsed {
    pub row: String,
    pub keys: Keys,
}
pub(super) fn tsv<R: Row>(body: &[u8]) -> Result<Vec<Parsed>, BadReply> {
    let text = std::str::from_utf8(body).map_err(|_| BadReply)?;
    let mut lines = text.lines();
    let header = lines.next().ok_or(BadReply)?;
    let names: Vec<_> = header.split('\t').collect();
    let unique: BTreeSet<_> = names.iter().collect();
    if names
        .iter()
        .any(|name| name.is_empty() || name.contains('"'))
        || unique.len() != names.len()
    {
        return Err(BadReply);
    }
    let mut rows = Vec::new();
    for line in lines {
        let values: Vec<_> = line.split('\t').collect();
        if values.len() != names.len() {
            return Err(BadReply);
        }
        let mut object = serde_json::Map::new();
        for (name, value) in names.iter().zip(values) {
            let value = if value.starts_with('"') || value.ends_with('"') {
                value
                    .strip_prefix('"')
                    .and_then(|v| v.strip_suffix('"'))
                    .filter(|v| !v.contains('"'))
                    .ok_or(BadReply)?
            } else {
                if value.contains('"') {
                    return Err(BadReply);
                }
                value
            };
            if !value.is_empty() {
                object.insert((*name).to_owned(), value.into());
            }
        }
        let value = serde_json::Value::Object(object);
        let decoded: R = serde_json::from_value(value.clone()).map_err(|_| BadReply)?;
        rows.push(Parsed {
            row: value.to_string(),
            keys: decoded.keys(),
        });
    }
    Ok(rows)
}

pub(super) fn part_rows<R: Row>(
    body: &[u8],
    format: RowFormat,
    part: &str,
) -> Result<Vec<Parsed>, BadReply> {
    let rows = rows::<R>(body, format)?;
    for row in &rows {
        let decoded: R = serde_json::from_str(&row.row).map_err(|_| BadReply)?;
        decoded.check_part(part)?;
    }
    Ok(rows)
}

pub(super) fn rows<R: Row>(body: &[u8], format: RowFormat) -> Result<Vec<Parsed>, BadReply> {
    match format {
        RowFormat::Document { .. } => R::decode_document(body)?
            .into_iter()
            .map(|row| {
                let decoded: R = serde_json::from_str(&row).map_err(|_| BadReply)?;
                decoded.check_bundle(0)?;
                Ok(Parsed {
                    row,
                    keys: decoded.keys(),
                })
            })
            .collect(),
        RowFormat::Json { rows, metadata } => json::<R>(body, rows, metadata),
        RowFormat::NativeTsv {
            gzip,
            max_decoded_bytes,
        } => super::native::rows::<R>(body, gzip, max_decoded_bytes),
        RowFormat::Tsv => tsv::<R>(body),
        RowFormat::Csv { banner_lines } => csv::<R>(body, banner_lines, false),
        RowFormat::NativeCsv { max_bytes } => {
            if max_bytes == 0
                || max_bytes > super::MAX_BUNDLE_BYTES
                || body.len() as u64 > max_bytes
            {
                return Err(BadReply);
            }
            csv::<R>(body, 0, true)
        }
    }
}
fn csv<R: Row>(body: &[u8], banner_lines: usize, native: bool) -> Result<Vec<Parsed>, BadReply> {
    let mut text = std::str::from_utf8(body).map_err(|_| BadReply)?;
    for _ in 0..banner_lines {
        text = text.split_once('\n').ok_or(BadReply)?.1;
    }
    let mut records = records(text)?.into_iter();
    let names = records.next().ok_or(BadReply)?;
    let unique: BTreeSet<_> = names.iter().collect();
    if names.iter().any(String::is_empty) || unique.len() != names.len() {
        return Err(BadReply);
    }
    let mut rows = Vec::new();
    for values in records {
        if banner_lines > 0
            && values
                .iter()
                .all(|v| !v.is_empty() && v.bytes().all(|c| c == b'+'))
        {
            continue;
        }
        if values.len() != names.len() {
            return Err(BadReply);
        }
        let object: serde_json::Map<_, _> = names
            .iter()
            .zip(values)
            .filter(|(_, v)| native || !v.is_empty())
            .map(|(k, v)| (k.clone(), serde_json::Value::String(v)))
            .collect();
        let value = serde_json::Value::Object(object);
        let decoded: R = serde_json::from_value(value.clone()).map_err(|_| BadReply)?;
        rows.push(Parsed {
            row: value.to_string(),
            keys: decoded.keys(),
        });
    }
    Ok(rows)
}
// Quoting is a wire boundary: keep the complete logical-record state machine here.
fn records(text: &str) -> Result<Vec<Vec<String>>, BadReply> {
    let mut chars = text.chars().peekable();
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut cell = String::new();
    let (mut quoted, mut closed, mut started) = (false, false, false);
    while let Some(c) = chars.next() {
        started = true;
        if quoted {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cell.push('"');
                } else {
                    quoted = false;
                    closed = true;
                }
            } else {
                cell.push(c);
            }
            continue;
        }
        match c {
            '"' if cell.is_empty() && !closed => quoted = true,
            ',' => {
                row.push(std::mem::take(&mut cell));
                closed = false;
            }
            '\n' | '\r' => {
                if c == '\r' && chars.next() != Some('\n') {
                    return Err(BadReply);
                }
                row.push(std::mem::take(&mut cell));
                rows.push(std::mem::take(&mut row));
                closed = false;
                started = false;
            }
            '"' => return Err(BadReply),
            _ if closed => return Err(BadReply),
            _ => cell.push(c),
        }
    }
    if quoted {
        return Err(BadReply);
    }
    if started {
        row.push(cell);
        rows.push(row);
    }
    Ok(rows)
}

fn json<R: Row>(body: &[u8], field: &str, metadata: &str) -> Result<Vec<Parsed>, BadReply> {
    let document: serde_json::Value = serde_json::from_slice(body).map_err(|_| BadReply)?;
    let meta = document
        .get(metadata)
        .filter(|v| v.is_object())
        .ok_or(BadReply)?;
    let values = document
        .get(field)
        .and_then(serde_json::Value::as_array)
        .filter(|rows| !rows.is_empty())
        .ok_or(BadReply)?;
    let mut ids = BTreeSet::new();
    values
        .iter()
        .map(|value| {
            let mut object = value.as_object().ok_or(BadReply)?.clone();
            if object.insert("_bundle_meta".into(), meta.clone()).is_some() {
                return Err(BadReply);
            }
            let value = serde_json::Value::Object(object);
            let decoded: R = serde_json::from_value(value.clone()).map_err(|_| BadReply)?;
            decoded.check_bundle(values.len())?;
            let keys = decoded.keys();
            for (kind, id) in &keys.exact {
                if matches!(kind, super::KeyKind::Id) && !ids.insert(id.to_ascii_lowercase()) {
                    return Err(BadReply);
                }
            }
            Ok(Parsed {
                row: value.to_string(),
                keys,
            })
        })
        .collect()
}
