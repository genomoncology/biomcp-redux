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
