//! Bounded blocking HTTP transport with independent request protocol state.
mod authority;
mod metadata;
mod wire;
use authority::Authority;
use serde_json::{Value, json};
use std::{io, net::{SocketAddr, TcpListener, TcpStream}, sync::Arc, thread};
const MAX_CONNECTIONS: usize = 8;
const HEALTH_BUDGET: std::time::Duration = std::time::Duration::from_secs(2);
pub(crate) enum StartError { Usage, Local }
struct Service {
    address: SocketAddr,
    hosts: Vec<Authority>,
    globals: crate::grammar::Globals,
    fetch: Result<Arc<biomcp_fetch::Fetch>, ()>,
}
pub(crate) fn serve(address: &str, hosts: Vec<String>, globals: crate::grammar::Globals) -> Result<(), StartError> {
    let address: SocketAddr = address.parse().map_err(|_| StartError::Usage)?;
    let mut names = crate::config::http_hosts().map_err(|_| StartError::Local)?;
    names.extend(hosts);
    let hosts = names.iter().map(|host| Authority::parse(host).ok_or(StartError::Usage))
        .collect::<Result<Vec<_>, _>>()?;
    if !address.ip().is_loopback() && hosts.is_empty() { return Err(StartError::Usage); }
    let listener = TcpListener::bind(address).map_err(|_| StartError::Local)?;
    let address = listener.local_addr().map_err(|_| StartError::Local)?;
    let service = Arc::new(Service { address, hosts, fetch: crate::config::fetch(&globals).map(Arc::new), globals });
    connections(listener, service).map_err(|_| StartError::Local)
}
#[expect(clippy::disallowed_methods, reason = "Own at most MAX_CONNECTIONS blocking socket workers")]
fn connections(listener: TcpListener, service: Arc<Service>) -> io::Result<()> {
    let mut workers: Vec<thread::JoinHandle<()>> = Vec::new();
    for stream in listener.incoming() {
        let mut retained = Vec::with_capacity(MAX_CONNECTIONS);
        for worker in workers {
            if worker.is_finished() { let _ = worker.join(); } else { retained.push(worker); }
        }
        workers = retained;
        let stream = stream?;
        if workers.len() >= MAX_CONNECTIONS { continue; }
        let service = service.clone();
        workers.push(thread::Builder::new().name("mcp-http".into()).spawn(move || {
            let _ = service.handle(stream);
        })?);
    }
    for worker in workers { let _ = worker.join(); }
    Ok(())
}
impl Service {
    fn handle(&self, mut stream: TcpStream) -> io::Result<()> {
        // A malformed or oversized framing closes without reflecting untrusted headers.
        let request = wire::read(&mut stream)?;
        let (status, value, revision) = self.route(&request);
        let body = value.map(|v| serde_json::to_vec(&v)).transpose()?.unwrap_or_default();
        wire::write(&mut stream, status, &body, revision)
    }
    fn route(&self, request: &wire::Request) -> (u16, Option<Value>, Option<&'static str>) {
        if !self.allowed(request) { return (403, Some(json!({"error":"Forbidden"})), None); }
        if request.path == "/health" {
            return if request.method == "GET" { self.health() }
                else { (405, Some(json!({"error":"Method not allowed"})), None) };
        }
        if request.path != "/mcp" { return (404, Some(json!({"error":"Not found"})), None); }
        if request.method != "POST" { return (405, Some(json!({"error":"Method not allowed"})), None); }
        let value = match serde_json::from_slice::<Value>(&request.body) {
            Ok(value) => value,
            Err(_) => return (400, Some(super::protocol::error(Value::Null, -32700, "Parse error")), None),
        };
        let revision = match metadata::validate(request, &value) {
            Ok(revision) => revision,
            Err(error) => return (400, Some(error), None),
        };
        let mut legacy = revision == super::protocol::REVISIONS[1];
        let result = match super::rpc::message(&request.body, &mut legacy) {
            super::rpc::Message::Reply(reply) => reply,
            super::rpc::Message::Notification { .. } => return (202, None, Some(revision)),
            super::rpc::Message::Tool { id, params, modern } => {
                let Some(args) = super::protocol::args(&params) else {
                    return (200, Some(super::protocol::error(id, -32602, "Invalid params")), Some(revision));
                };
                match crate::command::tool(&args, &self.globals, &self.fetch) {
                    Ok(outcome) => super::protocol::result(id, super::protocol::outcome(&outcome), modern),
                    Err(_) => super::protocol::error(id, -32603, "Internal error"),
                }
            }
        };
        let status = if result.pointer("/error/code") == Some(&json!(-32601)) { 404 } else { 200 };
        (status, Some(result), Some(revision))
    }
    fn allowed(&self, request: &wire::Request) -> bool {
        if request.one("origin") != Ok(None) { return false; }
        let Ok(Some(host)) = request.one("host") else { return false; };
        let Some(host) = Authority::parse(host) else { return false; };
        // Absolute-form targets are never accepted; no Forwarded header affects admission.
        request.path.starts_with('/') && !request.path.starts_with("//")
            && (self.address.ip().is_loopback() && host.loopback(self.address)
                || self.hosts.iter().any(|allowed| allowed.permits(&host)))
    }
    fn health(&self) -> (u16, Option<Value>, Option<&'static str>) {
        let healthy = self.fetch.as_ref().is_ok_and(|fetch|
            biomcp_query::store::health(fetch, HEALTH_BUDGET).is_ok());
        // Build the result through its public schema type, without source traffic.
        let value = biodata::Text::new(env!("CARGO_PKG_VERSION")).ok().and_then(|version|
            serde_json::to_value(biodata::HttpHealth { version, store: healthy }).ok());
        (if healthy { 200 } else { 503 }, value, None)
    }
}
