//! Synthetic case budgets; native table measurements remain separate.
use super::Check;
use serde_json::{Value, json};
pub(super) fn check(value: &Value) -> Check {
    let full = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    let comparable = crate::runner::source_terms::size(value, "")?;
    // j13466 measured complete shapes; j13429 measured failures and expiry.
    // j19310 measured the deadline reply with source recovery guidance.
    let (measured_full, measured_comparable): (usize, usize) = if value.get("error").is_some() {
        (224, 224)
    } else if value.get("status") != Some(&json!("complete")) {
        if value.pointer("/query/snapshot").is_some() {
            (373, 261)
        } else {
            let measured = match value.pointer("/sources/0/reason").and_then(Value::as_str) {
                Some("rate_limited") => 298,
                Some("unavailable") => 297,
                Some("deadline_exceeded") => 551,
                Some("rejected") => 294,
                _ => 295,
            };
            (measured, measured - 112)
        }
    } else if value
        .pointer("/query/offset")
        .and_then(Value::as_u64)
        .is_some_and(|v| v > 0)
    {
        (1835, 1723)
    } else if value.pointer("/query/section") == Some(&json!("seriousness")) {
        (1918, 1806)
    } else if value.pointer("/query/section") == Some(&json!("age")) {
        (1919, 1807)
    } else {
        (1993, 1881)
    };
    if full > (measured_full * 115).div_ceil(100)
        || comparable > (measured_comparable * 115).div_ceil(100)
    {
        return Err(format!(
            "VAERS synthetic output exceeds measured ceiling: {full}/{comparable}"
        ));
    }
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "VAERS synthetic status={} reason={} bytes={full}/{comparable}",
        value.get("status").unwrap_or(&Value::Null),
        value.pointer("/sources/0/reason").unwrap_or(&Value::Null)
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
