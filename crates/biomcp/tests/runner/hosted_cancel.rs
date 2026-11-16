//! Feature mcp-hosted: a running hosted HTTP call stops when its client closes the connection.
//! HTTP ignores `notifications/cancelled`: a client cancels by closing its connection.
use libtest_mimic::Trial;

pub(super) fn trials() -> Vec<Trial> {
    #[cfg(unix)]
    {
        vec![super::transcript::trial(
            "mcp-hosted/client-that-leaves-stops-running-call",
            || client_left().map_err(Into::into),
        )]
    }
    #[cfg(not(unix))]
    {
        Vec::new()
    }
}

/// One call holds the only slot on a held source request. A cancel notification leaves it
/// running; closing its connection closes the source request and frees the slot long before
/// the call's deadline, so the next call runs at once.
#[cfg(unix)]
fn client_left() -> Result<(), String> {
    use serde_json::{Value, json};
    use std::net::TcpListener;
    let certs = super::launch::Environment::new().map_err(|e| e.to_string())?;
    let listener = TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    let proxy = listener.local_addr().map_err(|e| e.to_string())?;
    let (certificate, _) = super::network_stub::identity(&["rest.uniprot.org"])?;
    let pem = certs.root().join("source.pem");
    std::fs::write(&pem, certificate).map_err(|e| e.to_string())?;
    // The deadline and the queue wait outlast every check here: only a client leaving frees the slot.
    let server = super::http::Server::configured_globals(
        "127.0.0.1:0",
        Some(json!({"proxy":format!("http://{proxy}"), "ca_bundle":pem, "http_max_calls":1,
            "mcp_queue_wait_seconds":60, "command_timeout_seconds":120})),
        None,
        None,
        None,
        &["--hosted".into()],
    )?;
    let call = |id: u64, args: Value| {
        super::mcp::request(
            json!(id),
            "tools/call",
            json!({"name":"biomcp", "arguments":{"args":args}}),
        )
        .to_string()
    };
    let client = super::call_queue::send(
        server.address,
        &call(1, json!(["get", "protein", "P15056"])),
    )?;
    let mut held = accept(&listener)?;
    let cancel = json!({"jsonrpc":"2.0", "method":"notifications/cancelled",
        "params":{"requestId":1}});
    let (status, _) = server.request("POST", "/mcp", &cancel.to_string())?;
    if status != 202 {
        return Err(format!("cancel notification got HTTP {status}, not 202"));
    }
    // A notification has no caller to match, so the running call keeps its source request.
    held.set_nonblocking(true).map_err(|e| e.to_string())?;
    match super::socket_read::read(&mut held, &mut [0]) {
        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => (),
        other => return Err(format!("a cancel notification closed the call: {other:?}")),
    }
    held.set_nonblocking(false).map_err(|e| e.to_string())?;
    drop(client);
    super::held_proxy::peer_eof(&mut held)
        .map_err(|e| format!("the source request outlived its client: {e}"))?;
    // The request's own five-second read limit bounds the wait for the freed slot.
    let (status, reply) = server.request("POST", "/mcp", &call(2, json!(["help"])))?;
    if status != 200
        || reply.get("id") != Some(&json!(2))
        || reply.pointer("/result/isError") != Some(&json!(false))
    {
        return Err(format!("the slot stayed held after its client left: {reply}"));
    }
    Ok(())
}

/// Take the held call's CONNECT to the source, and keep its socket open.
#[cfg(unix)]
fn accept(listener: &std::net::TcpListener) -> Result<std::net::TcpStream, String> {
    let end = std::time::Instant::now() + std::time::Duration::from_secs(30);
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    let stream = loop {
        match listener.accept() {
            Ok((stream, _)) => break stream,
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                if std::time::Instant::now() >= end {
                    return Err("the call made no source request".into());
                }
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            Err(error) => return Err(error.to_string()),
        }
    };
    let (connected, result) = std::sync::mpsc::channel();
    let stream = super::held_proxy::proxy_request(stream, &connected)?;
    result.recv().map_err(|e| e.to_string())??;
    Ok(stream)
}
