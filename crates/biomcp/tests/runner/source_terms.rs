//! Required publisher provenance is measured separately from unchanged payload caps.
#[path = "../../src/view/escape.rs"]
pub(super) mod text;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};

fn strip_row(row: &mut Value) -> Result<(), String> {
    let outcome: biomcp_types::SourceOutcome =
        serde_json::from_value(row.clone()).map_err(|e| e.to_string())?;
    let source = outcome.source;
    let terms = biomcp_adapters::registry::terms(source).ok_or("unregistered outcome source")?;
    let object = row
        .as_object_mut()
        .ok_or("source outcome is not an object")?;
    for (field, expected) in [
        ("use", json!(terms.usage)),
        ("terms", json!(terms.name)),
        ("terms_url", json!(terms.url)),
    ] {
        if object.remove(field) != Some(expected) {
            return Err(format!("{source}: missing or changed registry {field}"));
        }
    }
    Ok(())
}

/// Validate every source outcome and remove exactly its three new registry fields.
pub(super) fn comparable(value: &Value, pointer: &str) -> Result<Value, String> {
    let mut result = value.clone();
    if pointer.is_empty() {
        if let Some(rows) = result.get_mut("sources").and_then(Value::as_array_mut) {
            for row in rows {
                strip_row(row)?;
            }
        }
    } else if pointer == "/sources" {
        for row in result.as_array_mut().ok_or("sources is not an array")? {
            strip_row(row)?;
        }
    } else if pointer
        .strip_prefix("/sources/")
        .is_some_and(|p| p.parse::<usize>().is_ok())
    {
        strip_row(&mut result)?;
    }
    Ok(result)
}

pub(super) fn size(value: &Value, pointer: &str) -> Result<usize, String> {
    serde_json::to_vec(&comparable(value, pointer)?)
        .map(|b| b.len())
        .map_err(|e| e.to_string())
}

pub(super) fn raw_size(bytes: &[u8]) -> Result<usize, String> {
    let Ok(value) = serde_json::from_slice::<Value>(bytes) else {
        return Ok(bytes.len());
    };
    let compact = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
    Ok(bytes.len() - (compact - size(&value, "")?))
}

/// Measure only new publisher columns in the source table of the paired JSON view.
pub(super) fn markdown_bytes(text: &str, value: &Value) -> Result<usize, String> {
    comparable(value, "")?;
    let Some(rows) = value
        .get("sources")
        .and_then(Value::as_array)
        .filter(|r| !r.is_empty())
    else {
        return Ok(0);
    };
    let table = text
        .split_once("## Sources\n\n")
        .ok_or("missing source table")?
        .1;
    if !table.starts_with('|') {
        return markdown_object_bytes(table, rows);
    }
    let lines: Vec<_> = table
        .lines()
        .take_while(|line| line.starts_with('|'))
        .collect();
    if lines.len() != rows.len() + 2 {
        return Err("source table lost outcome rows".into());
    }
    let columns: Vec<_> = lines
        .first()
        .ok_or("missing source columns")?
        .split('|')
        .enumerate()
        .filter_map(|(i, cell)| match cell {
            " Terms " => Some((i, "terms")),
            " Terms url " => Some((i, "terms_url")),
            " Use " => Some((i, "use")),
            _ => None,
        })
        .collect();
    if columns.len() != 3
        || columns
            .iter()
            .map(|(_, field)| *field)
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != 3
    {
        return Err("source table lost required publisher columns".into());
    }
    let mut bytes = 0;
    for (number, line) in lines.iter().enumerate() {
        let cells: Vec<_> = line.split('|').collect();
        for (column, field) in &columns {
            let cell = cells.get(*column).ok_or("missing publisher cell")?;
            if number == 1 && *cell != " --- " {
                return Err("changed source separator".into());
            }
            if number > 1 {
                let expected = rows
                    .get(number - 2)
                    .and_then(|r| r.get(*field))
                    .and_then(Value::as_str)
                    .ok_or("missing publisher value")?;
                if *cell
                    != format!(
                        " {} ",
                        if *field == "terms_url" {
                            text::source_link(expected)
                        } else {
                            text::escape(expected)
                        }
                    )
                {
                    return Err("changed Markdown publisher terms".into());
                }
            }
            bytes += cell.len() + 1;
        }
    }
    Ok(bytes)
}
fn markdown_object_bytes(table: &str, rows: &[Value]) -> Result<usize, String> {
    let mut rest = table;
    let mut bytes = 0;
    for row in rows {
        for field in ["terms", "terms_url", "use"] {
            let value = row
                .get(field)
                .and_then(Value::as_str)
                .ok_or("missing publisher value")?;
            let rendered = if field == "terms_url" {
                text::source_link(value)
            } else {
                text::escape(value)
            };
            let line = format!("**{}:** {rendered}\n\n", text::escape(field));
            let (before, after) = rest
                .split_once(&line)
                .ok_or("changed Markdown publisher terms")?;
            if before.contains("**use:**") {
                return Err("source objects lost publisher fields".into());
            }
            bytes += line.len();
            rest = after;
        }
    }
    Ok(bytes)
}
