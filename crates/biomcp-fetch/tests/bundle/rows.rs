use super::*;

#[test]
fn indexed_tsv_rows_are_literal_and_offline() {
    let server = support::Server::<()>::new();
    let spec = server.spec(&QUOTA, KeyUse::None);
    let bundle = Bundle::<Gene> {
        spec,
        name: "genes",
        path: "/genes",
        format: RowFormat::Tsv,
        fresh: Freshness::Days(1),
        row: PhantomData,
    };
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cache.sqlite");
    let fetch = Fetch::open(FetchConfig {
        cache: Some(path.clone()),
        ..Default::default()
    })
    .unwrap();
    std::thread::scope(|scope| {
        let run = scope.spawn(|| fetch.sync(&bundle).unwrap());
        server.next().respond(
            200,
            "ETag: \"one\"\r\n",
            "symbol\tband\tname\nBRAF\t9p21.1\t\"B raf kinase\"\nOTHER\t19p21\tOther\n",
        );
        assert_eq!(run.join().unwrap().rows, 2);
    });
    // Reply and bundle entries use independent tables even for the same URL.
    std::thread::scope(|scope| {
        let call = scope.spawn(|| {
            fetch
                .leg(spec, fetch.deadline(Duration::from_secs(2)))
                .json::<support::Answer>(&spec.get("/genes").fresh(Freshness::Days(1)), &[])
        });
        server.next().ok();
        assert!(call.join().unwrap().unwrap().answer);
    });
    let leg = fetch.leg(spec, fetch.deadline(Duration::from_secs(2)));
    assert_eq!(
        leg.rows(&bundle, Find::Exact(KeyKind::Symbol, "braf"))
            .unwrap()[0]
            .symbol,
        "BRAF"
    );
    assert_eq!(
        leg.rows(&bundle, Find::Prefix(KeyKind::Band, "9p21."))
            .unwrap()
            .len(),
        1
    );
    assert!(
        leg.rows(&bundle, Find::Prefix(KeyKind::Band, "%"))
            .unwrap()
            .is_empty()
    );
    for term in ["\"", "*", "AND", "NEAR("] {
        let _ = leg.rows(&bundle, Find::Words(term)).unwrap();
    }
    assert_eq!(leg.rows(&bundle, Find::Words("kinase")).unwrap().len(), 1);
    let offline = Fetch::open(FetchConfig {
        cache: Some(path),
        mode: biomcp_fetch::Mode::Offline,
        ..Default::default()
    })
    .unwrap();
    assert_eq!(
        offline
            .leg(spec, offline.deadline(Duration::from_secs(2)))
            .rows(&bundle, Find::Exact(KeyKind::Symbol, "BRAF"))
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn revalidation_swaps_all_indexes_and_bad_rows_keep_old_rows() {
    let server = support::Server::new();
    let b = declared(&server);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cache.sqlite");
    let f = cached(&path);
    sync_reply(&f, &b, &server, 200, "ETag: \"one\"\r\n", TSV).unwrap();
    let db = rusqlite::Connection::open(&path).unwrap();
    db.execute("UPDATE bundle SET fresh_until=0", []).unwrap();
    std::thread::scope(|scope| {
        let call = scope.spawn(|| f.sync(&b));
        let request = server.next();
        assert!(
            request
                .text
                .to_ascii_lowercase()
                .contains("if-none-match: \"one\"")
        );
        request.respond(304, "", "");
        assert!(call.join().unwrap().unwrap().fresh);
    });
    assert!(
        db.query_row(
            "SELECT sql FROM sqlite_master WHERE name='bundle_words'",
            [],
            |r| r.get::<_, String>(0)
        )
        .unwrap()
        .contains("fts5")
    );
    for bad in [
        "symbol\tband\tname\nBAD\tx\n",
        "symbol\tsymbol\nA\tB\n",
        "\tband\nA\tB\n",
        "symbol\tband\tname\nBAD\tx\t\"broken\n",
        "symbol\tband\tname\nBAD\tx\tin\"ner\n",
        "symbol\tband\tname\n\tx\tname\n",
    ] {
        assert!(sync_reply(&f, &b, &server, 200, "ETag: \"bad\"\r\n", bad).is_err());
        assert_eq!(
            db.query_row(
                "SELECT count(*) FROM bundle_key WHERE key='braf'",
                [],
                |r| r.get::<_, u32>(0)
            )
            .unwrap(),
            1
        );
    }
    sync_reply(
        &f,
        &b,
        &server,
        200,
        "ETag: \"two\"\r\n",
        "symbol\tband\tname\nNEW\t1q\tNew words\n",
    )
    .unwrap();
    let leg = f.leg(b.spec, f.deadline(Duration::from_secs(2)));
    assert!(
        leg.rows(&b, Find::Exact(KeyKind::Symbol, "BRAF"))
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        leg.rows(&b, Find::Words("new words")).unwrap()[0].symbol,
        "NEW"
    );
}
#[test]
fn banner_csv_records_are_checked_before_atomic_publication() {
    let server = support::Server::new();
    let mut bundle = declared(&server);
    bundle.format = RowFormat::Csv { banner_lines: 2 };
    let dir = tempfile::tempdir().unwrap();
    let fetch = cached(&dir.path().join("cache.sqlite"));
    let good = "banner\r\ncreated today\r\n\"symbol\",\"band\",\"name\"\r\n+++,+++,+++\r\nBRAF,9p,\"B, raf \"\"kinase\"\"\nwhole\"\r\n";
    assert_eq!(
        sync_reply(&fetch, &bundle, &server, 200, "", good)
            .unwrap()
            .rows,
        1
    );
    std::thread::scope(|scope| {
        let call = scope.spawn(|| fetch.sync(&bundle));
        let request = server.next();
        assert!(request.text.contains("text/csv"));
        assert!(!request.text.to_ascii_lowercase().contains("if-"));
        request.respond(
            200,
            "ETag: \"generated\"\r\nLast-Modified: Wed, 21 Oct 2015 07:28:00 GMT\r\n",
            good,
        );
        call.join().unwrap().unwrap();
    });
    std::thread::scope(|scope| {
        let call = scope.spawn(|| fetch.sync(&bundle));
        let request = server.next();
        assert!(!request.text.to_ascii_lowercase().contains("if-"));
        request.respond(200, "", good);
        call.join().unwrap().unwrap();
    });
    for bad in [
        "banner\ncreated\nsymbol,band,name\nBRAF,9p,\"open",
        "banner\ncreated\nsymbol,band,name\nBRAF,9p,bad\"quote\n",
        "banner\ncreated\nsymbol,band,name\nBRAF,9p,\"closed\"x\n",
        "banner\ncreated\nsymbol,,name\nBRAF,9p,a\n",
        "banner\ncreated\nsymbol,symbol,name\nBRAF,9p,a\n",
        "banner\ncreated\nsymbol,band,name\nBRAF,9p\n",
    ] {
        assert!(sync_reply(&fetch, &bundle, &server, 200, "", bad).is_err());
        let leg = fetch.leg(bundle.spec, fetch.deadline(Duration::from_secs(2)));
        let rows = leg
            .rows(&bundle, Find::Exact(KeyKind::Symbol, "BRAF"))
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, "B, raf \"kinase\"\nwhole");
    }
    let capture = dir.path().join("capture");
    let record = Fetch::open(FetchConfig {
        cache: Some(dir.path().join("record.sqlite")),
        mode: biomcp_fetch::Mode::Record {
            dir: capture.clone(),
        },
        ..Default::default()
    })
    .unwrap();
    sync_reply(&record, &bundle, &server, 200, "", good).unwrap();
    let replay = Fetch::open(FetchConfig {
        mode: biomcp_fetch::Mode::Replay { dir: capture },
        ..Default::default()
    })
    .unwrap();
    assert_eq!(replay.sync(&bundle).unwrap().rows, 1);
}

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
