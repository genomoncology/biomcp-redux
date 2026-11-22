//! Narrow the rows of a finished reply to the chosen names and paths and the keys each
//! row always keeps, so the reply still validates against its schema.
use super::rows::{List, Step};
use serde_json::{Map, Value};
use std::collections::BTreeSet;

/// Narrow every row of one list that knows at least one chosen name or path whole; a list
/// that knows none of them stays whole, even when it shares a path's first name.
pub(super) fn list(value: &mut Value, list: &List, fields: &[String]) {
    let chosen = fields
        .iter()
        .filter(|name| list.row.knows(name))
        .map(String::as_str)
        .collect::<Vec<_>>();
    if chosen.is_empty() {
        return;
    }
    each(value, &list.at, &mut |row| {
        if let Some(row) = row.as_object_mut() {
            object(row, "", &chosen, &list.row.kept);
        }
    });
}

/// Visit each value the steps reach.
fn each(value: &mut Value, at: &[Step], visit: &mut dyn FnMut(&mut Value)) {
    match at.split_first() {
        None => visit(value),
        Some((Step::Key(name), rest)) => {
            if let Some(inner) = value.get_mut(name) {
                each(inner, rest, visit);
            }
        }
        Some((Step::Each, rest)) => {
            for item in value.as_array_mut().into_iter().flatten() {
                each(item, rest, visit);
            }
        }
    }
}

/// Keep a chosen or kept key whole; narrow an object a chosen path goes inside.
fn object(row: &mut Map<String, Value>, prefix: &str, chosen: &[&str], kept: &BTreeSet<String>) {
    row.retain(|name, value| {
        let path = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}.{name}")
        };
        let inside = format!("{path}.");
        if chosen.contains(&path.as_str()) {
            return true;
        }
        if chosen.iter().any(|name| name.starts_with(&inside)) {
            match value {
                Value::Object(inner) => object(inner, &path, chosen, kept),
                Value::Array(items) => {
                    for inner in items.iter_mut().filter_map(Value::as_object_mut) {
                        object(inner, &path, chosen, kept);
                    }
                }
                _ => {}
            }
            return true;
        }
        kept.contains(&path)
    });
}
