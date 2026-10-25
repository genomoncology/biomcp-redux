//! Synthetic case budgets; native table measurements remain separate.
use super::Check;
use serde_json::{Value, json};
pub(super) fn check(value: &Value) -> Check {
    let full = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    let comparable = crate::runner::source_terms::size(value, "")?;
    // j13424 measured the synthetic complete, disabled, bad-reply and expiry shapes.
    let (measured_full, measured_comparable): (usize, usize) = if value.get("error").is_some() {
        (224, 224)
    } else if value.get("status") != Some(&json!("complete")) {
        if value.pointer("/query/snapshot").is_some() {
            (373, 261)
        } else {
            let measured = match value.pointer("/sources/0/reason").and_then(Value::as_str) {
                Some("rate_limited") => 298,
                Some("unavailable") => 297,
                Some("deadline_exceeded") => 303,
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
        (1698, 1586)
    } else if value.pointer("/query/section") == Some(&json!("seriousness")) {
        (1781, 1669)
    } else if value.pointer("/query/section") == Some(&json!("age")) {
        (1782, 1670)
    } else {
        (1856, 1744)
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
