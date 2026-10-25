use super::*;
#[derive(Debug, Deserialize)]
struct Medicine {
    id: String,
    name: String,
}
impl Row for Medicine {
    fn keys(&self) -> Keys {
        Keys {
            exact: vec![(KeyKind::Id, self.id.clone().into())],
            words: vec![self.name.clone().into()],
        }
    }
}
fn bundle(server: &support::Server) -> Bundle<Medicine> {
    Bundle {
        spec: server.spec(&QUOTA, KeyUse::None),
        name: "medicines",
        path: "/medicines",
        format: RowFormat::Json {
            rows: "data",
            metadata: "meta",
        },
        fresh: Freshness::Days(1),
        row: PhantomData,
    }
}
#[test]
fn json_bundle_reads_never_sync_and_failed_replacements_keep_old_rows() {
    let server = support::Server::new();
    let b = bundle(&server);
    let dir = tempfile::tempdir().unwrap();
    let f = cached(&dir.path().join("cache.sqlite"));
    let leg = f
        .leg(b.spec, f.deadline(Duration::from_secs(5)))
        .fresh(true);
    assert!(leg.stored_rows(&b, Find::Words("Tibsovo")).is_err());
    let body =
        r#"{"meta":{"timestamp":"2026-10-24T18:22:32Z"},"data":[{"id":"old","name":"Tibsovo"}]}"#;
    std::thread::scope(|s| {
        let call = s.spawn(|| f.sync(&b));
        server.next().respond(200, "", body);
        assert_eq!(call.join().unwrap().unwrap().bytes, body.len() as u64);
    });
    for invalid in [
        "{",
        r#"{"meta":{},"data":[{"id":"bad"}]}"#,
        r#"{"meta":{},"data":[]}"#,
    ] {
        std::thread::scope(|s| {
            let call = s.spawn(|| f.sync(&b));
            server.next().respond(200, "", invalid);
            assert!(call.join().unwrap().is_err());
        });
        let (rows, info) = leg.stored_rows(&b, Find::Words("Tibsovo")).unwrap();
        assert_eq!(rows[0].id, "old");
        assert_eq!(info.rows, 1);
    }
    assert!(server.requests.try_recv().is_err());
}
#[test]
fn two_independent_readers_observe_one_complete_json_replacement() {
    let server = support::Server::new();
    let b = bundle(&server);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cache.sqlite");
    let writer = cached(&path);
    let old =
        r#"{"meta":{},"data":[{"id":"old1","name":"Tibsovo"},{"id":"old2","name":"Tibsovo"}]}"#;
    let new = r#"{"meta":{},"data":[{"id":"new1","name":"Tibsovo"},{"id":"new2","name":"Tibsovo"},{"id":"new3","name":"Tibsovo"}]}"#;
    std::thread::scope(|s| {
        let call = s.spawn(|| writer.sync(&b));
        server.next().respond(200, "", old);
        call.join().unwrap().unwrap();
    });
    let readers = [cached(&path), cached(&path)];
    let barrier = std::sync::Barrier::new(3);
    std::thread::scope(|s| {
        let threads: Vec<_> = readers
            .iter()
            .map(|reader| {
                s.spawn(|| {
                    let leg = reader.leg(b.spec, reader.deadline(Duration::from_secs(10)));
                    assert_eq!(
                        leg.stored_rows(&b, Find::Words("Tibsovo")).unwrap().0.len(),
                        2
                    );
                    barrier.wait();
                    loop {
                        let (rows, info) = leg.stored_rows(&b, Find::Words("Tibsovo")).unwrap();
                        let ids: Vec<_> = rows.iter().map(|row| row.id.as_str()).collect();
                        assert_eq!(info.rows, rows.len() as u64);
                        if ids == ["new1", "new2", "new3"] {
                            break;
                        }
                        assert_eq!(ids, ["old1", "old2"]);
                    }
                })
            })
            .collect();
        let update = s.spawn(|| writer.sync(&b));
        let request = server.next();
        barrier.wait();
        request.respond(200, "", new);
        update.join().unwrap().unwrap();
        for reader in threads {
            reader.join().unwrap();
        }
    });
    assert!(server.requests.try_recv().is_err());
}
#[test]
fn truncated_json_download_retains_the_complete_prior_bundle() {
    let server = support::Server::new();
    let b = bundle(&server);
    let dir = tempfile::tempdir().unwrap();
    let writer = cached(&dir.path().join("cache.sqlite"));
    std::thread::scope(|s| {
        let call = s.spawn(|| writer.sync(&b));
        server.next().respond(
            200,
            "",
            r#"{"meta":{},"data":[{"id":"old","name":"Tibsovo"}]}"#,
        );
        call.join().unwrap().unwrap();
    });
    std::thread::scope(|s| {
        let call = s.spawn(|| writer.sync(&b));
        server.next().raw(
            "HTTP/1.1 200 Stub\r\nContent-Length: 200\r\nConnection: close\r\n\r\n{\"meta\":{}",
        );
        assert!(call.join().unwrap().is_err());
    });
    let leg = writer.leg(b.spec, writer.deadline(Duration::from_secs(5)));
    assert_eq!(
        leg.stored_rows(&b, Find::Words("Tibsovo")).unwrap().0[0].id,
        "old"
    );
}
