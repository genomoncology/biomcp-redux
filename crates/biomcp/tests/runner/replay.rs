//! Feature gene: replay misses stay source failures and repeated server calls use checked cache.
use super::{configuration::invoke, launch::Environment, mcp};
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
        Trial::test("gene/replay-missing-recording-reason", || {
            missing().map_err(Into::into)
        }),
        Trial::test("gene/replay-server-reuses-checked-reply", || {
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
    let reply = invoke(&env, args, 5)?;
    if !reply
        .get("sources")
        .and_then(Value::as_array)
        .is_some_and(|sources| {
            sources
                .iter()
                .any(|s| s.get("reason") == Some(&json!("replay_miss")))
        })
    {
        return Err(format!("recording miss lost its source reason: {reply}"));
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
    for (id, gene) in [(1, "BRAF"), (2, "BRAF"), (3, "UNRECORDED")] {
        let frame = mcp::request(
            json!(id),
            "tools/call",
            json!({"name":"biomcp","arguments":{"args":["get","gene",gene]}}),
        );
        serde_json::to_writer(&mut *input, &frame).map_err(|e| e.to_string())?;
        input.write_all(b"\n").map_err(|e| e.to_string())?;
        input.flush().map_err(|e| e.to_string())?;
        let line = rx
            .recv_timeout(Duration::from_secs(30))
            .map_err(|_| "reply watchdog")??;
        let reply: Value = serde_json::from_str(&line).map_err(|e| e.to_string())?;
        let result = reply.get("result").ok_or("missing result")?;
        if reply.get("id") != Some(&json!(id)) || result.get("isError") != Some(&json!(id == 3)) {
            return Err(format!("replay call failed: {reply}"));
        }
        let content = result.get("structuredContent").ok_or("missing content")?;
        if id == 1 {
            first = Some(content.clone());
        }
        if id == 2 && first.as_ref() != Some(content) {
            return Err("repeat changed reply".into());
        }
    }
    Ok(())
}
