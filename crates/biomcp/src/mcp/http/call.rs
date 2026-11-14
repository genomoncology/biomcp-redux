//! Own bounded HTTP tool workers through completion and return hosted whole-call timeouts.
//! A call over the limit waits its turn in arrival order, within its own deadline.
use super::Service;
use crate::{
    command::Context,
    mcp::{protocol, research, worker},
};
use biomcp_fetch::{Cancellation, Secrets};
use serde_json::Value;
#[expect(
    clippy::disallowed_types,
    reason = "The HTTP service owns its bounded active tool workers"
)]
use std::{
    collections::VecDeque,
    net::TcpStream,
    sync::{Mutex, MutexGuard, mpsc},
    thread::JoinHandle,
    time::{Duration, Instant},
};

#[derive(Default)]
#[expect(
    clippy::disallowed_types,
    reason = "The HTTP service owns its bounded active tool workers"
)]
pub(super) struct Calls(Mutex<State>);
#[derive(Default)]
struct State {
    active: Vec<Active>,
    /// Tickets of waiting calls in arrival order; only the first may take a free slot.
    waiting: VecDeque<u64>,
    issued: u64,
    closed: bool,
}
struct Active {
    worker: JoinHandle<()>,
    /// When the call must have ended, which dates the retry advice.
    scope: Cancellation,
}
pub(super) struct ToolCall {
    pub(super) id: Value,
    pub(super) modern: bool,
    pub(super) call: research::Call,
    pub(super) secrets: Option<Secrets>,
    pub(super) excluded: std::collections::BTreeSet<biomcp_types::Source>,
    pub(super) declared_use: Option<crate::command::DeclaredUse>,
    /// The hosted whole-call deadline, which a slow source's declared budget may lengthen;
    /// plain HTTP has none, and its wait ends at the command budget.
    pub(super) scope: Option<Cancellation>,
}

