//! Options a checked call also takes, each with a placeholder to fill in.
//!
//! An option is offered only when the call with a sample value for it still passes the
//! same checked constructors the parser uses, so no per-source list is kept here.
use biomcp_types::Request;
use serde_json::Value;
use std::collections::BTreeMap;

/// Options that only continue a reply, never a caller's choice.
const CONTINUATIONS: &[&str] = &["civic_cursor", "bundle_revision", "next_page", "page", "state"];

/// Sample spellings tried for a text option, in order; the first the call accepts wins.
const TEXT_SAMPLES: &[&str] = &["x", "BRCA1", "1", "NDA000001", "P000001"];

/// Each value option the request does not set and would accept, as `--name` and a placeholder.
pub(crate) fn options(request: &Request) -> BTreeMap<Box<str>, Box<str>> {
    let mut found = BTreeMap::new();
    let (verb, entity) = request.path();
    let Some(schema) = super::validation::schema(verb, entity) else {
        return found;
    };
    let Ok(value) = super::rewrite::query_value(request) else {
        return found;
    };
    let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
        return found;
    };
    let via = request.rewrite();
    for (name, property) in properties {
        if name == "source"
            || CONTINUATIONS.contains(&name.as_str())
            || property.get("x-positional").is_some()
            || property.get("x-trailing-section").is_some()
            || property.get("x-alias-of").is_some()
            || set(value.get(name), property)
        {
            continue;
        }
        let resolved = super::validation::resolve(&schema, property);
        let Some(placeholder) = placeholder(property, resolved) else {
            continue;
        };
        let array = super::property_type(property) == Some("array");
        let accepted = samples(resolved).into_iter().any(|sample| {
            let mut candidate = value.clone();
            let sample = if array {
                Value::Array(vec![sample])
            } else {
                sample
            };
            candidate
                .as_object_mut()
                .map(|fields| fields.insert(name.clone(), sample))
                .is_some()
                && super::rewrite::request(via, verb, entity, candidate).is_ok()
        });
        if accepted {
            found.insert(
                format!("--{}", name.replace('_', "-")).into(),
                placeholder.into(),
            );
        }
    }
    found
}

/// A call that starts its reply: no page offset past the first and no continuation state.
pub(crate) fn first_page(request: &Request) -> bool {
    let Ok(value) = super::rewrite::query_value(request) else {
        return false;
    };
    ["offset", "section_offset"]
        .iter()
        .all(|name| value.get(*name).and_then(Value::as_u64).unwrap_or(0) == 0)
        && CONTINUATIONS
            .iter()
            .all(|name| value.get(*name).is_none_or(Value::is_null))
}

fn set(value: Option<&Value>, property: &Value) -> bool {
    match value {
        None | Some(Value::Null) => false,
        Some(Value::Array(values)) if values.is_empty() => false,
        Some(value) => property.get("default") != Some(value),
    }
}

fn samples(resolved: &Value) -> Vec<Value> {
    if let Some(choices) = resolved.get("enum").and_then(Value::as_array) {
        return choices.clone();
    }
    match super::property_type(resolved) {
        Some("integer") => {
            let minimum = resolved.get("minimum").and_then(Value::as_u64).unwrap_or(0);
            vec![Value::from(minimum.max(1))]
        }
        Some("string") if is_date(resolved) => vec![Value::from("2024-01-01")],
        Some("string") => TEXT_SAMPLES.iter().map(|s| Value::from(*s)).collect(),
        _ => Vec::new(),
    }
}

fn is_date(resolved: &Value) -> bool {
    resolved
        .get("pattern")
        .and_then(Value::as_str)
        .is_some_and(|pattern| pattern.starts_with("^(?!0000)[0-9]{4}"))
        || resolved.get("format").and_then(Value::as_str) == Some("date")
}

/// A fill-in form for the option: a date, a number range, a short choice list, or a kind.
fn placeholder(property: &Value, resolved: &Value) -> Option<String> {
    if let Some(choices) = resolved.get("enum").and_then(Value::as_array) {
        let words: Vec<_> = choices.iter().filter_map(Value::as_str).collect();
        return Some(if words.len() <= 6 {
            format!("<{}>", words.join("|"))
        } else {
            "<VALUE>".into()
        });
    }
    match super::property_type(resolved) {
        Some("integer") => {
            let minimum = resolved.get("minimum").and_then(Value::as_u64).unwrap_or(0);
            Some(match resolved.get("maximum").and_then(Value::as_u64) {
                Some(maximum) if maximum < u64::from(u32::MAX) => {
                    format!("<{minimum}-{maximum}>")
                }
                _ => "<N>".into(),
            })
        }
        Some("string") if is_date(resolved) => Some("<YYYY-MM-DD>".into()),
        Some("string") => {
            let kind = property
                .get("anyOf")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .chain(property.get("items"))
                .chain(std::iter::once(property))
                .find_map(|node| node.get("$ref").and_then(Value::as_str))
                .and_then(|reference| reference.rsplit('/').next())
                .filter(|name| *name != "Text");
            Some(match kind {
                Some(name) => format!("<{name}>"),
                None => "<TEXT>".into(),
            })
        }
        _ => None,
    }
}
