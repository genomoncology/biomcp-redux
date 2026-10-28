//! Feature vaers-summary: one server shares CDC spacing across concurrent commands.
use crate::runner::{launch::Environment, mcp, network_stub::Stub};
use serde_json::json;
use std::fs;
pub(super) fn run() -> Result<(), String> {
    let stub = Stub::for_source("wonder.cdc.gov", "application/xml", |_| {
        Ok("<synthetic-broken/>".into())
    })?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    stub.retain(&env);
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e|e.to_string())?;
    let frames = [1, 2].map(|id| mcp::request(json!(id), "tools/call", json!({
        "name":"biomcp", "arguments":{"args":["search","adverse-event","MMR","--source","vaers"]}
    })));
    let replies = mcp::exchange(&env, &frames)?;
    let refused = replies.iter().filter_map(|r|r.pointer("/result/structuredContent"))
        .find(|r|r.pointer("/sources/0/reason") == Some(&json!("local_quota")))
        .ok_or_else(||format!("missing immediate quota refusal: {replies:?}"))?;
    let notice = refused.pointer("/sources/0/notices/0").ok_or("quota guidance")?;
    let seconds = notice["retry_after_seconds"].as_u64().ok_or("seconds left")?;
    if refused["status"] != "failed" || notice["kind"] != "request_guidance"
        || seconds == 0 || seconds > 120 || !notice["message"].as_str().unwrap_or("")
            .contains("https://wonder.cdc.gov/wonder/help/wonder-api.html") {
        return Err(format!("quota refusal lost its next step: {refused}"));
    }
    if stub.finish()?.len() != 1 { return Err("quota refusal reached the source".into()); }
    Ok(())
}
