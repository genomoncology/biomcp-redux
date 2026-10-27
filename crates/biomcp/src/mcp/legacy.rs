//! Initialization-based 2025-11-25 protocol edge.
use super::protocol;
use serde_json::{Value, json};

pub(super) fn initialize(params: &Value) -> Option<Value> {
    params.get("protocolVersion")?.as_str()?;
    params.get("capabilities")?.as_object()?;
    let info = params.get("clientInfo")?;
    info.get("name")?.as_str()?;
    info.get("version")?.as_str()?;
    Some(json!({"protocolVersion":protocol::REVISIONS.last()?,
        "capabilities":{"tools":{}}, "serverInfo":protocol::info(), "instructions":protocol::INSTRUCTIONS}))
}
