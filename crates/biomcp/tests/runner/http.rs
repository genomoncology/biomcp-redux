//! MCP HTTP through the actual process and isolated local sockets.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    process::{Child, Stdio},
    time::{Duration, Instant},
};
type Check = Result<(), String>;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("mcp-http/local-health-and-discovery", || {
            lifecycle().map_err(Into::into)
        }),
        Trial::test(
            "mcp-http/revisions-parity-and-concurrent-identities",
            || protocols().map_err(Into::into),
        ),
        Trial::test(
            "mcp-http/host-origin-method-and-metadata-boundaries",
            || admission().map_err(Into::into),
        ),
        Trial::test("mcp-http/framing-resource-deadlines-and-shutdown", || {
            resources().map_err(Into::into)
        }),
        Trial::test("mcp-http/storage-refusal-and-nonlocal-bind", || {
            storage().map_err(Into::into)
        }),
    ]
}

struct Server {
    child: Child,
    address: SocketAddr,
    _environment: Environment,
}
impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
impl Server {
    fn start() -> Result<Self, String> {
        let environment = Environment::new().map_err(|e| e.to_string())?;
        let address: SocketAddr = "127.0.0.1:0".parse().map_err(|_| "local address")?;
        let mut args = vec!["serve".into(), "--http".into(), address.to_string()];
        args.extend(environment.globals(
            Some(&super::launch::fixtures().join("replies/gene/main")),
            false,
        ));
        let child = environment
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;
        let mut server = Self { child, address, _environment: environment };
        let stderr = server.child.stderr.take().ok_or("HTTP stderr missing")?;
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        let reader = std::thread::spawn(move || {
            use std::io::BufRead;
            let mut line = String::new();
            let result = std::io::BufReader::new(stderr).read_line(&mut line).map(|_| line);
            let _ = sender.send(result);
        });
        let event = receiver.recv_timeout(Duration::from_secs(5)).map_err(|_| "HTTP readiness event timed out")?
            .map_err(|e| e.to_string())?;
        reader.join().map_err(|_| "HTTP readiness reader stopped")?;
        let event: Value = serde_json::from_str(&event).map_err(|e| format!("HTTP readiness: {e}"))?;
        server.address = event.get("listening").and_then(Value::as_str).ok_or("missing listening address")?
            .parse().map_err(|_| "invalid listening address")?;
        Ok(server)
    }

    fn request(&self, method: &str, path: &str, body: &str) -> Result<(u16, Value), String> {
        let value = serde_json::from_str::<Value>(body).ok();
        let method_header = value
            .as_ref()
            .and_then(|v| v.get("method"))
            .and_then(Value::as_str)
            .map(|method| format!("Mcp-Method: {method}\r\n"))
            .unwrap_or_default();
        let name_header = value
            .as_ref()
            .and_then(|v| v.pointer("/params/name"))
            .and_then(Value::as_str)
            .map(|name| format!("Mcp-Name: {name}\r\n"))
            .unwrap_or_default();
        let headers = format!(
            "Content-Type: application/json\r\nAccept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2026-07-28\r\n{method_header}{name_header}"
        );
        let (status, body, _) =
            self.raw(method, path, &self.address.to_string(), &headers, body)?;
        Ok((status, body.unwrap_or(Value::Null)))
    }
    fn raw(
        &self,
        method: &str,
        path: &str,
        host: &str,
        headers: &str,
        body: &str,
    ) -> Result<(u16, Option<Value>, String), String> {
        let mut socket = TcpStream::connect(self.address).map_err(|e| e.to_string())?;
        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .map_err(|e| e.to_string())?;
        let wire = format!(
            "{method} {path} HTTP/1.1\r\nHost: {host}\r\n{headers}Connection: close\r\nContent-Length: {}\r\n\r\n{body}",
            body.len()
        );
        socket
            .write_all(wire.as_bytes())
            .map_err(|e| e.to_string())?;
        let mut bytes = Vec::new();
        socket
            .take(1024 * 1024)
            .read_to_end(&mut bytes)
            .map_err(|e| e.to_string())?;
        let response = String::from_utf8(bytes).map_err(|e| e.to_string())?;
        let (headers, body) = response
            .split_once("\r\n\r\n")
            .ok_or("missing HTTP headers")?;
        let status = headers
            .split_whitespace()
            .nth(1)
            .ok_or("missing status")?
            .parse()
            .map_err(|_| "bad status")?;
        Ok((
            status,
            if body.is_empty() {
                None
            } else {
                Some(serde_json::from_str(body).map_err(|e| format!("HTTP JSON body: {e}"))?)
            },
            headers.to_owned(),
        ))
    }
}
fn lifecycle() -> Check {
    let server = Server::start()?;
    let (status, health) = server.request("GET", "/health", "")?;
    if status != 200 || health.get("version") != Some(&json!(env!("CARGO_PKG_VERSION"))) {
        return Err(format!("local health: {status} {health}"));
    }
    let request = json!({"jsonrpc":"2.0","id":1,"method":"server/discover","params":{
        "_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28",
        "io.modelcontextprotocol/clientCapabilities":{}}}});
    let (status, reply) = server.request("POST", "/mcp", &request.to_string())?;
    if status != 200
        || reply.pointer("/result/supportedVersions") != Some(&json!(["2026-07-28", "2025-11-25"]))
    {
        return Err(format!("HTTP discover: {status} {reply}"));
    }
    Ok(())
}

