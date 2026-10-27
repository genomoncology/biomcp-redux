//! A public request cannot intentionally break a worker invariant.
#[test]
fn panicking_worker_completes_with_internal_error() {
    use super::super::{Event, Server};
    use serde_json::json;
    let (sender, receiver) = std::sync::mpsc::sync_channel(32);
    let mut server = Server {
        sender,
        active: std::collections::BTreeMap::new(),
        legacy: false,
        globals: crate::grammar::Globals::default(),
        fetch: Err(crate::config::StartupError(None)),
    };
    server
        .start_call("panic".into(), json!("panic"), true, None, || {
            panic!("injected worker failure")
        })
        .unwrap();
    let Event::Completed(key, outcome) = receiver
        .recv_timeout(std::time::Duration::from_secs(5))
        .unwrap()
    else {
        panic!("worker did not send its completion")
    };
    let reply = server.complete(&key, outcome).unwrap().unwrap();
    assert_eq!(reply.get("id"), Some(&json!("panic")));
    assert_eq!(reply.pointer("/error/code"), Some(&json!(-32603)));
    assert!(
        server.active.is_empty(),
        "EOF would retain an occupied slot"
    );
}
