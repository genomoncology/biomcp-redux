//! Hosted FDA PDF admission, memory, and worker cleanup through real HTTP calls.
use super::super::{self as runner, http::Server, launch::Environment};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    fs,
    io::{Read as _, Write as _},
    net::TcpStream,
    thread,
    time::{Duration, Instant},
};

#[path = "pdf_hosted_history.rs"]
mod history;

type Error = Box<dyn std::error::Error>;
const GUARD: Duration = Duration::from_secs(30);
const BUDGET: u64 = 8;
const HANGING: &str = "P150047";
const RECORDED: &str = "P170019";

pub(super) fn trials() -> Vec<Trial> {
    vec![
        runner::transcript_trial("mcp-hosted/pdf-read-cap", || {
            capacity().map_err(|e| e.to_string().into())
        }),
        runner::transcript_trial("mcp-hosted/pdf-waits-for-a-short-read", || {
            waits_for_release().map_err(|e| e.to_string().into())
        }),
        runner::transcript_trial("drug-label-history/pdf-busy-leaves-version-unread", || {
            history::busy(false).map_err(|e| e.to_string().into())
        }),
        runner::transcript_trial(
            "drug-label-history/pdf-busy-after-match-leaves-history-open",
            || history::busy(true).map_err(|e| e.to_string().into()),
        ),
        runner::transcript_trial("fda-document-text/pdf-worker-dies-with-server", || {
            parent_death().map_err(|e| e.to_string().into())
        }),
    ]
}

fn server(oldest_on_hold: bool) -> Result<(Server, Environment), Error> {
    let env = Environment::new()?;
    let replay = env.root().join("replay");
    let directory = replay.join("fda-pma");
    fs::create_dir_all(&directory)?;
    for name in ["pma-summary-find", "fda-documents"] {
        let input = runner::launch::fixtures()
            .join("replies/diagnostic")
            .join(name)
            .join("fda-pma");
        for file in fs::read_dir(input)? {
            let file = file?;
            let destination = directory.join(file.file_name());
            if name == "pma-summary-find" && file.path().extension().is_some_and(|e| e == "bin") {
                let mut pdf = b"%PDF-1.7\n%BIOMCP-TEST hang\n".to_vec();
                pdf.resize(24 * 1024 * 1024, b' ');
                fs::write(destination, pdf)?;
            } else {
                fs::copy(file.path(), destination)?;
            }
        }
    }
    history::fixtures(&replay, oldest_on_hold)?;
    let server = Server::configured_globals(
        "127.0.0.1:0",
        Some(json!({"command_timeout_seconds": BUDGET})),
        Some(replay),
        None,
        None,
        &["--hosted".into()],
    )?;
    Ok((server, env))
}

fn request(id: u32, document: &str) -> String {
    runner::mcp::request(
        json!(id),
        "tools/call",
        json!({"name": "biomcp", "arguments": {
            "args": ["get", "diagnostic", document, "summary", "--source", "fda-pma"]
        }}),
    )
    .to_string()
}

fn start_read(server: &Server) -> Result<TcpStream, Error> {
    send(server, &request(1, HANGING))
}

