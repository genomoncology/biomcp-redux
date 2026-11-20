//! Hosted FDA PDF admission, memory, and worker cleanup through real HTTP calls.
use super::{http::Server, launch::Environment};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    fs,
    io::Write as _,
    net::TcpStream,
    thread,
    time::{Duration, Instant},
};

type Error = Box<dyn std::error::Error>;
const GUARD: Duration = Duration::from_secs(30);
const BUDGET: u64 = 90;
const HANGING: &str = "P150047";
const RECORDED: &str = "P170019";

pub(super) fn trials() -> Vec<Trial> {
    vec![
        super::transcript_trial("mcp-hosted/pdf-read-cap", || {
            capacity().map_err(|e| e.to_string().into())
        }),
        super::transcript_trial("fda-document-text/pdf-worker-dies-with-server", || {
            parent_death().map_err(|e| e.to_string().into())
        }),
    ]
}

fn server() -> Result<(Server, Environment), Error> {
    let env = Environment::new()?;
    let replay = env.root().join("replay");
    let directory = replay.join("fda-pma");
    fs::create_dir_all(&directory)?;
    for name in ["pma-summary-find", "fda-documents"] {
        let input = super::launch::fixtures()
            .join("replies/diagnostic")
            .join(name)
            .join("fda-pma");
        for file in fs::read_dir(input)? {
            let file = file?;
            let destination = directory.join(file.file_name());
            if name == "pma-summary-find" && file.path().extension().is_some_and(|e| e == "bin") {
                fs::write(destination, b"%PDF-1.7\n%BIOMCP-TEST hang\n")?;
            } else {
                fs::copy(file.path(), destination)?;
            }
        }
    }
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
    super::mcp::request(
        json!(id),
        "tools/call",
        json!({"name": "biomcp", "arguments": {
            "args": ["get", "diagnostic", document, "summary", "--source", "fda-pma"]
        }}),
    )
    .to_string()
}

fn start_read(server: &Server) -> Result<TcpStream, Error> {
    let address = server.address;
    let mut stream = TcpStream::connect(address)?;
    let body = request(1, HANGING);
    write!(
        stream,
        "POST /mcp HTTP/1.1\r\nHost: {address}\r\nContent-Type: application/json\r\n\
         Accept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2025-11-25\r\n\
         Connection: close\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    )?;
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

fn held_worker(server: &Server) -> Result<u32, Error> {
    until("the held reader", || Ok(workers(server)?.first().copied()))
}

fn content(reply: &Value) -> Result<&Value, Error> {
    reply
        .pointer("/result/structuredContent")
        .ok_or_else(|| format!("missing tool reply: {reply}").into())
}

fn capacity() -> Result<(), Error> {
    let (mut server, _replay) = server()?;
    let held = start_read(&server)?;
    let pid = held_worker(&server)?;
    let (_, overflow) = server.request("POST", "/mcp", &request(2, RECORDED))?;
    let overflow = content(&overflow)?;
    if !overflow.to_string().to_ascii_lowercase().contains("retry") {
        return Err(format!("read above the cap did not say to retry: {overflow}").into());
    }
    if workers(&server)? != [pid] {
        return Err("an extra worker started while the hosted PDF slot was held".into());
    }
    let status = fs::read_to_string(format!("/proc/{pid}/status"))?;
    let peak: u64 = status
        .lines()
        .find_map(|line| line.strip_prefix("VmHWM:"))
        .and_then(|line| line.split_whitespace().next())
        .ok_or("the held reader has no resident memory high-water mark")?
        .parse()?;
    if peak == 0 || peak >= 170 * 1024 {
        return Err(
            format!("one hosted reader peaked at {peak} KiB, expected under 170 MiB").into(),
        );
    }
    drop(held);
    until("the cancelled reader to be reaped", || {
        Ok(workers(&server)?.is_empty().then_some(()))
    })?;
    let (_, resumed) = server.request("POST", "/mcp", &request(3, RECORDED))?;
    let resumed = content(&resumed)?;
    if resumed.pointer("/result/document/state") != Some(&json!("text")) {
        return Err(
            format!("after cancellation, the next read did not return text: {resumed}").into(),
        );
    }
    server.stop()?;
    Ok(())
}

fn parent_death() -> Result<(), Error> {
    let (mut server, _replay) = server()?;
    let _held = start_read(&server)?;
    let worker = Orphan(held_worker(&server)?);
    server.child.kill()?;
    server.child.wait()?;
    until("the reader to die with its server", || {
        let status = fs::read_to_string(format!("/proc/{}/status", worker.0));
        match status {
            Ok(status) if status.lines().any(|line| line.starts_with("State:\tZ")) => Ok(Some(())),
            Ok(_) => Ok(None),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Some(())),
            Err(error) => Err(error.into()),
        }
    })
}

/// Clean up a worker when this test runs against a binary that leaves it behind.
struct Orphan(u32);
impl Drop for Orphan {
    fn drop(&mut self) {
        if let Ok(pid) = i32::try_from(self.0) {
            let _ = nix::sys::signal::kill(
                nix::unistd::Pid::from_raw(pid),
                nix::sys::signal::Signal::SIGKILL,
            );
        }
    }
}
