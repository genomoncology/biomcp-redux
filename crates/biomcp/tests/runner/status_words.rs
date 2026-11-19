//! Every replayed reply's status word matches its sources (1059): `complete` never sits beside a
//! failed, stale or syncing source row, and every `partial` reply says what it is missing.
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
    let entries = reply
        .get("incomplete")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
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
