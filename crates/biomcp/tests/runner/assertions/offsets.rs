//! Byte positions in the raw reply: an agent that reads only the start of a reply sees these values.
use super::{Step, markdown, validate_pointer};
use serde_json::value::RawValue;
use std::collections::BTreeMap;

pub(super) fn validate(step: &Step) -> Result<(), String> {
    if !step.starts_within.is_empty() && (markdown(step) || step.stdout.is_some()) {
        return Err("starts_within requires a JSON reply".into());
    }
    for pointer in step.starts_within.keys() {
        validate_pointer(pointer)?;
        if pointer.split('/').any(|token| token == "*") {
            return Err(format!(
                "{pointer}: starts_within requires an exact pointer"
            ));
        }
    }
    Ok(())
}

/// Each selected value must start within its first-bytes limit of the raw output.
pub(super) fn check(stdout: &[u8], step: &Step) -> Result<(), String> {
    for (pointer, limit) in &step.starts_within {
        let start = start(stdout, pointer)?;
        if start >= *limit {
            return Err(format!(
                "{pointer} starts_within: expected a start before byte {limit}, actual byte {start}"
            ));
        }
    }
    Ok(())
}

/// Byte offset at which the value selected by an exact JSON pointer starts.
fn start(stdout: &[u8], pointer: &str) -> Result<usize, String> {
    let text = std::str::from_utf8(stdout).map_err(|error| format!("stdout UTF-8: {error}"))?;
    let mut value: &RawValue =
        serde_json::from_str(text).map_err(|error| format!("stdout JSON: {error}"))?;
    for token in pointer.split('/').skip(1) {
        let token = token.replace("~1", "/").replace("~0", "~");
        let raw = value.get();
        let next = if raw.starts_with('[') {
            let members: Vec<&RawValue> =
                serde_json::from_str(raw).map_err(|error| error.to_string())?;
            token
                .parse::<usize>()
                .ok()
                .and_then(|index| members.get(index).copied())
        } else if raw.starts_with('{') {
            let fields: BTreeMap<String, &RawValue> =
                serde_json::from_str(raw).map_err(|error| error.to_string())?;
            fields.get(&token).copied()
        } else {
            None
        };
        value = next.ok_or_else(|| format!("{pointer} starts_within: missing at {token}"))?;
    }
    Ok(value.get().as_ptr().addr() - text.as_ptr().addr())
}

tool_only! {
/// The check reads raw byte positions, including nested and escaped keys, and refuses late values.
pub(crate) fn proof() -> Result<(), String> {
    let reply = br#"{"result":{"items":[{"a/b":1}],"total":7},"page":{"total":7}}"#;
    if start(reply, "/result/items/0/a~1b")? != 27 || start(reply, "/page/total")? != 58 {
        return Err("starts_within read the wrong byte".into());
    }
    let step: Step = serde_json::from_value(serde_json::json!({"starts_within":{"/page/total":58}}))
        .map_err(|error| error.to_string())?;
    match check(reply, &step) {
        Err(_) => Ok(()),
        Ok(()) => Err("a value at the limit passed starts_within".into()),
    }
}
}
