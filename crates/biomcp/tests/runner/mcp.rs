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
    let mut args = vec!["serve".into()];
    args.extend_from_slice(globals);
    let transcript_args = frames
        .iter()
        .map(serde_json::to_string)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    let mut command =
        environment.command_with_arguments(env!("CARGO_BIN_EXE_biomcp"), &args, &transcript_args);
    let child = command
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
    command
        .record(status, &stdout, "mcp")
        .map_err(|e| e.to_string())?;
    if let Some(directory) = directory {
        std::fs::write(directory.join("mcp.stdout"), &stdout).map_err(|e| e.to_string())?;
        std::fs::write(directory.join("mcp.stderr"), &stderr).map_err(|e| e.to_string())?;
    }
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
    let text = result
        .pointer("/content/0/text")
        .and_then(Value::as_str)
        .ok_or("MCP missing text")?;
    let parsed: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    if &parsed != structured {
        return Err("MCP text differs from structuredContent".into());
    }
    Ok((structured.clone(), text.to_owned()))
}

// A persistent server must start each command at its completed replay epoch,
// while concurrent completions advance to their maximum endpoint only once.
pub(super) fn article_sequence(
    environment: &Environment,
    globals: &[String],
) -> Result<(), String> {
    use std::{
        io::{BufRead, BufReader},
        sync::mpsc,
        thread,
        time::Duration,
    };
    let mut args = vec!["serve".into()];
    args.extend_from_slice(globals);
    let mut child = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let mut stdin = child.stdin.take().ok_or("stdin missing")?;
    let stdout = child.stdout.take().ok_or("stdout missing")?;
    let mut process = super::process::Process::new(child)?;
    let (sender, receiver) = mpsc::channel();
    let reader = thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            if sender.send(line.map_err(|e| e.to_string())).is_err() {
                break;
            }
        }
    });
    let result: Result<(), String> = (|| {
        let mut pools = std::collections::BTreeSet::new();
        for batch in (1..=24)
            .map(|id| vec![id])
            .chain(std::iter::once(vec![25, 26]))
        {
            for id in &batch {
                let frame = request(
                    json!(id),
                    "tools/call",
                    json!({"name":"biomcp","arguments":{"args":["search","article","AKT1 E17K","--limit","2","--fresh"]}}),
                );
                serde_json::to_writer(&mut stdin, &frame).map_err(|e| e.to_string())?;
                stdin.write_all(b"\n").map_err(|e| e.to_string())?;
            }
            stdin.flush().map_err(|e| e.to_string())?;
            let mut ids = std::collections::BTreeSet::new();
            for _ in &batch {
                let line = receiver
                    .recv_timeout(Duration::from_secs(30))
                    .map_err(|_| "MCP reply watchdog")??;
                check_article_reply(&line, &batch, &mut ids, &mut pools)?;
            }
        }
        Ok(())
    })();
    if result.is_err() {
        let _ = process.child.kill();
    }
    drop(stdin);
    let finish = process.finish();
    reader.join().map_err(|_| "reply reader panicked")?;
    result?;
    let (status, _, stderr) = finish?;
    if !status.success() || !stderr.is_empty() {
        return Err("persistent article server failed".into());
    }
    Ok(())
}

fn check_article_reply(
    line: &str,
    batch: &[u64],
    ids: &mut std::collections::BTreeSet<u64>,
    pools: &mut std::collections::BTreeSet<String>,
) -> Result<(), String> {
    let reply: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
    let id = reply
        .get("id")
        .and_then(Value::as_u64)
        .ok_or("correlation missing")?;
    if !batch.contains(&id)
        || !ids.insert(id)
        || reply.pointer("/result/isError") != Some(&json!(false))
        || reply.pointer("/result/structuredContent/status") != Some(&json!("complete"))
    {
        return Err(format!("persistent article replay failed: {reply}"));
    }
    let pool = reply
        .pointer("/result/structuredContent/page/pool/id")
        .and_then(Value::as_str)
        .ok_or("pool missing")?;
    if !pools.insert(pool.to_owned()) {
        return Err("fresh command reused pool".into());
    }
    Ok(())
}
