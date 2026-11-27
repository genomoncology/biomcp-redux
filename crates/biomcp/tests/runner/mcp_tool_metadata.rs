//! Check display and retry metadata over the wire.
use super::{Environment, reply, request};
use crate::runner::mcp::exchange_with_globals;
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|error| error.to_string())?;
    let mut compact = request(json!("compact"), "tools/list", json!({}));
    compact
        .pointer_mut("/params/_meta")
        .and_then(Value::as_object_mut)
        .ok_or("missing request metadata")?
        .insert("biomcp/compactCatalog".into(), json!(true));
    let frames = [
        request(json!("discover"), "server/discover", json!({})),
        request(json!("typed"), "tools/list", json!({})),
        compact,
    ];
    let flags = ["--offline".into(), "--tool-mode".into(), "typed".into()];
    let replies = exchange_with_globals(&env, &frames, &flags)?;
    let discover = reply(&replies, &json!("discover"))?;
    if discover.pointer("/result/_meta/io.modelcontextprotocol~1serverInfo/title")
        != Some(&json!("BioMCP"))
    {
        return Err("discovery serverInfo must have a display title".into());
    }
    check_catalog(&replies, "typed", "/title", 18)?;
    check_catalog(&replies, "compact", "/title", 2)?;

    for (revision, title_path, server_title) in [
        ("2025-06-18", "/title", Some(json!("BioMCP"))),
        ("2025-03-26", "/annotations/title", None),
    ] {
        let frames = [
            json!({"jsonrpc":"2.0","id":"init","method":"initialize","params":{
                "protocolVersion":revision,"capabilities":{},"clientInfo":{"name":"test","version":"1"}}}),
            json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
            json!({"jsonrpc":"2.0","id":"typed","method":"tools/list"}),
        ];
        let replies = exchange_with_globals(&env, &frames, &flags)?;
        let init = reply(&replies, &json!("init"))?;
        if init.pointer("/result/protocolVersion") != Some(&json!(revision))
            || init.pointer("/result/serverInfo/title") != server_title.as_ref()
        {
            return Err(format!("wrong initialize metadata for {revision}"));
        }
        check_catalog(&replies, "typed", title_path, 18)?;
    }
    Ok(())
}

fn check_catalog(
    replies: &[Value],
    id: &str,
    title_path: &str,
    count: usize,
) -> Result<(), String> {
    let listed = reply(replies, &json!(id))?;
    let tools = listed
        .pointer("/result/tools")
        .and_then(Value::as_array)
        .filter(|tools| tools.len() == count)
        .ok_or_else(|| format!("{id}: expected {count} tools"))?;
    for tool in tools {
        let name = tool.get("name").and_then(Value::as_str).ok_or("tool name")?;
        let title = tool.pointer(title_path).and_then(Value::as_str);
        if !title.is_some_and(|title| title.starts_with(char::is_uppercase))
            || title_path != "/title" && tool.get("title").is_some()
        {
            return Err(format!("{name}: expected a display title at {title_path}"));
        }
        let read_only = tool
            .pointer("/annotations/readOnlyHint")
            .and_then(Value::as_bool)
            .ok_or_else(|| format!("{name}: missing readOnlyHint"))?;
        let expected = match name {
            "help" | "search" | "fetch" | "search_tools" => true,
            "saved" | "biomcp" | "call_tool" => false,
            _ => read_only,
        };
        if read_only != expected
            || tool.pointer("/annotations/idempotentHint")
                != expected.then_some(Value::Bool(true)).as_ref()
        {
            return Err(format!(
                "{name}: idempotentHint must be true only for read-only tools"
            ));
        }
    }
    Ok(())
}
