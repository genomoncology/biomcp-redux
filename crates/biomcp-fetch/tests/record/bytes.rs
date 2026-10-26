use super::*;
fn bytes(fetch: &Fetch, spec: &'static SourceSpec) -> Result<Vec<u8>, RequestError> {
    fetch
        .leg(spec, fetch.deadline(Duration::from_secs(10)))
        .bytes(&spec.get("/bytes").bytes(20), &[])
}
fn json_file(dir: &Path) -> std::path::PathBuf {
    std::fs::read_dir(dir.join("pubmed"))
        .unwrap()
        .map(|v| v.unwrap().path())
        .find(|v| v.extension().is_some_and(|v| v == "json"))
        .unwrap()
}
pub(super) fn ordered_binary_sidecars_round_trip_and_refuse_hostile_paths() {
    let dir = tempfile::tempdir().unwrap();
    let server = Server::new();
    let spec = spec(&server);
    let capture = Fetch::open(FetchConfig {
        mode: Mode::Record {
            dir: dir.path().to_owned(),
        },
        max_body: 2,
        ..FetchConfig::default()
    })
    .unwrap();
    thread::scope(|scope| {
        let worker = scope.spawn(|| [bytes(&capture, spec), bytes(&capture, spec)]);
        for payload in [b"first\x00\xff".as_slice(), b"second\x00\xfe"] {
            let mut response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\nConnection: \
                close\r\n\r\n",
                payload.len()
            )
            .into_bytes();
            response.extend_from_slice(payload);
            server.next().bytes(response);
        }
        assert_eq!(
            worker.join().unwrap().map(Result::unwrap),
            [b"first\x00\xff".to_vec(), b"second\x00\xfe".to_vec()]
        );
    });
    drop(capture);
    let replay = fetch(Mode::Replay {
        dir: dir.path().to_owned(),
    });
    assert_eq!(bytes(&replay, spec).unwrap(), b"first\x00\xff");
    assert_eq!(bytes(&replay, spec).unwrap(), b"second\x00\xfe");
    let path = json_file(dir.path());
    let original = std::fs::read(&path).unwrap();
    let original_value: serde_json::Value = serde_json::from_slice(&original).unwrap();
    let name = original_value
        .pointer("/exchanges/0/body/value")
        .unwrap()
        .as_str()
        .unwrap();
    for hostile in [
        "../outside.bin",
        "/absolute.bin",
        "nested/file.bin",
        "missing.bin",
    ] {
        let mut value = original_value.clone();
        *value.pointer_mut("/exchanges/0/body/value").unwrap() = hostile.into();
        std::fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
        let replay = fetch(Mode::Replay {
            dir: dir.path().to_owned(),
        });
        assert!(matches!(
            bytes(&replay, spec),
            Err(RequestError::Recording(_))
        ));
    }
    // Disabled policy wins even when replay would open a hostile sidecar.
    let disabled = replay.restrict_sources([biomcp_types::Source::PubMed].into());
    assert!(
        matches!(bytes(&disabled, spec), Err(RequestError::Source(e)) if e.reason == biomcp_types::Reason::Disabled)
    );
    assert!(matches!(disabled.leg(spec,
 disabled.deadline(Duration::from_secs(10))).bytes_checked(&spec.get("/bytes").bytes(20),
 &[],
 |_| Err(biomcp_types::Reason::BadReply)),
 Err(RequestError::Source(e)) if e.reason == biomcp_types::Reason::Disabled));
    std::fs::write(&path, &original).unwrap();
    let sidecar = path.parent().unwrap().join(name);
    std::fs::remove_file(&sidecar).unwrap();
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&path, &sidecar).unwrap();
        let replay = fetch(Mode::Replay {
            dir: dir.path().to_owned(),
        });
        assert!(matches!(
            bytes(&replay, spec),
            Err(RequestError::Recording(RecordError::Malformed))
        ));
    }
    assert!(server.requests.try_recv().is_err());
}

pub(super) fn cut_binary_body_is_refused_before_returning_bytes() {
    let server = Server::new();
    let spec = spec(&server);
    let fetch = fetch(Mode::default());
    thread::scope(|scope| {
        let worker = scope.spawn(|| bytes(&fetch, spec));
        server
            .next()
            .raw("HTTP/1.1 200 OK\r\nContent-Type: application/pdf\r\nContent-Length: 19\r\nConnection: close\r\n\r\n%PDF-");
        assert!(
            matches!(worker.join().unwrap(), Err(RequestError::Source(e)) if e.reason == biomcp_types::Reason::BadReply)
        );
    });
    assert!(server.requests.try_recv().is_err());
}
