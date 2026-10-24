use super::launch::Environment;
use serde_json::{Value, json};
use std::{io::Write, process::Stdio};

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
    let child = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &["serve".into()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let mut process = super::process::Process::new(child)?;
    let mut stdin = process.child.stdin.take().ok_or("stdin missing")?;
    for frame in frames {
        serde_json::to_writer(&mut stdin, frame).map_err(|e| e.to_string())?;
        stdin.write_all(b"\n").map_err(|e| e.to_string())?;
    }
    drop(stdin);
    let (status, stdout, stderr) = process.finish()?;
    if !status.success() || !stderr.is_empty() {
        return Err(format!(
            "MCP process: {:?}; {}",
            status.code(),
            String::from_utf8_lossy(&stderr)
        ));
    }
    stdout
        .split(|b| *b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice(line).map_err(|e| e.to_string()))
        .collect()
}

pub(super) fn call(
    environment: &Environment,
    args: &[String],
    is_error: bool,
) -> Result<Value, String> {
    let replies = exchange(
        environment,
        &[request(
            json!(1),
            "tools/call",
            json!({"name":"biomcp","arguments":{"args":args}}),
        )],
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
    let text = result
        .pointer("/content/0/text")
        .and_then(Value::as_str)
        .ok_or("MCP missing text")?;
    let parsed: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    if &parsed != structured {
        return Err("MCP text differs from structuredContent".into());
    }
    Ok(structured.clone())
}
