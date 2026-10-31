//! Bounded blocking HTTP transport with independent request protocol state.
mod authority;
mod call;
mod chunked;
mod metadata;
mod wire;
use authority::Authority;
use call::{Calls, HostedCall};
use serde_json::{Value, json};
use std::{
    io,
    net::{SocketAddr, TcpListener, TcpStream},
    sync::Arc,
    thread,
};
const ACCEPT_POLL: std::time::Duration = std::time::Duration::from_millis(20);
const MAX_CONNECTIONS: usize = 8;
const HEALTH_BUDGET: std::time::Duration = std::time::Duration::from_secs(2);
pub(crate) enum StartError {
    Configuration(crate::config::StartupError),
    Usage,
    AllowHost,
    Local,
}
struct Service {
    calls: Calls,
    address: SocketAddr,
    hosts: Vec<Authority>,
    globals: crate::grammar::Globals,
    fetch: Result<Arc<biomcp_fetch::Fetch>, crate::config::StartupError>,
}
pub(crate) fn serve(
    address: &str,
    hosts: Vec<String>,
    globals: crate::grammar::Globals,
) -> Result<(), StartError> {
    let address: SocketAddr = address.parse().map_err(|_| StartError::Usage)?;
    let names = crate::config::http_hosts(&globals)
        .map_err(|error| StartError::Configuration(error.into()))?;
    let mut allowed = names
        .iter()
        .map(|host| {
            Authority::parse(host).ok_or_else(|| {
                StartError::Configuration(
                    crate::config::ConfigError::field("http.allowed_hosts").into(),
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    allowed.extend(
        hosts
            .iter()
            .map(|host| Authority::parse(host).ok_or(StartError::Usage))
            .collect::<Result<Vec<_>, _>>()?,
    );
    let hosts = allowed;
    if !address.ip().is_loopback() && hosts.is_empty() {
        return Err(StartError::AllowHost);
    }
    let listener = TcpListener::bind(address).map_err(|_| StartError::Local)?;
    let address = listener.local_addr().map_err(|_| StartError::Local)?;
    let service = Arc::new(Service {
        calls: Calls::default(),
        address,
        hosts,
        fetch: crate::config::fetch(&globals).map(Arc::new),
        globals,
    });
    let (stop, stopped) = std::sync::mpsc::sync_channel(1);
    ctrlc::try_set_handler(move || {
        let _ = stop.try_send(());
    })
    .map_err(|_| StartError::Local)?;
    let mut ready = serde_json::to_vec(&json!({"listening":address.to_string()}))
        .map_err(|_| StartError::Local)?;
    ready.push(b'\n');
    std::io::Write::write_all(&mut std::io::stderr().lock(), &ready)
        .map_err(|_| StartError::Local)?;
    connections(listener, service, stopped).map_err(|_| StartError::Local)
}
#[expect(
    clippy::disallowed_methods,
    reason = "Own at most MAX_CONNECTIONS blocking socket workers"
)]
fn connections(
    listener: TcpListener,
    service: Arc<Service>,
    stopped: std::sync::mpsc::Receiver<()>,
) -> io::Result<()> {
    listener.set_nonblocking(true)?;
    let mut workers: Vec<thread::JoinHandle<()>> = Vec::new();
    loop {
        if stopped.try_recv().is_ok() {
            break;
        }
        let mut stream = match listener.accept() {
            Ok((stream, _)) => {
                stream.set_nonblocking(false)?;
                stream
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                match stopped.recv_timeout(ACCEPT_POLL) {
                    Ok(()) | Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
                }
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        };
        let mut retained = Vec::with_capacity(MAX_CONNECTIONS);
        for worker in workers {
            if worker.is_finished() {
                let _ = worker.join();
            } else {
                retained.push(worker);
            }
        }
        workers = retained;
        if workers.len() >= MAX_CONNECTIONS {
            let _ = wire::read(&mut stream);
            let _ = wire::write(
                &mut stream,
                503,
                br#"{"error":"Service unavailable"}"#,
                None,
                None,
            );
            continue;
        }
        let service = service.clone();
        workers.push(
            thread::Builder::new()
                .name("mcp-http".into())
                .spawn(move || {
                    let _ = service.handle(stream);
                })?,
        );
    }
    drop(listener);
    service.calls.cancel();
    for worker in workers {
        worker
            .join()
            .map_err(|_| io::Error::other("HTTP worker stopped"))?;
    }
    Ok(())
}
impl Service {
    fn handle(&self, mut stream: TcpStream) -> io::Result<()> {
        // A malformed or oversized framing closes without reflecting untrusted headers.
        let request = wire::read(&mut stream)?;
        let (status, value, revision) = self.route(&request);
        let body = value
            .map(|v| serde_json::to_vec(&v))
            .transpose()?
            .unwrap_or_default();
        let allow = (status == 405).then_some(if request.path == "/health" {
            "GET"
        } else {
            "POST"
        });
        wire::write(&mut stream, status, &body, revision, allow)
    }
    fn route(&self, request: &wire::Request) -> (u16, Option<Value>, Option<&'static str>) {
        if !self.allowed(request) {
            return (403, Some(json!({"error":"Forbidden"})), None);
        }
        if request.path == "/health" {
            return if request.method == "GET" {
                self.health()
            } else {
                (405, Some(json!({"error":"Method not allowed"})), None)
            };
        }
        if request.path != "/mcp" {
            return (404, Some(json!({"error":"Not found"})), None);
        }
        if request.method != "POST" {
            return (405, Some(json!({"error":"Method not allowed"})), None);
        }
        let value = match serde_json::from_slice::<Value>(&request.body) {
            Ok(value) => value,
            Err(_) => {
                return (
                    400,
                    Some(super::protocol::error(Value::Null, -32700, "Parse error")),
                    None,
                );
            }
        };
        let revision = match metadata::validate(request, &value) {
            Ok(revision) => revision,
            Err(error) => return (400, Some(error), None),
        };
        let mut legacy = revision != super::protocol::REVISIONS[0];
        let result = match super::rpc::message(&request.body, &mut legacy, true) {
            super::rpc::Message::Reply(reply) => reply,
            super::rpc::Message::Notification { .. } => return (202, None, Some(revision)),
            super::rpc::Message::Tool { id, params, modern } => {
                return self.tool(request, id, &params, modern, revision);
            }
        };
        let status = if result.pointer("/error/code") == Some(&json!(-32601)) {
            404
        } else {
            200
        };
        (status, Some(result), Some(revision))
    }
    fn tool(
        &self,
        request: &wire::Request,
        id: Value,
        params: &Value,
        modern: bool,
        revision: &'static str,
    ) -> (u16, Option<Value>, Option<&'static str>) {
        let scope = self
            .globals
            .hosted
            .then(|| biomcp_fetch::Cancellation::with_budget(crate::config::BUDGET));
        let Some(call) = super::research::parse(params) else {
            return (
                200,
                Some(super::protocol::error(id, -32602, "Invalid params")),
                Some(revision),
            );
        };
        let keys = if self.globals.hosted {
            let keys = match request.source_keys() {
                Ok(keys) => keys,
                Err(_) => {
                    return (
                        400,
                        Some(json!({"error":"Invalid source key headers"})),
                        Some(revision),
                    );
                }
            };
            Some(keys)
        } else {
            None
        };
        let result = if let Some(scope) = scope {
            self.calls.run(
                self,
                HostedCall {
                    id,
                    modern,
                    call,
                    secrets: keys,
                    scope,
                },
            )
        } else {
            match super::research::execute(
                &call,
                crate::command::Context {
                    globals: &self.globals,
                    fetch: &self.fetch,
                    secrets: keys.as_ref(),
                    scope: None,
                },
                true,
            ) {
                Ok(outcome) => super::protocol::tool_result(id, &outcome, modern),
                Err(_) => super::protocol::error(id, -32603, "Internal error"),
            }
        };
        (200, Some(result), Some(revision))
    }
    fn allowed(&self, request: &wire::Request) -> bool {
        if !matches!(request.one("origin"), Ok(None)) {
            return false;
        }
        let Ok(Some(host)) = request.one("host") else {
            return false;
        };
        let Some(host) = Authority::parse(host) else {
            return false;
        };
        // Absolute-form targets are never accepted; no Forwarded header affects admission.
        request.path.starts_with('/')
            && !request.path.starts_with("//")
            && (self.address.ip().is_loopback() && host.loopback(self.address)
                || self.hosts.iter().any(|allowed| allowed.permits(&host)))
    }
    fn health(&self) -> (u16, Option<Value>, Option<&'static str>) {
        let healthy = self
            .fetch
            .as_ref()
            .is_ok_and(|fetch| biomcp_query::store::health(fetch, HEALTH_BUDGET).is_ok());
        // Build the result through its public schema type, without source traffic.
        let value = biodata::Text::new(env!("CARGO_PKG_VERSION"))
            .ok()
            .and_then(|version| {
                serde_json::to_value(biomcp_types::HttpHealth {
                    version,
                    store: healthy,
                })
                .ok()
            });
        (if healthy { 200 } else { 503 }, value, None)
    }
}