fn send(server: &Server, body: &str) -> Result<TcpStream, Error> {
    let address = server.address;
    let mut stream = TcpStream::connect(address)?;
    write!(
        stream,
        "POST /mcp HTTP/1.1\r\nHost: {address}\r\nContent-Type: application/json\r\n\
         Accept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2026-07-28\r\n\
         Mcp-Method: tools/call\r\nMcp-Name: biomcp\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    )?;
    stream.set_read_timeout(Some(GUARD))?;
    Ok(stream)
}

fn workers(server: &Server) -> Result<Vec<u32>, Error> {
    let mut workers = Vec::new();
    for task in fs::read_dir(format!("/proc/{}/task", server.child.id()))? {
        let listing = fs::read_to_string(task?.path().join("children"));
        match listing {
            Ok(listing) => {
                for pid in listing.split_whitespace() {
                    workers.push(pid.parse()?);
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }
    workers.sort_unstable();
    workers.dedup();
    Ok(workers)
}

fn until<T>(
    event: &str,
    mut observe: impl FnMut() -> Result<Option<T>, Error>,
) -> Result<T, Error> {
    let end = Instant::now() + GUARD;
    loop {
        if let Some(observed) = observe()? {
            return Ok(observed);
        }
        if Instant::now() >= end {
            return Err(format!("timed out waiting for {event}").into());
        }
        thread::sleep(Duration::from_millis(5));
    }
}

fn peak(pid: u32) -> Result<u64, Error> {
    let status = fs::read_to_string(format!("/proc/{pid}/status"))?;
    Ok(status
        .lines()
        .find_map(|line| line.strip_prefix("VmHWM:"))
        .and_then(|line| line.split_whitespace().next())
        .map(str::parse)
        .transpose()?
        .unwrap_or_default())
}

fn held_worker(server: &Server) -> Result<u32, Error> {
    until("the worker to read the padded PDF", || {
        let Some(pid) = workers(server)?.first().copied() else {
            return Ok(None);
        };
        Ok((peak(pid)? >= 24 * 1024).then_some(pid))
    })
}

fn content(reply: &Value) -> Result<&Value, Error> {
    reply
        .pointer("/result/structuredContent")
        .ok_or_else(|| format!("missing tool reply: {reply}").into())
}

fn retry(reply: &Value) -> bool {
    reply
        .pointer("/sources/0/notices")
        .and_then(Value::as_array)
        .is_some_and(|notices| {
            notices.iter().any(|notice| {
                notice
                    .get("message")
                    .and_then(Value::as_str)
                    .is_some_and(|message| {
                        message.contains("hosted server") && message.contains("Retry shortly")
                    })
            })
        })
}

fn capacity() -> Result<(), Error> {
    let (mut server, _replay) = server(false)?;
    let held = start_read(&server)?;
    let pid = held_worker(&server)?;
    let overflow = receive(send(&server, &request(2, RECORDED))?)?;
    let overflow = content(&overflow)?;
    if !retry(overflow) {
        return Err(format!("read above the cap did not say to retry: {overflow}").into());
    }
    if workers(&server)? != [pid] {
        return Err("an extra worker started while the hosted PDF slot was held".into());
    }
    let peak = peak(pid)?;
    if peak >= 170 * 1024 {
        return Err(
            format!("one hosted reader peaked at {peak} KiB, expected under 170 MiB").into(),
        );
    }
    drop(held);
    until("the cancelled reader to be reaped", || {
        Ok(workers(&server)?.is_empty().then_some(()))
    })?;
    until("the next PDF to return its text", || {
        let (_, resumed) = server.request("POST", "/mcp", &request(3, RECORDED))?;
        let resumed = content(&resumed)?;
        if resumed.pointer("/result/document/state") == Some(&json!("text")) {
            return Ok(Some(()));
        }
        if retry(resumed) {
            return Ok(None);
        }
        Err(format!("after cancellation, the next read did not return text: {resumed}").into())
    })?;
    server.stop()?;
    Ok(())
}

fn receive(mut stream: TcpStream) -> Result<Value, Error> {
    let mut wire = String::new();
    stream.read_to_string(&mut wire)?;
    let (_, body) = wire.split_once("\r\n\r\n").ok_or("missing HTTP reply")?;
    Ok(runner::mcp::readable(serde_json::from_str(body)?))
}

fn pending(stream: &TcpStream) -> Result<(), Error> {
    stream.set_read_timeout(Some(Duration::from_millis(200)))?;
    let result = stream.peek(&mut [0]);
    stream.set_read_timeout(Some(GUARD))?;
    match result {
        Err(error)
            if matches!(
                error.kind(),
                std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
            ) =>
        {
            Ok(())
        }
        other => Err(format!("request did not wait for the PDF slot: {other:?}").into()),
    }
}

fn waits_for_release() -> Result<(), Error> {
    let (mut server, _replay) = server(false)?;
    let held = start_read(&server)?;
    held_worker(&server)?;
    let next = send(&server, &request(2, RECORDED))?;
    pending(&next)?;
    drop(held);
    let reply = receive(next)?;
    if content(&reply)?.pointer("/result/document/state") != Some(&json!("text")) {
        return Err(format!("waiting read did not return text: {reply}").into());
    }
    server.stop()?;
    Ok(())
}

fn parent_death() -> Result<(), Error> {
    let (mut server, _replay) = server(false)?;
    let _held = start_read(&server)?;
    let mut worker = Orphan(Some(held_worker(&server)?));
    server.child.kill()?;
    server.child.wait()?;
    let pid = worker.0.ok_or("missing PDF reader PID")?;
    until("the reader to die with its server", || {
        let status = fs::read_to_string(format!("/proc/{pid}/status"));
        match status {
            Ok(status) if status.lines().any(|line| line.starts_with("State:\tZ")) => Ok(Some(())),
            Ok(_) => Ok(None),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Some(())),
            Err(error) => Err(error.into()),
        }
    })?;
    worker.0 = None;
    Ok(())
}

/// Clean up a worker when this test runs against a binary that leaves it behind.
struct Orphan(Option<u32>);
impl Drop for Orphan {
    fn drop(&mut self) {
        if let Some(pid) = self.0.and_then(|pid| i32::try_from(pid).ok()) {
            let _ = nix::sys::signal::kill(
                nix::unistd::Pid::from_raw(pid),
                nix::sys::signal::Signal::SIGKILL,
            );
        }
    }
}
