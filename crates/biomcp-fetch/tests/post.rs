//! Declared POST bodies share the checked transport and modes.
#![expect(clippy::unwrap_used, clippy::disallowed_methods, reason = "Local HTTP fixtures and scoped public API callers")]
mod support;
use biomcp_fetch::{Arg, Fetch, FetchConfig, KeyUse, Quota, Rate, SourceKey};
use std::{thread, time::Duration};
use support::Answer;
type Server = support::Server<()>;
fn spec(server: &Server) -> &'static biomcp_fetch::SourceSpec {
    server.spec(Box::leak(Box::new(Quota::new(SourceKey::declare("post"), Rate::per_second(1000), None, 4))), KeyUse::None)
}
#[test]
fn declared_form_preserves_order_duplicates_and_encodes_sorted_ids() {
    let server = Server::new();
    let spec = spec(&server);
    let fetch = Fetch::open(FetchConfig::default()).unwrap();
    let ids = ["Z".into(), "A".into()];
    thread::scope(|scope| {
        let worker = scope.spawn(|| fetch.leg(spec, fetch.deadline(Duration::from_secs(10))).json::<Answer>(&spec.post_form("/answer"), &[("q", Arg::Text("a +é")), ("q", Arg::Num(2)), ("ids", Arg::Ids(&ids))]));
        let request = server.next();
        assert!(request.text.starts_with("POST /answer HTTP/1.1\r\n"));
        assert!(request.text.to_ascii_lowercase().contains("content-type: application/x-www-form-urlencoded\r\n"));
        assert_eq!(request.body, b"q=a+%2B%C3%A9&q=2&ids=A%2CZ");
        request.ok();
        assert!(worker.join().unwrap().unwrap().answer);
    });
}
