//! Checked reply cache through the public transport API and documented SQLite schema.
#![expect(
    clippy::unwrap_used,
    clippy::panic,
    reason = "Fixture setup, child processes and failed source assertions must abort the contract test"
)]
#![expect(
    clippy::disallowed_methods,
    reason = "Scoped callers exercise public cache behavior against controlled local peers"
)]
#[path = "support/cache_process.rs"]
mod cache_process;
mod support;
use biodata::Reason;
use biomcp_fetch::{
    Arg, BadReply, CacheError, Credential, Endpoint, Fetch, FetchConfig, FetchError, Freshness,
    KeyPlace, KeyUse, Mode, Quota, Rate, Reply, RequestError, Secret, SecretName, SourceKey,
    SourceSpec,
};
use rusqlite::Connection;
use std::{path::Path, thread, time::Duration};
use support::Answer;
type Server = support::Server<()>;

fn spec(server: &Server, key: KeyUse) -> &'static SourceSpec {
    server.spec(
        Box::leak(Box::new(Quota::new(
            SourceKey::declare("cache"),
            Rate::per_second(1000),
            None,
            8,
        ))),
        key,
    )
}
fn config(path: &Path) -> FetchConfig {
    FetchConfig {
        cache: Some(path.to_owned()),
        ..FetchConfig::default()
    }
}
fn get(
    fetch: &Fetch,
    spec: &'static SourceSpec,
    endpoint: &Endpoint,
    params: &[(&str, Arg<'_>)],
) -> Result<Answer, RequestError> {
    fetch
        .leg(spec, fetch.deadline(Duration::from_secs(10)))
        .json(endpoint, params)
}
fn exchange<T: Send>(
    server: &Server,
    status: u16,
    body: &str,
    call: impl FnOnce() -> T + Send,
) -> T {
    thread::scope(|scope| {
        let worker = scope.spawn(call);
        let request = server.next();
        assert!(request.text.starts_with("GET /answer"));
        request.respond(status, "", body);
        worker.join().unwrap()
    })
}
fn expire(path: &Path) {
    Connection::open(path)
        .unwrap()
        .execute("UPDATE reply SET fresh_until=0", [])
        .unwrap();
}
fn source(error: RequestError) -> biomcp_fetch::SourceError {
    let RequestError::Source(error) = error else {
        panic!("expected provider failure: {error}");
    };
    error
}

#[test]
fn fresh_hit_and_force_refresh_and_never_and_disabled_storage() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let spec = spec(&server, KeyUse::None);
    let fetch = Fetch::open(config(&path)).unwrap();
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    assert!(
        exchange(&server, 200, r#"{"answer":true}"#, || get(
            &fetch,
            spec,
            &endpoint,
            &[]
        ))
        .unwrap()
        .answer
    );
    assert!(get(&fetch, spec, &endpoint, &[]).unwrap().answer);
    assert!(server.requests.try_recv().is_err());
    let forced = Fetch::open(FetchConfig {
        mode: Mode::Live { fresh: true },
        ..config(&path)
    })
    .unwrap();
    exchange(&server, 200, r#"{"answer":true,"refreshed":true}"#, || {
        get(&forced, spec, &endpoint, &[])
    })
    .unwrap();
    let never = spec.get("/answer");
    exchange(&server, 200, r#"{"answer":true,"never":true}"#, || {
        get(&fetch, spec, &never, &[])
    })
    .unwrap();
    let disabled = Fetch::open(FetchConfig::default()).unwrap();
    for _ in 0..2 {
        exchange(&server, 200, r#"{"answer":true}"#, || {
            get(&disabled, spec, &endpoint, &[])
        })
        .unwrap();
    }
    let rows: u32 = Connection::open(&path)
        .unwrap()
        .query_row("SELECT count(*) FROM reply", [], |r| r.get(0))
        .unwrap();
    assert_eq!(rows, 1);
    let stored: String = Connection::open(&path)
        .unwrap()
        .query_row("SELECT CAST(body AS TEXT) FROM reply", [], |r| r.get(0))
        .unwrap();
    assert_eq!(stored, r#"{"answer":true,"refreshed":true}"#);
}

