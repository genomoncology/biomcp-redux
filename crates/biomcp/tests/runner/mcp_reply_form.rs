//! Configured reply forms preserve JSON values under matching transport policies.
use super::super::{
    http::Server,
    launch::Environment,
    mcp::{exchange_wire, http_wire, request},
};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::io::Write;

pub(super) fn trials() -> Vec<Trial> {
    vec![transcript_trial("mcp-parity/reply-forms", || {
        check().map_err(Into::into)
    })]
}

pub(super) fn check() -> Result<(), String> {
    for hosted in [false, true] {
        let mut globals = vec!["--offline".into()];
        if hosted {
            globals.push("--hosted".into());
        }
        let mut reference = Vec::new();
        for form in [
            None,
            Some("once"),
            Some("both"),
            Some("structured-only"),
            Some("text-only"),
        ] {
            let config = form.map_or(
                json!({"hosted_max_use":"academic"}),
                |form| json!({"mcp_reply_form":form,"hosted_max_use":"academic"}),
            );
            let environment = Environment::new().map_err(|e| e.to_string())?;
            environment.config(&config).map_err(|e| e.to_string())?;
            let calls = [
                request(
                    json!(1),
                    "tools/call",
                    json!({"name":"biomcp","arguments":{"args":["help"]}}),
                ),
                request(
                    json!(2),
                    "tools/call",
                    json!({"name":"search","arguments":{"query":"article --limit invalid"}}),
                ),
                request(
                    json!(3),
                    "tools/call",
                    json!({"name":"fetch","arguments":{"id":"invalid-id"}}),
                ),
                request(
                    json!(4),
                    "tools/call",
                    json!({"name":"biomcp","arguments":{"args":["invalid-command"]}}),
                ),
            ];
            let replies = exchange_wire(&environment, &calls, &globals, None)?;
            let server = Server::configured_globals(
                "127.0.0.1:0",
                Some(config),
                None,
                None,
                None,
                &globals,
            )?;
            for (index, call) in calls.iter().enumerate() {
                let stdio = replies
                    .iter()
                    .find(|reply| reply.get("id") == call.get("id"))
                    .and_then(|reply| reply.get("result"))
                    .ok_or("stdio result missing")?;
                let http = http_reply(&server, call, hosted)?;
                let default = if hosted { "both" } else { "once" };
                let value = check_pair(stdio, &http, form.unwrap_or(default), index)
                    .map_err(|error| format!("hosted={hosted}: {error}"))?;
                if form.is_none() {
                    reference.push(value);
                } else if reference.get(index) != Some(&value) {
                    return Err("reply form changed JSON value".into());
                }
                writeln!(
                    std::io::stdout(),
                    "MCP_REPLY hosted={hosted} form={} call={index} bytes={} chars={}",
                    form.unwrap_or("default"),
                    stdio.to_string().len(),
                    stdio.to_string().chars().count()
                )
                .map_err(|e| e.to_string())?;
            }
        }
    }
    invalid_settings()
}

fn http_reply(server: &Server, call: &Value, hosted: bool) -> Result<Value, String> {
    let extra = if hosted {
        "x-biomcp-use: academic\r\n"
    } else {
        ""
    };
    http_wire(server, call, extra)?
        .get("result")
        .cloned()
        .ok_or("HTTP result missing".into())
}

fn check_pair(stdio: &Value, http: &Value, form: &str, index: usize) -> Result<Value, String> {
    let value = check_content(stdio, form)?;
    let failed = json!(index != 0);
    if stdio.get("isError") != Some(&failed) || http.get("isError") != Some(&failed) {
        return Err("reply form lost tool error status".into());
    }
    let http_value = check_content(http, form)?;
    if let Some(difference) = super::super::assertions::difference(&value, &http_value, "") {
        if let Some(context) = super::super::transcript::current() {
            context.record(
                "trial/mcp-parity/reply-forms/mismatch",
                json!({
                    "kind":"reply-form-mismatch", "form":form, "call":index,
                    "stdio":&value, "http":&http_value
                }),
            )?;
        }
        return Err(format!(
            "stdio/HTTP values differ form={form} call={index}: {difference}"
        ));
    }
    Ok(value)
}

fn invalid_settings() -> Result<(), String> {
    for value in [json!("unsupported-reply-form"), json!(42)] {
        let environment = Environment::new().map_err(|e| e.to_string())?;
        environment
            .config(&json!({"mcp_reply_form":value}))
            .map_err(|e| e.to_string())?;
        let output = environment
            .command(
                env!("CARGO_BIN_EXE_biomcp"),
                &[
                    "serve".into(),
                    "--http".into(),
                    "127.0.0.1:0".into(),
                    "--offline".into(),
                ],
            )
            .output()
            .map_err(|e| e.to_string())?;
        let text = String::from_utf8_lossy(&output.stdout);
        if output.status.success()
            || !text.contains("mcp_reply_form")
            || text.contains("unsupported-reply-form")
        {
            return Err("invalid MCP reply form did not name only its field".into());
        }
    }
    Ok(())
}

/// The reply's JSON value, checked to sit where `form` puts it, once, with nothing before it.
pub(super) fn check_content(result: &Value, form: &str) -> Result<Value, String> {
    let blocks = result
        .get("content")
        .and_then(Value::as_array)
        .ok_or("content list missing")?;
    let structured = result.get("structuredContent");
    let failed = result.get("isError") == Some(&json!(true));
    if form == "once" && !failed {
        if !blocks.is_empty() {
            return Err(format!("once repeats JSON as text: {result}"));
        }
        return structured.cloned().ok_or("structured JSON missing".into());
    }
    let [block] = blocks.as_slice() else {
        return Err("expected one content block".into());
    };
    let text = block
        .get("text")
        .and_then(Value::as_str)
        .ok_or("text missing")?;
    if block.get("type") != Some(&json!("text")) {
        return Err("expected text block".into());
    }
    if form == "structured-only" {
        if text != "JSON reply is in structuredContent." {
            return Err("structured note differs".into());
        }
        return structured.cloned().ok_or("structured JSON missing".into());
    }
    // The whole text is the JSON value: no status line or summary precedes it.
    let value: Value = serde_json::from_str(text).map_err(|e| format!("{e}: {text:.200}"))?;
    match (form, structured) {
        ("both", Some(copy)) if copy == &value => Ok(value),
        ("both", _) => Err("JSON copies differ".into()),
        ("once" | "text-only", None) => Ok(value),
        ("once" | "text-only", Some(_)) => Err(format!("{form} repeats JSON")),
        _ => Err("unknown test form".into()),
    }
}
