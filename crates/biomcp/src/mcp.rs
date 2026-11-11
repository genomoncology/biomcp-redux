//! Stdio scheduling and the single response writer.
mod current;
mod framing;
mod hosted;
pub(crate) mod http;
mod legacy;
mod output_schema;
mod protocol;
mod queue;
mod research;
mod rpc;
mod typed;
/// Each use case's worked example with the typed call this server derives from its command.
pub(crate) use typed::example_calls;
mod worker;

use framing::Frame;
use serde_json::Value;
use std::{
    collections::BTreeMap,
    io::{self, Write},
    sync::mpsc,
    thread,
    time::Instant,
};

const MAX_RETAINED_EVENTS: usize = 32;

enum Event {
    Input(Frame),
    Completed(String, io::Result<research::Completed>),
}

struct Call {
    id: Value,
    modern: bool,
    cancelled: bool,
    cancellation: Option<biomcp_fetch::Cancellation>,
    worker: thread::JoinHandle<()>,
    ends: Instant,
}

struct Server {
    sender: mpsc::SyncSender<Event>,
    active: BTreeMap<String, Call>,
    max_calls: usize,
    queue: queue::Queue,
    legacy: bool,
    globals: crate::grammar::Globals,
    root: std::sync::Arc<crate::grammar::Root>,
    settings: std::sync::Arc<crate::config::Settings>,
    fetch: Result<std::sync::Arc<biomcp_fetch::Fetch>, crate::config::StartupError>,
    recovery: Option<std::sync::Arc<Recovery>>,
}

pub(crate) fn serve(
    globals: crate::grammar::Globals,
    fetch: Result<biomcp_fetch::Fetch, crate::config::StartupError>,
    root: std::sync::Arc<crate::grammar::Root>,
    settings: std::sync::Arc<crate::config::Settings>,
) -> io::Result<()> {
    let (sender, receiver) = mpsc::sync_channel(MAX_RETAINED_EVENTS);
    start_input(sender.clone())?;
    let recovery = fetch.is_err().then(|| std::sync::Arc::new(Recovery::new()));
    let mut server = Server {
        sender,
        active: BTreeMap::new(),
        max_calls: crate::config::mcp_capacity(&settings),
        queue: queue::Queue::new(&settings),
        legacy: false,
        globals,
        root,
        settings,
        fetch: fetch.map(|fetch| std::sync::Arc::new(fetch.with_background_refresh())),
        recovery,
    };
    let mut output = io::stdout().lock();
    let mut eof = false;
    while !eof || !server.active.is_empty() {
        let event = next_event(&receiver, server.queue.next_expiry())?;
        let mut replies = server.refuse_expired();
        replies.extend(match event {
            None => None,
            Some(Event::Input(Frame::Eof)) => {
                eof = true;
                None
            }
            Some(Event::Input(Frame::Malformed)) => {
                eof = true;
                Some(protocol::error(Value::Null, -32700, "Parse error"))
            }
            Some(Event::Input(Frame::Oversized)) => {
                Some(protocol::error(Value::Null, -32600, "Frame too large"))
            }
            Some(Event::Input(Frame::Message(frame))) => server.message(&frame)?,
            Some(Event::Completed(key, outcome)) => server.complete(&key, outcome)?,
        });
        for reply in replies {
            let mut bytes = serde_json::to_vec(&reply)?;
            bytes.push(b'\n');
            output.write_all(&bytes)?;
            output.flush()?;
        }
    }
    // The input thread may still be in a blocking stdin read after output breaks.
    // Dropping its handle lets main return without joining that read.
    Ok(())
}

/// Wait for the next event, or only until the longest-waiting queued call must be refused.
fn next_event(
    receiver: &mpsc::Receiver<Event>,
    expiry: Option<Instant>,
) -> io::Result<Option<Event>> {
    let stopped = || io::Error::other("MCP input stopped");
    let Some(expiry) = expiry else {
        return receiver.recv().map(Some).map_err(|_| stopped());
    };
    match receiver.recv_timeout(expiry.saturating_duration_since(Instant::now())) {
        Ok(event) => Ok(Some(event)),
        Err(mpsc::RecvTimeoutError::Timeout) => Ok(None),
        Err(mpsc::RecvTimeoutError::Disconnected) => Err(stopped()),
    }
}

