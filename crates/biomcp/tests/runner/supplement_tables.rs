//! A rejected workbook leaves the same MCP server available to its next caller.
use super::{launch::Environment, mcp, process::Process};
use serde_json::{Value, json};
use std::{
    io::{BufRead, BufReader, Write},
    process::Stdio,
    sync::mpsc,
    thread,
    time::Duration,
};
pub(super) fn ping_after_refusal() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let recording =
        crate::runner::launch::fixtures().join("replies/article/supplements/synthetic-huge-count");
    let mut args = vec!["serve".into()];
    args.extend(env.globals(Some(&recording), false));
    let mut child = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let mut stdin = child.stdin.take().ok_or("stdin missing")?;
    let stdout = child.stdout.take().ok_or("stdout missing")?;
    let mut process = Process::new(child)?;
    let (sender, receiver) = mpsc::channel();
    let reader = thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            if sender.send(line.map_err(|e| e.to_string())).is_err() {
                break;
            }
        }
    });
    let result = (|| {
        let request = mcp::request(
            json!(1),
            "tools/call",
            json!({"name":"biomcp","arguments":{"args":["article","supplement","28944238","MGG3-5-553-s002.xlsx"]}}),
        );
        send(&mut stdin, &request)?;
        let reply = receive(&receiver)?;
        if reply.get("id") != Some(&json!(1))
            || reply.pointer("/result/structuredContent/result/content/reason")
                != Some(&json!("too_large"))
        {
            return Err(format!("workbook was not bounded: {reply}"));
        }
        send(&mut stdin, &mcp::request(json!(2), "ping", json!({})))?;
        let ping = receive(&receiver)?;
        if ping.get("id") != Some(&json!(2))
            || ping.pointer("/result/resultType") != Some(&json!("complete"))
        {
            return Err(format!("MCP unavailable after refusal: {ping}"));
        }
        Ok(())
    })();
    if result.is_err() {
        let _ = process.child.kill();
    }
    drop(stdin);
    let finish = process.finish();
    reader.join().map_err(|_| "reader failed")?;
    result?;
    let (status, _, stderr) = finish?;
    if !status.success() || !stderr.is_empty() {
        return Err("MCP failed after bounded workbook refusal".into());
    }
    Ok(())
}
fn send(stdin: &mut impl Write, value: &Value) -> Result<(), String> {
    serde_json::to_writer(&mut *stdin, value).map_err(|e| e.to_string())?;
    stdin
        .write_all(b"\n")
        .and_then(|()| stdin.flush())
        .map_err(|e| e.to_string())
}
fn receive(receiver: &mpsc::Receiver<Result<String, String>>) -> Result<Value, String> {
    let line = receiver
        .recv_timeout(Duration::from_secs(30))
        .map_err(|_| "MCP reply watchdog")??;
    serde_json::from_str(&line).map_err(|e| e.to_string())
}
