//! Discovery describes the same file and source authority as execution.
use super::super::{http::Server, launch::Environment, mcp::{exchange_with_globals, request}};
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"mcp_tool_mode":"typed"})).map_err(|e| e.to_string())?;
    let frames = [request(json!(1), "tools/list", json!({}))];
    let local = exchange_with_globals(&env, &frames, &["--offline".into()])?;
    let tools = local.first().and_then(|r| r.pointer("/result/tools")).and_then(Value::as_array).ok_or("local tools")?;
    for tool in tools {
        let name = tool.get("name").and_then(Value::as_str).ok_or("tool name")?;
        let writes = matches!(name, "article_pdf" | "article_text" | "article_supplement" | "trial_document");
        if tool.pointer("/annotations/readOnlyHint") != Some(&json!(!writes)) {
            return Err(format!("wrong file capability hint: {name}"));
        }
    }
    let server = Server::configured_globals("127.0.0.1:0", Some(json!({"mcp_tool_mode":"typed"})), None, None, None, &["--offline".into(), "--hosted".into()])?;
    let (status, hosted) = server.request("POST", "/mcp", &frames.first().ok_or("list frame")?.to_string())?;
    if status != 200 { return Err(format!("catalog status {status}")); }
    let tools = hosted.pointer("/result/tools").and_then(Value::as_array).ok_or("hosted tools")?;
    for tool in tools {
        if matches!(tool.get("name").and_then(Value::as_str), Some("article_pdf" | "trial_document")) {
            return Err("hosted catalog advertises mandatory save".into());
        }
        if tool.to_string().contains("oncokb") || tool.to_string().contains("cosmic") {
            return Err(format!("restricted catalog advertises refused source: {}", tool.get("name").unwrap_or(&Value::Null)));
        }
    }
    let call = request(json!(2), "tools/call", json!({"name":"article_pdf","arguments":{"input":"28944238","save":"saved.pdf"}}));
    let (_, reply) = server.request("POST", "/mcp", &call.to_string())?;
    if reply.pointer("/result/structuredContent/error/code") != Some(&json!("usage.mcp_unavailable")) {
        return Err(format!("omitted tool escaped execution policy: {reply}"));
    }
    Ok(())
}
