//! Typed follow-ups remain executable calls across replay, errors and reply forms.
//! live_check: local transport replay using public recorded gene replies; no live calls.
use super::super::{
    launch::{Environment, fixtures},
    mcp::{exchange_with_globals, request},
};
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    for form in ["structured-only", "text-only", "both"] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        env.config(
            &json!({"mcp_tool_mode":"typed", "mcp_typed_output_schema":true,
            "mcp_reply_form":form}),
        )
        .map_err(|e| e.to_string())?;
        let replay = fixtures().join("replies/gene/main");
        let globals = env.globals(Some(&replay), false);
        let replies = exchange_with_globals(
            &env,
            &[
                request(json!(1), "tools/list", json!({})),
                request(
                    json!(2),
                    "tools/call",
                    json!({"name":"get_gene","arguments":{"input":["BRAF"]}}),
                ),
                request(
                    json!(3),
                    "tools/call",
                    json!({"name":"get_gene","arguments":{}}),
                ),
            ],
            &globals,
        )?;
        let tools = replies
            .iter()
            .find(|v| v["id"] == 1)
            .and_then(|v| v.pointer("/result/tools"))
            .and_then(Value::as_array)
            .ok_or("tools")?;
        let mut sections = 0;
        for reply in replies.iter().filter(|v| v["id"] != 1) {
            let text = reply
                .pointer("/result/content/0/text")
                .and_then(Value::as_str)
                .ok_or("text")?;
            let value = reply
                .pointer("/result/structuredContent")
                .cloned()
                .or_else(|| serde_json::from_str(text.split_once('\n')?.1).ok())
                .ok_or("typed body")?;
            check_next(&value, tools, &mut sections)?;
            if text.contains("--config")
                || text.contains("--replay")
                || text.contains("--case-cache")
            {
                return Err(format!("server arguments leaked: {text}"));
            }
        }
        if sections == 0 {
            return Err("gene replay supplied no typed section follow-up".into());
        }
    }
    Ok(())
}
fn check_next(value: &Value, tools: &[Value], sections: &mut usize) -> Result<(), String> {
    match value {
        Value::Object(fields) => {
            if let Some(next) = fields.get("next").and_then(Value::as_array) {
                for call in next {
                    if call.get("args").is_some() {
                        return Err(format!("CLI follow-up survived: {call}"));
                    }
                    let tool = tools
                        .iter()
                        .find(|tool| tool.get("name") == call.get("name"))
                        .ok_or_else(|| format!("unadvertised follow-up: {call}"))?;
                    jsonschema::validator_for(&tool["inputSchema"])
                        .map_err(|e| e.to_string())?
                        .validate(&call["arguments"])
                        .map_err(|e| format!("{call}: {e}"))?;
                    if call.get("kind") == Some(&json!("section")) {
                        if call.pointer("/arguments/section").is_none() {
                            return Err(format!("section stayed in input: {call}"));
                        }
                        *sections += 1;
                    }
                }
            }
            for (key, node) in fields {
                if matches!(key.as_str(), "error" | "problems") {
                    check_next(node, tools, sections)?;
                }
            }
        }
        Value::Array(nodes) => {
            for node in nodes {
                check_next(node, tools, sections)?;
            }
        }
        _ => {}
    }
    Ok(())
}

// Other typed parity tests compare shared data while this test owns the next-call contract.
pub(super) fn data(value: &Value) -> Value {
    let mut value = value.clone();
    strip_next(&mut value);
    value
}
fn strip_next(value: &mut Value) {
    match value {
        Value::Object(fields) => {
            fields.remove("next");
            for (key, node) in fields {
                if matches!(key.as_str(), "error" | "problems") {
                    strip_next(node);
                }
            }
        }
        Value::Array(nodes) => {
            for node in nodes {
                strip_next(node);
            }
        }
        _ => {}
    }
}
