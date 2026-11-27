//! Bounded blocking HTTP transport with independent request protocol state.
use super::protocol::ReplyVersion;

mod admission;
mod authority;
mod call;
mod chunked;
mod connections;
mod key_check;
mod logging;
mod metadata;
mod wire;
use authority::Authority;
use call::{Calls, ToolCall};
use serde_json::{Value, json};
use std::{
    io,
    net::{SocketAddr, TcpListener, TcpStream},
    sync::Arc,
};
const ACCEPT_POLL: std::time::Duration = std::time::Duration::from_millis(20);
const HEALTH_BUDGET: std::time::Duration = std::time::Duration::from_secs(2);
pub(crate) enum StartError {
    Configuration(crate::config::StartupError),
    Usage,
    AllowHost,
    Local,
}
/// What one request's hosted headers allow, before any source work.
#[derive(Default)]
struct Caller {
    declared_use: Option<crate::command::DeclaredUse>,
    excluded: std::collections::BTreeSet<biomcp_types::Source>,
    /// The account header the hosting service sets from its verified user; it names the
    /// saved-work file and is never a path.
    account: Option<String>,
}
struct Service {
    calls: Calls,
    connections: admission::Slots,
    hosted_max_use: crate::command::DeclaredUse,
    max_calls: usize,
    queue_wait: std::time::Duration,
    max_connections: usize,
    address: SocketAddr,
    hosts: Vec<Authority>,
    globals: crate::grammar::Globals,
    root: Arc<crate::grammar::Root>,
    settings: Arc<crate::config::Settings>,
    fetch: Result<Arc<biomcp_fetch::Fetch>, crate::config::StartupError>,
    recovery: Option<Arc<super::Recovery>>,
}
pub(crate) fn serve(
    address: &str,
    hosts: Vec<String>,
    globals: crate::grammar::Globals,
    root: Arc<crate::grammar::Root>,
    settings: Arc<crate::config::Settings>,
) -> Result<(), StartError> {
    let address: SocketAddr = address.parse().map_err(|_| {
        let cause = "expected an IP address and port";
        let message = format!("biomcp: invalid HTTP bind address {address:?}; {cause}\n");
        crate::output::diagnostic(message.as_bytes());
        StartError::Usage
    })?;
    let names = crate::config::http_hosts(&settings)
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
            .map(|host| cli_host(host))
            .collect::<Result<Vec<_>, _>>()?,
    );
    let hosts = allowed;
    if !address.ip().is_loopback() && hosts.is_empty() {
        return Err(StartError::AllowHost);
    }
    let (max_calls, max_connections) = crate::config::http_capacity(&globals, &settings)
        .map_err(|error| StartError::Configuration(error.into()))?;
    let hosted_max_use = crate::config::hosted_max_use(&settings)
        .map_err(|error| StartError::Configuration(error.into()))?;
    let fetch = crate::config::fetch(&globals, &settings)
        .map(|fetch| Arc::new(fetch.with_background_refresh()));
    if let Err(error) = &fetch
        && error.blocks_server_startup()
    {
        return Err(StartError::Configuration(error.clone()));
    }
    let listener = TcpListener::bind(address).map_err(|error| {
        crate::output::diagnostic(
            format!("biomcp: could not bind HTTP listener at {address}: {error}\n").as_bytes(),
        );
        StartError::Local
    })?;
    let address = listener.local_addr().map_err(|_| StartError::Local)?;
    let recovery = fetch.is_err().then(|| Arc::new(super::Recovery::new()));
    let service = Arc::new(Service {
        calls: Calls::default(),
        connections: admission::Slots::new(max_connections),
        hosted_max_use,
        max_calls,
        queue_wait: crate::config::queue_wait(&settings),
        max_connections,
        address,
        hosts,
        fetch,
        recovery,
        globals,
        root,
        settings,
    });
    let (stop, stopped) = std::sync::mpsc::sync_channel(1);
    // This standalone server owns shutdown, including inherited ignored signals.
    ctrlc::set_handler(move || {
        let _ = stop.try_send(());
    })
    .map_err(|_| {
        crate::output::diagnostic(b"biomcp: could not install HTTP shutdown handler\n");
        StartError::Local
    })?;
    let mut ready = serde_json::to_vec(&json!({"listening":address.to_string()}))
        .map_err(|_| StartError::Local)?;
    ready.push(b'\n');
    std::io::Write::write_all(&mut std::io::stderr().lock(), &ready)
        .map_err(|_| StartError::Local)?;
    connections::run(listener, service, stopped).map_err(|error| {
        record(Err(error));
        StartError::Local
    })
}
fn cli_host(host: &str) -> Result<Authority, StartError> {
    Authority::parse(host).ok_or_else(|| {
        let message = format!(
            "biomcp: invalid --allow-host value {host:?}: expected a hostname or IP address (bracket IPv6), with an optional port\n"
        );
        crate::output::diagnostic(message.as_bytes());
        StartError::Usage
    })
}

