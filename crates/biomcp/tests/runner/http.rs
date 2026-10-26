//! MCP HTTP through the actual process and isolated local sockets.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    process::{Child, Stdio},
    time::Duration,
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
            super::http_limits::check().map_err(Into::into)
        }),
        Trial::test("mcp-http/storage-refusal-and-nonlocal-bind", || {
            storage().map_err(Into::into)
        }),
    ]
}

pub(super) struct Server {
    pub(super) child: Child,
    pub(super) address: SocketAddr,
    _environment: Environment,
}
impl Drop for Server {
    fn drop(&mut self) {
        if self.stop().is_err() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}
impl Server {
    pub(super) fn stop(&mut self) -> Check {
        if self.child.try_wait().map_err(|e| e.to_string())?.is_some() {
            return Ok(());
        }
        #[cfg(unix)]
        {
            let pid = nix::unistd::Pid::from_raw(
                self.child.id().try_into().map_err(|_| "HTTP process ID")?,
            );
            nix::sys::signal::kill(pid, nix::sys::signal::Signal::SIGINT)
                .map_err(|e| e.to_string())?;
        }
        // Native Windows console Ctrl-C is exercised by the platform rehearsal.
        #[cfg(windows)]
        self.child.kill().map_err(|e| e.to_string())?;
        let end = std::time::Instant::now() + Duration::from_secs(5);
        loop {
            if let Some(status) = self.child.try_wait().map_err(|e| e.to_string())? {
                if cfg!(unix) && !status.success() {
                    return Err(format!("HTTP shutdown: {status}"));
                }
                return Ok(());
            }
            if std::time::Instant::now() >= end {
                return Err("HTTP shutdown deadline".into());
            }
            std::thread::yield_now();
        }
    }
    pub(super) fn start() -> Result<Self, String> {
        Self::configured("127.0.0.1:0", None, true)
    }
    pub(super) fn environment(&self) -> &Environment {
        &self._environment
    }
    pub(super) fn keys(config: Value, key: Option<(&str, &str)>) -> Result<Self, String> {
        Self::configured_key("127.0.0.1:0", Some(config), None, key)
    }
    fn configured(bind: &str, config: Option<Value>, replay: bool) -> Result<Self, String> {
        Self::configured_key(
            bind,
            config,
            replay.then(|| super::launch::fixtures().join("replies/gene/main")),
            None,
        )
    }
    pub(super) fn start_replay(path: std::path::PathBuf) -> Result<Self, String> {
        Self::configured_key("127.0.0.1:0", None, Some(path), None)
    }
    fn configured_key(
        bind: &str,
        config: Option<Value>,
        replay: Option<std::path::PathBuf>,
        key: Option<(&str, &str)>,
    ) -> Result<Self, String> {
        let environment = Environment::new().map_err(|e| e.to_string())?;
        if let Some(config) = config {
            environment.config(&config).map_err(|e| e.to_string())?;
        }
        let address: SocketAddr = bind.parse().map_err(|_| "local address")?;
        let mut args = vec!["serve".into(), "--http".into(), address.to_string()];
        if let Some(path) = replay {
            args.extend(environment.globals(Some(&path), false));
        }
        let mut command = environment.command(env!("CARGO_BIN_EXE_biomcp"), &args);
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            // Installed services must also start without a console window.
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            command.creation_flags(CREATE_NO_WINDOW);
        }
        if let Some((name, key)) = key {
            command.env(name, key);
        }
        let child = command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;
        let mut server = Self {
            child,
            address,
            _environment: environment,
        };
        let stderr = server.child.stderr.take().ok_or("HTTP stderr missing")?;
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        let reader = std::thread::spawn(move || {
            use std::io::BufRead;
            let mut line = String::new();
            let result = std::io::BufReader::new(stderr)
                .read_line(&mut line)
                .map(|_| line);
            let _ = sender.send(result);
        });
        let event = receiver
            .recv_timeout(Duration::from_secs(5))
            .map_err(|_| "HTTP readiness event timed out")?
            .map_err(|e| e.to_string())?;
        reader.join().map_err(|_| "HTTP readiness reader stopped")?;
        let event: Value =
            serde_json::from_str(&event).map_err(|e| format!("HTTP readiness: {e}"))?;
        server.address = event
            .get("listening")
            .and_then(Value::as_str)
            .ok_or("missing listening address")?
            .parse()
            .map_err(|_| "invalid listening address")?;
        Ok(server)
    }

