//! Feature mcp-call-queue: calls over the concurrency limit wait their turn and then run.
//! A blocked settings read holds a slot: every call rereads its credentials from that file.
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
        transcript_trial("mcp-call-queue/stdio-full-queue-refuses-at-once", || {
            stdio_full().map_err(Into::into)
        }),
        #[cfg(unix)]
        transcript_trial("mcp-call-queue/stdio-wait-ends-at-call-deadline", || {
            stdio_deadline().map_err(Into::into)
        }),
        #[cfg(unix)]
        transcript_trial("mcp-call-queue/hosted-call-waits-for-held-slot", || {
            turn(true).map_err(Into::into)
        }),
        #[cfg(unix)]
        transcript_trial("mcp-call-queue/plain-http-call-waits-for-held-slot", || {
            turn(false).map_err(Into::into)
        }),
        #[cfg(unix)]
        transcript_trial(
            "mcp-call-queue/hosted-client-that-leaves-gives-up-its-place",
            || hosted_left().map_err(Into::into),
        ),
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
fn turn(hosted: bool) -> Result<(), String> {
    use std::{
        io::{Read, Write},
        time::Duration,
    };
    // No connection setting: the default must leave the waiting call a connection.
    let config = json!({"http_max_calls":1, "mcp_queue_wait_seconds":60,
        "command_timeout_seconds":60});
    let globals = if hosted {
        vec!["--hosted".into()]
    } else {
        Vec::new()
    };
    let server = super::http::Server::configured_globals(
        "127.0.0.1:0",
        Some(config.clone()),
        None,
        None,
        None,
        &globals,
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
        return Err(format!(
            "the call waited {waited:?}, past its two-second deadline"
        ));
    }
    Ok(())
}

/// A client that closes its connection while it waits is sent away at once, not at its deadline.
#[cfg(unix)]
fn hosted_left() -> Result<(), String> {
    let config = json!({"http_max_calls":1, "mcp_queue_wait_seconds":120,
        "command_timeout_seconds":120});
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
    let (_held, _writer) = super::http_limits::hold_settings_reader(
        &server,
        &call(1, json!(["get", "gene", "KRAS", "--offline"])),
    )?;
    drop(send(server.address, &call(2, json!(["help"])))?);
    // The server logs each finished tool call; the held call cannot finish, so the first line
    // is the call that left. Kept waiting, it would log nothing until its two-minute deadline.
    let diagnostics = server
        .diagnostics
        .lock()
        .map_err(|_| "diagnostic receiver poisoned")?;
    let line = diagnostics
        .recv_timeout(std::time::Duration::from_secs(60))
        .map_err(|_| "the call whose client left kept its place")?;
    let event: Value = serde_json::from_str(&line).map_err(|e| format!("{e}: {line}"))?;
    if event.get("event") != Some(&json!("mcp_call"))
        || event.get("status") != Some(&json!("error"))
    {
        return Err(format!(
            "the call whose client left was not sent away: {event}"
        ));
    }
    Ok(())
}

/// A stdio server whose calls block reading settings until the test writes them.
#[cfg(unix)]
struct Stdio {
    environment: Environment,
    config: Value,
    stdin: std::process::ChildStdin,
    replies: std::sync::mpsc::Receiver<Result<Value, String>>,
    process: super::process::Process,
    reader: std::thread::JoinHandle<()>,
}

