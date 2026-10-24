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