#[test]
fn rejected_bodies_are_not_stored_and_negative_expiry_never_falls_back() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let spec = spec(&server, KeyUse::None);
    let fetch = Fetch::open(config(&path)).unwrap();
    let endpoint = spec.get("/answer").fresh(Freshness::Minutes(30));
    for body in ["{broken", r#"{"answer":false}"#] {
        let error = exchange(&server, 200, body, || get(&fetch, spec, &endpoint, &[])).unwrap_err();
        assert_eq!(source(error).reason, Reason::BadReply);
        let rows: u32 = Connection::open(&path)
            .unwrap()
            .query_row("SELECT count(*) FROM reply", [], |r| r.get(0))
            .unwrap();
        assert_eq!(rows, 0);
    }
    exchange(&server, 200, r#"{"answer":true}"#, || {
        get(&fetch, spec, &endpoint, &[])
    })
    .unwrap();
    expire(&path);
    assert!(
        exchange(&server, 404, "unchecked-private-error-text", || fetch
            .leg(spec, fetch.deadline(Duration::from_secs(10)))
            .json_opt::<Answer>(&endpoint, &[]))
        .unwrap()
        .is_none()
    );
    // A negative hit returns absence without decoding the empty stored body.
    assert!(
        fetch
            .leg(spec, fetch.deadline(Duration::from_secs(10)))
            .json_opt::<Answer>(&endpoint, &[])
            .unwrap()
            .is_none()
    );
    let connection = Connection::open(&path).unwrap();
    let (status, body, ttl): (u16, Vec<u8>, i64) = connection
        .query_row(
            "SELECT status,body,fresh_until-fetched_at FROM reply",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .unwrap();
    assert_eq!((status, body.len(), ttl), (404, 0, 600));
    expire(&path);
    let error = exchange(&server, 400, "bad", || get(&fetch, spec, &endpoint, &[])).unwrap_err();
    assert_eq!(source(error).reason, Reason::Rejected);
}

#[test]
fn stale_fallback_retains_original_failure_across_later_success() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let spec = spec(&server, KeyUse::None);
    let fetch = Fetch::open(config(&path)).unwrap();
    let endpoint = spec.get("/answer").fresh(Freshness::Days(1));
    exchange(&server, 200, r#"{"answer":true}"#, || {
        get(&fetch, spec, &endpoint, &[])
    })
    .unwrap();
    expire(&path);
    let report = thread::scope(|scope| {
        let worker = scope.spawn(|| {
            let leg = fetch.leg(spec, fetch.deadline(Duration::from_secs(10)));
            for _ in 0..3 {
                assert!(leg.json::<Answer>(&endpoint, &[]).unwrap().answer);
            }
            assert!(leg.json::<Answer>(&endpoint, &[]).unwrap().answer);
            leg.finish()
        });
        for (status, body) in [
            (200, "{broken"),
            (200, r#"{"answer":false}"#),
            (400, "error"),
        ] {
            server.next().respond(status, "", body);
        }
        server.next().ok();
        worker.join().unwrap()
    });
    assert!(report.stale);
    assert_eq!(
        report
            .failures
            .iter()
            .map(|error| (error.reason, error.status))
            .collect::<Vec<_>>(),
        [
            (Reason::BadReply, Some(200)),
            (Reason::BadReply, Some(200)),
            (Reason::Rejected, Some(400))
        ]
    );
    expire(&path);
    let forced = Fetch::open(FetchConfig {
        mode: Mode::Live { fresh: true },
        ..config(&path)
    })
    .unwrap();
    assert_eq!(
        source(
            exchange(&server, 200, "{broken", || get(
                &forced,
                spec,
                &endpoint,
                &[]
            ))
            .unwrap_err()
        )
        .reason,
        Reason::BadReply
    );
}

#[test]
fn every_cached_read_obeys_current_reply_type_body_limit_and_sql_contract() {
    #[derive(Debug, serde::Deserialize)]
    struct Strict {
        answer: bool,
    }
    impl Reply for Strict {
        fn check(&self) -> Result<(), BadReply> {
            if self.answer { Err(BadReply) } else { Ok(()) }
        }
    }
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let spec = spec(&server, KeyUse::None);
    let fetch = Fetch::open(config(&path)).unwrap();
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    exchange(&server, 200, r#"{"answer":true}"#, || {
        get(&fetch, spec, &endpoint, &[])
    })
    .unwrap();
    // A stricter public type invalidates and replaces the previous checked body.
    let strict = exchange(&server, 200, r#"{"answer":false}"#, || {
        fetch
            .leg(spec, fetch.deadline(Duration::from_secs(10)))
            .json::<Strict>(&endpoint, &[])
    })
    .unwrap();
    assert!(!strict.answer);
    let limited = Fetch::open(FetchConfig {
        max_body: 1,
        ..config(&path)
    })
    .unwrap();
    // The oversized old entry is evicted. Its stale body cannot hide this failure.
    let error = exchange(&server, 400, "rejected", || {
        limited
            .leg(spec, limited.deadline(Duration::from_secs(10)))
            .json::<Strict>(&endpoint, &[])
    })
    .unwrap_err();
    assert_eq!(source(error).reason, Reason::Rejected);
    let count: u32 = Connection::open(&path)
        .unwrap()
        .query_row("SELECT count(*) FROM reply", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 0);
    Connection::open(&path)
        .unwrap()
        .execute_batch("DROP TABLE reply")
        .unwrap();
    assert_eq!(
        get(&fetch, spec, &endpoint, &[]).unwrap_err(),
        RequestError::Cache(CacheError::Database)
    );
    assert!(server.requests.try_recv().is_err());
}

