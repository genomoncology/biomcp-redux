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
        #[cfg(unix)]
        transcript_trial("mcp-call-queue/hosted-wait-ends-at-call-deadline", || {
            hosted_deadline().map_err(Into::into)
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
    let args = ["search", "gene", "--band", "9p21.3"].map(String::from);
    // A recorded exchange answers once per process, so the burst reads the recorded HGNC
    // bundle, which this first call stores.
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
            || content.and_then(|content| content.pointer("/page/total")) != Some(&json!(65))
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
    // No connection setting: the hosted default must leave the waiting call a connection.
    let config = json!({"http_max_calls":1, "mcp_queue_wait_seconds":60,
        "command_timeout_seconds":60});
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

/// The call deadline is far shorter than the queue wait, so the refusal comes at the deadline.
#[cfg(unix)]
fn hosted_deadline() -> Result<(), String> {
    let config = json!({"http_max_calls":1, "mcp_queue_wait_seconds":120,
        "command_timeout_seconds":2});
    let server = super::http::Server::configured_globals(
        "127.0.0.1:0",
        Some(config),
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
    // The held worker keeps the only slot after its own deadline, until its writer drops.
    let (_held, _writer) = super::http_limits::hold_settings_reader(&server, &first)?;
    let sent = std::time::Instant::now();
    let mut waiting = send(server.address, &call(2, json!(["help"])))?;
    waiting
        .set_read_timeout(Some(std::time::Duration::from_secs(90)))
        .map_err(|e| e.to_string())?;
    let mut response = String::new();
    std::io::Read::read_to_string(&mut waiting, &mut response)
        .map_err(|e| format!("the waiting call got no answer: {e}"))?;
    let waited = sent.elapsed();
    let (_, body) = response
        .split_once("\r\n\r\n")
        .ok_or("refused call response has no body")?;
    let reply: Value = serde_json::from_str(body).map_err(|e| e.to_string())?;
    if reply.pointer("/error/code") != Some(&json!(-31000)) || !super::capacity_refusal(&reply, 1) {
        return Err(format!("no timed refusal at the call deadline: {reply}"));
    }
    if waited >= std::time::Duration::from_secs(60) {
        return Err(format!("the call waited {waited:?}, past its two-second deadline"));
    }
    Ok(())
}
