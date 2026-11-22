//! Installed tool catalogs keep command argument documentation and fit discovery budgets.
use crate::runner::{
    launch::Environment,
    mcp::{exchange_with_globals, request},
};
use serde_json::{Value, json};
use std::io::Write;

fn field<'a>(node: &'a Value, key: &str) -> &'a Value {
    node.get(key).unwrap_or(&Value::Null)
}

pub(super) fn check(env: &Environment) -> Result<(), String> {
    let tools = catalog(env, json!({"mcp_tool_mode":"typed"}))?;
    let views = crate::runner::protocol::typed_catalog::commands(&tools);
    let mut failures = Vec::new();
    top_level(&tools, &mut failures);
    let bytes = serde_json::to_vec(&tools)
        .map_err(|error| error.to_string())?
        .len();
    if bytes >= 200_000 {
        return Err(format!(
            "the default typed catalog uses {bytes} bytes, budget is under 200,000"
        ));
    }
    for (verb, entity, id) in biomcp_types::command_paths() {
        let name = format!("{verb}_{entity}").replace('-', "_");
        let tool = views
            .iter()
            .find(|tool| tool["name"] == name)
            .ok_or_else(|| format!("{name}: no advertised action"))?;
        if field(tool, "description")
            .as_str()
            .is_some_and(|s| s.contains('\n'))
        {
            failures.push(format!(
                "{name}: full schema prose leaked into action summary"
            ));
        }
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &["schema".into(), id.into()])
            .output()
            .map_err(|e| e.to_string())?;
        let schema: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        for (key, node) in schema
            .get("properties")
            .and_then(Value::as_object)
            .ok_or("properties")?
        {
            // A field that renames another is accepted without being advertised (0901).
            if node.get("x-cli-flags").is_some() || node.get("x-alias-of").is_some() {
                continue;
            }
            let property = tool
                .get("inputSchema")
                .and_then(|s| s.get("properties"))
                .and_then(|p| p.get(advertised(verb, &schema, key)))
                .ok_or("property")?;
            let expected = description(node);
            let own = property.get("description").and_then(Value::as_str);
            if own.map(|text| text.strip_prefix("Required. ").unwrap_or(text))
                != expected.as_deref()
            {
                failures.push(format!("{name}.{key}: lost own description"));
            }
            if nested_description(property, true) {
                failures.push(format!("{name}.{key}: nested description"));
            }
            let required = schema
                .get("required")
                .and_then(Value::as_array)
                .is_some_and(|v| v.contains(&json!(key)));
            if !required && nullable(property) {
                failures.push(format!("{name}.{key}: optional nullable branch"));
            }
            if oversized_enum(property, 30) {
                failures.push(format!("{name}.{key}: oversized enum"));
            }
            // Section words are listed in the description, so a wrong one reaches the server.
            if key == "section" && oversized_enum(property, 0) {
                failures.push(format!("{name}.{key}: section words fixed in the schema"));
            }
        }
    }
    report(&tools)?;
    // Exercise the same budget rejection used for the installed catalog.
    if budget(
        &json!({"name":"oversized", "description":"x".repeat(ACTION_BUDGET + 1), "inputSchema":{}}),
    )
    .is_ok()
    {
        return Err("oversized synthetic tool passed budget".into());
    }
    for tool in &views {
        if let Err(error) = budget(tool) {
            failures.push(error);
        }
    }
    if !failures.is_empty() {
        return Err(failures.join("\n"));
    }
    super::typed_catalog::budgets(env, &tools)?;
    settings(env, &tools)
}
/// The typed tools themselves: one-line descriptions and compact default output schemas.
fn top_level(tools: &[Value], failures: &mut Vec<String>) {
    for tool in tools {
        let name = field(tool, "name");
        if ["biomcp", "search", "fetch"].contains(&name.as_str().unwrap_or_default()) {
            continue; // Protocol tools keep the output schemas they had before.
        }
        if field(tool, "description")
            .as_str()
            .is_some_and(|s| s.contains('\n'))
        {
            failures.push(format!(
                "{name}: full schema prose leaked into tool summary"
            ));
        }
        if let Err(error) = compact_output(tool) {
            failures.push(format!("{name}: {error}"));
        }
    }
}
fn compact_output(tool: &Value) -> Result<(), String> {
    let schema = tool
        .get("outputSchema")
        .ok_or("missing default output schema")?;
    if nested_description(schema, false)
        || has_keyword(schema, "$defs")
        || has_keyword(schema, "enum")
    {
        return Err("output schema contains definitions, descriptions or enums".into());
    }
    let variants = schema
        .pointer("/anyOf/0/anyOf")
        .and_then(Value::as_array)
        .ok_or("output schema lacks reply variants")?;
    if variants.is_empty() || schema.pointer("/anyOf/1/properties").is_none() {
        return Err("output schema lacks success or error replies".into());
    }
    for variant in variants {
        if variant
            .pointer("/properties/schema/const")
            .and_then(Value::as_str)
            .is_none()
        {
            return Err("reply variant lacks its canonical schema id".into());
        }
        let result = variant
            .pointer("/properties/result/properties")
            .and_then(Value::as_object)
            .filter(|fields| !fields.is_empty())
            .ok_or("reply variant lacks result fields")?;
        for field in result.values() {
            if has_keyword(field, "properties")
                && (field.get("properties").is_some()
                    || field
                        .pointer("/items/properties")
                        .and_then(Value::as_object)
                        .is_none_or(|row| row.values().any(|cell| has_keyword(cell, "properties"))))
            {
                return Err("result fields contain schemas below the row level".into());
            }
        }
    }
    Ok(())
}