#[test]
fn query_identity_distinguishes_arguments_and_excludes_configured_credentials() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let credential = Credential {
        secret: SecretName::NcbiApiKey,
        place: KeyPlace::Query("api_key"),
    };
    let spec = spec(&server, KeyUse::Required(credential));
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    let mut first = config(&path);
    first
        .secrets
        .insert(SecretName::NcbiApiKey, Secret::new("first-credential"));
    let fetch = Fetch::open(first).unwrap();
    for arg in ["a b", "a+b"] {
        exchange(&server, 200, r#"{"answer":true}"#, || {
            get(
                &fetch,
                spec,
                &endpoint,
                &[
                    ("q", Arg::Text(arg)),
                    ("api_key", Arg::Text("discard-this")),
                ],
            )
        })
        .unwrap();
    }
    let mut second = config(&path);
    second
        .secrets
        .insert(SecretName::NcbiApiKey, Secret::new("second-credential"));
    let fetch = Fetch::open(second).unwrap();
    for arg in ["a b", "a+b"] {
        assert!(
            get(&fetch, spec, &endpoint, &[("q", Arg::Text(arg))])
                .unwrap()
                .answer
        );
    }
    let connection = Connection::open(&path).unwrap();
    let urls = connection
        .prepare("SELECT url FROM reply ORDER BY url")
        .unwrap()
        .query_map([], |row| row.get::<_, String>(0))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(urls.len(), 2);
    assert!(urls.iter().all(|url| !url.contains("credential")
        && !url.contains("api_key")
        && !url.contains("discard")));
    let missing = Fetch::open(config(&path)).unwrap();
    assert_eq!(
        source(get(&missing, spec, &endpoint, &[("q", Arg::Text("a b"))]).unwrap_err()).reason,
        Reason::NoKey
    );
    let foreign = spec.get("/answer").fresh(Freshness::Hours(1));
    let other = server.spec(spec.quota, KeyUse::None);
    assert_eq!(
        source(
            fetch
                .leg(other, fetch.deadline(Duration::from_secs(10)))
                .json::<Answer>(&foreign, &[])
                .unwrap_err()
        )
        .reason,
        Reason::Rejected
    );
    assert!(server.requests.try_recv().is_err());
}

#[test]
fn ownership_is_checked_before_any_schema_or_journal_change() {
    let directory = tempfile::tempdir().unwrap();
    for (name, sql) in [
        (
            "version",
            "CREATE TABLE meta(owner TEXT PRIMARY KEY,version INTEGER NOT NULL); INSERT INTO meta VALUES('biomcp-fetch',999);",
        ),
        (
            "orphan",
            "CREATE TABLE reply(untouched TEXT); INSERT INTO reply VALUES('keep');",
        ),
        (
            "foreign",
            "CREATE TABLE meta(owner TEXT PRIMARY KEY,version INTEGER NOT NULL); INSERT INTO meta VALUES('other',7); CREATE TABLE reply(untouched TEXT);",
        ),
    ] {
        let path = directory.path().join(name);
        let connection = Connection::open(&path).unwrap();
        connection.execute_batch(sql).unwrap();
        let before: Vec<String> = connection
            .prepare("SELECT sql FROM sqlite_master WHERE sql IS NOT NULL ORDER BY name")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        assert!(matches!(
            Fetch::open(config(&path)),
            Err(FetchError::Cache(CacheError::Version))
        ));
        let after: Vec<String> = connection
            .prepare("SELECT sql FROM sqlite_master WHERE sql IS NOT NULL ORDER BY name")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(before, after);
        assert_eq!(
            connection
                .query_row("PRAGMA journal_mode", [], |r| r.get::<_, String>(0))
                .unwrap(),
            "delete"
        );
    }
    let path = directory.path().join("shared");
    let connection = Connection::open(&path).unwrap();
    connection.execute_batch("CREATE TABLE meta(owner TEXT PRIMARY KEY,version INTEGER NOT NULL); INSERT INTO meta VALUES('other',7); CREATE TABLE other_data(value TEXT); INSERT INTO other_data VALUES('keep');").unwrap();
    Fetch::open(config(&path)).unwrap();
    assert_eq!(
        connection
            .query_row("SELECT version FROM meta WHERE owner='other'", [], |r| r
                .get::<_, u32>(0))
            .unwrap(),
        7
    );
    assert_eq!(
        connection
            .query_row("SELECT value FROM other_data", [], |r| r
                .get::<_, String>(0))
            .unwrap(),
        "keep"
    );
}

