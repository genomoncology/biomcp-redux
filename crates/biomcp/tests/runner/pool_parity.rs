//! Compare published pages while validating each cache's local pool identifiers and clocks.
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

// Store::publish and publish_source_pool retain a one-day immutable pool.
const POOL_TTL_SECONDS: u64 = 86_400;

/// Reconcile only page-pool metadata and references to its ID in executable next calls.
pub(super) fn expected(before: &Value, after: &Value) -> Result<Value, String> {
    let mut paths = Vec::new();
    for pool in ["/page/pool", "/result/page/pool"] {
        let id_path = format!("{pool}/id");
        let Some(id) = before.pointer(&id_path) else {
            continue;
        };
        paths.extend([
            id_path,
            format!("{pool}/created"),
            format!("{pool}/expires"),
        ]);
        for next in ["/next", "/result/next"] {
            for (step, value) in before
                .pointer(next)
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .enumerate()
            {
                for (index, pair) in value
                    .get("args")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flat_map(|args| args.windows(2))
                    .enumerate()
                {
                    if pair[0].as_str() == Some("--pool") && &pair[1] == id {
                        paths.push(format!("{next}/{step}/args/{}", index + 1));
                    }
                }
            }
        }
    }
    project(&paths, before, after)
}

pub(super) fn project(paths: &[String], before: &Value, after: &Value) -> Result<Value, String> {
    let mut expected = before.clone();
    let mut handles = BTreeMap::<String, String>::new();
    for path in paths {
        let old = before
            .pointer(path)
            .ok_or_else(|| format!("missing approved field {path}"))?;
        let new = after
            .pointer(path)
            .ok_or_else(|| format!("missing approved field {path}"))?;
        if path.ends_with("/created")
            || path.ends_with("/expires")
            || path == "/result/snapshot/acquired"
        {
            if old.as_u64().is_none() || new.as_u64().is_none() {
                return Err(format!("approved local timestamp changed type at {path}"));
            }
        } else {
            let old = handle(old, path)?;
            let new = handle(new, path)?;
            if handles.get(old).is_some_and(|prior| prior != new) {
                return Err(format!(
                    "one captured handle has inconsistent references at {path}"
                ));
            }
            handles.insert(old.to_owned(), new.to_owned());
        }
        *expected
            .pointer_mut(path)
            .ok_or("approved field missing from expected reply")? = new.clone();
    }
    let distinct: BTreeSet<_> = handles.values().collect();
    if distinct.len() != handles.len() {
        return Err("distinct captured handles collapsed into one handle".into());
    }
    references(before, after, &handles, "")?;
    pool_ttl(before, &handles.keys().cloned().collect())?;
    pool_ttl(after, &handles.values().cloned().collect())?;
    Ok(expected)
}

fn handle<'a>(value: &'a Value, path: &str) -> Result<&'a str, String> {
    value
        .as_str()
        .filter(|value| {
            value.len() == 32
                && value
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
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
                return Err(format!(
                    "captured handle reference is inconsistent at {path}"
                ));
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
            if fields
                .get("id")
                .and_then(Value::as_str)
                .is_some_and(|id| handles.contains(id))
                && (fields.contains_key("created") || fields.contains_key("expires"))
            {
                let created = fields
                    .get("created")
                    .and_then(Value::as_u64)
                    .ok_or("pool created time")?;
                let expires = fields
                    .get("expires")
                    .and_then(Value::as_u64)
                    .ok_or("pool expiry time")?;
                if created.checked_add(POOL_TTL_SECONDS) != Some(expires) {
                    return Err(
                        "captured pool expiry does not equal creation plus the declared TTL".into(),
                    );
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
