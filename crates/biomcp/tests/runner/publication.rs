//! Both installed processes must accept searches while one ordinary cache is locked.
use super::{launch::Environment, process::Process};
use serde_json::{Value, json};
use std::{
    io::{BufRead, BufReader, Write},
    process::{ChildStdin, Stdio},
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

pub(super) struct Peer {
    process: Option<Process>,
    input: Option<ChildStdin>,
    replies: Receiver<Result<Value, String>>,
    reader: Option<thread::JoinHandle<()>>,
}
impl Peer {
    pub(super) fn with_globals(env: &Environment, globals: &[String]) -> Result<Self, String> {
        let mut args = vec!["serve".into()];
        args.extend_from_slice(globals);
        let mut child = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;
        let input = child.stdin.take();
        let output = child.stdout.take().ok_or("stdout missing")?;
        let (tx, replies) = mpsc::channel();
        let reader = thread::spawn(move || {
            for line in BufReader::new(output).lines() {
                let value = line.map_err(|e| e.to_string()).and_then(|line| {
                    let value: Value = serde_json::from_str(&line).map_err(|e| e.to_string())?;
                    if value.get("jsonrpc") != Some(&json!("2.0")) {
                        return Err("non-JSON-RPC stdout".into());
                    }
                    Ok(value)
                });
                if tx.send(value).is_err() {
                    break;
                }
            }
        });
        Ok(Self {
            process: Some(Process::new(child)?),
            input,
            replies,
            reader: Some(reader),
        })
    }
    pub(super) fn send(&mut self, frame: &Value) -> Result<(), String> {
        let writer = self.input.as_mut().ok_or("stdin missing")?;
        serde_json::to_writer(&mut *writer, frame).map_err(|e| e.to_string())?;
        writer
            .write_all(b"\n")
            .and_then(|()| writer.flush())
            .map_err(|e| e.to_string())
    }
    pub(super) fn next(&self) -> Result<Value, String> {
        self.replies
            .recv_timeout(Duration::from_secs(30))
            .map_err(|_| "publication watchdog")?
    }
    pub(super) fn finish(mut self) -> Result<(), String> {
        drop(self.input.take());
        let (status, _, stderr) = self.process.take().ok_or("process missing")?.finish()?;
        self.reader
            .take()
            .ok_or("reader missing")?
            .join()
            .map_err(|_| "reader panicked")?;
        if !status.success() || !stderr.is_empty() || self.replies.try_recv().is_ok() {
            return Err("extra stdout or unsuccessful MCP shutdown".into());
        }
        Ok(())
    }
}
impl Drop for Peer {
    fn drop(&mut self) {
        if let Some(process) = &mut self.process {
            let _ = process.child.kill();
            let _ = process.child.wait();
        }
    }
}