#[test]
fn fresh_hits_skip_quota_and_stale_positives_survive_quota_deadline() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let quota = Box::leak(Box::new(Quota::new(
        SourceKey::declare("cache-blocked"),
        Rate::per_second(1000),
        None,
        1,
    )));
    let spec = server.spec(quota, KeyUse::None);
    let fetch = Fetch::open(config(&path)).unwrap();
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    exchange(&server, 200, r#"{"answer":true}"#, || {
        get(&fetch, spec, &endpoint, &[])
    })
    .unwrap();
    thread::scope(|scope| {
        let held =
            scope.spawn(|| get(&fetch, spec, &spec.get("/answer"), &[("hold", Arg::Num(1))]));
        let request = server.next();
        assert!(
            fetch
                .leg(spec, fetch.deadline(Duration::from_millis(100)))
                .json::<Answer>(&endpoint, &[])
                .unwrap()
                .answer
        );
        expire(&path);
        let leg = fetch.leg(spec, fetch.deadline(Duration::from_millis(100)));
        assert!(leg.json::<Answer>(&endpoint, &[]).unwrap().answer);
        let report = leg.finish();
        assert!(report.stale);
        assert_eq!(report.failures.len(), 1);
        assert_eq!(report.failures[0].reason, Reason::DeadlineExceeded);
        assert_eq!(report.failures[0].status, None);
        assert!(server.requests.try_recv().is_err());
        request.ok();
        held.join().unwrap().unwrap();
    });
}

#[test]
fn body_and_send_failures_can_use_only_a_checked_stale_positive() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let spec = spec(&server, KeyUse::None);
    let fetch = Fetch::open(config(&path)).unwrap();
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    exchange(&server, 200, r#"{"answer":true}"#, || {
        get(&fetch, spec, &endpoint, &[])
    })
    .unwrap();
    expire(&path);
    let report = thread::scope(|scope| {
        let worker = scope.spawn(|| {
            let leg = fetch.leg(spec, fetch.deadline(Duration::from_secs(10)));
            assert!(leg.json::<Answer>(&endpoint, &[]).unwrap().answer);
            leg.finish()
        });
        server.next().raw("HTTP/1.1 200 Stub\r\nContent-Type: application/json\r\nContent-Length: 100\r\nConnection: close\r\n\r\n{cut");
        worker.join().unwrap()
    });
    assert_eq!(report.failures[0].reason, Reason::BadReply);
    assert_eq!(report.failures[0].status, Some(200));
    drop(server);
    let leg = fetch.leg(spec, fetch.deadline(Duration::from_secs(10)));
    assert!(leg.json::<Answer>(&endpoint, &[]).unwrap().answer);
    assert_eq!(leg.finish().failures[0].reason, Reason::Unavailable);
}

#[test]
fn local_write_busy_respects_deadline_and_never_uses_stale_fallback() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let spec = spec(&server, KeyUse::None);
    let fetch = Fetch::open(config(&path)).unwrap();
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    exchange(&server, 200, r#"{"answer":true}"#, || {
        get(&fetch, spec, &endpoint, &[])
    })
    .unwrap();
    expire(&path);
    let connection = Connection::open(&path).unwrap();
    connection.execute_batch("BEGIN IMMEDIATE").unwrap();
    let deadline = fetch.deadline(Duration::from_millis(500));
    let (error, report) = exchange(&server, 200, r#"{"answer":true}"#, || {
        let leg = fetch.leg(spec, deadline);
        let error = leg.json::<Answer>(&endpoint, &[]).unwrap_err();
        (error, leg.finish())
    });
    assert_eq!(error, RequestError::Cache(CacheError::Busy));
    assert!(deadline.remaining() < Duration::from_millis(20));
    assert!(!report.stale);
    assert!(report.failures.is_empty());
    connection.execute_batch("ROLLBACK").unwrap();
}