#[cfg(unix)]
impl Stdio {
    fn start(config: Value) -> Result<Self, String> {
        use std::io::BufRead;
        let environment = Environment::new().map_err(|e| e.to_string())?;
        environment.config(&config).map_err(|e| e.to_string())?;
        let mut child = environment
            .command(env!("CARGO_BIN_EXE_biomcp"), &["serve".into()])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;
        let stdin = child.stdin.take().ok_or("missing stdin")?;
        let stdout = child.stdout.take().ok_or("missing stdout")?;
        let process = super::process::Process::new(child)?;
        let (sender, replies) = std::sync::mpsc::channel();
        let reader = std::thread::spawn(move || {
            for line in std::io::BufReader::new(stdout).lines() {
                let reply = line
                    .map_err(|e| e.to_string())
                    .and_then(|line| serde_json::from_str(&line).map_err(|e| e.to_string()));
                if sender.send(reply).is_err() {
                    break;
                }
            }
        });
        Ok(Self {
            environment,
            config,
            stdin,
            replies,
            process,
            reader,
        })
    }
    fn send(&mut self, id: &str, args: Value) -> Result<(), String> {
        use std::io::Write;
        let frame = request(
            json!(id),
            "tools/call",
            json!({"name":"biomcp", "arguments":{"args":args}}),
        );
        let mut bytes = serde_json::to_vec(&frame).map_err(|e| e.to_string())?;
        bytes.push(b'\n');
        self.stdin.write_all(&bytes).map_err(|e| e.to_string())?;
        self.stdin.flush().map_err(|e| e.to_string())
    }
    fn reply(&self) -> Result<Value, String> {
        self.replies
            .recv_timeout(std::time::Duration::from_secs(90))
            .map_err(|_| "no reply from the stdio server".to_string())?
    }
    /// Send a call that blocks reading settings, and return once it holds its slot.
    fn hold(&mut self) -> Result<std::fs::File, String> {
        use std::os::unix::fs::OpenOptionsExt;
        // A ping answers only after startup, so startup has read the ordinary settings.
        {
            use std::io::Write;
            let ping = request(json!("ping"), "ping", json!({}));
            writeln!(self.stdin, "{ping}").map_err(|e| e.to_string())?;
            self.stdin.flush().map_err(|e| e.to_string())?;
        }
        let pong = self.reply()?;
        if pong.get("id") != Some(&json!("ping")) {
            return Err(format!("no ping reply: {pong}"));
        }
        let path = self.settings_path();
        std::fs::remove_file(&path).map_err(|e| e.to_string())?;
        nix::unistd::mkfifo(
            &path,
            nix::sys::stat::Mode::S_IRUSR | nix::sys::stat::Mode::S_IWUSR,
        )
        .map_err(|e| e.to_string())?;
        self.send("held", json!(["get", "gene", "KRAS", "--offline"]))?;
        // A writer cannot open until the held call has opened the FIFO for reading.
        let end = std::time::Instant::now() + std::time::Duration::from_secs(60);
        loop {
            match std::fs::OpenOptions::new()
                .write(true)
                .custom_flags(nix::libc::O_NONBLOCK)
                .open(&path)
            {
                Ok(writer) => return Ok(writer),
                Err(error)
                    if error.raw_os_error() == Some(nix::libc::ENXIO)
                        && std::time::Instant::now() < end =>
                {
                    std::thread::yield_now();
                }
                Err(error) => return Err(format!("the held call did not read settings: {error}")),
            }
        }
    }
    fn settings_path(&self) -> std::path::PathBuf {
        self.environment.root().join("config/biomcp/config.json")
    }
    /// Restore ordinary settings, free the held call, and stop the server.
    fn release(self, writer: std::fs::File) -> Result<Vec<Value>, String> {
        use std::io::Write;
        std::fs::remove_file(self.settings_path()).map_err(|e| e.to_string())?;
        self.environment
            .config(&self.config)
            .map_err(|e| e.to_string())?;
        let mut writer = writer;
        writer.write_all(b"{}").map_err(|e| e.to_string())?;
        drop(writer);
        drop(self.stdin);
        let (status, _, stderr) = self.process.finish()?;
        self.reader.join().map_err(|_| "reply reader panicked")?;
        if !status.success() {
            return Err(format!(
                "stdio server failed: {status} {}",
                String::from_utf8_lossy(&stderr)
            ));
        }
        self.replies.try_iter().collect()
    }
}

/// With the only slot held and the one queue place taken, the next call is refused at once.
#[cfg(unix)]
fn stdio_full() -> Result<(), String> {
    // Neither wait nor deadline can end before the test frees the held call.
    let mut server = Stdio::start(json!({"mcp_max_calls":1, "mcp_queue_max_calls":1,
        "mcp_queue_wait_seconds":120, "command_timeout_seconds":120}))?;
    let writer = server.hold()?;
    server.send("queued", json!(["help"]))?;
    server.send("over", json!(["help"]))?;
    let refused = server.reply()?;
    if refused.get("id") != Some(&json!("over"))
        || refused.pointer("/error/code") != Some(&json!(-31000))
        || !super::capacity_refusal(&refused, 1)
        || !refused
            .pointer("/error/message")
            .and_then(Value::as_str)
            .is_some_and(|message| message.contains("waited 0 seconds"))
    {
        return Err(format!(
            "no refusal at once past the queue length: {refused}"
        ));
    }
    let replies = server.release(writer)?;
    // The queued call kept its place and ran once the held call finished.
    let queued = replies
        .iter()
        .find(|reply| reply.get("id") == Some(&json!("queued")))
        .ok_or_else(|| format!("the queued call got no reply: {replies:?}"))?;
    if queued.pointer("/result/isError") != Some(&json!(false)) {
        return Err(format!("the queued call did not run: {queued}"));
    }
    Ok(())
}

/// The command deadline is far shorter than the queue wait, so the refusal comes at the deadline.
#[cfg(unix)]
fn stdio_deadline() -> Result<(), String> {
    let mut server = Stdio::start(json!({"mcp_max_calls":1, "mcp_queue_wait_seconds":120,
        "command_timeout_seconds":2}))?;
    let writer = server.hold()?;
    let sent = std::time::Instant::now();
    server.send("queued", json!(["help"]))?;
    let refused = server.reply()?;
    let waited = sent.elapsed();
    if refused.get("id") != Some(&json!("queued"))
        || refused.pointer("/error/code") != Some(&json!(-31000))
        || !super::capacity_refusal(&refused, 1)
        || !refused
            .pointer("/error/message")
            .and_then(Value::as_str)
            .is_some_and(|message| message.contains("waited 2 seconds"))
    {
        return Err(format!("no timed refusal at the call deadline: {refused}"));
    }
    // The server refuses only once the wait has run, so this bound cannot race.
    if waited < std::time::Duration::from_secs(2) || waited >= std::time::Duration::from_secs(60) {
        return Err(format!(
            "the queued call waited {waited:?}, not its two-second deadline"
        ));
    }
    server.release(writer)?;
    Ok(())
}
