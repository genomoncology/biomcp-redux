#[path = "typed_baseline.rs"]
mod baseline;
#[path = "typed_policy.rs"]
mod policy;
#[path = "typed_replay.rs"]
mod replay;
// Installed-artifact typed tool discovery, schema refusal and CLI equivalence.
use super::super::{
    launch::Environment,
    mcp::{exchange_with_globals, request},
};
use serde_json::{Value, json};
use std::io::Write;

pub(super) fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"mcp_tool_mode":"typed"}))
        .map_err(|e| e.to_string())?;
    let globals = vec!["--offline".into()];
    let replies = exchange_with_globals(
        &env,
        &[
            request(json!(1), "tools/list", json!({})),
            request(json!(2), "server/discover", json!({})),
        ],
        &globals,
    )?;
    let tools = replies
        .iter()
        .find(|r| r["id"] == 1)
        .and_then(|r| r.pointer("/result/tools"))
        .and_then(Value::as_array)
        .ok_or("missing tools")?;
    let paths = biomcp_types::command_paths();
    if tools.len() != paths.len() {
        return Err("missing registry tools".into());
    }
    let instructions = replies
        .iter()
        .find(|r| r["id"] == 2)
        .and_then(|r| r.pointer("/result/instructions"))
        .and_then(Value::as_str)
        .ok_or("instructions")?;
    if instructions.chars().count() > 4096 {
        return Err("instruction budget exceeded".into());
    }
    for (verb, entity, _) in paths {
        let name = format!("{verb}_{entity}").replace('-', "_");
        let tool = tools
            .iter()
            .find(|tool| tool["name"] == name)
            .ok_or_else(|| format!("missing {name}"))?;
        let description = tool["description"].as_str().ok_or("description")?;
        if description.chars().count() > 300 {
            return Err(format!(
                "{name} description {}: {description}",
                description.chars().count()
            ));
        }
        descriptions(&tool["inputSchema"])?;
    }
    writeln!(
        std::io::stdout(),
        "TYPED_TOOL_LIST characters={} tools={} response_characters={}",
        json!(tools).to_string().chars().count(),
        tools.len(),
        replies
            .iter()
            .find(|r| r["id"] == 1)
            .ok_or("tool-list reply")?
            .to_string()
            .chars()
            .count()
    )
    .map_err(|e| e.to_string())?;
    let refused = exchange_with_globals(
        &env,
        &[request(
            json!(3),
            "tools/call",
            json!({
        "name":"search_trial","arguments":{"source":"not-a-source"}}),
        )],
        &globals,
    )?;
    if refused[0].pointer("/error/code") != Some(&json!(-32602)) {
        return Err("wrong enum accepted".into());
    }
    for (name, arguments) in [
        (
            "get_variant",
            json!({"input":"rs113488022","civic_profile":"1"}),
        ),
        ("search_disease", json!({"text":42})),
    ] {
        let reply = exchange_with_globals(
            &env,
            &[request(
                json!(9),
                "tools/call",
                json!({"name":name,"arguments":arguments}),
            )],
            &globals,
        )?;
        if reply[0].pointer("/error/code") != Some(&json!(-32602)) {
            return Err(format!(
                "nullable scalar type accepted by {name}: {}",
                reply[0]
            ));
        }
    }
    // Every entity gets one search and get through both command boundaries.
    for tool in tools.iter().filter(|tool| {
        tool["name"]
            .as_str()
            .is_some_and(|name| name.starts_with("search_") || name.starts_with("get_"))
    }) {
        replay::check(tool)?;
    }
    for arguments in [
        json!({"condition":42}),
        json!({"unexpected":"value"}),
        json!({"limit":"10"}),
    ] {
        let reply = exchange_with_globals(
            &env,
            &[request(
                json!(4),
                "tools/call",
                json!({"name":"search_trial","arguments":arguments}),
            )],
            &globals,
        )?;
        if reply[0].pointer("/error/code") != Some(&json!(-32602)) {
            return Err("invalid argument shape accepted".into());
        }
    }
    policy::check()?;
    baseline::check()
}
fn descriptions(node: &Value) -> Result<(), String> {
    if node
        .get("description")
        .and_then(Value::as_str)
        .is_some_and(|s| s.chars().count() > 80)
    {
        return Err("argument description budget".into());
    }
    match node {
        Value::Object(map) => {
            for value in map.values() {
                descriptions(value)?;
            }
        }
        Value::Array(values) => {
            for value in values {
                descriptions(value)?;
            }
        }
        _ => {}
    }
    Ok(())
}