#[test]
fn per_leg_freshness_reaches_batch_children_without_leaking_to_siblings() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let spec = spec(&server, KeyUse::None);
    let fetch = Fetch::open(config(&path)).unwrap();
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    exchange(&server, 200, r#"{"answer":true}"#, || {
        get(&fetch, spec, &endpoint, &[])
    })
    .unwrap();
    let fresh = fetch
        .leg(spec, fetch.deadline(Duration::from_secs(10)))
        .fresh(true);
    exchange(&server, 200, r#"{"answer":true,"fresh":true}"#, || {
        let results = fresh.batch(&[()], |leg, ()| leg.json::<Answer>(&endpoint, &[]));
        assert!(results.into_iter().all(|result| result.unwrap().answer));
    });
    assert!(get(&fetch, spec, &endpoint, &[]).unwrap().answer);
    assert!(server.requests.try_recv().is_err());
    expire(&path);
    let failure = exchange(&server, 400, "rejected", || {
        fresh.json::<Answer>(&endpoint, &[])
    })
    .unwrap_err();
    assert_eq!(source(failure).reason, Reason::Rejected);
    assert!(!fresh.finish().stale);
}

#[test]
fn concurrent_replay_legs_share_the_cache_without_spurious_local_busy_errors() {
    use sha2::{Digest, Sha256};
    let directory = tempfile::tempdir().unwrap();
    let records = directory.path().join("pubmed");
    std::fs::create_dir(&records).unwrap();
    let spec: &'static SourceSpec = Box::leak(Box::new(SourceSpec {
        source: biodata::Source::PubMed,
        base: "https://example.invalid",
        key: KeyUse::None,
        quota: Box::leak(Box::new(Quota::new(
            SourceKey::declare("parallel-cache"),
            Rate::new(1000, 1000),
            None,
            8,
        ))),
        deadline: Duration::from_secs(10),
    }));
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    for index in 0..8 {
        let url = format!("https://example.invalid/answer?index={index}");
        let key = Sha256::digest(format!("GET\0{url}").as_bytes())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        let recording = serde_json::json!({"method":"GET","source":"pubmed","url":url,"exchanges":[{"status":200,"content_type":"application/json","delay_ms":0,"retry_after_ms":null,"body":{"kind":"json","value":{"answer":true}}}]});
        std::fs::write(
            records.join(format!("{key}.json")),
            serde_json::to_vec(&recording).unwrap(),
        )
        .unwrap();
    }
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: directory.path().into(),
        },
        ..Default::default()
    })
    .unwrap();
    let start = std::sync::Barrier::new(8);
    thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|index| {
                let fetch = &fetch;
                let start = &start;
                let endpoint = &endpoint;
                scope.spawn(move || {
                    let leg = fetch.leg(spec, fetch.deadline(Duration::from_secs(10)));
                    start.wait();
                    leg.json::<Answer>(endpoint, &[("index", Arg::Num(index))])
                })
            })
            .collect();
        for worker in workers {
            assert!(worker.join().unwrap().unwrap().answer);
        }
    });
}

#[test]
fn keyed_cache_refuses_missing_credentials_and_never_masks_rejected_credentials() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let spec = spec(
        &server,
        KeyUse::Required(Credential {
            secret: SecretName::OncokbApiKey,
            place: KeyPlace::Bearer,
        }),
    );
    let endpoint = spec.get("/answer").fresh(Freshness::Days(1));
    let mut configured = config(&path);
    configured.secrets.insert(
        SecretName::OncokbApiKey,
        Secret::new("synthetic-placeholder"),
    );
    let fetch = Fetch::open(configured).unwrap();
    assert!(fetch.has_key(SecretName::OncokbApiKey));
    assert!(
        exchange(&server, 200, r#"{"answer":true}"#, || get(
            &fetch,
            spec,
            &endpoint,
            &[]
        ))
        .unwrap()
        .answer
    );
    for mode in [Mode::default(), Mode::Offline] {
        let refused = Fetch::open(FetchConfig {
            mode,
            ..config(&path)
        })
        .unwrap();
        assert!(!refused.has_key(SecretName::OncokbApiKey));
        assert_eq!(
            source(get(&refused, spec, &endpoint, &[]).unwrap_err()).reason,
            Reason::NoKey
        );
        assert!(server.requests.try_recv().is_err());
    }
    expire(&path);
    for status in [401, 403] {
        let failure =
            exchange(&server, status, "{}", || get(&fetch, spec, &endpoint, &[])).unwrap_err();
        assert_eq!(source(failure).status, Some(status));
    }
}
