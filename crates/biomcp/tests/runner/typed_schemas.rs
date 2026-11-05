//! Installed tool catalogs keep command argument documentation and fit discovery budgets.
use crate::runner::{
    launch::Environment,
    mcp::{exchange_with_globals, request},
};
use serde_json::{Value, json};
use std::io::Write;

pub(super) fn check(env: &Environment) -> Result<(), String> {
    let tools = catalog(env, json!({"mcp_tool_mode":"typed"}))?;
    let mut failures = Vec::new();
    for (verb, entity, id) in biomcp_types::command_paths() {
        let name = format!("{verb}_{entity}").replace('-', "_");
        let tool = tools
            .iter()
            .find(|t| t["name"] == name)
            .ok_or("missing tool")?;
        if tool.get("outputSchema").is_some() {
            failures.push(format!("{name}: outputSchema must default off"));
        }
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &["schema".into(), id.into()])
            .output()
            .map_err(|e| e.to_string())?;
        let schema: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        for (key, node) in schema["properties"].as_object().ok_or("properties")? {
            if node.get("x-cli-flags").is_some() {
                continue;
            }
            let property = &tool["inputSchema"]["properties"][key];
            let expected = node.get("description").and_then(Value::as_str).map(|s| {
                s.lines()
                    .next()
                    .unwrap_or_default()
                    .chars()
                    .take(80)
                    .collect::<String>()
            });
            if property.get("description").and_then(Value::as_str) != expected.as_deref() {
                failures.push(format!("{name}.{key}: lost own description"));
            }
            if nested_description(property, true) {
                failures.push(format!("{name}.{key}: nested description"));
            }
            let required = schema["required"]
                .as_array()
                .is_some_and(|v| v.contains(&json!(key)));
            if !required && nullable(property) {
                failures.push(format!("{name}.{key}: optional nullable branch"));
            }
            if oversized_enum(property, 30) {
                failures.push(format!("{name}.{key}: oversized enum"));
            }
        }
    }
    report(&tools)?;
    // Exercise the same budget rejection used for the installed catalog.
    if budget(&json!({"name":"oversized", "description":"x".repeat(2001), "inputSchema":{}}))
        .is_ok()
    {
        return Err("oversized synthetic tool passed budget".into());
    }
    for tool in &tools {
        if let Err(error) = budget(tool) {
            failures.push(error);
        }
    }
    if !failures.is_empty() {
        return Err(failures.join("\n"));
    }
    settings(env, &tools)
}
fn catalog(env: &Environment, config: Value) -> Result<Vec<Value>, String> {
    env.config(&config).map_err(|e| e.to_string())?;
    let replies = exchange_with_globals(
        env,
        &[request(json!(1), "tools/list", json!({}))],
        &["--offline".into()],
    )?;
    replies
        .first()
        .and_then(|r| r.pointer("/result/tools"))
        .and_then(Value::as_array)
        .cloned()
        .ok_or("catalog missing".into())
}
fn nested_description(node: &Value, top: bool) -> bool {
    match node {
        Value::Object(map) => {
            (!top && map.contains_key("description"))
                || map
                    .iter()
                    .any(|(k, v)| k != "description" && nested_description(v, false))
        }
        Value::Array(nodes) => nodes.iter().any(|n| nested_description(n, false)),
        _ => false,
    }
}
fn nullable(node: &Value) -> bool {
    match node {
        Value::Object(map) => {
            map.get("type").is_some_and(|v| {
                v == "null" || v.as_array().is_some_and(|a| a.contains(&json!("null")))
            }) || map.values().any(nullable)
        }
        Value::Array(nodes) => nodes.iter().any(nullable),
        _ => false,
    }
}
fn oversized_enum(node: &Value, limit: usize) -> bool {
    match node {
        Value::Object(map) => {
            map.get("enum")
                .and_then(Value::as_array)
                .is_some_and(|v| v.len() > limit)
                || map.values().any(|v| oversized_enum(v, limit))
        }
        Value::Array(nodes) => nodes.iter().any(|n| oversized_enum(n, limit)),
        _ => false,
    }
}
fn size(tool: &Value) -> usize {
    ["name", "description"]
        .iter()
        .map(|key| tool[key].as_str().unwrap_or_default().chars().count())
        .sum::<usize>()
        + tool["inputSchema"].to_string().chars().count()
}
fn budget(tool: &Value) -> Result<(), String> {
    let length = size(tool);
    if length > 2000 {
        Err(format!(
            "{}: {length} characters exceeds 2000",
            tool["name"]
        ))
    } else {
        Ok(())
    }
}
fn report(tools: &[Value]) -> Result<(), String> {
    let mut output = std::io::stdout().lock();
    for tool in tools {
        writeln!(
            output,
            "TYPED_TOOL_SIZE {} characters={} output_schema_characters={}",
            tool["name"],
            size(tool),
            tool.get("outputSchema")
                .map_or(0, |v| v.to_string().chars().count())
        )
        .map_err(|e| e.to_string())?;
    }
    writeln!(
        output,
        "TYPED_TOOL_LIST characters={} tools={} budget_characters={}",
        json!(tools).to_string().chars().count(),
        tools.len(),
        tools.iter().map(size).sum::<usize>()
    )
    .map_err(|e| e.to_string())
}
fn settings(env: &Environment, default: &[Value]) -> Result<(), String> {
    let enabled = catalog(
        env,
        json!({"mcp_tool_mode":"typed", "mcp_typed_output_schema":true}),
    )?;
    report(&enabled)?;
    for tool in enabled {
        if tool
            .pointer("/outputSchema/anyOf/0/properties/schema/enum")
            .and_then(Value::as_array)
            .is_none()
            || tool.pointer("/outputSchema/anyOf/1/properties").is_none()
        {
            return Err(format!("{}: output schema not restored", tool["name"]));
        }
        let mut without = tool.clone();
        without
            .as_object_mut()
            .ok_or("tool object")?
            .remove("outputSchema");
        if !default.contains(&without) {
            return Err("output schema setting changed input catalog".into());
        }
    }
    let large = catalog(
        env,
        json!({"mcp_tool_mode":"typed", "mcp_typed_enum_limit":1000}),
    )?;
    if !large
        .iter()
        .any(|tool| oversized_enum(&tool["inputSchema"], 30))
    {
        return Err("raised enum limit did not restore large choices".into());
    }
    if !default
        .iter()
        .any(|tool| oversized_enum(&tool["inputSchema"], 1))
    {
        return Err("short enums lost their choices".into());
    }
    let small = catalog(
        env,
        json!({"mcp_tool_mode":"typed", "mcp_typed_enum_limit":1}),
    )?;
    if small
        .iter()
        .any(|tool| oversized_enum(&tool["inputSchema"], 1))
    {
        return Err("enum limit setting ignored".into());
    }
    Ok(())
}
