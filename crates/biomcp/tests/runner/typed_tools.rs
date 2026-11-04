//! Installed-artifact typed tool discovery, schema refusal and CLI equivalence.
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
        if description.chars().count() > 320 {
            return Err(format!(
                "{name} description {}: {description}",
                description.chars().count()
            ));
        }
        descriptions(&tool["inputSchema"])?;
    }
    writeln!(
        std::io::stdout(),
        "TYPED_TOOL_LIST characters={} tools={}",
        json!(tools).to_string().chars().count(),
        tools.len()
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
    // Every entity gets one search and get through both command boundaries.
    for tool in tools.iter().filter(|tool| {
        tool["name"]
            .as_str()
            .is_some_and(|name| name.starts_with("search_") || name.starts_with("get_"))
    }) {
        parity(&env, tool, &globals)?;
    }
    Ok(())
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
fn sample(node: &Value) -> Value {
    if let Some(value) = node
        .get("enum")
        .and_then(Value::as_array)
        .and_then(|v| v.first())
    {
        return value.clone();
    }
    if let Some(value) = node
        .get("anyOf")
        .and_then(Value::as_array)
        .and_then(|v| v.iter().find(|v| v["type"] != "null"))
    {
        return sample(value);
    }
    match node["type"].as_str() {
        Some("array") => json!([sample(&node["items"])]),
        Some("integer") => node.get("minimum").cloned().unwrap_or(json!(1)),
        Some("boolean") => json!(false),
        _ => json!("example"),
    }
}
fn parity(env: &Environment, tool: &Value, globals: &[String]) -> Result<(), String> {
    let name = tool["name"].as_str().ok_or("tool name")?;
    let (verb, entity) = name.split_once('_').ok_or("path")?;
    let entity = entity.replace('_', "-");
    let schema = &tool["inputSchema"];
    let mut input = serde_json::Map::new();
    for key in schema["required"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        input.insert(key.into(), sample(&schema["properties"][key]));
    }
    let response = exchange_with_globals(
        env,
        &[request(
            json!(1),
            "tools/call",
            json!({"name":name,"arguments":input}),
        )],
        globals,
    )?;
    let structured = response[0]
        .pointer("/result/structuredContent")
        .ok_or_else(|| format!("{name}: {}", response[0]))?;
    if response[0]
        .pointer("/result/content/0/text")
        .and_then(Value::as_str)
        .is_none_or(|text| text.lines().count() != 1)
    {
        return Err("summary must be one line".into());
    }
    // Canonical query schemas own the positional and flag spelling used for comparison.
    let (_, _, id) = biomcp_types::command_paths()
        .into_iter()
        .find(|(v, e, _)| *v == verb && *e == entity)
        .ok_or("command")?;
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &["schema".into(), id.into()])
        .output()
        .map_err(|e| e.to_string())?;
    let query: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    let mut cli = vec![verb.into(), entity];
    let mut positions = Vec::new();
    for (key, value) in input {
        let words = value
            .as_array()
            .cloned()
            .unwrap_or_else(|| vec![value])
            .into_iter()
            .map(|v| {
                v.as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| v.to_string())
            })
            .collect::<Vec<_>>();
        if let Some(index) = query["properties"][&key]["x-positional"].as_u64() {
            positions.push((index, words));
        } else {
            for word in words {
                cli.push(format!("--{}={word}", key.replace('_', "-")));
            }
        }
    }
    positions.sort_by_key(|(index, _)| *index);
    cli.extend(positions.into_iter().flat_map(|(_, words)| words));
    cli.extend_from_slice(globals);
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &cli)
        .output()
        .map_err(|e| e.to_string())?;
    let expected: Value =
        serde_json::from_slice(&output.stdout).map_err(|e| format!("{name} CLI: {e}"))?;
    if structured.to_string() != expected.to_string() {
        return Err(format!(
            "{name} differs from CLI: {structured} versus {expected}"
        ));
    }
    Ok(())
}
