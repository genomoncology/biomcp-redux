//! Feature gene-identity: replay misses stay source failures and repeated server calls use checked cache.
use super::{launch::Environment, mcp};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    io::{BufRead, BufReader, Write},
    process::Stdio,
    sync::mpsc,
    thread,
    time::Duration,
};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("gene/replay-missing-recording-reason", || {
            missing().map_err(Into::into)
        }),
        transcript_trial("gene/replay-server-reuses-checked-reply", || {
            repeated().map_err(Into::into)
        }),
    ]
}
fn missing() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let directory = env.root().join("empty");
    std::fs::create_dir(&directory).map_err(|e| e.to_string())?;
    let mut args = env.globals(Some(&directory), false);
    args.extend(["get".into(), "gene".into(), "BRAF".into()]);
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(5) || !output.stderr.is_empty() {
        return Err("missing replay command failed".into());
    }
    // Measured raw CLI output: 394 bytes, with fifteen percent headroom.
    if output.stdout.len() > 454 {
        return Err("replay CLI output exceeded its budget".into());
    }
    let reply: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if !replay_miss(&reply) {
        return Err("recording miss lost its source reason".into());
    }
    Ok(())
}
fn repeated() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let mut args = vec!["serve".into()];
    args.extend(env.globals(Some(&super::configuration::root()), false));
    let mut child = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let mut input = child.stdin.take().ok_or("stdin missing")?;
    let output = child.stdout.take().ok_or("stdout missing")?;
    let mut process = super::process::Process::new(child)?;
    let (tx, rx) = mpsc::channel();
    let reader = thread::spawn(move || {
        for line in BufReader::new(output).lines() {
            if tx.send(line.map_err(|e| e.to_string())).is_err() {
                break;
            }
        }
    });
    let result = repeat_calls(&mut input, &rx);
    if result.is_err() {
        let _ = process.child.kill();
    }
    drop(input);
    let finish = process.finish();
    reader.join().map_err(|_| "reader panicked")?;
    result?;
    let (status, _, stderr) = finish?;
    if !status.success() || !stderr.is_empty() {
        return Err("server failed".into());
    }
    Ok(())
}
fn repeat_calls(
    input: &mut impl Write,
    rx: &mpsc::Receiver<Result<String, String>>,
) -> Result<(), String> {
    let mut first = None;
    // Measured MCP text: 2365, 2365, 2672 and 1428 bytes, plus fifteen percent.
    for (id, args, limit) in [
        (1, vec!["get", "gene", "BRAF"], 2720),
        (2, vec!["get", "gene", "BRAF"], 2720),
        (3, vec!["get", "gene", "BRAF", "protein"], 3073),
        (4, vec!["get", "gene", "BRAF", "--fresh"], 1643),
    ] {
        let frame = mcp::request(
            json!(id),
            "tools/call",
            json!({"name":"biomcp","arguments":{"args":args}}),
        );
        serde_json::to_writer(&mut *input, &frame).map_err(|e| e.to_string())?;
        input.write_all(b"\n").map_err(|e| e.to_string())?;
        input.flush().map_err(|e| e.to_string())?;
        let line = rx
            .recv_timeout(Duration::from_secs(30))
            .map_err(|_| "reply watchdog")??;
        let reply: Value = serde_json::from_str(&line).map_err(|e| e.to_string())?;
        let result = reply.get("result").ok_or("missing result")?;
        if reply.get("id") != Some(&json!(id)) || result.get("isError") != Some(&json!(id >= 3)) {
            return Err(format!("replay call failed: {reply}"));
        }
        let content = result.get("structuredContent").ok_or("missing content")?;
        let text = result
            .pointer("/content/0/text")
            .and_then(Value::as_str)
            .ok_or("missing text")?;
        if serde_json::from_str::<Value>(text).map_err(|e| e.to_string())? != *content {
            return Err("text differs from structured content".into());
        }
        if text.len() > limit {
            return Err("replay MCP text exceeded its budget".into());
        }
        if id >= 3 && !replay_miss(content) {
            return Err("new or exhausted request lost replay_miss".into());
        }
        if id == 1 {
            first = Some(content.clone());
        }
        if id == 2 && first.as_ref() != Some(content) {
            return Err("repeat changed reply".into());
        }
    }
    Ok(())
}

fn replay_miss(reply: &Value) -> bool {
    reply
        .get("sources")
        .and_then(Value::as_array)
        .is_some_and(|sources| {
            sources
                .iter()
                .any(|source| source.get("reason") == Some(&json!("replay-miss")))
        })
}
