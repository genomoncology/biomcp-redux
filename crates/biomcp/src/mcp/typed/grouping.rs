//! One clinical catalog policy shared by discovery and executable follow-ups.
use serde_json::{Value, json};
use std::io;

fn direct(name: &str) -> bool {
    name.split_once('_').is_some_and(|(verb, entity)| {
        matches!(verb, "search" | "get")
            && matches!(entity, "drug" | "trial" | "article" | "variant" | "disease" | "diagnostic")
            || verb == "get" && entity == "gene"
            || verb == "search" && entity == "adverse_event"
    })
}

pub(super) fn catalog(
    context: crate::command::Context<'_>,
    read_only: bool,
    commands: Vec<Value>,
) -> io::Result<Vec<Value>> {
    let (mut tools, grouped): (Vec<_>, Vec<_>) = commands
        .into_iter()
        .partition(|tool| tool["name"].as_str().is_some_and(direct));
    tools.push(reference(&grouped));
    tools.push(super::help::tool());
    tools.extend(super::super::protocol::tools(context.root, read_only)?);
    let (tool_limit, byte_limit) = crate::config::mcp_typed_budget(context.settings);
    let bytes = serde_json::to_vec(&tools)?.len();
    if tools.len() > tool_limit || bytes > byte_limit {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!(
            "Typed catalog requires {} tools and {bytes} definition bytes; mcp_typed_tool_limit={tool_limit}, mcp_typed_definition_bytes={byte_limit}. Clinical tools and complete descriptions are retained; configure sufficient budgets.",
            tools.len(),
        )));
    }
    Ok(tools)
}

fn reference(commands: &[Value]) -> Value {
    let names = commands.iter().map(|tool| tool["name"].clone()).collect::<Vec<_>>();
    let read_only = commands.iter().all(|tool| tool.pointer("/annotations/readOnlyHint") == Some(&json!(true)));
    json!({"name":"biomcp_reference",
        "description":"Run specialist reference commands and additional entity operations with their typed arguments. IDs and options depend on command; read biomcp_help with its CLI path for accepted ID forms and examples. Example: {\"command\":\"get_protein\",\"arguments\":{\"id\":\"P04637\"}}.",
        "inputSchema":{"$schema":super::super::protocol::SCHEMA_DIALECT,"type":"object",
            "properties":{
                "command":{"type":"string","enum":names,"description":"Registered typed command name, such as get_protein. Use biomcp_help for its argument schema."},
                "arguments":{"type":"object","additionalProperties":true,"description":"The selected command's typed arguments; example for get_protein: {\"id\":\"P04637\"}."}},
            "required":["command","arguments"],"additionalProperties":false},
        "annotations":{"readOnlyHint":read_only,"destructiveHint":false,"openWorldHint":true}})
}

// Unwrap without introducing another validator or argument-to-CLI converter.
pub(super) fn unwrap(params: &Value) -> Option<Value> {
    let fields = params.get("arguments")?.as_object()?;
    if fields.len() != 2 || !fields.get("arguments")?.is_object() {
        return None;
    }
    Some(json!({"name":fields.get("command")?.as_str()?,"arguments":fields.get("arguments")?}))
}

pub(super) fn call(canonical: Value, tools: &[Value]) -> Option<Value> {
    let name = canonical.get("name")?.as_str()?;
    if tools.iter().any(|tool| tool["name"] == name) {
        return Some(canonical);
    }
    let reference = tools.iter().find(|tool| tool["name"] == "biomcp_reference")?;
    if !reference.pointer("/inputSchema/properties/command/enum")?.as_array()?.contains(&json!(name)) {
        return None;
    }
    Some(json!({"name":"biomcp_reference","arguments":{"command":name,"arguments":canonical.get("arguments")?}}))
}
