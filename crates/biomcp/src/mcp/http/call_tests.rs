//! Private worker ownership needs held exit events and the actual service capacity.
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
        calls: Calls::default(),
        address: "127.0.0.1:1".parse().unwrap(),
        hosts: Vec::new(),
        globals: Globals::default(),
        fetch: Err(StartupError::Local),
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
