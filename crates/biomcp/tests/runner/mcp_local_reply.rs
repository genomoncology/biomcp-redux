//! Feature mcp-parity: local clients can read a successful tool reply as text.
use super::super::{
    http::Server,
    launch::Environment,
    mcp::{exchange_wire, request},
};
use super::reply_form::check_content;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};

const LEGACY: &str = "2025-03-26";
const CURRENT: &str = "2026-07-28";

pub(super) fn trials() -> Vec<Trial> {
    [LEGACY, CURRENT]
        .into_iter()
        .map(|revision| {
            transcript_trial(
                format!("mcp-parity/local-reply-text/{revision}"),
                move || check(revision).map_err(Into::into),
            )
        })
        .collect()
}

fn check(revision: &str) -> Result<(), String> {
    let default = replies(revision, None, "both")?;
    let once = replies(revision, Some(json!({"mcp_reply_form":"once"})), "once")?;
    if default != once {
        return Err("explicit once changed the structured reply".into());
    }
    Ok(())
}

fn replies(revision: &str, config: Option<Value>, expected: &str) -> Result<[Value; 2], String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    if let Some(config) = &config {
        env.config(config).map_err(|e| e.to_string())?;
    }
    let globals = env.globals(None, true);
    let server = Server::configured_globals("127.0.0.1:0", config, None, None, None, &globals)?;
    let params = json!({"name":"biomcp","arguments":{"args":["help"]}});
    let call = if revision == LEGACY {
        json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":params})
    } else {
        request(json!(2), "tools/call", params)
    };
    let mut frames = Vec::new();
    if revision == LEGACY {
        frames.extend([
            json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{
                "protocolVersion":revision,"capabilities":{},"clientInfo":{"name":"local-proof","version":"1"}}}),
            json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
        ]);
    }
    frames.push(call.clone());
    let stdio = exchange_wire(&env, &frames, &globals, None)?;
    if revision == LEGACY
        && stdio
            .first()
            .and_then(|reply| reply.pointer("/result/protocolVersion"))
            != Some(&json!(revision))
    {
        return Err("stdio did not negotiate the requested revision".into());
    }
    let stdio = stdio
        .iter()
        .find(|reply| reply.get("id") == Some(&json!(2)))
        .ok_or("stdio tool reply missing")?;
    let headers = format!(
        "Content-Type: application/json\r\nAccept: application/json, text/event-stream\r\n\
         MCP-Protocol-Version: {revision}\r\nMcp-Method: tools/call\r\nMcp-Name: biomcp\r\n"
    );
    let (status, http, _) = server.raw(
        "POST",
        "/mcp",
        &server.address.to_string(),
        &headers,
        &call.to_string(),
    )?;
    if status != 200 {
        return Err(format!("HTTP tool call failed: {status}"));
    }
    Ok([
        checked(stdio, revision, expected).map_err(|e| format!("stdio {e}"))?,
        checked(&http.ok_or("HTTP tool reply missing")?, revision, expected)
            .map_err(|e| format!("HTTP {e}"))?,
    ])
}

fn checked(reply: &Value, revision: &str, expected: &str) -> Result<Value, String> {
    let result = reply.get("result").ok_or("tool result missing")?;
    if result.get("isError") != Some(&json!(false)) {
        return Err("help call did not succeed".into());
    }
    if result.get("resultType").is_some() != (revision == CURRENT) {
        return Err("tool reply used the wrong protocol revision".into());
    }
    check_content(result, Some(expected))
}
