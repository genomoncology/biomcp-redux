//! Compare only the named volatile fields approved in PM ruling m25543.
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

// Store::publish and publish_source_pool retain a one-day immutable pool.
const POOL_TTL_SECONDS: u64 = 86_400;

fn pointers(case: &str, step: usize) -> Result<Option<Vec<String>>, String> {
    let inventory: BTreeMap<String, Vec<String>> =
        serde_json::from_str(include_str!("typed_volatile_cases.json"))
            .map_err(|error| error.to_string())?;
    Ok(inventory.get(&format!("{case} step {step}")).cloned())
}

pub(super) fn expected(
    case: &str,
    step: usize,
    before: &Value,
    after: &Value,
) -> Result<Option<Value>, String> {
    let Some(paths) = pointers(case, step)? else {
        return Ok(None);
    };
    let mut expected = before.clone();
    let mut handles = BTreeMap::new();
    for path in &paths {
        let old = before.pointer(path).ok_or_else(|| format!("missing approved field {path}"))?;
        let new = after.pointer(path).ok_or_else(|| format!("missing approved field {path}"))?;
        if path.ends_with("/created") || path.ends_with("/expires") {
            if old.as_u64().is_none() || new.as_u64().is_none() {
                return Err(format!("approved pool timestamp changed type at {path}"));
            }
        } else {
            let old = handle(old, path)?;
            let new = handle(new, path)?;
            if handles.get(old).is_some_and(|prior| prior != new) {
                return Err(format!("one captured handle has inconsistent references at {path}"));
            }
            handles.insert(old.to_owned(), new.to_owned());
        }
        *expected.pointer_mut(path).ok_or("approved field missing from expected reply")? = new.clone();
    }
    let distinct: BTreeSet<_> = handles.values().collect();
    if distinct.len() != handles.len() {
        return Err("distinct captured handles collapsed into one handle".into());
    }
    references(before, after, &handles, "")?;
    pool_ttl(before, &handles.keys().cloned().collect())?;
    pool_ttl(after, &handles.values().cloned().collect())?;
    Ok(Some(expected))
}

fn handle<'a>(value: &'a Value, path: &str) -> Result<&'a str, String> {
    value
        .as_str()
        .filter(|value| {
            value.len() == 32
                && value.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
        })
        .ok_or_else(|| format!("approved handle changed its declared hexadecimal form at {path}"))
}

fn references(
    before: &Value,
    after: &Value,
    handles: &BTreeMap<String, String>,
    path: &str,
) -> Result<(), String> {
    match before {
        Value::String(value) => {
            if let Some(expected) = handles.get(value)
                && after.pointer(path).and_then(Value::as_str) != Some(expected.as_str())
            {
                return Err(format!("captured handle reference is inconsistent at {path}"));
            }
        }
        Value::Object(fields) => {
            for (key, value) in fields {
                let key = key.replace('~', "~0").replace('/', "~1");
                references(value, after, handles, &format!("{path}/{key}"))?;
            }
        }
        Value::Array(values) => {
            for (index, value) in values.iter().enumerate() {
                references(value, after, handles, &format!("{path}/{index}"))?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn pool_ttl(value: &Value, handles: &BTreeSet<String>) -> Result<(), String> {
    match value {
        Value::Object(fields) => {
            if fields.get("id").and_then(Value::as_str).is_some_and(|id| handles.contains(id))
                && (fields.contains_key("created") || fields.contains_key("expires"))
            {
                let created = fields.get("created").and_then(Value::as_u64).ok_or("pool created time")?;
                let expires = fields.get("expires").and_then(Value::as_u64).ok_or("pool expiry time")?;
                if created.checked_add(POOL_TTL_SECONDS) != Some(expires) {
                    return Err("captured pool expiry does not equal creation plus the declared TTL".into());
                }
            }
            for value in fields.values() {
                pool_ttl(value, handles)?;
            }
        }
        Value::Array(values) => {
            for value in values {
                pool_ttl(value, handles)?;
            }
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn raw(case: &str, step: usize, before: &str, after: &str) -> Result<bool, String> {
    let old: Value = serde_json::from_str(before).map_err(|error| error.to_string())?;
    let new: Value = serde_json::from_str(after).map_err(|error| error.to_string())?;
    if old.to_string() != before {
        return Err("single raw reply is not in its existing canonical wire form".into());
    }
    let old_body = old.pointer("/result/structuredContent").ok_or("single structured reply")?;
    let new_body = new.pointer("/result/structuredContent").ok_or("single structured reply")?;
    let Some(expected_body) = expected(case, step, old_body, new_body)? else {
        return Ok(false);
    };
    let mut expected = old.clone();
    if let Some(text) = old.pointer("/result/content/0/text").and_then(Value::as_str) {
        if text != old_body.to_string() {
            return Err("single text is not the exact existing serialized structured reply".into());
        }
        *expected.pointer_mut("/result/content/0/text").ok_or("single text")? =
            Value::String(expected_body.to_string());
    }
    *expected.pointer_mut("/result/structuredContent").ok_or("single structured reply")? = expected_body;
    Ok(expected.to_string() == after)
}
