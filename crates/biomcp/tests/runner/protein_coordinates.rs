//! Render the real synthetic span replies through both public text surfaces.
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
    // The default form sends a success's JSON alone; a form with JSON text adds the note.
    let path = env.root().join("config/biomcp/config.json");
    let original = std::fs::read(&path).ok();
    let mut config: Value = original
        .as_deref()
        .and_then(|bytes| serde_json::from_slice(bytes).ok())
        .unwrap_or_else(|| json!({}));
    config
        .as_object_mut()
        .ok_or("configuration object")?
        .insert("mcp_reply_form".into(), json!("both"));
    env.config(&config).map_err(|e| e.to_string())?;
    let messages = super::mcp::exchange(
        env,
        &[super::mcp::request(
            json!(1),
            "tools/call",
            json!({"name":"biomcp","arguments":{"args":args}}),
        )],
    );
    match original {
        Some(bytes) => std::fs::write(&path, bytes),
        None => std::fs::remove_file(&path),
    }
    .map_err(|e| e.to_string())?;
    let messages = messages?;
    let reply = messages.first().ok_or("missing protein MCP reply")?;
    let structured = reply
        .pointer("/result/structuredContent")
        .ok_or("protein structured reply")?;
    if structured.pointer(&format!("{pointer}/start/min")) != Some(&json!(start))
        || structured.pointer(&format!("{pointer}/end/min")) != Some(&json!(end))
    {
        return Err(format!("MCP numeric span changed: {reply}"));
    }
    let blocks = reply
        .pointer("/result/content")
        .and_then(Value::as_array)
        .ok_or("protein MCP text")?;
    let first_text = blocks
        .first()
        .and_then(|v| v.get("text"))
        .and_then(Value::as_str)
        .ok_or("MCP JSON text")?;
    let parsed: Value = serde_json::from_str(first_text).map_err(|e| e.to_string())?;
    if &parsed != structured {
        return Err("coordinate guidance changed the MCP JSON text contract".into());
    }
    let text = blocks
        .iter()
        .filter_map(|v| v.get("text").and_then(Value::as_str))
        .collect::<Vec<_>>()
        .join("\n");
    guidance(&text)
}

fn guidance(text: &str) -> Result<(), String> {
    for expected in [
        "Zero-based interbase",
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
