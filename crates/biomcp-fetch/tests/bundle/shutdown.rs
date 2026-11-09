//! The server owner interrupts a held publisher before joining its worker.
use super::*;

#[test]
fn last_refresh_owner_stops_a_held_download_before_its_response_is_released() {
    // Keep the upstream open beyond the shutdown watchdog: its own timeout
    // must not accidentally satisfy the cancellation contract.
    let server = support::Server::with_reply_timeout(Duration::from_secs(60));
    let bundle = declared(&server);
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let fetch = cached(&path).with_background_refresh();
    sync_reply(&fetch, &bundle, &server, 200, "", TSV).unwrap();
    let db = rusqlite::Connection::open(&path).unwrap();
    db.execute("UPDATE bundle SET fresh_until=0", []).unwrap();
    let (call, cancellation) = fetch.cancellable().unwrap();
    let leg = call.leg(bundle.spec, call.deadline(Duration::from_secs(30)));
    let rows = leg
        .rows(&bundle, Find::Exact(KeyKind::Symbol, "BRAF"))
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert!(leg.finish().stale);
    let held = server.next();
    cancellation.cancel();
    drop(call);

    let (stopped, completion) = std::sync::mpsc::channel();
    let owner = std::thread::spawn(move || {
        drop(fetch);
        stopped.send(()).unwrap();
    });
    let result = completion.recv_timeout(Duration::from_secs(30));
    // Release the fixture even on the old implementation, so the failed
    // regression reaps its worker instead of leaving a detached download.
    held.respond(400, "", "cancelled fixture");
    owner.join().unwrap();
    assert!(
        result.is_ok(),
        "owner waited for the held upstream: {result:?}"
    );
    assert!(
        server.requests.try_recv().is_err(),
        "shutdown retried the download"
    );
    let offline = cached(&path).offline();
    let leg = offline.leg(bundle.spec, offline.deadline(Duration::from_secs(30)));
    let retained = leg
        .rows(&bundle, Find::Exact(KeyKind::Symbol, "BRAF"))
        .unwrap();
    assert_eq!(retained.len(), 1);
    assert_eq!(retained[0].name, rows[0].name);
    assert!(
        leg.finish().stale,
        "shutdown published an incomplete refresh"
    );
}
