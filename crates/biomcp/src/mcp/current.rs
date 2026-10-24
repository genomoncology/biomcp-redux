//! Stateless 2026-07-28 protocol edge.
use super::protocol::{self, CAPABILITIES_KEY, REVISIONS, VERSION_KEY};
use serde_json::{Value, json};

pub(super) fn validate(id: &Value, params: &Value) -> Result<(), Value> {
    let meta = params.get("_meta");
    let version = meta.and_then(|m| m.get(VERSION_KEY)).and_then(Value::as_str);
    if version.is_none() || meta.and_then(|m| m.get(CAPABILITIES_KEY)).is_none_or(|v| !v.is_object()) {
        return Err(protocol::error(id.clone(), -32602, "Invalid params"));
    }
    if version != REVISIONS.first().copied() {
        let mut error = protocol::error(id.clone(), -32022, "Unsupported protocol version");
        error["error"]["data"] = json!({"supported":REVISIONS,"requested":version});
        return Err(error);
    }
    Ok(())
}

pub(super) fn discover() -> Value {
    json!({"supportedVersions":REVISIONS,"capabilities":{"tools":{}}})
}