#[expect(
    clippy::disallowed_methods,
    reason = "Stdio input owns the bounded frame channel"
)]
fn start_input(sender: mpsc::SyncSender<Event>) -> io::Result<()> {
    thread::Builder::new()
        .name("mcp-input".into())
        .spawn(move || {
            let mut input = io::stdin().lock();
            loop {
                let frame = framing::read(&mut input).unwrap_or(Frame::Malformed);
                let terminal = matches!(frame, Frame::Malformed | Frame::Eof);
                if sender.send(Event::Input(frame)).is_err() || terminal {
                    break;
                }
            }
        })?;
    Ok(())
}

impl Server {
    fn message(&mut self, frame: &[u8]) -> io::Result<Option<Value>> {
        match rpc::message(
            frame,
            &mut self.legacy,
            self.globals.hosted,
            crate::command::Context::new(&self.globals, &self.fetch, &self.root, &self.settings),
            crate::config::mcp_tool_mode(&self.globals, &self.settings),
        ) {
            rpc::Message::Reply(reply) => Ok(Some(reply)),
            rpc::Message::Tool { id, params, modern } => self.admit(id, &params, modern),
            rpc::Message::Notification { method, params } => {
                if method == "notifications/cancelled"
                    && let Some(id) = params.get("requestId").filter(|id| protocol::valid_id(id))
                {
                    self.cancel(&id.to_string());
                }
                Ok(None)
            }
        }
    }

    fn admit(&mut self, id: Value, params: &Value, modern: bool) -> io::Result<Option<Value>> {
        let Some(call) = typed::parse(
            crate::command::Context::new(&self.globals, &self.fetch, &self.root, &self.settings),
            params,
            crate::config::mcp_tool_mode(&self.globals, &self.settings),
            self.globals.fresh,
            false,
        ) else {
            return Ok(Some(protocol::error(id, -32602, "Invalid params")));
        };
        let call = match call {
            Ok(call) => call,
            Err(rejected) => {
                return typed::rejection(
                    id,
                    rejected,
                    modern,
                    crate::command::Context::new(
                        &self.globals,
                        &self.fetch,
                        &self.root,
                        &self.settings,
                    ),
                    false,
                )
                .map(Some);
            }
        };
        let key = id.to_string();
        if self.active.contains_key(&key) || self.queue.contains(&key) {
            return Ok(Some(protocol::error(id, -32600, "Request already active")));
        }
        let full = self.active.len() >= self.max_calls;
        if full && self.queue.wait().is_zero() {
            return Ok(Some(self.refusal(id)));
        }
        let globals = self.globals.clone();
        let root = self.root.clone();
        let settings = self.settings.clone();
        let recovery = self.recovery.clone();
        let (fetch, cancellation) = match &self.fetch {
            Ok(fetch) => {
                let Ok((fetch, cancellation)) = fetch.cancellable() else {
                    return Ok(Some(protocol::error(id, -32603, "Internal error")));
                };
                (Ok(std::sync::Arc::new(fetch)), Some(cancellation))
            }
            Err(error) => (
                Err(error.clone()),
                Some(biomcp_fetch::Cancellation::with_budget(
                    crate::config::budget(&settings),
                )),
            ),
        };
        let scope = recovery.as_ref().and(cancellation.clone());
        let work: queue::Work = Box::new(move || {
            let mut context = crate::command::Context::new(&globals, &fetch, &root, &settings);
            context.recovery = recovery.as_deref();
            context.scope = scope.as_ref();
            research::execute(&call, context, false)
        });
        if full {
            self.queue.push(key, id, modern, cancellation, work);
        } else {
            self.start_call(key, id, modern, cancellation, work)?;
        }
        Ok(None)
    }

    fn cancel(&mut self, key: &str) {
        if let Some(call) = self.active.get_mut(key) {
            call.cancelled = true;
            if let Some(cancellation) = &call.cancellation {
                cancellation.cancel();
            }
        }
        self.queue.remove(key);
    }

