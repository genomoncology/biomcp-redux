//! Tell a never-issued saved-result name, or an offset without one, from an expired pool.
use biomcp_types::{ErrorMessage, Request};
use serde_json::Value;

const NAMES: [&str; 3] = ["pool", "snapshot", "name_snapshot"];

/// The option at fault when a restart is not caused by expiry, with its guidance.
pub(super) fn fault(request: &Request) -> Option<(Box<str>, ErrorMessage)> {
    let value = serde_json::to_value(request).ok()?;
    let mut names = Vec::new();
    let mut offsets = Vec::new();
    walk(&value, &mut names, &mut offsets);
    if let Some((key, _)) = names.iter().find(|(_, name)| !issued(name)) {
        return Some((flag(key), ErrorMessage::SavedNameUnknown));
    }
    if names.is_empty()
        && let Some(key) = offsets.first()
    {
        return Some((flag(key), ErrorMessage::OffsetWithoutSaved));
    }
    None
}

fn walk(value: &Value, names: &mut Vec<(String, String)>, offsets: &mut Vec<String>) {
    match value {
        Value::Object(fields) => {
            for (key, field) in fields {
                match field {
                    Value::String(name) if NAMES.contains(&key.as_str()) => {
                        names.push((key.clone(), name.clone()));
                    }
                    Value::Number(number)
                        if (key == "offset" || key.ends_with("_offset"))
                            && number.as_u64().is_some_and(|n| n > 0) =>
                    {
                        offsets.push(key.clone());
                    }
                    _ => walk(field, names, offsets),
                }
            }
        }
        Value::Array(items) => items.iter().for_each(|item| walk(item, names, offsets)),
        _ => {}
    }
}

/// The local store names pools with 32 and snapshots with 64 lowercase hex digits.
fn issued(name: &str) -> bool {
    matches!(name.len(), 32 | 64) && name.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

fn flag(key: &str) -> Box<str> {
    format!("--{}", key.replace('_', "-")).into()
}

#[cfg(test)]
mod tests {
    use super::issued;

    #[test]
    fn issued_names_are_store_hex() {
        assert!(issued(&"a".repeat(32)));
        assert!(issued(&"0".repeat(64)));
        assert!(!issued("kabuki_case_q4"));
        assert!(!issued("memo-001"));
        assert!(!issued(&"A".repeat(32)));
    }
}