fn has_keyword(schema: &Value, key: &str) -> bool {
    match schema {
        Value::Object(fields) => {
            fields.contains_key(key) || fields.values().any(|value| has_keyword(value, key))
        }
        Value::Array(values) => values.iter().any(|value| has_keyword(value, key)),
        _ => false,
    }
}
fn catalog(env: &Environment, config: Value) -> Result<Vec<Value>, String> {
    env.config(&config).map_err(|e| e.to_string())?;
    let replies = exchange_with_globals(
        env,
        &[request(json!(1), "tools/list", json!({}))],
        &["--offline".into()],
    )?;
    let tools = replies
        .first()
        .and_then(|r| r.pointer("/result/tools"))
        .and_then(Value::as_array)
        .cloned()
        .ok_or("catalog missing")?;
    super::typed_catalog::check(&tools)?;
    Ok(tools)
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
pub(super) fn size(tool: &Value) -> usize {
    ["name", "description"]
        .iter()
        .map(|key| {
            field(tool, key)
                .as_str()
                .unwrap_or_default()
                .chars()
                .count()
        })
        .sum::<usize>()
        + field(tool, "inputSchema").to_string().chars().count()
}
/// One action's description and schema. The names agents guess for search text, record IDs
/// and sections are accepted without being advertised, so they cost nothing here. 2,000 was a
/// design target, not a client limit; ticket 0905 raised it to 2,400 so a new filter need not trim
/// other descriptions. The all-tools definition cap (`DEFAULT_TYPED_DEFINITION_BYTES`, 128 KiB
/// since ticket 0763) still guards clients that load every tool.
const ACTION_BUDGET: usize = 2400;
/// The name an action advertises for a registry field: `query` for search text and `id`
/// for the record ID.
fn advertised<'a>(verb: &str, schema: &Value, key: &'a str) -> &'a str {
    let has = |name: &str| schema.pointer(&format!("/properties/{name}")).is_some();
    let first = schema.pointer(&format!("/properties/{key}/x-positional")) == Some(&json!(0));
    let text = ["query", "text", "name"].into_iter().find(|name| has(name));
    if verb == "search" && text == Some(key) {
        "query"
    } else if first && key == "input" && !has("id") {
        "id"
    } else {
        key
    }
}
pub(super) fn budget(tool: &Value) -> Result<(), String> {
    let length = size(tool);
    // search_trial exposes the complete supported filter set. Even with no descriptions,
    // its supported filter schema structure is 2058 characters; these optional filters cannot fit
    // the ordinary budget. Restore source-owned numeric guidance within the PM-approved exception of 3500.
    // search_diagnostic serves five FDA and NCBI sources with their own filters; 0964's condition filter
    // measured 2432, so the PM set 2800 (sdlc/decisions/2026-10-26-measured-entity-caps-need-no-ask.md,
    // "search_diagnostic action budget for 0964"). get_drug's CIViC evidence significance filter (1008)
    // measured 2517 (j42018); the PM set 2900, measured plus 15 percent (sdlc/decisions/2026-10-26-measured-entity-caps-need-no-ask.md, get_drug section).
    // PM ruling (1127): the search_drug mechanism-of-action growth is accepted.
    // Main measured at most 2905 characters across stdio and hosted catalogs (j44818).
    // The ceiling is that maximum plus 15 percent, rounded up.
    let limit = match field(tool, "name").as_str() {
        Some("search_trial") => 3500,
        Some("search_drug") => 3341,
        Some("search_diagnostic") => 2800,
        Some("get_drug") => 2900,
        _ => ACTION_BUDGET,
    };
    if length > limit {
        Err(format!(
            "{}: {length} characters exceeds {limit}",
            field(tool, "name")
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
            field(tool, "name"),
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
        json!({"mcp_tool_mode":"typed", "mcp_typed_output_schema":true,"mcp_typed_definition_bytes":200000}),
    )?;
    report(&enabled)?;
    if enabled != default {
        return Err("explicit output schemas differ from the default".into());
    }
    for (name, id) in [
        (
            "disease",
            <biomcp_types::DiseaseSites as biomcp_types::ResponseBody>::SCHEMA,
        ),
        (
            "cell_line",
            <biomcp_types::CellLinePage as biomcp_types::ResponseBody>::SCHEMA,
        ),
        (
            "cell_line",
            <biomcp_types::CellLineFilteredPage as biomcp_types::ResponseBody>::SCHEMA,
        ),
    ] {
        let schema = enabled
            .iter()
            .find(|tool| field(tool, "name") == name)
            .and_then(|tool| tool.get("outputSchema"))
            .ok_or("output schema missing")?;
        if !super::super::schema_proof::reply_ids(schema).contains(id) {
            return Err(format!("{name} output schema omits {id}"));
        }
    }
    let disabled = catalog(
        env,
        json!({"mcp_tool_mode":"typed","mcp_typed_output_schema":false}),
    )?;
    for (tool, disabled) in enabled.into_iter().zip(disabled) {
        let mut expected = tool;
        if !matches!(
            field(&expected, "name").as_str(),
            Some("biomcp" | "search" | "fetch")
        ) {
            expected
                .as_object_mut()
                .ok_or("tool object")?
                .remove("outputSchema");
        }
        if expected != disabled {
            return Err("explicit opt-out changed more than the typed output schemas".into());
        }
    }
    enum_settings(env, default)
}
fn enum_settings(env: &Environment, default: &[Value]) -> Result<(), String> {
    let large = crate::runner::protocol::typed_catalog::commands(&catalog(
        env,
        json!({"mcp_tool_mode":"typed", "mcp_typed_enum_limit":1000}),
    )?);
    let default = crate::runner::protocol::typed_catalog::commands(default);
    if !large
        .iter()
        .any(|tool| oversized_enum(field(tool, "inputSchema"), 30))
    {
        return Err("raised enum limit did not restore large choices".into());
    }
    if !default
        .iter()
        .any(|tool| oversized_enum(field(tool, "inputSchema"), 1))
    {
        return Err("short enums lost their choices".into());
    }
    let small = crate::runner::protocol::typed_catalog::commands(&catalog(
        env,
        json!({"mcp_tool_mode":"typed", "mcp_typed_enum_limit":1}),
    )?);
    if small
        .iter()
        .any(|tool| oversized_enum(field(tool, "inputSchema"), 1))
    {
        return Err("enum limit setting ignored".into());
    }
    for tool in &large {
        let name = field(tool, "name").as_str().ok_or("tool name")?;
        let compact = small
            .iter()
            .find(|t| field(t, "name").as_str() == Some(name))
            .ok_or("lowered tool")?;
        for (key, property) in field(field(tool, "inputSchema"), "properties")
            .as_object()
            .ok_or("properties")?
        {
            if oversized_enum(property, 1) {
                let description = field(field(field(compact, "inputSchema"), "properties"), key)
                    .get("description")
                    .and_then(Value::as_str)
                    .ok_or("choice description")?;
                if description.is_empty()
                    || description.contains("; help ")
                    || Some(description) != property.get("description").and_then(Value::as_str)
                {
                    return Err(format!(
                        "{name}.{key}: lowering enum limit changed the standalone description"
                    ));
                }
            }
        }
    }
    Ok(())
}

fn description(node: &Value) -> Option<String> {
    let text = node
        .get("x-section-rule")
        .or_else(|| node.get("description"))?
        .as_str()?;
    let mut lines = text.lines();
    let first = lines.next().unwrap_or_default();
    let text = if first.contains("; help ") {
        lines
            .next()
            .unwrap_or(first)
            .split("; help ")
            .next()
            .unwrap_or(first)
    } else {
        first
    };
    Some(
        text.split(". ")
            .next()
            .unwrap_or_default()
            .trim_end_matches('.')
            .to_owned()
            + ".",
    )
}
