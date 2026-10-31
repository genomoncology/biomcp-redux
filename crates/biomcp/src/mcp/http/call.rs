//! Own bounded hosted tool workers through completion and return whole-call timeouts.
use super::{MAX_CONNECTIONS, Service};
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
    sync::{Mutex, mpsc},
    thread::JoinHandle,
};

#[derive(Default)]
#[expect(
    clippy::disallowed_types,
    reason = "The HTTP service owns its bounded active tool workers"
)]
pub(super) struct Calls(Mutex<Vec<Active>>);
struct Active {
    worker: JoinHandle<()>,
    scope: Cancellation,
}
pub(super) struct HostedCall {
    pub(super) id: Value,
    pub(super) modern: bool,
    pub(super) call: research::Call,
    pub(super) secrets: Option<Secrets>,
    pub(super) scope: Cancellation,
}

impl Calls {
    #[expect(
        clippy::disallowed_methods,
        reason = "Retain every hosted tool worker until it completes"
    )]
    pub(super) fn run(&self, service: &Service, work: HostedCall) -> Value {
        let HostedCall {
            id,
            modern,
            call,
            secrets,
            scope,
        } = work;
        let mut active = self.0.lock().unwrap_or_else(|e| e.into_inner());
        for index in (0..active.len()).rev() {
            if active
                .get(index)
                .is_some_and(|call| call.worker.is_finished())
            {
                let _ = active.swap_remove(index).worker.join();
            }
        }
        if active.len() >= MAX_CONNECTIONS {
            return protocol::error(id, -31000, "Tool capacity exceeded");
        }
        if scope.remaining().is_zero() {
            return protocol::timeout(id);
        }
        let globals = service.globals.clone();
        let root = service.root.clone();
        let settings = service.settings.clone();
        let fetch = service.fetch.clone();
        let recovery = service.recovery.clone();
        let worker_scope = scope.clone();
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
                    scope: Some(&worker_scope),
                };
                let outcome = worker::execute(|| {
                    research::execute(&call, context, true)
                        .map(|outcome| protocol::tool_result(worker_id, &outcome, modern))
                });
                let _ = sender.send(outcome);
            });
        let Ok(worker) = worker else {
            return protocol::error(id, -32603, "Internal error");
        };
        active.push(Active {
            worker,
            scope: scope.clone(),
        });
        drop(active);
        wait(&scope, receiver, id)
    }
    pub(super) fn cancel(&self) {
        for call in self.0.lock().unwrap_or_else(|e| e.into_inner()).iter() {
            call.scope.cancel();
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
            .drain(..)
        {
            let _ = call.worker.join();
        }
    }
}

fn wait(
    scope: &Cancellation,
    receiver: mpsc::Receiver<std::io::Result<Value>>,
    id: Value,
) -> Value {
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
    use super::{Active, Calls, HostedCall, MAX_CONNECTIONS, Service};
    use crate::{
        config::StartupError,
        grammar::Globals,
        mcp::research::{Call, Projection},
    };
    use biomcp_fetch::Cancellation;
    use serde_json::json;
    use std::{sync::mpsc, time::Duration};

    const DEADLOCK_GUARD: Duration = Duration::from_secs(60);

    #[test]
    #[expect(
        clippy::disallowed_methods,
        reason = "Held worker exit events prove private HTTP capacity and cleanup ownership"
    )]
    fn expired_calls_keep_capacity_and_shutdown_waits_for_actual_worker_exit() {
        let service = Service {
            root: std::sync::Arc::new(crate::grammar::root().unwrap()),
            settings: std::sync::Arc::new(Ok(None)),
            calls: Calls::default(),
            address: "127.0.0.1:1".parse().unwrap(),
            hosts: Vec::new(),
            globals: Globals::default(),
            fetch: Err(StartupError::Local),
            recovery: None,
        };
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
            service.calls.0.lock().unwrap().push(Active {
                worker,
                scope: Cancellation::with_budget(Duration::ZERO),
            });
        }
        let reply = service.calls.run(
            &service,
            HostedCall {
                id: json!(1),
                modern: false,
                call: Call {
                    args: Vec::new(),
                    projection: Projection::Cli,
                },
                secrets: None,
                scope: Cancellation::with_budget(DEADLOCK_GUARD),
            },
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
