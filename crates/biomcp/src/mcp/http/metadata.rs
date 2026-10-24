//! Revision-specific HTTP mirrors are checked against the JSON body.
use super::wire::Request;
use crate::mcp::protocol;
use base64::Engine;
use serde_json::{Value, json};
pub(super) fn validate(request: &Request, value: &Value) -> Result<&'static str, Value> {
    let id = value.get("id").filter(|id| protocol::valid_id(id)).cloned().unwrap_or(Value::Null);
    let mismatch = || protocol::error(id.clone(), -32020, "Header mismatch");
    let revision = request.one("mcp-protocol-version").map_err(|_| mismatch())?;
    let body_revision = value.pointer("/params/_meta/io.modelcontextprotocol~1protocolVersion").and_then(Value::as_str);
    let revision = revision.or_else(|| (value.get("method") == Some(&json!("initialize"))).then_some(protocol::REVISIONS[1]))
        .ok_or_else(mismatch)?;
    let Some(revision) = protocol::REVISIONS.iter().copied().find(|supported| *supported == revision) else {
        return Err(json!({"jsonrpc":"2.0","id":id,"error":{"code":-32022,
            "message":"Unsupported protocol version","data":{"supported":protocol::REVISIONS,"requested":revision}}}));
    };
    if request.one("content-type").map_err(|_| mismatch())?.is_none_or(|v|
        !v.split(';').next().is_some_and(|v| v.trim().eq_ignore_ascii_case("application/json"))) {
        return Err(mismatch());
    }
    let accept = request.one("accept").map_err(|_| mismatch())?.unwrap_or_default();
    if !["application/json", "text/event-stream"].iter().all(|wanted|
        accept.split(',').any(|v| v.trim() == *wanted)) { return Err(mismatch()); }
    if revision == protocol::REVISIONS[0] && value.get("id").is_some() {
        let method = value.get("method").and_then(Value::as_str).ok_or_else(mismatch)?;
        if body_revision != Some(revision) || request.one("mcp-method").map_err(|_| mismatch())? != Some(method) {
            return Err(mismatch());
        }
        if matches!(method, "tools/call" | "resources/read" | "prompts/get") {
            let name = value.pointer(if method == "resources/read" { "/params/uri" } else { "/params/name" })
                .and_then(Value::as_str).ok_or_else(mismatch)?;
            let header = request.one("mcp-name").map_err(|_| mismatch())?.ok_or_else(mismatch)?;
            if decode(header).as_deref() != Some(name) { return Err(mismatch()); }
        }
    } else if body_revision.is_some_and(|body| body != revision) { return Err(mismatch()); }
    Ok(revision)
}
fn decode(value: &str) -> Option<String> {
    if let Some(encoded) = value.strip_prefix("=?base64?").and_then(|v| v.strip_suffix("?=")) {
        String::from_utf8(base64::prelude::BASE64_STANDARD.decode(encoded).ok()?).ok()
    } else {
        value.bytes().all(|b| b == b'\t' || (0x20..=0x7e).contains(&b)).then(|| value.to_owned())
    }
}
