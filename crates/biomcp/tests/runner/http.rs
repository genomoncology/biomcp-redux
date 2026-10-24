//! MCP HTTP through the actual process and isolated local sockets.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    process::{Child, Stdio},
    time::{Duration, Instant},
};
type Check = Result<(), String>;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("mcp-http/local-health-and-discovery", || {
        lifecycle().map_err(Into::into)
    })]
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
        let reservation = TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
        let address = reservation.local_addr().map_err(|e| e.to_string())?;
        drop(reservation);
        let args = vec!["serve".into(), "--http".into(), address.to_string(), "--offline".into()];
        let child = environment.command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::piped())
            .spawn().map_err(|e| e.to_string())?;
        let mut server = Self { child, address, _environment: environment };
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if let Some(status) = server.child.try_wait().map_err(|e| e.to_string())? {
                let mut error = String::new();
                if let Some(mut stderr) = server.child.stderr.take() {
                    let _ = stderr.read_to_string(&mut error);
                }
                return Err(format!("HTTP server exited {status}: {error}"));
            }
            if TcpStream::connect_timeout(&address, Duration::from_millis(20)).is_ok() {
                return Ok(server);
            }
            if Instant::now() >= deadline { return Err("HTTP listener never became ready".into()); }
            std::thread::yield_now();
        }
    }
    fn request(&self, method: &str, path: &str, body: &str) -> Result<(u16, Value), String> {
        let mut socket = TcpStream::connect(self.address).map_err(|e| e.to_string())?;
        socket.set_read_timeout(Some(Duration::from_secs(5))).map_err(|e| e.to_string())?;
        let wire = format!("{method} {path} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nAccept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2026-07-28\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{body}", self.address, body.len());
        socket.write_all(wire.as_bytes()).map_err(|e| e.to_string())?;
        let mut bytes = Vec::new();
        socket.take(1024 * 1024).read_to_end(&mut bytes).map_err(|e| e.to_string())?;
        let response = String::from_utf8(bytes).map_err(|e| e.to_string())?;
        let (headers, body) = response.split_once("\r\n\r\n").ok_or("missing HTTP headers")?;
        let status = headers.split_whitespace().nth(1).ok_or("missing status")?.parse().map_err(|_| "bad status")?;
        Ok((status, serde_json::from_str(body).map_err(|e| format!("HTTP JSON body: {e}"))?))
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
    if status != 200 || reply.pointer("/result/supportedVersions") != Some(&json!(["2026-07-28", "2025-11-25"])) {
        return Err(format!("HTTP discover: {status} {reply}"));
    }
    Ok(())
}
