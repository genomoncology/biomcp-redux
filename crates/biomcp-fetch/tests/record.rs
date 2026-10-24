//! Recordings preserve the public checked transport contract without live providers.
#![expect(
    clippy::unwrap_used,
    clippy::panic,
    clippy::disallowed_methods,
    reason = "Local HTTP fixtures and scoped callers exercise public transport modes"
)]
mod support;
use biomcp_fetch::{
    Arg, Fetch, FetchConfig, KeyUse, Mode, Quota, Rate, RecordError, RequestError, SourceKey,
    SourceSpec,
};
use std::{path::Path, thread, time::Duration};
use support::Answer;
type Server = support::Server<()>;
fn spec(server: &Server) -> &'static SourceSpec {
    server.spec(
        Box::leak(Box::new(Quota::new(
            SourceKey::declare("record"),
            Rate::per_second(1000),
            None,
            4,
        ))),
        KeyUse::None,
    )
}
fn fetch(mode: Mode) -> Fetch {
    Fetch::open(FetchConfig {
        mode,
        ..FetchConfig::default()
    })
    .unwrap()
}
fn get(fetch: &Fetch, spec: &'static SourceSpec) -> Result<Answer, RequestError> {
    fetch
        .leg(spec, fetch.deadline(Duration::from_secs(10)))
        .json(&spec.get("/answer"), &[])
}
fn file(dir: &Path) -> std::path::PathBuf {
    std::fs::read_dir(dir.join("pubmed"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path()
}
#[test]
fn repeated_key_consumes_ordered_replies_and_never_opens_replay_http() {
    let dir = tempfile::tempdir().unwrap();
    let server = Server::new();
    let spec = spec(&server);
    let capture = fetch(Mode::Record {
        dir: dir.path().to_owned(),
    });
    thread::scope(|scope| {
        let worker = scope.spawn(|| [get(&capture, spec), get(&capture, spec)]);
        server.next().ok();
        server.next().respond(200, "", "{broken");
        let replies = worker.join().unwrap();
        assert!(replies[0].as_ref().unwrap().answer);
        assert!(replies[1].is_err());
    });
    drop(capture);
    let replay = fetch(Mode::Replay {
        dir: dir.path().to_owned(),
    });
    assert!(get(&replay, spec).unwrap().answer);
    assert!(
        matches!(get(&replay, spec), Err(RequestError::Source(e)) if e.reason == biodata::Reason::BadReply)
    );
    assert_eq!(
        get(&replay, spec).unwrap_err(),
        RequestError::Recording(RecordError::Exhausted)
    );
    assert!(server.requests.try_recv().is_err());
    let error = replay
        .leg(spec, replay.deadline(Duration::from_secs(10)))
        .json::<Answer>(&spec.get("/answer"), &[("other", Arg::Num(1))])
        .unwrap_err();
    assert_eq!(error, RequestError::Recording(RecordError::Missing));
}
#[test]
fn capture_directory_has_one_owner_and_refuses_existing_data() {
    let dir = tempfile::tempdir().unwrap();
    let first = fetch(Mode::Record {
        dir: dir.path().to_owned(),
    });
    assert!(matches!(
        Fetch::open(FetchConfig {
            mode: Mode::Record {
                dir: dir.path().to_owned()
            },
            ..FetchConfig::default()
        }),
        Err(biomcp_fetch::FetchError::Recording(RecordError::Conflict))
    ));
    drop(first);
    std::fs::write(dir.path().join("keep"), "keep").unwrap();
    assert!(matches!(
        Fetch::open(FetchConfig {
            mode: Mode::Record {
                dir: dir.path().to_owned()
            },
            ..FetchConfig::default()
        }),
        Err(biomcp_fetch::FetchError::Recording(RecordError::Conflict))
    ));
    assert_eq!(
        std::fs::read_to_string(dir.path().join("keep")).unwrap(),
        "keep"
    );
}

fn capture(server: &Server, spec: &'static SourceSpec, dir: &Path, replies: &[(u16, &str, &str)]) {
    let recording = fetch(Mode::Record {
        dir: dir.to_owned(),
    });
    thread::scope(|scope| {
        let worker = scope.spawn(|| {
            for _ in replies {
                let _ = get(&recording, spec);
            }
        });
        for (status, headers, body) in replies {
            server.next().respond(*status, headers, body);
        }
        worker.join().unwrap();
    });
}
fn change(dir: &Path, update: impl FnOnce(&mut serde_json::Value)) {
    let path = file(dir);
    let mut value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    update(&mut value);
    std::fs::write(path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
}
#[test]
fn replay_advances_long_response_and_retry_delays_and_enforces_each_leg_budget() {
    let dir = tempfile::tempdir().unwrap();
    let server = Server::new();
    let original = spec(&server);
    capture(
        &server,
        original,
        dir.path(),
        &[(200, "", r#"{"answer":true}"#)],
    );
    change(dir.path(), |v| {
        let mut success = v["exchanges"][0].clone();
        success["delay_ms"] = 0.into();
        v["exchanges"] = serde_json::json!([
            {"status":503,"content_type":"application/json","delay_ms":120000,"retry_after_ms":240000,"body":{"kind":"text","value":"unavailable"}}, success]);
    });
    let replay = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: dir.path().to_owned(),
        },
        source_deadlines: [(biodata::Source::PubMed, Duration::from_secs(1000))].into(),
        ..FetchConfig::default()
    })
    .unwrap();
    let clock = replay.deadline(Duration::from_secs(1000));
    assert!(
        replay
            .leg(original, clock)
            .json::<Answer>(&original.get("/answer"), &[])
            .unwrap()
            .answer
    );
    assert_eq!(clock.remaining(), Duration::from_secs(640));
    assert!(server.requests.try_recv().is_err());
    let replay = fetch(Mode::Replay {
        dir: dir.path().to_owned(),
    });
    let clock = replay.deadline(Duration::from_secs(1000));
    assert!(
        matches!(replay.leg(original, clock).json::<Answer>(&original.get("/answer"), &[]), Err(RequestError::Source(e)) if e.reason == biodata::Reason::DeadlineExceeded)
    );
    assert_eq!(clock.remaining(), Duration::from_secs(990));
}
#[test]
fn concurrent_same_key_reserves_each_exchange_once_and_shares_virtual_quota() {
    #[derive(Debug, serde::Deserialize)]
    struct Number {
        value: u32,
    }
    impl biomcp_fetch::Reply for Number {
        fn check(&self) -> Result<(), biomcp_fetch::BadReply> {
            Ok(())
        }
    }
    let dir = tempfile::tempdir().unwrap();
    let server = Server::new();
    let original = spec(&server);
    capture(
        &server,
        original,
        dir.path(),
        &[(200, "", r#"{"answer":true}"#)],
    );
    change(dir.path(), |v| {
        let first = v["exchanges"][0].clone();
        v["exchanges"] = (0..8)
            .map(|i| {
                let mut entry = first.clone();
                entry["delay_ms"] = 0.into();
                entry["body"] = serde_json::json!({"kind":"json","value":{"value":i}});
                entry
            })
            .collect();
    });
    let quota = Box::leak(Box::new(Quota::new(
        SourceKey::declare("virtual-shared"),
        Rate::per_second(1),
        None,
        2,
    )));
    let slow = server.spec(quota, KeyUse::None);
    let replay = fetch(Mode::Replay {
        dir: dir.path().to_owned(),
    });
    let clock = replay.deadline(Duration::from_secs(20));
    let mut numbers = thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    replay
                        .leg(slow, clock)
                        .json::<Number>(&slow.get("/answer"), &[])
                        .unwrap()
                        .value
                })
            })
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect::<Vec<_>>()
    });
    numbers.sort();
    assert_eq!(numbers, (0..8).collect::<Vec<_>>());
    assert_eq!(clock.remaining(), Duration::from_secs(13));
    assert_eq!(
        get(&replay, slow).unwrap_err(),
        RequestError::Recording(RecordError::Exhausted)
    );
    assert!(server.requests.try_recv().is_err());
}
#[test]
fn invalid_recordings_are_local_bounded_errors_with_no_fallback() {
    let dir = tempfile::tempdir().unwrap();
    let server = Server::new();
    let spec = spec(&server);
    capture(
        &server,
        spec,
        dir.path(),
        &[(200, "", r#"{"answer":true}"#)],
    );
    let original = std::fs::read(file(dir.path())).unwrap();
    for (field, value) in [
        ("status", serde_json::json!(999)),
        ("delay_ms", serde_json::json!(-1)),
        ("retry_after_ms", serde_json::json!(18446744073709551616.0)),
    ] {
        std::fs::write(file(dir.path()), &original).unwrap();
        change(dir.path(), |v| v["exchanges"][0][field] = value);
        let replay = fetch(Mode::Replay {
            dir: dir.path().to_owned(),
        });
        assert_eq!(
            get(&replay, spec).unwrap_err(),
            RequestError::Recording(RecordError::Malformed)
        );
    }
    std::fs::write(file(dir.path()), &original).unwrap();
    change(dir.path(), |v| v["url"] = "wrong".into());
    assert_eq!(
        get(
            &fetch(Mode::Replay {
                dir: dir.path().to_owned()
            }),
            spec
        )
        .unwrap_err(),
        RequestError::Recording(RecordError::Malformed)
    );
    std::fs::write(file(dir.path()), vec![b' '; 16 * 1024 * 1024 + 1]).unwrap();
    assert_eq!(
        get(
            &fetch(Mode::Replay {
                dir: dir.path().to_owned()
            }),
            spec
        )
        .unwrap_err(),
        RequestError::Recording(RecordError::Limit)
    );
    assert!(server.requests.try_recv().is_err());
}
#[test]
fn capture_keeps_retry_status_headers_and_rejected_json_without_credentials() {
    let dir = tempfile::tempdir().unwrap();
    let server = Server::new();
    let credential = biomcp_fetch::Credential {
        secret: biomcp_fetch::SecretName::NcbiApiKey,
        place: biomcp_fetch::KeyPlace::Query("api_key"),
    };
    let spec = server.spec(
        Box::leak(Box::new(Quota::new(
            SourceKey::declare("capture-key"),
            Rate::per_second(1000),
            None,
            1,
        ))),
        KeyUse::Required(credential),
    );
    let mut config = FetchConfig {
        mode: Mode::Record {
            dir: dir.path().to_owned(),
        },
        ..FetchConfig::default()
    };
    config
        .secrets
        .insert(credential.secret, biomcp_fetch::Secret::new("private-key"));
    let recording = Fetch::open(config).unwrap();
    thread::scope(|scope| {
        let worker = scope.spawn(|| get(&recording, spec));
        server.next().respond(429, "Retry-After: 0\r\n", "limited");
        server.next().respond(200, "", r#"{"answer":false}"#);
        assert!(
            matches!(worker.join().unwrap(), Err(RequestError::Source(e)) if e.reason == biodata::Reason::BadReply)
        );
    });
    drop(recording);
    let text = std::fs::read_to_string(file(dir.path())).unwrap();
    assert!(!text.contains("private-key") && !text.contains("api_key"));
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(value["exchanges"][0]["retry_after_ms"], 0);
    let replay = fetch(Mode::Replay {
        dir: dir.path().to_owned(),
    });
    assert!(
        matches!(get(&replay, spec), Err(RequestError::Source(e)) if e.reason == biodata::Reason::NoKey)
    );
    // Missing credentials win even over a now malformed recording.
    std::fs::write(file(dir.path()), "bad").unwrap();
    assert!(
        matches!(get(&replay, spec), Err(RequestError::Source(e)) if e.reason == biodata::Reason::NoKey)
    );
}

#[test]
fn non_utf8_capture_is_a_typed_refusal_and_never_publishes_a_partial_file() {
    let dir = tempfile::tempdir().unwrap();
    let server = Server::new();
    let spec = spec(&server);
    let recording = fetch(Mode::Record {
        dir: dir.path().to_owned(),
    });
    thread::scope(|scope| {
        let worker = scope.spawn(|| get(&recording, spec));
        let mut response = b"HTTP/1.1 200 Stub\r\nContent-Type: application/json\r\nContent-Length: 1\r\nConnection: close\r\n\r\n".to_vec();
        response.push(255);
        server.next().bytes(response);
        assert_eq!(
            worker.join().unwrap().unwrap_err(),
            RequestError::Recording(RecordError::NonUtf8)
        );
    });
    assert!(!dir.path().join("pubmed").exists());
}
