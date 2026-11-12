//! The real synthetic span replies carry their coordinate note on the CLI and in default MCP data.
use super::launch::Environment;
use serde_json::{Value, json};

pub(super) fn check(env: &Environment, args: &[&str], first: &Value) -> Result<(), String> {
    let features = args.get(1) == Some(&"features");
    let (pointer, start, end) = if features {
        ("/result/items/0/feature/span", 9, 20)
    } else {
        ("/result/items/0/locations/0/fragments/1/span", 19, 30)
    };
    if first.pointer(&format!("{pointer}/start/min")) != Some(&json!(start))
        || first.pointer(&format!("{pointer}/end/min")) != Some(&json!(end))
    {
        return Err(format!("converted numeric span changed: {first}"));
    }
    let mut words: Vec<_> = args.iter().map(|s| (*s).to_owned()).collect();
    words.extend(["--format".into(), "md".into()]);
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &words)
        .output()
        .map_err(|e| e.to_string())?;
    let markdown = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    if !output.status.success() || !output.stderr.is_empty() {
        return Err(format!("protein coordinate Markdown failed: {markdown}"));
    }
    guidance(&markdown)?;
    let args: Vec<_> = args.iter().map(|s| (*s).to_owned()).collect();
    // The default form sends a success once, as structured data that carries the note itself.
    let messages = super::mcp::exchange(
        env,
        &[super::mcp::request(
            json!(1),
            "tools/call",
            json!({"name":"biomcp","arguments":{"args":args}}),
        )],
    )?;
    let reply = messages.first().ok_or("missing protein MCP reply")?;
    if reply.pointer("/result/content") != Some(&json!([])) {
        return Err(format!("default protein MCP reply sent text: {reply}"));
    }
    let structured = reply
        .pointer("/result/structuredContent")
        .ok_or("protein structured reply")?;
    if structured.pointer(&format!("{pointer}/start/min")) != Some(&json!(start))
        || structured.pointer(&format!("{pointer}/end/min")) != Some(&json!(end))
    {
        return Err(format!("MCP numeric span changed: {reply}"));
    }
    guidance(
        structured
            .pointer("/result/coordinate_note")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("protein MCP reply lacks its coordinate note: {reply}"))?,
    )
}

fn guidance(text: &str) -> Result<(), String> {
    for expected in [
        "zero-based interbase",
        "half-open [start, end)",
        "one-based inclusive",
        "--position selector remains one-based",
    ] {
        if !text.contains(expected) {
            return Err(format!(
                "protein text lacks coordinate guidance {expected:?}: {text}"
            ));
        }
    }
    Ok(())
}