    pub(super) fn request(
        &self,
        method: &str,
        path: &str,
        body: &str,
    ) -> Result<(u16, Value), String> {
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
    let cache = server.environment().root().join("case-cache");
    let database = std::fs::read_dir(&cache)
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| {
            path.extension()
                .is_some_and(|extension| extension == "sqlite")
        })
        .ok_or("missing cache database")?;
    let connection = rusqlite::Connection::open(&database).map_err(|e| e.to_string())?;
    connection
        .execute_batch("BEGIN IMMEDIATE")
        .map_err(|e| e.to_string())?;
    let (status, health) = server.request("GET", "/health", "")?;
    if status != 200 || health.get("version") != Some(&json!(env!("CARGO_PKG_VERSION"))) {
        return Err(format!("local health: {status} {health}"));
    }
    let schema = super::schema::ARTIFACTS
        .iter()
        .find(|(id, _)| *id == "biomcp.http-health/1")
        .ok_or("HTTP health schema missing")?
        .1;
    let schema: Value = serde_json::from_str(schema).map_err(|e| e.to_string())?;
    jsonschema::validator_for(&schema)
        .map_err(|e| e.to_string())?
        .validate(&health)
        .map_err(|e| e.to_string())?;
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
    let (status, reply) = server.request("POST", "/mcp", &call.to_string())?;
    if status != 200 || reply.pointer("/result/structuredContent") != Some(&expected) {
        return Err("replayed HTTP call lost stdio parity".into());
    }
    concurrent(&server, &globals)
}
fn concurrent(server: &Server, globals: &[String]) -> Check {
    // Distinct replies expose cross-client ID and protocol state mistakes.
    let modern_args = vec!["help".to_string()];
    let old_args = vec!["help".to_string(), "get".into(), "gene".into()];
    let modern_expected =
        super::mcp::call_with_globals(&server._environment, &modern_args, false, globals)?;
    let old_expected =
        super::mcp::call_with_globals(&server._environment, &old_args, false, globals)?;
    let modern = super::mcp::request(
        json!(7),
        "tools/call",
        json!({"name":"biomcp","arguments":{"args":modern_args}}),
    );
    let old = legacy(
        7,
        "tools/call",
        json!({"name":"biomcp","arguments":{"args":old_args}}),
    );
    let barrier = std::sync::Barrier::new(3);
    let (modern, old, health) = std::thread::scope(|scope| {
        let a = scope.spawn(|| {
            barrier.wait();
            server.request("POST", "/mcp", &modern.to_string())
        });
        let b = scope.spawn(|| {
            barrier.wait();
            server.raw(
                "POST",
                "/mcp",
                &server.address.to_string(),
                legacy_headers(),
                &old.to_string(),
            )
        });
        let health = scope.spawn(|| {
            barrier.wait();
            server.request("GET", "/health", "")
        });
        Ok::<_, String>((
            a.join().map_err(|_| "modern HTTP caller stopped")?,
            b.join().map_err(|_| "legacy HTTP caller stopped")?,
            health.join().map_err(|_| "HTTP health caller stopped")?,
        ))
    })?;
    let (health_status, health) = health?;
    if health_status != 200 || health.get("store") != Some(&json!(true)) {
        return Err("concurrent local store health failed".into());
    }
    let (modern_status, modern) = modern?;
    let (old_status, old, _) = old?;
    let old = old.ok_or("legacy response missing")?;
    if modern_status != 200
        || old_status != 200
        || modern.pointer("/result/structuredContent") != Some(&modern_expected)
        || old.pointer("/result/structuredContent") != Some(&old_expected)
        || modern.pointer("/result/resultType") != Some(&json!("complete"))
        || old.pointer("/result/resultType").is_some()
    {
        return Err("concurrent equal IDs crossed client results or protocol state".into());
    }
    Ok(())
}

