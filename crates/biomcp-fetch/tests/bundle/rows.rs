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
