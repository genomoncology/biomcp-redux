//! Every replayed reply's status word matches its sources (1059): `complete` never sits beside a
//! failed, stale or syncing source row, every `partial` reply says what it is missing, and every
//! `incomplete` entry points inside the reply.
use serde_json::Value;

pub(super) fn check(reply: &Value) -> Result<(), String> {
    let status = reply.get("status").and_then(Value::as_str);
    let failed = reply
        .get("sources")
        .and_then(Value::as_array)
        .is_some_and(|rows| {
            rows.iter().any(|row| {
                matches!(
                    row.get("state").and_then(Value::as_str),
                    Some("failed" | "stale" | "syncing")
                )
            })
        });
    let listed = reply
        .get("incomplete")
        .and_then(Value::as_array)
        .map_or(&[][..], Vec::as_slice);
    for entry in listed {
        let at = entry.get("at").and_then(Value::as_str).unwrap_or_default();
        if !at.starts_with('/') || reply.pointer(at).is_none() {
            return Err(format!(
                "status words: incomplete entry {at:?} points at nothing in the reply"
            ));
        }
    }
    let entries = listed.len();
    match status {
        Some("complete") if failed || entries > 0 => {
            Err("status words: a complete reply has a failed, stale or syncing source row or an incomplete entry".into())
        }
        Some("partial") if entries == 0 => {
            Err("status words: a partial reply has no incomplete entry".into())
        }
        Some("partial" | "complete") | None => Ok(()),
        Some(_) if entries > 0 => Err("status words: only a partial reply has incomplete entries".into()),
        Some(_) => Ok(()),
    }
}

/// Check every reply in one command's output, a JSON document or JSON lines such as MCP frames,
/// so runner trials obey the same rule as cases (1059). Output that is not JSON has no reply.
pub(super) fn check_output(stdout: &[u8]) -> Result<(), String> {
    let documents: Vec<Value> = match serde_json::from_slice(stdout) {
        Ok(value) => vec![value],
        Err(_) => stdout
            .split(|byte| *byte == b'\n')
            .filter_map(|line| serde_json::from_slice(line).ok())
            .collect(),
    };
    documents.iter().try_for_each(replies)
}

/// Every object with a `schema` and a `status` word is a reply, wherever it sits.
fn replies(value: &Value) -> Result<(), String> {
    match value {
        Value::Object(map) => {
            if map.get("schema").is_some_and(Value::is_string)
                && map.get("status").is_some_and(Value::is_string)
            {
                return check(value);
            }
            map.values().try_for_each(replies)
        }
        Value::Array(items) => items.iter().try_for_each(replies),
        _ => Ok(()),
    }
}
