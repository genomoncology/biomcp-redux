use super::launch::Environment;
use serde_json::{Value, json};
use std::process::Stdio;

pub(super) fn metadata() -> Value {
    json!({"io.modelcontextprotocol/protocolVersion":"2026-07-28",
           "io.modelcontextprotocol/clientCapabilities":{}})
}

pub(super) fn request(id: Value, method: &str, mut params: Value) -> Value {
    if let Some(params) = params.as_object_mut() {
        params.insert("_meta".into(), metadata());
    }
    json!({"jsonrpc":"2.0","id":id,"method":method,"params":params})
}

pub(super) fn exchange(environment: &Environment, frames: &[Value]) -> Result<Vec<Value>, String> {
    exchange_with_globals(environment, frames, &[])
}
pub(super) fn exchange_with_globals(
    environment: &Environment,
    frames: &[Value],
    globals: &[String],
) -> Result<Vec<Value>, String> {
    exchange_retained(environment, frames, globals, None)
}
pub(super) fn exchange_retained(
    environment: &Environment,
    frames: &[Value],
    globals: &[String],
    directory: Option<&std::path::Path>,
) -> Result<Vec<Value>, String> {
    exchange_wire(environment, frames, globals, directory)
        .map(|replies| replies.into_iter().map(readable).collect())
}

/// Most trials check a reply's JSON value, not where its form puts it: an error reply's
/// JSON text is also offered to them as `structuredContent`. Reply-form trials read the wire.
pub(super) fn readable(mut reply: Value) -> Value {
    if let Some(result) = reply.get_mut("result").and_then(Value::as_object_mut)
        && result.get("isError") == Some(&json!(true))
        && !result.contains_key("structuredContent")
        && let Some(value) = result
            .get("content")
            .and_then(|content| content.pointer("/0/text"))
            .and_then(Value::as_str)
            .and_then(|text| serde_json::from_str::<Value>(text).ok())
            .filter(Value::is_object)
    {
        result.insert("structuredContent".into(), value);
    }
    reply
}

/// The replies exactly as the server sent them.
pub(super) fn exchange_wire(
    environment: &Environment,
    frames: &[Value],
    globals: &[String],
    directory: Option<&std::path::Path>,
) -> Result<Vec<Value>, String> {
    let mut args = vec!["serve".into()];
    args.extend_from_slice(globals);
    let mut transcript_args = args.clone();
    transcript_args.push("<requests>".into());
    transcript_args.extend(
        frames
            .iter()
            .map(serde_json::to_string)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?,
    );
    let mut command =
        environment.command_with_arguments(env!("CARGO_BIN_EXE_biomcp"), &args, &transcript_args);
    let child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let (status, stdout, stderr) = super::process::exchange(child, frames)?;
    command
        .record(status, &stdout, "mcp")
        .map_err(|e| e.to_string())?;
    if let Some(directory) = directory {
        std::fs::write(directory.join("mcp.stdout"), &stdout).map_err(|e| e.to_string())?;
        std::fs::write(directory.join("mcp.stderr"), &stderr).map_err(|e| e.to_string())?;
    }
    super::process::messages(status, &stdout, &stderr)
}

pub(super) fn call_with_globals(
    environment: &Environment,
    args: &[String],
    is_error: bool,
    globals: &[String],
) -> Result<Value, String> {
    call_retained(environment, args, is_error, globals, None)
}
pub(super) fn call_retained(
    environment: &Environment,
    args: &[String],
    is_error: bool,
    globals: &[String],
    directory: Option<&std::path::Path>,
) -> Result<Value, String> {
    call_text_retained(environment, args, is_error, globals, directory).map(|(value, _)| value)
}

pub(super) fn call_text_with_globals(
    environment: &Environment,
    args: &[String],
    is_error: bool,
    globals: &[String],
) -> Result<(Value, String), String> {
    call_text_retained(environment, args, is_error, globals, None)
}

fn call_text_retained(
    environment: &Environment,
    args: &[String],
    is_error: bool,
    globals: &[String],
    directory: Option<&std::path::Path>,
) -> Result<(Value, String), String> {
    let replies = exchange_retained(
        environment,
        &[request(
            json!(1),
            "tools/call",
            json!({"name":"biomcp","arguments":{"args":args}}),
        )],
        globals,
        directory,
    )?;
    let reply = replies
        .first()
        .filter(|_| replies.len() == 1)
        .ok_or("MCP expected one reply")?;
    if reply.get("id") != Some(&json!(1)) {
        return Err("MCP correlation".into());
    }
    let result = reply
        .get("result")
        .ok_or_else(|| format!("MCP tool error: {reply}"))?;
    if result.get("isError") != Some(&json!(is_error)) {
        return Err("MCP is_error mismatch".into());
    }
    let structured = result
        .get("structuredContent")
        .ok_or("MCP missing structuredContent")?;
    let value = one_value(result)?;
    Ok((value, structured.to_string()))
}

/// One HTTP tool call's reply exactly as the server sent it.
pub(super) fn http_wire(
    server: &super::http::Server,
    call: &Value,
    extra: &str,
) -> Result<Value, String> {
    let name = call.pointer("/params/name").and_then(Value::as_str);
    let headers = format!(
        "Content-Type: application/json\r\nAccept: application/json, text/event-stream\r\n\
         MCP-Protocol-Version: 2026-07-28\r\nMcp-Method: tools/call\r\n{}{extra}",
        name.map(|name| format!("Mcp-Name: {name}\r\n"))
            .unwrap_or_default()
    );
    let host = server.address.to_string();
    let (status, body, _) = server.raw("POST", "/mcp", &host, &headers, &call.to_string())?;
    if status != 200 {
        return Err(format!("HTTP tool call failed: {status}"));
    }
    body.ok_or_else(|| "HTTP reply missing".into())
}

/// A tool result's JSON value; a text copy, when the form sends one, must equal it.
pub(super) fn one_value(result: &Value) -> Result<Value, String> {
    let text = result.pointer("/content/0/text").and_then(Value::as_str);
    let parsed = text
        .map(serde_json::from_str::<Value>)
        .transpose()
        .map_err(|e| e.to_string())?;
    match (result.get("structuredContent"), parsed) {
        (Some(value), Some(copy)) if value != &copy => {
            Err("tool text differs from structured result".into())
        }
        (Some(value), _) => Ok(value.clone()),
        (None, Some(copy)) => Ok(copy),
        (None, None) => Err(format!("tool result carries no JSON: {result}")),
    }
}