fn record(result: io::Result<()>) {
    if let Err(error) = result {
        crate::output::diagnostic(
            format!("biomcp: HTTP connection failed ({:?})\n", error.kind()).as_bytes(),
        );
    }
}
impl Service {
    fn handle(
        &self,
        mut stream: TcpStream,
        reader: admission::Permit,
        connection: Option<admission::Permit>,
    ) -> io::Result<()> {
        // A malformed or oversized framing closes without reflecting untrusted headers.
        let request = wire::read(&mut stream)?;
        let started = std::time::Instant::now();
        // A reserved reader classifies health even when ordinary connections are full.
        let _connection = if request.path == "/health" {
            drop(connection);
            None
        } else {
            let Some(permit) = connection else {
                logging::record(&request, started.elapsed(), 503, None);
                return wire::unavailable(&mut stream);
            };
            drop(reader);
            Some(permit)
        };
        let (status, value, revision) = self.route(&request, Some(&stream));
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
    /// The peer lets a waiting tool call notice that its client left.
    fn route(
        &self,
        request: &wire::Request,
        peer: Option<&TcpStream>,
    ) -> (u16, Option<Value>, Option<&'static str>) {
        let started = std::time::Instant::now();
        let result = self.route_inner(request, peer);
        logging::record(request, started.elapsed(), result.0, result.1.as_ref());
        result
    }
    fn route_inner(
        &self,
        request: &wire::Request,
        peer: Option<&TcpStream>,
    ) -> (u16, Option<Value>, Option<&'static str>) {
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
        if request.path == key_check::PATH && self.globals.hosted && self.address.ip().is_loopback()
        {
            return self.key_check(request);
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
        let caller = match self.caller(request) {
            Ok(caller) => caller,
            Err(error) => return (400, Some(error), Some(revision)),
        };
        let keys = self.catalog_keys(request, &value);
        let Ok(keys) = keys else {
            return (
                400,
                Some(json!({"error":"Invalid source key headers"})),
                Some(revision),
            );
        };
        let mut context =
            crate::command::Context::new(&self.globals, &self.fetch, &self.root, &self.settings);
        context.declared_use = caller.declared_use;
        context.secrets = keys.as_ref();
        context.excluded = Some(&caller.excluded);
        let mut version = ReplyVersion::for_revision(revision);
        let result = match super::rpc::message(
            &request.body,
            &mut version,
            super::protocol::HTTP_REVISIONS,
            true,
            context,
            crate::config::mcp_tool_mode(&self.globals, &self.settings),
        ) {
            super::rpc::Message::Reply(reply) | super::rpc::Message::Initialized { reply, .. } => {
                reply
            }
            super::rpc::Message::ClientResponse { id, .. } => {
                super::protocol::error(id, -32600, "Invalid Request")
            }
            super::rpc::Message::Notification { .. } => return (202, None, Some(revision)),
            super::rpc::Message::Tool {
                id,
                params,
                version,
            } => {
                return self.tool(request, id, &params, version, (revision, caller), peer);
            }
        };
        let status = match result.pointer("/error/code").and_then(Value::as_i64) {
            Some(-32601) if revision == super::protocol::REVISIONS[0] => 404,
            _ => 200,
        };
        (status, Some(result), Some(revision))
    }
    /// Refuse bad caller policy before any source or catalog work; ordinary HTTP ignores it.
    fn caller(&self, request: &wire::Request) -> Result<Caller, Value> {
        if !self.globals.hosted {
            return Ok(Caller::default());
        }
        let declared_use = request
            .declared_use()
            .ok()
            .filter(|&usage| self.hosted_max_use.permits(usage))
            .ok_or_else(|| {
                json!({"error":format!("Invalid x-biomcp-use header; allowed values: {}", self.hosted_max_use.allowed())})
            })?;
        let excluded = request.excluded_sources().map_err(|_| {
            let allowed = biomcp_query::sources::switchable()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            json!({"error":format!(
                "Unknown source name in {} header; allowed values: {allowed}",
                crate::command::EXCLUDE_HEADER
            )})
        })?;
        let invalid =
            || json!({"error":format!("Invalid {} header", crate::saved_mcp::ACCOUNT_HEADER)});
        let account = match request
            .one(crate::saved_mcp::ACCOUNT_HEADER)
            .map_err(|_| invalid())?
        {
            None => None,
            Some(value) => Some(
                crate::saved_mcp::account(value)
                    .ok_or_else(invalid)?
                    .to_owned(),
            ),
        };
        Ok(Caller {
            declared_use: Some(declared_use),
            excluded,
            account,
        })
    }
    fn catalog_keys(
        &self,
        request: &wire::Request,
        value: &Value,
    ) -> io::Result<Option<biomcp_fetch::Secrets>> {
        let method = value.get("method").and_then(Value::as_str);
        let typed = crate::config::mcp_tool_mode(&self.globals, &self.settings)
            == crate::config::ToolMode::Typed;
        let catalog = typed
            && match method {
                Some("tools/list" | "server/discover" | "initialize") => true,
                Some("tools/call") => value.get("params").is_some_and(super::discovery::called),
                _ => false,
            };
        let skills = crate::config::mcp_skills(&self.settings)
            && matches!(
                method,
                Some("skills/list" | "skills/get" | "resources/list" | "resources/read")
            );
        (self.globals.hosted && (catalog || skills))
            .then(|| request.source_keys())
            .transpose()
    }

    fn tool(
        &self,
        request: &wire::Request,
        id: Value,
        params: &Value,
        version: ReplyVersion,
        (revision, caller): (&'static str, Caller),
        peer: Option<&TcpStream>,
    ) -> (u16, Option<Value>, Option<&'static str>) {
        let scope = self
            .globals
            .hosted
            .then(|| biomcp_fetch::Cancellation::hosted(crate::config::budget(&self.settings)));
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
        let context = crate::command::Context {
            globals: &self.globals,
            root: &self.root,
            settings: &self.settings,
            fetch: &self.fetch,
            recovery: self.recovery.as_deref(),
            secrets: keys.as_ref(),
            excluded: Some(&caller.excluded),
            declared_use: caller.declared_use,
            scope: scope.as_ref(),
            account: caller.account.as_deref(),
        };
        let call = match self.parse_tool(&id, params, version, context) {
            Ok(call) => call,
            Err(reply) => return (200, Some(reply), Some(revision)),
        };
        let declared_use = self.globals.hosted.then(|| {
            caller
                .declared_use
                .unwrap_or(crate::command::DeclaredUse::Restricted)
        });
        let confirmation =
            if version == ReplyVersion::Current && super::elicitation::delete_all(&call) {
                super::current::confirmation(params)
            } else {
                None
            };
        let result = self.calls.run(
            self,
            ToolCall {
                id,
                version,
                call,
                secrets: keys,
                declared_use,
                excluded: caller.excluded,
                scope,
                account: caller.account,
                confirmation,
            },
            peer,
        );
        (200, Some(result), Some(revision))
    }
    fn parse_tool(
        &self,
        id: &Value,
        params: &Value,
        version: ReplyVersion,
        context: crate::command::Context<'_>,
    ) -> Result<super::research::Call, Value> {
        let call = super::typed::parse(
            context,
            params,
            crate::config::mcp_tool_mode(&self.globals, &self.settings),
            self.globals.fresh,
            true,
        )
        .ok_or_else(|| super::protocol::error(id.clone(), -32602, "Invalid params"))?;
        call.map_err(|rejected| {
            super::typed::rejection(id.clone(), rejected, version, context, true)
                .unwrap_or_else(|_| super::protocol::error(id.clone(), -32603, "Internal error"))
        })
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
