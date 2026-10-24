//! Bundle contracts through the public transport boundary.
#![expect(clippy::unwrap_used, reason = "Fixture failures fail the contract")]
mod support;
use biomcp_fetch::{
    Bundle, Fetch, FetchConfig, Find, Freshness, KeyKind, KeyUse, Keys, Quota, Rate, Row,
    RowFormat, SourceKey,
};
use serde::Deserialize;
use std::{marker::PhantomData, time::Duration};
#[derive(Debug, Deserialize)]
struct Gene {
    symbol: String,
    band: String,
    name: String,
}
impl Row for Gene {
    fn keys(&self) -> Keys {
        Keys {
            exact: vec![
                (KeyKind::Symbol, self.symbol.clone().into()),
                (KeyKind::Band, self.band.clone().into()),
            ],
            words: vec![self.name.clone().into()],
        }
    }
}
const QUOTA: Quota = Quota::new(
    SourceKey::declare("bundle-contract"),
    Rate::per_second(1000),
    None,
    4,
);
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

fn declared(server: &support::Server) -> Bundle<Gene> {
    Bundle {
        spec: server.spec(&QUOTA, KeyUse::None),
        name: "genes",
        path: "/genes",
        format: RowFormat::Tsv,
        fresh: Freshness::Days(1),
        row: PhantomData,
    }
}
const TSV: &str = "symbol\tband\tname\nBRAF\t9p21.1\t\"B raf kinase\"\n";
fn cached(path: &std::path::Path) -> Fetch {
    Fetch::open(FetchConfig {
        cache: Some(path.to_owned()),
        ..Default::default()
    })
    .unwrap()
}
fn sync_reply(
    fetch: &Fetch,
    bundle: &Bundle<Gene>,
    server: &support::Server,
    status: u16,
    headers: &str,
    body: &str,
) -> Result<biomcp_fetch::BundleInfo, biomcp_fetch::RequestError> {
    std::thread::scope(|scope| {
        let call = scope.spawn(|| fetch.sync(bundle));
        server.next().respond(status, headers, body);
        call.join().unwrap()
    })
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
fn source_failure_retains_stale_rows_but_local_errors_do_not() {
    let server = support::Server::new();
    let b = declared(&server);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cache.sqlite");
    let f = cached(&path);
    sync_reply(&f, &b, &server, 200, "", TSV).unwrap();
    let db = rusqlite::Connection::open(&path).unwrap();
    db.execute("UPDATE bundle SET fresh_until=0", []).unwrap();
    let leg = f.leg(b.spec, f.deadline(Duration::from_secs(5)));
    std::thread::scope(|scope| {
        let call = scope.spawn(|| leg.rows(&b, Find::Exact(KeyKind::Symbol, "BRAF")));
        for _ in 0..3 {
            server.next().respond(500, "", "");
        }
        assert_eq!(call.join().unwrap().unwrap().len(), 1);
    });
    let report = leg.finish();
    assert!(report.stale);
    assert_eq!(report.failures[0].status, Some(500));
    db.execute_batch("DROP TABLE bundle_key").unwrap();
    assert!(matches!(
        f.leg(b.spec, f.deadline(Duration::from_secs(1)))
            .rows(&b, Find::Exact(KeyKind::Symbol, "BRAF")),
        Err(biomcp_fetch::RequestError::Cache(_))
    ));
}
#[test]
fn recording_replay_replaces_warm_case_rows_once_and_ignores_user_cache() {
    let server = support::Server::new();
    let b = declared(&server);
    let dir = tempfile::tempdir().unwrap();
    let capture = dir.path().join("recordings");
    let path = dir.path().join("live.sqlite");
    let initial = cached(&path);
    sync_reply(&initial, &b, &server, 200, "", TSV).unwrap();
    let record = Fetch::open(FetchConfig {
        cache: Some(path.clone()),
        mode: biomcp_fetch::Mode::Record {
            dir: capture.clone(),
        },
        ..Default::default()
    })
    .unwrap();
    std::thread::scope(|scope| {
        let call = scope.spawn(|| record.sync(&b));
        let request = server.next();
        assert!(!request.text.to_ascii_lowercase().contains("if-"));
        request.respond(200, "", "symbol\tband\tname\nNEW\t9p\t\"tab\\line\"\n");
        call.join().unwrap().unwrap();
    });
    drop(record);
    let case = biomcp_fetch::ReplayCache::temporary().unwrap();
    for _ in 0..2 {
        let replay = Fetch::open(FetchConfig {
            cache: Some(dir.path().join("must-not-exist.sqlite")),
            mode: biomcp_fetch::Mode::Replay {
                dir: capture.clone(),
            },
            replay_cache: Some(case.clone()),
            ..Default::default()
        })
        .unwrap();
        let leg = replay.leg(b.spec, replay.deadline(Duration::from_secs(5)));
        assert_eq!(
            leg.rows(&b, Find::Exact(KeyKind::Symbol, "NEW"))
                .unwrap()
                .len(),
            1
        );
        assert!(
            leg.rows(&b, Find::Exact(KeyKind::Symbol, "BRAF"))
                .unwrap()
                .is_empty()
        );
        assert!(!dir.path().join("must-not-exist.sqlite").exists());
    }
    let offline = Fetch::open(FetchConfig {
        mode: biomcp_fetch::Mode::Offline,
        replay_cache: Some(case),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(
        offline
            .leg(b.spec, offline.deadline(Duration::from_secs(1)))
            .rows(&b, Find::Prefix(KeyKind::Symbol, ""))
            .unwrap()
            .len(),
        1
    );
    assert!(server.requests.try_recv().is_err());
}
#[test]
fn auto_download_limit_keeps_owned_part_and_offline_miss_stays_offline() {
    let server = support::Server::new();
    let b = declared(&server);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cache.sqlite");
    let f = Fetch::open(FetchConfig {
        cache: Some(path.clone()),
        auto_sync_max_bytes: 1,
        ..Default::default()
    })
    .unwrap();
    let leg = f.leg(b.spec, f.deadline(Duration::from_secs(2)));
    std::thread::scope(|scope| {
        let call = scope.spawn(|| leg.rows(&b, Find::Words("kinase")));
        server.next().respond(200, "ETag: \"one\"\r\n", TSV);
        assert!(
            matches!(call.join().unwrap(),Err(biomcp_fetch::RequestError::Source(e)) if e.reason==biodata::Reason::Syncing)
        );
    });
    assert_eq!(
        std::fs::read_dir(path.with_extension("parts"))
            .unwrap()
            .count(),
        1
    );
    let offline = Fetch::open(FetchConfig {
        cache: Some(path),
        mode: biomcp_fetch::Mode::Offline,
        ..Default::default()
    })
    .unwrap();
    assert!(
        matches!(offline.leg(b.spec,offline.deadline(Duration::from_secs(1))).rows(&b,Find::Words("kinase")),Err(biomcp_fetch::RequestError::Source(e)) if e.reason==biodata::Reason::Offline)
    );
    assert!(server.requests.try_recv().is_err());
}