impl Calls {
    #[expect(
        clippy::disallowed_methods,
        reason = "Retain every hosted tool worker until it completes"
    )]
    pub(super) fn run(&self, service: &Service, work: ToolCall, peer: Option<&TcpStream>) -> Value {
        let ToolCall {
            id,
            modern,
            call,
            secrets,
            excluded,
            declared_use,
            scope,
        } = work;
        let budget = crate::config::budget(&service.settings);
        let patience = service
            .queue_wait
            .min(scope.as_ref().map_or(budget, Cancellation::remaining));
        let mut state = match self.turn(service.max_calls, patience, peer) {
            Ok(state) => state,
            Err((waited, retry)) => {
                return protocol::capacity(id, service.max_calls, waited, retry);
            }
        };
        let expired = scope
            .as_ref()
            .is_some_and(|scope| scope.remaining().is_zero());
        if expired {
            return protocol::timeout(id);
        }
        let globals = service.globals.clone();
        let root = service.root.clone();
        let settings = service.settings.clone();
        let fetch = service.fetch.clone();
        let recovery = service.recovery.clone();
        let worker_scope = scope.clone();
        let held = scope
            .clone()
            .unwrap_or_else(|| Cancellation::with_budget(budget));
        let worker_id = id.clone();
        let (sender, receiver) = mpsc::sync_channel(1);
        let worker = std::thread::Builder::new()
            .name("mcp-http-tool".into())
            .spawn(move || {
                let context = Context {
                    globals: &globals,
                    root: &root,
                    settings: &settings,
                    fetch: &fetch,
                    recovery: recovery.as_deref(),
                    secrets: secrets.as_ref(),
                    excluded: Some(&excluded),
                    declared_use,
                    scope: worker_scope.as_ref(),
                };
                let outcome = worker::execute(|| {
                    research::execute(&call, context, true).map(|outcome| {
                        super::super::typed::tool_result(
                            worker_id,
                            &outcome,
                            modern,
                            (context.globals, context.settings),
                        )
                    })
                });
                let _ = sender.send(outcome);
            });
        let worker = match worker {
            Ok(worker) => worker,
            Err(error) => {
                super::connections::failure("tool thread start", error);
                return protocol::error(id, -32603, "Internal error");
            }
        };
        state.active.push(Active {
            worker,
            scope: held,
        });
        drop(state);
        wait(scope.as_ref(), receiver, id)
    }
    /// Hold the lock with a free slot, or say how long this call waited and when to retry.
    fn turn(
        &self,
        max_calls: usize,
        wait: Duration,
        peer: Option<&TcpStream>,
    ) -> Result<MutexGuard<'_, State>, (Duration, Duration)> {
        let arrived = Instant::now();
        let mut ticket = None;
        loop {
            let mut state = self.0.lock().unwrap_or_else(|e| e.into_inner());
            state.reap();
            let first = match ticket {
                Some(ticket) => state.waiting.front() == Some(&ticket),
                None => state.waiting.is_empty(),
            };
            if first && state.active.len() < max_calls {
                if ticket.is_some() {
                    state.waiting.pop_front();
                }
                return Ok(state);
            }
            let waited = arrived.elapsed();
            // Only a waiting call checks its client, so a free slot never costs a socket probe.
            let left = ticket.is_some() && peer.is_some_and(gone);
            if state.closed || waited >= wait || left {
                state.waiting.retain(|waiting| Some(*waiting) != ticket);
                let retry = state.active.iter().map(|call| call.scope.remaining()).min();
                return Err((waited, retry.unwrap_or_default()));
            }
            if ticket.is_none() {
                state.issued = state.issued.wrapping_add(1);
                let issued = state.issued;
                state.waiting.push_back(issued);
                ticket = Some(issued);
            }
            drop(state);
            std::thread::sleep(super::ACCEPT_POLL.min(wait.saturating_sub(waited)));
        }
    }
    /// Stop running calls and send every waiting call away; the server is shutting down.
    pub(super) fn cancel(&self) {
        let mut state = self.0.lock().unwrap_or_else(|e| e.into_inner());
        state.closed = true;
        for call in &state.active {
            call.scope.cancel();
        }
    }
}
/// A client that closed its connection can get no answer, so its waiting call gives up its place.
/// A client that half-closes after its request is treated as gone; HTTP clients wait open.
fn gone(peer: &TcpStream) -> bool {
    if peer.set_nonblocking(true).is_err() {
        return false;
    }
    let closed = match peer.peek(&mut [0]) {
        Ok(read) => read == 0,
        Err(error) => !matches!(
            error.kind(),
            std::io::ErrorKind::WouldBlock | std::io::ErrorKind::Interrupted
        ),
    };
    // A socket that cannot return to blocking writes cannot carry the answer either.
    peer.set_nonblocking(false).is_err() || closed
}
impl State {
    fn reap(&mut self) {
        for index in (0..self.active.len()).rev() {
            if self
                .active
                .get(index)
                .is_some_and(|call| call.worker.is_finished())
            {
                let _ = self.active.swap_remove(index).worker.join();
            }
        }
    }
}
impl Drop for Calls {
    fn drop(&mut self) {
        self.cancel();
        for call in self
            .0
            .get_mut()
            .unwrap_or_else(|e| e.into_inner())
            .active
            .drain(..)
        {
            let _ = call.worker.join();
        }
    }
}

