//! Scoped chunk callbacks preserve caller order and transport ownership.
#![expect(
    clippy::disallowed_methods,
    reason = "Scoped callers exercise the public batch API against event-driven local peers"
)]
mod support;
use biomcp_fetch::{Fetch, FetchConfig, KeyUse, Quota, Rate, RequestError, SourceKey};
use std::{sync::mpsc, thread, time::Duration};
use support::{Answer, Server};

fn quota(in_flight: u8) -> &'static Quota {
    Box::leak(Box::new(Quota::new(
        SourceKey::declare("batch"), Rate::new(1000, 1000), None, in_flight,
    )))
}

#[test]
fn overlapping_requests_keep_input_order_and_independent_failures() {
    let server = Server::new();
    let spec = server.spec(quota(3), KeyUse::None);
    let fetch = Fetch::open(FetchConfig::default()).unwrap();
    let leg = fetch.leg(spec, fetch.deadline(Duration::from_secs(10)));
    thread::scope(|scope| {
        let worker = scope.spawn(|| leg.batch(&[0, 1, 2], |leg, chunk| {
            leg.json::<Answer>(&spec.get(&format!("/{chunk}")), &[])
        }));
        let requests: Vec<_> = (0..3).map(|_| server.next()).collect();
        for request in requests.into_iter().rev() {
            if request.text.contains("/1 ") {
                request.respond(400, "", "refused");
            } else {
                request.ok();
            }
        }
        let results = worker.join().unwrap();
        assert!(results[0].as_ref().unwrap().answer);
        assert!(matches!(results[1], Err(RequestError::Source(error)) if error.reason == biodata::Reason::Rejected));
        assert!(results[2].as_ref().unwrap().answer);
    });
}

#[test]
fn nested_callbacks_stay_on_their_worker_thread() {
    let server = Server::new();
    let spec = server.spec(quota(1), KeyUse::None);
    let fetch = Fetch::open(FetchConfig::default()).unwrap();
    let leg = fetch.leg(spec, fetch.deadline(Duration::from_secs(10)));
    let results = leg.batch(&[1, 2], |leg, value| {
        let worker = thread::current().id();
        let nested = leg.batch(&[3, 4], |_, inner| {
            assert_eq!(worker, thread::current().id());
            Ok(value + inner)
        });
        Ok(nested.into_iter().collect::<Result<Vec<_>, _>>()?)
    });
    assert_eq!(results, vec![Ok(vec![4, 5]), Ok(vec![5, 6])]);
}