fn legacy(id: u32, method: &str, params: Value) -> Value {
    json!({"jsonrpc":"2.0","id":id,"method":method,"params":params})
}
fn legacy_headers() -> &'static str {
    "Content-Type: application/json\r\nAccept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2025-11-25\r\n"
}
fn protocols() -> Check {
    let server = Server::start()?;
    let init = legacy(
        7,
        "initialize",
        json!({"protocolVersion":"2025-11-25", "capabilities":{},
        "clientInfo":{"name":"local-proof", "version":"1"}}),
    );
    let (_, initialized, headers) = server.raw(
        "POST",
        "/mcp",
        &server.address.to_string(),
        legacy_headers(),
        &init.to_string(),
    )?;
    if initialized
        .as_ref()
        .and_then(|v| v.pointer("/result/protocolVersion"))
        != Some(&json!("2025-11-25"))
        || headers.to_ascii_lowercase().contains("session-id")
    {
        return Err("legacy initialization or minted session".into());
    }
    let discover = super::mcp::request(json!(7), "server/discover", json!({}));
    server.request("POST", "/mcp", &discover.to_string())?;
    let list = legacy(7, "tools/list", json!({}));
    let (status, reply, _) = server.raw(
        "POST",
        "/mcp",
        &server.address.to_string(),
        legacy_headers(),
        &list.to_string(),
    )?;
    if status != 200
        || reply
            .as_ref()
            .and_then(|v| v.pointer("/result/resultType"))
            .is_some()
    {
        return Err("modern client changed legacy request state".into());
    }
    let args: Vec<String> = ["get", "gene", "BRAF"].map(str::to_owned).into();
    let globals = server._environment.globals(
        Some(&super::launch::fixtures().join("replies/gene/main")),
        false,
    );
    let expected = super::mcp::call_with_globals(&server._environment, &args, false, &globals)?;
    let call = super::mcp::request(
        json!(7),
        "tools/call",
        json!({"name":"biomcp","arguments":{"args":args}}),
    );
    let barrier = std::sync::Barrier::new(2);
    let results = std::thread::scope(|scope| {
        let a = scope.spawn(|| {
            barrier.wait();
            server.request("POST", "/mcp", &call.to_string())
        });
        let b = scope.spawn(|| {
            barrier.wait();
            server.request("POST", "/mcp", &call.to_string())
        });
        (a.join().unwrap(), b.join().unwrap())
    });
    for result in [results.0, results.1] {
        let (status, value) = result?;
        if status != 200 || value.pointer("/result/structuredContent") != Some(&expected) {
            return Err("concurrent equal IDs lost stdio parity".into());
        }
    }
    Ok(())
}
fn admission() -> Check {
    let server = Server::start()?;
    let host = server.address.to_string();
    for (authority, headers) in [
        ("foreign.example", ""),
        ("localhost.evil", ""),
        ("localhost:0", ""),
        ("user@localhost", ""),
        ("[::1]extra", ""),
        ("localhost:65536", ""),
        (&host, "Host: foreign.example\r\n"),
        (&host, "Origin: http://localhost\r\n"),
        (&host, "Origin: null\r\n"),
        (&host, "Origin: invalid\r\nOrigin: invalid\r\n"),
    ] {
        for path in ["/health", "/missing", "/mcp"] {
            let (status, _, response) = server.raw("GET", path, authority, headers, "")?;
            if status != 403
                || response
                    .to_ascii_lowercase()
                    .contains("access-control-allow")
            {
                return Err(format!(
                    "authority/origin admitted: {authority} {headers:?} {status}"
                ));
            }
        }
    }
    let forwarded = "Forwarded: host=foreign.example\r\nX-Forwarded-Host: foreign.example\r\n";
    if server.raw("GET", "/health", &host, forwarded, "")?.0 != 200 {
        return Err("forwarded headers affected admission".into());
    }
    for method in ["GET", "DELETE", "OPTIONS", "PUT"] {
        if server.raw(method, "/mcp", &host, "", "")?.0 != 405 {
            return Err(format!("method {method} admitted"));
        }
    }
    let request = super::mcp::request(json!(1), "tools/list", json!({}));
    for headers in [
        "Content-Type: application/json\r\nAccept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2026-07-28\r\n",
        "Content-Type: application/json\r\nAccept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2026-07-28\r\nMcp-Method: ping\r\n",
        "Content-Type: application/json\r\nAccept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2025-11-25\r\n",
    ] {
        let (status, reply, _) =
            server.raw("POST", "/mcp", &host, headers, &request.to_string())?;
        if status != 400
            || reply.as_ref().and_then(|v| v.pointer("/error/code")) != Some(&json!(-32020))
        {
            return Err("HTTP metadata mismatch was not rejected".into());
        }
    }
    let notification = json!({"jsonrpc":"2.0","method":"notifications/initialized"});
    let (status, reply, _) = server.raw(
        "POST",
        "/mcp",
        &host,
        legacy_headers(),
        &notification.to_string(),
    )?;
    if status != 202 || reply.is_some() {
        return Err("notification must have an empty 202 response".into());
    }
    Ok(())
}
fn closed(address: SocketAddr, data: &[u8]) -> Check {
    let mut socket = TcpStream::connect(address).map_err(|e| e.to_string())?;
    socket
        .set_read_timeout(Some(Duration::from_secs(4)))
        .map_err(|e| e.to_string())?;
    socket.write_all(data).map_err(|e| e.to_string())?;
    let mut byte = [0];
    match socket.read(&mut byte) {
        Ok(0) => Ok(()),
        Err(e)
            if matches!(
                e.kind(),
                std::io::ErrorKind::ConnectionReset | std::io::ErrorKind::BrokenPipe
            ) =>
        {
            Ok(())
        }
        other => Err(format!("rejected framing did not close: {other:?}")),
    }
}
fn resources() -> Check {
    let mut server = Server::start()?;
    for bytes in [
        format!("POST /mcp HTTP/1.1\r\nHost: {}\r\nContent-Length: 65537\r\n\r\n", server.address),
        format!("POST /mcp HTTP/1.1\r\nHost: {}\r\nContent-Length: 1\r\nContent-Length: 1\r\n\r\nx", server.address),
        format!("POST /mcp HTTP/1.1\r\nHost: {}\r\nTransfer-Encoding: gzip\r\n\r\n0\r\n\r\n", server.address),
        format!("GET /health HTTP/1.1\r\nHost: {}\r\nX-Fill: {}", server.address, "x".repeat(16384)),
    ] { closed(server.address, bytes.as_bytes())?; }
    let mut slow = TcpStream::connect(server.address).map_err(|e| e.to_string())?;
    slow.set_read_timeout(Some(Duration::from_secs(4)))
        .map_err(|e| e.to_string())?;
    slow.write_all(b"GET /health HTTP/1.1\r\nX-Slow: ")
        .map_err(|e| e.to_string())?;
    let started = Instant::now();
    // Continuously send data while waiting for the server's close event. An idle
    // timeout would stay open forever; the absolute deadline must end the read.
    let copy = slow.try_clone().map_err(|e| e.to_string())?;
    let writer = std::thread::spawn(move || {
        let mut copy = copy;
        while copy.write_all(b"x").is_ok() {
            std::thread::yield_now();
        }
    });
    let mut byte = [0];
    let result = slow.read(&mut byte);
    let _ = stop.send(());
    let _ = slow.shutdown(std::net::Shutdown::Both);
    writer.join().map_err(|_| "slow writer stopped")?;
    let ended = match result {
        Ok(0) => true,
        Err(error) => error.kind() == std::io::ErrorKind::ConnectionReset,
        _ => false,
    };
    if !ended || started.elapsed() > Duration::from_secs(4) {
        return Err("slow header retained a worker beyond the absolute deadline".into());
    }
    server.child.kill().map_err(|e| e.to_string())?;
    server.child.wait().map_err(|e| e.to_string())?;
    if TcpStream::connect(server.address).is_ok() {
        return Err("shutdown left a listener".into());
    }
    Ok(())
}
fn storage() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    for args in [
        vec!["serve", "--http", "0.0.0.0:0"],
        vec!["serve", "--http", "127.0.0.1:0", "--allow-host", "bad/path"],
    ] {
        let args: Vec<String> = args.into_iter().map(str::to_owned).collect();
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        if output.status.code() != Some(2) {
            return Err("unsafe bind must fail with usage status".into());
        }
    }
    Ok(())
}