    /// The timed refusal: how long this call waited and when a running call must have ended.
    fn refusal(&self, id: Value) -> Value {
        let now = Instant::now();
        let retry = self
            .active
            .values()
            .map(|call| call.ends.saturating_duration_since(now))
            .min()
            .unwrap_or_default();
        protocol::capacity(id, self.max_calls, self.queue.wait(), retry)
    }

    fn refuse_expired(&mut self) -> Vec<Value> {
        let mut replies = Vec::new();
        while let Some(call) = self.queue.pop_expired() {
            replies.push(self.refusal(call.id));
        }
        replies
    }

    fn start_waiting(&mut self) -> io::Result<()> {
        while self.active.len() < self.max_calls
            && let Some(call) = self.queue.pop()
        {
            self.start_call(call.key, call.id, call.modern, call.cancellation, call.work)?;
        }
        Ok(())
    }

    #[expect(
        clippy::disallowed_methods,
        reason = "Bounded tool workers own command execution"
    )]
    fn start_call(
        &mut self,
        key: String,
        id: Value,
        modern: bool,
        cancellation: Option<biomcp_fetch::Cancellation>,
        work: impl FnOnce() -> io::Result<research::Completed> + Send + 'static,
    ) -> io::Result<()> {
        let sender = self.sender.clone();
        let worker_key = key.clone();
        let worker = thread::Builder::new()
            .name("mcp-tool".into())
            .spawn(move || {
                let outcome = worker::execute(work);
                let _ = sender.send(Event::Completed(worker_key, outcome));
            })?;
        let now = Instant::now();
        self.active.insert(
            key,
            Call {
                id,
                modern,
                cancelled: false,
                cancellation,
                worker,
                ends: now.checked_add(self.queue.budget).unwrap_or(now),
            },
        );
        Ok(())
    }

    fn complete(
        &mut self,
        key: &str,
        outcome: io::Result<research::Completed>,
    ) -> io::Result<Option<Value>> {
        let Some(call) = self.active.remove(key) else {
            return Ok(None);
        };
        call.worker
            .join()
            .map_err(|_| io::Error::other("MCP worker stopped"))?;
        self.start_waiting()?;
        if call.cancelled {
            return Ok(None);
        }
        Ok(Some(match outcome {
            Ok(outcome) => typed::tool_result(call.id, &outcome, call.modern, &self.settings),
            Err(_) => protocol::error(call.id, -32603, "Internal error"),
        }))
    }
}

/// A failed startup may publish exactly one successful static transport snapshot.
#[derive(Clone)]
pub(crate) struct Snapshot {
    pub(crate) fetch: std::sync::Arc<biomcp_fetch::Fetch>,
    pub(crate) settings: std::sync::Arc<crate::config::Settings>,
}
#[expect(
    clippy::disallowed_types,
    reason = "Serialize failed startup retries until one shared transport snapshot succeeds"
)]
pub(crate) struct Recovery(std::sync::Mutex<Option<Snapshot>>);
impl Recovery {
    #[expect(
        clippy::disallowed_types,
        reason = "The MCP setup owner alone publishes the first successful static snapshot"
    )]
    pub(crate) fn new() -> Self {
        Self(std::sync::Mutex::new(None))
    }
    pub(crate) fn snapshot(
        &self,
        globals: &crate::grammar::Globals,
    ) -> Result<(Snapshot, bool), crate::config::StartupError> {
        let mut stored = self.0.lock().unwrap_or_else(|error| error.into_inner());
        if let Some(snapshot) = stored.as_ref() {
            return Ok((snapshot.clone(), false));
        }
        let settings = std::sync::Arc::new(crate::config::file::read_file(globals.config.as_ref()));
        let fetch = std::sync::Arc::new(
            crate::config::fetch(globals, &settings)?.with_background_refresh(),
        );
        let snapshot = Snapshot { fetch, settings };
        *stored = Some(snapshot.clone());
        Ok((snapshot, true))
    }
}
