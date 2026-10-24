use super::{Keys, Row};
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
