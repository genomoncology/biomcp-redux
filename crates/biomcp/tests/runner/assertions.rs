use super::Step;
use serde_json::Value;

pub(super) fn check(stdout: &[u8], step: &Step) -> Result<(), String> {
    ceiling("stdout", "max_bytes", stdout.len(), step.max_bytes)?;
    if step.expect.is_empty() && step.max_tokens.is_none() {
        return Ok(());
    }
    let value: Value =
        serde_json::from_slice(stdout).map_err(|error| format!("stdout JSON: {error}"))?;
    let compact = serde_json::to_vec(&value).map_err(|error| error.to_string())?;
    ceiling(
        "stdout",
        "max_tokens",
        compact.len().div_ceil(4),
        step.max_tokens,
    )?;
    for (pointer, assertion) in &step.expect {
        let actual = value
            .pointer(pointer)
            .ok_or_else(|| format!("{pointer}: expected {assertion}, actual <missing>"))?;
        assertion_at(pointer, actual, assertion)?;
    }
    Ok(())
}

fn assertion_at(pointer: &str, actual: &Value, assertion: &Value) -> Result<(), String> {
    let options = assertion.as_object().filter(|map| {
        map.keys()
            .any(|key| matches!(key.as_str(), "equals" | "max_bytes" | "max_tokens"))
    });
    if let Some(options) = options {
        if options
            .keys()
            .any(|key| !matches!(key.as_str(), "equals" | "max_bytes" | "max_tokens"))
        {
            return Err(format!("{pointer}: unknown assertion option"));
        }
        if let Some(expected) = options.get("equals") {
            equal(pointer, expected, actual)?;
        }
        let bytes = serde_json::to_vec(actual)
            .map_err(|error| error.to_string())?
            .len();
        for (key, size) in [("max_bytes", bytes), ("max_tokens", bytes.div_ceil(4))] {
            if let Some(limit) = options.get(key) {
                let limit = limit
                    .as_u64()
                    .and_then(|v| usize::try_from(v).ok())
                    .ok_or_else(|| format!("{pointer}: {key} requires a nonnegative integer"))?;
                ceiling(pointer, key, size, Some(limit))?;
            }
        }
        Ok(())
    } else {
        equal(pointer, assertion, actual)
    }
}

fn equal(pointer: &str, expected: &Value, actual: &Value) -> Result<(), String> {
    if expected == actual {
        Ok(())
    } else {
        Err(format!("{pointer}: expected {expected}, actual {actual}"))
    }
}

fn ceiling(pointer: &str, key: &str, actual: usize, limit: Option<usize>) -> Result<(), String> {
    if let Some(limit) = limit.filter(|limit| actual > *limit) {
        return Err(format!(
            "{pointer} {key}: expected <= {limit}, actual {actual}"
        ));
    }
    Ok(())
}
