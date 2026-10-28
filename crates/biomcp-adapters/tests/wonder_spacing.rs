//! Feature vaers-summary: CDC spacing refuses queued work without another source call.
#![expect(clippy::disallowed_methods, reason = "Outside transport test owns scoped local callers")]
#[path = "../../biomcp-fetch/tests/support/mod.rs"]
mod support;
use biomcp_fetch::{Fetch, FetchConfig, KeyUse, RequestError};
use biomcp_types::Reason;
use std::{thread, time::Duration};
#[test]
fn wonder_refuses_shared_spacing_with_retry_delay_before_network() {
    let quota = biomcp_adapters::vaers::SPEC.quota;
    let server = support::Server::<()>::new();
    let first = server.spec(quota, KeyUse::None);
    let second = server.spec(quota, KeyUse::None);
    let fetch = Fetch::open(FetchConfig::default()).unwrap();
    thread::scope(|scope| {
        let call = scope.spawn(|| fetch.leg(first, fetch.deadline(Duration::from_secs(5)))
            .json::<support::Answer>(&first.get("/first"), &[]));
        server.next().ok();
        assert!(call.join().unwrap().unwrap().answer);
    });
    let RequestError::Source(error) = fetch.leg(second, fetch.deadline(Duration::from_millis(500)))
        .json::<support::Answer>(&second.get("/second"), &[]).unwrap_err() else {panic!("source failure")};
    // On the old behavior, this reports deadline_exceeded after waiting for admission.
    assert_eq!(error.reason.to_string(), "local_quota");
    assert_ne!(error.reason, Reason::DeadlineExceeded);
    assert_eq!(error.status, None);
    let retry = error.retry_after.unwrap();
    assert!(retry > Duration::from_secs(110) && retry <= Duration::from_secs(120));
    assert!(server.requests.try_recv().is_err());
    assert_eq!(quota.interval(false), Duration::from_secs(120));
}
