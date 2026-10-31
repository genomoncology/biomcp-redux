//! Public quota declaration boundaries and shared provider behavior.
#![expect(
    clippy::disallowed_methods,
    reason = "Scoped callers exercise shared public transport state against controlled local HTTP peers"
)]
use biomcp_fetch::{Quota, Rate, SourceKey};

#[test]
fn quota_declarations_reject_empty_invalid_and_zero_limits() {
    for key in ["", "Upper", "space here", "slash/name", "é"] {
        assert!(std::panic::catch_unwind(|| SourceKey::declare(key)).is_err());
    }
    for (rate, burst) in [(0, 1), (1, 0), (0, 0)] {
        assert!(std::panic::catch_unwind(|| Rate::new(rate, burst)).is_err());
    }
    assert!(std::panic::catch_unwind(|| Rate::every_seconds(0)).is_err());
    assert!(
        std::panic::catch_unwind(|| Quota::new(
            SourceKey::declare("valid-1"),
            Rate::per_second(1),
            None,
            0
        ))
        .is_err()
    );
    const VALID: Quota = Quota::new(
        SourceKey::declare("valid-1"),
        Rate::new(u16::MAX, u16::MAX),
        Some(Rate::per_second(1)),
        u8::MAX,
    );
    assert!(format!("{VALID:?}").contains("valid-1"));
}

mod support;
#[path = "support/timing.rs"]
mod timing;
use biomcp_fetch::{Credential, Fetch, FetchConfig, KeyPlace, KeyUse, Secret, SecretName};
use biomcp_types::Reason;
use std::{sync::mpsc, thread, time::Duration};
use support::Answer;
use timing::Server;

