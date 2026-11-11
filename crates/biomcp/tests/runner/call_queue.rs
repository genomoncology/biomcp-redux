//! Feature mcp-call-queue: calls over the concurrency limit wait their turn and then run.
//! The timed refusals live with the fixtures that hold calls: the held proxy and HTTP limits.
use super::{launch::Environment, mcp::request};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};

const LIMIT: usize = 32;
const CALLS: u64 = 64;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("mcp-call-queue/stdio-burst-over-limit-completes", || {
            burst().map_err(Into::into)
        }),
        #[cfg(unix)]
        transcript_trial("mcp-call-queue/hosted-call-waits-for-held-slot", || {
            hosted_turn().map_err(Into::into)
        }),
    ]
}

fn burst() -> Result<(), String> {
    let environment = Environment::new().map_err(|e| e.to_string())?;
    // The wait is generous so a loaded machine cannot turn a queued call into a refusal.
    environment
        .config(&json!({"mcp_max_calls":LIMIT, "mcp_queue_wait_seconds":30,
            "command_timeout_seconds":30}))
        .map_err(|e| e.to_string())?;
    let replay = super::launch::fixtures().join("replies/gene/main");
    let globals = environment.globals(Some(replay.as_path()), false);
    let args = ["get", "gene", "KRAS"].map(String::from);
    // A recorded exchange answers once; this first call stores it for the burst to share.
    super::mcp::call_with_globals(&environment, &args, false, &globals)?;
    let frames = (1..=CALLS)
        .map(|id| {
            request(
                json!(id),
                "tools/call",
                json!({"name":"biomcp", "arguments":{"args":args}}),
            )
        })
        .collect::<Vec<_>>();
    let replies = super::mcp::exchange_with_globals(&environment, &frames, &globals)?;
    let mut answered = std::collections::BTreeSet::new();
    for reply in &replies {
        let content = reply.pointer("/result/structuredContent");
        if reply.pointer("/result/isError") != Some(&json!(false))
            || content.and_then(|content| content.get("status")) != Some(&json!("complete"))
            || content.and_then(|content| content.pointer("/result/items/0/answer/gene/symbol"))
                != Some(&json!("KRAS"))
        {
            let shown = content.map_or_else(
                || reply.to_string(),
                |content| {
                    format!(
                        "status {:?} sources {:?} error {:?}",
                        content.get("status"),
                        content.get("sources"),
                        content.get("error")
                    )
                },
            );
            return Err(format!("a call in the burst did not complete: {shown}"));
        }
        answered.insert(reply.get("id").and_then(Value::as_u64));
    }
    if replies.len() != frames.len() || !(1..=CALLS).all(|id| answered.contains(&Some(id))) {
        return Err(format!(
            "{} replies for {CALLS} calls, {} distinct",
            replies.len(),
            answered.len()
        ));
    }
    Ok(())
}

#[cfg(unix)]
fn send(address: std::net::SocketAddr, body: &str) -> Result<std::net::TcpStream, String> {
    use std::io::Write;
    let mut stream = std::net::TcpStream::connect(address).map_err(|e| e.to_string())?;
    let frame = format!(
        "POST /mcp HTTP/1.1\r\nHost: {address}\r\nContent-Type: application/json\r\n\
         Accept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2026-07-28\r\n\
         Mcp-Method: tools/call\r\nMcp-Name: biomcp\r\nConnection: close\r\n\
         Content-Length: {}\r\n\r\n{body}",
        body.len()
    );
    stream
        .write_all(frame.as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(stream)
}

/// One call holds the only slot; the next gets no answer until that slot frees, then runs.
#[cfg(unix)]
fn hosted_turn() -> Result<(), String> {
    use std::{
        io::{Read, Write},
        time::Duration,
    };
    let config = json!({"http_max_calls":1, "http_max_connections":4,
        "mcp_queue_wait_seconds":60, "command_timeout_seconds":60});
    let server = super::http::Server::configured_globals(
        "127.0.0.1:0",
        Some(config.clone()),
        None,
        None,
        None,
        &["--hosted".into()],
    )?;
    let call = |id: u64, args: Value| {
        request(
            json!(id),
            "tools/call",
            json!({"name":"biomcp", "arguments":{"args":args}}),
        )
        .to_string()
    };
    let first = call(1, json!(["get", "gene", "KRAS", "--offline"]));
    let (_held, mut writer) = super::http_limits::hold_settings_reader(&server, &first)?;
    let second = call(2, json!(["help"]));
    let mut waiting = send(server.address, &second)?;
    // A queued call is silent however long this pause runs; only a refusal can break it.
    waiting
        .set_read_timeout(Some(Duration::from_millis(500)))
        .map_err(|e| e.to_string())?;
    let mut response = Vec::new();
    let mut early = [0; 4096];
    match waiting.read(&mut early) {
        Err(error)
            if matches!(
                error.kind(),
                std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
            ) => {}
        Ok(read) => {
            return Err(format!(
                "the second call was answered while the only slot was held: {}",
                String::from_utf8_lossy(early.get(..read).unwrap_or_default())
            ));
        }
        Err(error) => return Err(error.to_string()),
    }
    // The freed worker and the queued call both read ordinary settings from here on.
    std::fs::remove_file(
        server
            .environment()
            .root()
            .join("config/biomcp/config.json"),
    )
    .map_err(|e| e.to_string())?;
    server
        .environment()
        .config(&config)
        .map_err(|e| e.to_string())?;
    writer.write_all(b"{}").map_err(|e| e.to_string())?;
    drop(writer);
    waiting
        .set_read_timeout(Some(Duration::from_secs(60)))
        .map_err(|e| e.to_string())?;
    waiting
        .read_to_end(&mut response)
        .map_err(|e| format!("the queued call got no answer after the slot freed: {e}"))?;
    let response = String::from_utf8(response).map_err(|e| e.to_string())?;
    let (_, body) = response
        .split_once("\r\n\r\n")
        .ok_or("queued call response has no body")?;
    let reply: Value = serde_json::from_str(body).map_err(|e| e.to_string())?;
    if reply.get("id") != Some(&json!(2)) || reply.pointer("/result/isError") != Some(&json!(false))
    {
        return Err(format!("the queued call did not run: {reply}"));
    }
    Ok(())
}
