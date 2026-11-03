//! Configured reply forms reach both transports and every tool, without changing JSON values.
use super::super::{
    http::Server,
    launch::Environment,
    mcp::{exchange_with_globals, request},
};
use serde_json::{Value, json};
use std::io::Write;

pub(super) fn check() -> Result<(), String> {
    for hosted in [false, true] {
        let mut globals = vec!["--offline".into()];
        if hosted {
            globals.push("--hosted".into());
        }
        let mut reference = Vec::new();
        for form in [
            None,
            Some("both"),
            Some("structured-only"),
            Some("text-only"),
        ] {
            let config = form.map_or(json!({}), |form| json!({"mcp_reply_form":form}));
            let environment = Environment::new().map_err(|e| e.to_string())?;
            environment.config(&config).map_err(|e| e.to_string())?;
            let calls = [
                request(
                    json!(1),
                    "tools/call",
                    json!({"name":"biomcp","arguments":{"args":["help"]}}),
                ),
                request(
                    json!(2),
                    "tools/call",
                    json!({"name":"search","arguments":{"query":"article --limit invalid"}}),
                ),
                request(
                    json!(3),
                    "tools/call",
                    json!({"name":"fetch","arguments":{"id":"invalid-id"}}),
                ),
                request(
                    json!(4),
                    "tools/call",
                    json!({"name":"biomcp","arguments":{"args":["invalid-command"]}}),
                ),
            ];
            let replies = exchange_with_globals(&environment, &calls, &globals)?;
            let server = Server::configured_globals(
                "127.0.0.1:0",
                Some(config),
                None,
                None,
                None,
                &globals,
            )?;
            for (index, call) in calls.iter().enumerate() {
                let stdio = replies
                    .iter()
                    .find(|reply| reply.get("id") == call.get("id"))
                    .and_then(|reply| reply.get("result"))
                    .ok_or("stdio result missing")?;
                let (status, http) = server.request("POST", "/mcp", &call.to_string())?;
                if status != 200 {
                    return Err("HTTP tool call failed".into());
                }
                let http = http.get("result").ok_or("HTTP result missing")?;
                let value = check_content(stdio, form.unwrap_or("both"))?;
                let failed = json!(index != 0);
                if stdio.get("isError") != Some(&failed) || http.get("isError") != Some(&failed) {
                    return Err("reply form lost tool error status".into());
                }
                if check_content(http, form.unwrap_or("both"))? != value {
                    return Err("HTTP and stdio reply values differ".into());
                }
                if form.is_none() {
                    reference.push(value);
                } else if reference.get(index) != Some(&value) {
                    return Err("reply form changed JSON value".into());
                }
                writeln!(
                    std::io::stdout(),
                    "MCP_REPLY hosted={hosted} form={} call={index} bytes={} chars={}",
                    form.unwrap_or("default"),
                    stdio.to_string().len(),
                    stdio.to_string().chars().count()
                )
                .map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

fn check_content(result: &Value, form: &str) -> Result<Value, String> {
    let blocks = result
        .get("content")
        .and_then(Value::as_array)
        .filter(|blocks| blocks.len() == 1)
        .ok_or("expected one content block")?;
    let text = blocks[0]
        .get("text")
        .and_then(Value::as_str)
        .ok_or("text missing")?;
    if blocks[0].get("type") != Some(&json!("text")) {
        return Err("expected text block".into());
    }
    match form {
        "structured-only" => {
            if text != "JSON reply is in structuredContent." {
                return Err("structured note differs".into());
            }
            result
                .get("structuredContent")
                .cloned()
                .ok_or("structured JSON missing".into())
        }
        "both" => {
            let value: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
            if result.get("structuredContent") != Some(&value) {
                return Err("JSON copies differ".into());
            }
            Ok(value)
        }
        "text-only" => {
            if result.get("structuredContent").is_some() {
                return Err("text-only repeats JSON".into());
            }
            serde_json::from_str(text).map_err(|e| e.to_string())
        }
        _ => Err("unknown test form".into()),
    }
}