fn wait(
    scope: Option<&Cancellation>,
    receiver: mpsc::Receiver<std::io::Result<Value>>,
    id: Value,
) -> Value {
    let Some(scope) = scope else {
        // Plain HTTP has no whole-call deadline; the command's own budget ends its work.
        let reply = receiver
            .recv()
            .unwrap_or_else(|_| Err(std::io::ErrorKind::Other.into()));
        return reply.unwrap_or_else(|_| protocol::error(id, -32603, "Internal error"));
    };
    loop {
        let remaining = scope.remaining();
        if remaining.is_zero() {
            scope.cancel();
            return protocol::timeout(id);
        }
        match receiver.recv_timeout(remaining.min(super::ACCEPT_POLL)) {
            Ok(reply) => {
                if scope.remaining().is_zero() {
                    scope.cancel();
                    return protocol::timeout(id);
                }
                return reply.unwrap_or_else(|_| protocol::error(id, -32603, "Internal error"));
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                return protocol::error(id, -32603, "Internal error");
            }
            Err(mpsc::RecvTimeoutError::Timeout) => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Active, Calls, Service, ToolCall};
    use crate::config::DEFAULT_HTTP_CAPACITY as MAX_CONNECTIONS;
    use crate::{
        config::StartupError,
        grammar::Globals,
        mcp::research::{Call, Projection},
    };
    use biomcp_fetch::Cancellation;
    use serde_json::json;
    use std::{sync::mpsc, time::Duration};

    pub(super) fn test_service() -> Service {
        Service {
            hosted_max_use: Default::default(),
            root: std::sync::Arc::new(crate::grammar::root().unwrap()),
            settings: std::sync::Arc::new(Ok(None)),
            calls: Calls::default(),
            connections: super::super::admission::Slots::new(MAX_CONNECTIONS),
            max_calls: MAX_CONNECTIONS,
            queue_wait: Duration::ZERO,
            max_connections: MAX_CONNECTIONS,
            address: "127.0.0.1:1".parse().unwrap(),
            hosts: Vec::new(),
            globals: Globals::default(),
            fetch: Err(StartupError::Local),
            recovery: None,
        }
    }
    #[test]
    #[expect(
        clippy::disallowed_methods,
        reason = "Inject OS failures and observe recovery over a real local socket"
    )]
    fn accept_and_spawn_failures_leave_server_running() {
        use std::{
            io::{Read, Write},
            net::{TcpListener, TcpStream},
            sync::Arc,
        };
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let mut service = test_service();
        service.address = address;
        let (stop, stopped) = mpsc::sync_channel(1);
        let (failed, observed) = mpsc::channel();
        let server = std::thread::spawn(move || {
            let mut first_accept = true;
            let mut first_spawn = true;
            super::super::connections::run_with(
                listener,
                Arc::new(service),
                stopped,
                |listener| {
                    if std::mem::take(&mut first_accept) {
                        return Err(std::io::Error::other("forced accept failure"));
                    }
                    listener.accept().map(|(stream, _)| stream)
                },
                |work| {
                    if std::mem::take(&mut first_spawn) {
                        failed.send(()).unwrap();
                        return Err(std::io::Error::other("forced thread failure"));
                    }
                    std::thread::Builder::new().spawn(work)
                },
            )
        });
        let rejected = TcpStream::connect(address).unwrap();
        observed.recv_timeout(DEADLOCK_GUARD).unwrap();
        drop(rejected);
        let mut client = TcpStream::connect(address).unwrap();
        client.set_read_timeout(Some(DEADLOCK_GUARD)).unwrap();
        write!(client, "GET /health HTTP/1.1\r\nHost: {address}\r\n\r\n").unwrap();
        let mut reply = String::new();
        client.read_to_string(&mut reply).unwrap();
        assert!(reply.starts_with("HTTP/1.1 503 "), "{reply}");
        assert!(reply.contains("store"));
        stop.send(()).unwrap();
        server.join().unwrap().unwrap();
    }
    #[test]
    fn call_diagnostic_keeps_only_safe_fields() {
        let params = json!({"name":"gene", "arguments":{"action":"get","get":{"id":"private-query", "key":"private-key"}}});
        let reply = json!({"result":{"structuredContent":{
            "status":"partial", "query":"private-query", "sources":[
                {"source":"mygene", "state":"failed", "term":"private-key"},
                {"source":"private-key", "state":"failed"}
            ]
        }}});
        let line =
            super::super::logging::line(&params, Duration::from_millis(7), 200, Some(&reply));
        assert_eq!(
            line,
            json!({"event":"mcp_call", "tool":"gene", "duration_ms":7, "status":"partial", "failed_sources":["mygene"]})
        );
        let envelope = &reply["result"]["structuredContent"];
        for content in [
            json!({"content":[{"text":format!("private-query\n{envelope}")}]}),
            json!({"structuredContent":{"metadata":{"envelope":envelope}}}),
            json!({"structuredContent":{"metadata":envelope}}),
        ] {
            let reply = json!({"result":content});
            assert_eq!(
                super::super::logging::line(&params, Duration::from_millis(7), 200, Some(&reply)),
                line
            );
        }
        let unsafe_params = json!({"name":"private-query"});
        let line = super::super::logging::line(&unsafe_params, Duration::ZERO, 200, None);
        assert_eq!(line["tool"], "unknown");
        assert!(!line.to_string().contains("private"));
    }

    #[test]
    fn call_diagnostic_names_every_served_tool() {
        let service = test_service();
        let context = crate::command::Context::new(
            &service.globals,
            &service.fetch,
            &service.root,
            &service.settings,
        );
        let tools = crate::mcp::typed::tools(context, false).unwrap();
        assert_eq!(tools.len(), 17);
        let log = |name: &serde_json::Value| {
            super::super::logging::line(&json!({"name":name}), Duration::ZERO, 200, None)
        };
        for name in tools.iter().map(|tool| &tool["name"]) {
            assert_eq!(&log(name)["tool"], name);
        }
        for name in [
            "biomcp_private",
            "get_private",
            "get_gene",
            "biomcp_reference",
            "biomcp_help",
            "BIOMCP",
            "",
        ] {
            assert_eq!(log(&json!(name))["tool"], "unknown");
        }
    }

    const DEADLOCK_GUARD: Duration = Duration::from_secs(60);

    #[test]
    #[expect(
        clippy::disallowed_methods,
        reason = "Held worker exit events prove private HTTP capacity and cleanup ownership"
    )]
    fn expired_calls_keep_capacity_and_shutdown_waits_for_actual_worker_exit() {
        let service = test_service();
        let mut releases = Vec::new();
        let mut exits = Vec::new();
        let (leaving, observed) = mpsc::channel();
        for _ in 0..MAX_CONNECTIONS {
            let (release, released) = mpsc::channel::<()>();
            let (exit, exiting) = mpsc::channel::<()>();
            releases.push(release);
            exits.push(exit);
            let leaving = leaving.clone();
            let worker = std::thread::spawn(move || {
                let _ = released.recv();
                leaving.send(()).unwrap();
                let _ = exiting.recv();
            });
            service.calls.0.lock().unwrap().active.push(Active {
                worker,
                scope: Cancellation::with_budget(Duration::ZERO),
            });
        }
        let reply = service.calls.run(
            &service,
            ToolCall {
                id: json!(1),
                modern: false,
                call: Call {
                    args: Vec::new(),
                    projection: Projection::Cli,
                    catalog: None,
                },
                secrets: None,
                excluded: Default::default(),
                declared_use: Some(crate::command::DeclaredUse::Academic),
                scope: Some(Cancellation::with_budget(DEADLOCK_GUARD)),
            },
            None,
        );
        assert_eq!(
            reply.pointer("/error/code"),
            Some(&json!(-31000)),
            "expired running workers must retain all server slots"
        );
        let (finished, finish) = mpsc::channel();
        let cleanup = std::thread::spawn(move || {
            drop(service);
            finished.send(()).unwrap();
        });
        drop(releases);
        for _ in 0..MAX_CONNECTIONS {
            observed.recv_timeout(DEADLOCK_GUARD).unwrap();
        }
        assert!(
            finish.try_recv().is_err(),
            "shutdown must own workers still held before actual exit"
        );
        drop(exits);
        finish.recv_timeout(DEADLOCK_GUARD).unwrap();
        cleanup.join().unwrap();
    }
}

#[cfg(test)]
#[expect(
    clippy::disallowed_types,
    clippy::disallowed_methods,
    reason = "Measure explicitly invoked bounded HTTP load against one owned service and Fetch"
)]
#[path = "load_benchmark.rs"]
mod load_benchmark;
