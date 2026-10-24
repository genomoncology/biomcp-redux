//! Checked reply cache through the public transport API.
mod support;
use biomcp_fetch::{Fetch, FetchConfig, Freshness, KeyUse, Quota, Rate, SourceKey};
use std::{thread, time::Duration};
use support::{Answer, Server};

#[test]
fn fresh_checked_reply_avoids_a_second_request() {
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new();
    let quota = Box::leak(Box::new(Quota::new(
        SourceKey::declare("cache"),
        Rate::per_second(1000),
        None,
        8,
    )));
    let spec = server.spec(quota, KeyUse::None);
    let fetch = Fetch::open(FetchConfig {
        cache: Some(directory.path().join("cache.sqlite")),
        ..FetchConfig::default()
    })
    .unwrap();
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    thread::scope(|scope| {
        let first = scope.spawn(|| {
            fetch
                .leg(spec, fetch.deadline(Duration::from_secs(10)))
                .json::<Answer>(&endpoint, &[])
                .unwrap()
        });
        server.next().ok();
        assert!(first.join().unwrap().answer);
    });
    assert!(
        fetch
            .leg(spec, fetch.deadline(Duration::from_secs(10)))
            .json::<Answer>(&endpoint, &[])
            .unwrap()
            .answer
    );
    assert!(server.requests.try_recv().is_err());
}
