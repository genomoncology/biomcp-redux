//! Feature mcp-hosted: the call timer covers a blocked settings read before source work.
use libtest_mimic::Trial;

pub(super) fn trials() -> Vec<Trial> {
    #[cfg(unix)]
    {
        vec![super::transcript::trial(
            "mcp-hosted/pacing/whole-call-deadline",
            || blocked_settings().map_err(Into::into),
        )]
    }
    #[cfg(not(unix))]
    {
        Vec::new()
    }
}

#[cfg(unix)]
fn blocked_settings() -> Result<(), String> {
    use super::http::Server;
    use nix::{sys::stat::Mode, unistd::mkfifo};
    use std::{
        fs::OpenOptions,
        io::Write,
        os::unix::fs::OpenOptionsExt,
        time::{Duration, Instant},
    };
    let mut server = Server::configured_globals(
        "127.0.0.1:0",
        Some(serde_json::json!({})),
        None,
        None,
        None,
        &["--hosted".into()],
    )?;
    let path = server
        .environment()
        .root()
        .join("config/biomcp/config.json");
    std::fs::remove_file(&path).map_err(|e| e.to_string())?;
    mkfifo(&path, Mode::S_IRUSR | Mode::S_IWUSR).map_err(|e| e.to_string())?;
    let address = server.address;
    let workers: Vec<_> = (0..8)
        .map(|_| std::thread::spawn(move || request(address)))
        .collect();
    // A successful nonblocking writer open witnesses the blocked reader's admission.
    let end = Instant::now() + Duration::from_secs(5);
    let mut writer = loop {
        match OpenOptions::new()
            .write(true)
            .custom_flags(nix::libc::O_NONBLOCK)
            .open(&path)
        {
            Ok(writer) => break writer,
            Err(error)
                if error.raw_os_error() == Some(nix::libc::ENXIO) && Instant::now() < end =>
            {
                std::thread::yield_now();
            }
            Err(error) => {
                std::fs::remove_file(&path).map_err(|e| e.to_string())?;
                server
                    .environment()
                    .config(&serde_json::json!({}))
                    .map_err(|e| e.to_string())?;
                for worker in workers {
                    let _ = worker.join();
                }
                return Err(format!("settings reader did not enter the FIFO: {error}"));
            }
        }
    };
    // The response must arrive while the settings read still has no EOF.
    let replies = workers
        .into_iter()
        .map(|worker| worker.join().map_err(|_| "deadline client stopped"))
        .collect::<Result<Vec<_>, _>>()?;
    let capacity = request(address);
    writer.write_all(b"{}").map_err(|e| e.to_string())?;
    drop(writer);
    std::fs::remove_file(&path).map_err(|e| e.to_string())?;
    server
        .environment()
        .config(&serde_json::json!({}))
        .map_err(|e| e.to_string())?;
    verify(replies, capacity)?;
    recovered(address)?;
    server.stop()
}

#[cfg(unix)]
fn request(address: std::net::SocketAddr) -> Result<serde_json::Value, String> {
    use std::io::{Read, Write};
    let body = serde_json::json!({"jsonrpc":"2.0","id":1,"method":"tools/call",
        "params":{"name":"biomcp","arguments":{"args":["get","gene","KRAS","--offline"]}}})
    .to_string();
    let mut stream = std::net::TcpStream::connect(address).map_err(|e| e.to_string())?;
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(30)))
        .map_err(|e| e.to_string())?;
    stream.write_all(format!("POST /mcp HTTP/1.1\r\nHost: {address}\r\nContent-Type: application/json\r\nAccept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2025-11-25\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{body}", body.len()).as_bytes()).map_err(|e| e.to_string())?;
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|e| format!("whole-call response failed while settings remained blocked: {e}"))?;
    let (_, body) = response
        .split_once("\r\n\r\n")
        .ok_or("deadline response has no body")?;
    serde_json::from_str(body).map_err(|e| e.to_string())
}

#[cfg(unix)]
fn verify(
    replies: Vec<Result<serde_json::Value, String>>,
    capacity: Result<serde_json::Value, String>,
) -> Result<(), String> {
    for reply in replies {
        let reply = reply?;
        if reply.pointer("/error/data/reason") != Some(&serde_json::json!("deadline-exceeded")) {
            return Err(format!(
                "whole call did not return the typed timeout: {reply}"
            ));
        }
    }
    let capacity = capacity?;
    if capacity.pointer("/error/code") != Some(&serde_json::json!(-31000)) {
        return Err(format!(
            "timed-out running work released server capacity: {capacity}"
        ));
    }
    Ok(())
}
#[cfg(unix)]
fn recovered(address: std::net::SocketAddr) -> Result<(), String> {
    let end = std::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        let reply = request(address)?;
        if reply.get("result").is_some() {
            return Ok(());
        }
        if reply.pointer("/error/code") != Some(&serde_json::json!(-31000))
            || std::time::Instant::now() >= end
        {
            return Err(format!(
                "service did not recover after timed-out settings reads: {reply}"
            ));
        }
        std::thread::yield_now();
    }
}
