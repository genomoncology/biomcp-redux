//! Typed follow-ups remain executable calls across replay, errors and reply forms.
//! live_check: local transport replay using public recorded gene replies; no live calls.
use super::super::{
    http::Server,
    launch::{Environment, fixtures},
    mcp::{exchange_with_globals, request},
};
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    for form in ["structured-only", "text-only", "both"] {
        for hosted in [false, true] {
            check_form(form, hosted)?;
        }
    }
    Ok(())
}
fn check_form(form: &str, hosted: bool) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let config = json!({"mcp_tool_mode":"typed", "mcp_typed_output_schema":true,"mcp_typed_definition_bytes":200000,
        "mcp_reply_form":form,"disable_sources":["oncokb"]});
    env.config(&config).map_err(|e| e.to_string())?;
    let replay = fixtures().join("replies/gene/main");
    let mut globals = env.globals(Some(&replay), false);
    let hosted_globals = if hosted {
        vec!["--hosted".into()]
    } else {
        vec![]
    };
    globals.extend(hosted_globals.clone());
    globals.extend([
        "--config".into(),
        env.root()
            .join("config/biomcp/config.json")
            .to_string_lossy()
            .into_owned(),
    ]);
    let frames = [
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
    ];
    let replies = exchange_with_globals(&env, &frames, &globals)?;
    let server = Server::configured_globals(
        "127.0.0.1:0",
        Some(config),
        Some(replay),
        None,
        None,
        &hosted_globals,
    )?;
    let tools = replies
        .iter()
        .find(|v| v["id"] == 1)
        .and_then(|v| v.pointer("/result/tools"))
        .and_then(Value::as_array)
        .ok_or("tools")?;
    for (index, frame) in frames.iter().enumerate().skip(1) {
        let (status, http) = server.request("POST", "/mcp", &frame.to_string())?;
        if status != 200 {
            return Err(format!("HTTP {status}: {http}"));
        }
        let stdio = replies
            .iter()
            .find(|v| v["id"] == frame["id"])
            .ok_or("stdio reply")?;
        for reply in [stdio, &http] {
            check_reply(reply, tools, index == 1)?;
        }
    }
    Ok(())
}

fn check_reply(reply: &Value, tools: &[Value], section_expected: bool) -> Result<(), String> {
    let text = reply
        .pointer("/result/content/0/text")
        .and_then(Value::as_str)
        .ok_or("text")?;
    let value = reply
        .pointer("/result/structuredContent")
        .cloned()
        .or_else(|| serde_json::from_str(text.split_once('\n')?.1).ok())
        .ok_or("typed body")?;
    let mut sections = 0;
    check_next(&value, tools, &mut sections)?;
    if section_expected && sections == 0 {
        return Err("gene replay supplied no typed section follow-up".into());
    }
    if text.contains("--config") || text.contains("--replay") || text.contains("--case-cache") {
        return Err(format!("server arguments leaked: {text}"));
    }
    let owner = tools
        .iter()
        .find(|tool| tool["name"] == "get_gene")
        .ok_or("get_gene")?;
    jsonschema::validator_for(&owner["outputSchema"])
        .map_err(|e| e.to_string())?
        .validate(&value)
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub(super) fn check_next(
    value: &Value,
    tools: &[Value],
    sections: &mut usize,
) -> Result<(), String> {
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