fn quota(key: &'static str, rate: Rate, keyed: Option<Rate>, in_flight: u8) -> &'static Quota {
    Box::leak(Box::new(Quota::new(
        SourceKey::declare(key),
        rate,
        keyed,
        in_flight,
    )))
}

#[test]
fn canceled_waiter_leaves_shared_in_flight_queue_and_releases_successor() {
    let server = Server::new();
    let shared = quota("shared", Rate::per_second(1000), None, 1);
    let first = server.spec(shared, KeyUse::None);
    let second = server.spec(shared, KeyUse::None);
    let independent = server.spec(
        quota("independent", Rate::per_second(1000), None, 1),
        KeyUse::None,
    );
    let fetch = Fetch::open(FetchConfig::default()).unwrap();
    let restricted = fetch.restrict_sources([biomcp_types::Source::Oncokb].into());
    assert!(!fetch.disabled(biomcp_types::Source::Oncokb));
    assert!(
        restricted
            .restrict_sources(Default::default())
            .disabled(biomcp_types::Source::Oncokb)
    );
    thread::scope(|scope| {
        let holder = scope.spawn(|| {
            fetch
                .leg(first, fetch.deadline(Duration::from_secs(5)))
                .json::<Answer>(&first.get("/holder"), &[])
        });
        let held = server.next();
        let canceled = scope.spawn(|| {
            restricted
                .leg(second, restricted.deadline(Duration::from_millis(100)))
                .json::<Answer>(&second.get("/cancel"), &[])
        });
        // Completion with the holder still blocked proves the waiter exhausted its queue budget.
        assert_eq!(
            source(canceled.join().unwrap().unwrap_err()).reason,
            Reason::DeadlineExceeded
        );
        assert!(server.requests.try_recv().is_err());
        let successor = scope.spawn(|| {
            restricted
                .leg(second, restricted.deadline(Duration::from_secs(5)))
                .json::<Answer>(&second.get("/successor"), &[])
        });
        let other = scope.spawn(|| {
            fetch
                .leg(independent, fetch.deadline(Duration::from_secs(5)))
                .json::<Answer>(&independent.get("/other"), &[])
        });
        let request = server.next();
        assert!(request.text.contains("/other"));
        request.ok();
        assert!(other.join().unwrap().unwrap().answer);
        held.ok();
        server.next().ok();
        assert!(holder.join().unwrap().unwrap().answer);
        assert!(successor.join().unwrap().unwrap().answer);
    });
}

#[test]
fn configured_optional_key_selects_keyed_rate_without_bypassing_shared_queue() {
    for configured in [false, true] {
        let server = Server::new();
        let shared = quota(
            "keyed",
            Rate::every_seconds(2),
            Some(Rate::per_second(1)),
            1,
        );
        let spec = server.spec(
            shared,
            KeyUse::Optional(Credential {
                tier: biomcp_types::KeyTier::Fallback,
                secret: SecretName::SemanticScholarApiKey,
                place: KeyPlace::Header("x-api-key"),
            }),
        );
        assert_eq!(
            shared.interval(configured),
            Duration::from_secs(if configured { 1 } else { 2 })
        );
        let mut config = FetchConfig::default();
        if configured {
            config.secrets.insert(
                SecretName::SemanticScholarApiKey,
                Secret::new("placeholder"),
            );
        }
        let fetch = Fetch::open(config).unwrap();
        let (sent, receive) = mpsc::channel();
        thread::scope(|scope| {
            scope.spawn(|| {
                for _ in 0..2 {
                    let result = fetch
                        .leg(spec, fetch.deadline(Duration::from_secs(5)))
                        .json::<Answer>(&spec.get("/answer"), &[]);
                    sent.send(result).unwrap();
                }
            });
            let first = server.next();
            let started = first.started;
            assert_eq!(
                first
                    .text
                    .to_ascii_lowercase()
                    .contains("x-api-key: placeholder"),
                configured
            );
            first.ok();
            assert!(receive.recv().unwrap().unwrap().answer);
            let second = server.next();
            let floor = if configured { 995 } else { 1995 };
            assert!(second.started.duration_since(started) >= Duration::from_millis(floor));
            second.ok();
            assert!(receive.recv().unwrap().unwrap().answer);
        });
        assert!(server.requests.try_recv().is_err());
    }
}

#[test]
fn declared_burst_admits_only_its_capacity_and_expired_rate_waiters_leave() {
    let server = Server::new();
    let spec = server.spec(quota("burst", Rate::new(1, 2), None, 3), KeyUse::None);
    let fetch = Fetch::open(FetchConfig::default()).unwrap();
    thread::scope(|scope| {
        let first = scope.spawn(|| {
            fetch
                .leg(spec, fetch.deadline(Duration::from_secs(5)))
                .json::<Answer>(&spec.get("/first"), &[])
        });
        let held = server.next();
        let second = scope.spawn(|| {
            fetch
                .leg(spec, fetch.deadline(Duration::from_secs(5)))
                .json::<Answer>(&spec.get("/second"), &[])
        });
        let also_held = server.next();
        let canceled = scope.spawn(|| {
            fetch
                .leg(spec, fetch.deadline(Duration::from_millis(100)))
                .json::<Answer>(&spec.get("/expired"), &[])
        });
        assert_eq!(
            source(canceled.join().unwrap().unwrap_err()).reason,
            Reason::DeadlineExceeded
        );
        assert!(server.requests.try_recv().is_err());
        held.ok();
        also_held.ok();
        assert!(first.join().unwrap().unwrap().answer);
        assert!(second.join().unwrap().unwrap().answer);
        let successor = scope.spawn(|| {
            fetch
                .leg(spec, fetch.deadline(Duration::from_secs(5)))
                .json::<Answer>(&spec.get("/successor"), &[])
        });
        server.next().ok();
        assert!(successor.join().unwrap().unwrap().answer);
    });
}

#[test]
fn shared_queue_applies_each_arriving_sources_credential_rate() {
    let server = Server::new();
    let shared = quota("mixed", Rate::per_second(5), Some(Rate::per_second(100)), 1);
    let keyed = server.spec(
        shared,
        KeyUse::Optional(Credential {
            tier: biomcp_types::KeyTier::Fallback,
            secret: SecretName::NcbiApiKey,
            place: KeyPlace::Header("x-api-key"),
        }),
    );
    let unkeyed = server.spec(shared, KeyUse::None);
    let mut config = FetchConfig::default();
    config
        .secrets
        .insert(SecretName::NcbiApiKey, Secret::new("placeholder"));
    let fetch = Fetch::open(config).unwrap();
    thread::scope(|scope| {
        let first = scope.spawn(|| {
            fetch
                .leg(keyed, fetch.deadline(Duration::from_secs(5)))
                .json::<Answer>(&keyed.get("/keyed"), &[])
        });
        let request = server.next();
        let started = request.started;
        request.ok();
        assert!(first.join().unwrap().unwrap().answer);
        let second = scope.spawn(|| {
            fetch
                .leg(unkeyed, fetch.deadline(Duration::from_secs(5)))
                .json::<Answer>(&unkeyed.get("/unkeyed"), &[])
        });
        let request = server.next();
        let spacing = request.started.duration_since(started);
        request.ok();
        assert!(second.join().unwrap().unwrap().answer);
        assert!(spacing >= Duration::from_millis(195), "{spacing:?}");
    });
}

#[expect(
    clippy::panic,
    reason = "A cache error must never satisfy a provider-failure assertion"
)]
fn source(error: biomcp_fetch::RequestError) -> biomcp_fetch::SourceError {
    let biomcp_fetch::RequestError::Source(error) = error else {
        panic!("expected provider failure: {error}");
    };
    error
}