fn admission() -> Check {
    let server = Server::start()?;
    metadata_cases(&server)?;
    let host = server.address.to_string();
    for (authority, headers) in [
        ("foreign.example", ""),
        ("localhost.evil", ""),
        ("localhost:0", ""),
        ("user@localhost", ""),
        ("[::1]extra", ""),
        ("localhost:65536", ""),
        ("localhost:+8080", ""),
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
        let (status, _, headers) = server.raw(method, "/mcp", &host, "", "")?;
        if status != 405 || !headers.contains("Allow: POST\r\n") {
            return Err(format!("method {method} admitted or missing Allow"));
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
    let failed = Server::configured(
        "127.0.0.1:0",
        Some(json!({"ca_bundle":"missing-private-ca.pem"})),
        false,
    )?;
    let (status, body, _) = failed.raw("GET", "/health", &failed.address.to_string(), "", "")?;
    if status != 503 || body != Some(json!({"version":env!("CARGO_PKG_VERSION"),"store":false})) {
        return Err("local initialization failure leaked details or passed health".into());
    }
    let ipv6 = Server::configured("[::1]:0", None, true)?;
    let authority = format!("[0:0:0:0:0:0:0:1]:{}", ipv6.address.port());
    if ipv6.raw("GET", "/health", &authority, "", "")?.0 != 200 {
        return Err("equivalent IPv6 authority was rejected".into());
    }
    let mut exposed = Server::configured(
        "0.0.0.0:0",
        Some(json!({"http":{"allowed_hosts":["local.example"]}})),
        true,
    )?;
    exposed.address = SocketAddr::new(
        std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
        exposed.address.port(),
    );
    if exposed.raw("GET", "/health", "LOCAL.EXAMPLE", "", "")?.0 != 200
        || exposed.raw("GET", "/health", "other.example", "", "")?.0 != 403
    {
        return Err("configured host allowlist was not enforced".into());
    }
    Ok(())
}

fn metadata_cases(server: &Server) -> Check {
    let request = super::mcp::request(json!(9), "tools/list", json!({}));
    for (accept, expected) in [
        ("Application/JSON, Text/Event-Stream", 200),
        ("application/*, text/*", 200),
        ("*/*;q=1, application/json;q=0, text/event-stream", 400),
        ("*/*;q=0, application/json, text/event-stream", 200),
        ("application/*;q=0, */*;q=1, application/json;q=1", 200),
        ("application/json; q=1.000, text/event-stream; q=0.5", 200),
        ("application/json; q=0.001, text/event-stream", 200),
        ("application/json; q=0, text/event-stream", 400),
        ("application/json; q=1.001, text/event-stream", 400),
        ("application/json; q=0.0001, text/event-stream", 400),
    ] {
        let headers = format!(
            "Content-Type: application/json\r\nAccept: {accept}\r\nMCP-Protocol-Version: 2026-07-28\r\nMcp-Method: tools/list\r\n"
        );
        let (status, _, _) = server.raw(
            "POST",
            "/mcp",
            &server.address.to_string(),
            &headers,
            &request.to_string(),
        )?;
        if status != expected {
            return Err(format!(
                "Accept media range {accept:?}: expected {expected}, received {status}"
            ));
        }
    }
    let request = super::mcp::request(
        json!(9),
        "tools/call",
        json!({"name":"biomcp","arguments":{"args":["help"]}}),
    );
    let prefix = "Content-Type: application/json\r\nAccept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2026-07-28\r\nMcp-Method: tools/call\r\n";
    for (name, expected) in [
        ("=?base64?YmlvbWNw?=", 200),
        ("=?base64?***?=", 400),
        ("other", 400),
    ] {
        let headers = format!("{prefix}Mcp-Name: {name}\r\n");
        let (status, reply, _) = server.raw(
            "POST",
            "/mcp",
            &server.address.to_string(),
            &headers,
            &request.to_string(),
        )?;
        if status != expected
            || expected == 400
                && reply.as_ref().and_then(|v| v.pointer("/error/code")) != Some(&json!(-32020))
        {
            return Err("encoded tool name did not match its body".into());
        }
    }
    let request = super::mcp::request(json!(9), "unavailable-method", json!({}));
    let (status, reply) = server.request("POST", "/mcp", &request.to_string())?;
    if status != 404 || reply.pointer("/error/code") != Some(&json!(-32601)) {
        return Err("unsupported modern method needs HTTP 404 and RPC -32601".into());
    }
    let headers = "Content-Type: application/json\r\nAccept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2024-11-05\r\n";
    let (status, reply, _) = server.raw(
        "POST",
        "/mcp",
        &server.address.to_string(),
        headers,
        &request.to_string(),
    )?;
    if status != 400
        || reply.as_ref().and_then(|v| v.pointer("/error/code")) != Some(&json!(-32022))
    {
        return Err("unsupported protocol version was not reported".into());
    }
    Ok(())
}
