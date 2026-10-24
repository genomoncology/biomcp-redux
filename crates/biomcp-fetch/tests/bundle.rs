//! Bundle contracts through the public transport boundary.
#![expect(
    clippy::unwrap_used,
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "Fixture failures fail the contract"
)]
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
    let recording = recording_path(&capture);
    for round in 0..2 {
        if round == 1 {
            replace_recorded_body(&recording, TSV);
        }
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
        let (present, absent) = if round == 0 {
            ("NEW", "BRAF")
        } else {
            ("BRAF", "NEW")
        };
        assert_eq!(
            leg.rows(&b, Find::Exact(KeyKind::Symbol, present))
                .unwrap()
                .len(),
            1
        );
        assert!(
            leg.rows(&b, Find::Exact(KeyKind::Symbol, absent))
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

#[test]
fn interrupted_download_resumes_only_the_same_representation() {
    for scenario in [
        "safe", "ignored", "changed", "offset", "total", "missing", "412",
    ] {
        let server = support::Server::new();
        let b = declared(&server);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("cache.sqlite");
        let f = cached(&path);
        let cut = TSV.len() / 2;
        std::thread::scope(|scope| {
            let call = scope.spawn(|| f.sync(&b));
            server.next().raw(&format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nETag: \"one\"\r\nConnection: close\r\n\r\n{}",TSV.len(),&TSV[..cut]));
            assert!(call.join().unwrap().is_err());
        });
        std::thread::scope(|scope| {
            let call = scope.spawn(|| f.sync(&b));
            let request = server.next();
            let lower = request.text.to_ascii_lowercase();
            assert!(lower.contains(&format!("range: bytes={cut}-")));
            assert!(lower.contains("if-match: \"one\""));
            assert!(!lower.contains("if-range"));
            match scenario {
                "ignored" => request.respond(200, "ETag: \"two\"\r\n", TSV),
                "412" => {
                    request.respond(412, "", "");
                    let next = server.next();
                    assert!(!next.text.to_ascii_lowercase().contains("range:"));
                    next.respond(200, "ETag: \"two\"\r\n", TSV);
                }
                _ => {
                    let etag = if scenario == "changed" { "two" } else { "one" };
                    let offset = if scenario == "offset" { cut + 1 } else { cut };
                    let total = if scenario == "total" {
                        TSV.len() + 1
                    } else {
                        TSV.len()
                    };
                    let range = if scenario == "missing" {
                        String::new()
                    } else {
                        format!(
                            "Content-Range: bytes {offset}-{}/{total}\r\n",
                            TSV.len() - 1
                        )
                    };
                    request.respond(206, &format!("ETag: \"{etag}\"\r\n{range}"), &TSV[cut..]);
                }
            }
            let result = call.join().unwrap();
            if ["safe", "ignored", "412"].contains(&scenario) {
                assert_eq!(result.unwrap().rows, 1);
            } else {
                assert!(result.is_err());
            }
        });
    }
}

struct Child(std::process::Child);
impl Child {
    fn start(path: &std::path::Path, base: &str, expected: &str) -> Self {
        let mut command = std::process::Command::new(std::env::current_exe().unwrap());
        command
            .args(["--exact", "bundle_sync_child", "--nocapture"])
            .env_clear()
            .env("BUNDLE_PATH", path)
            .env("BUNDLE_BASE", base)
            .env("BUNDLE_EXPECTED", expected)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());
        if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {
            command.env("LLVM_PROFILE_FILE", profile);
        }
        Self(command.spawn().unwrap())
    }
    fn finish(&mut self) {
        let end = std::time::Instant::now() + Duration::from_secs(15);
        loop {
            if let Some(status) = self.0.try_wait().unwrap() {
                let mut output = String::new();
                use std::io::Read;
                self.0
                    .stdout
                    .take()
                    .unwrap()
                    .read_to_string(&mut output)
                    .unwrap();
                self.0
                    .stderr
                    .take()
                    .unwrap()
                    .read_to_string(&mut output)
                    .unwrap();
                assert!(status.success(), "{output}");
                break;
            }
            assert!(std::time::Instant::now() < end, "bundle child stalled");
            std::thread::park_timeout(Duration::from_millis(1));
        }
    }
}
impl Drop for Child {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
#[test]
fn bundle_sync_child() {
    let Ok(path) = std::env::var("BUNDLE_PATH") else {
        return;
    };
    let base = Box::leak(std::env::var("BUNDLE_BASE").unwrap().into_boxed_str());
    let spec = Box::leak(Box::new(biomcp_fetch::SourceSpec {
        base,
        source: biodata::Source::PubMed,
        key: KeyUse::None,
        quota: &QUOTA,
        deadline: Duration::from_secs(10),
    }));
    let b = Bundle::<Gene> {
        spec,
        name: "genes",
        path: "/genes",
        format: RowFormat::Tsv,
        fresh: Freshness::Days(1),
        row: PhantomData,
    };
    let f = cached(std::path::Path::new(&path));
    let result = f.sync(&b);
    if std::env::var("BUNDLE_EXPECTED").unwrap() == "syncing" {
        assert!(
            matches!(result,Err(biomcp_fetch::RequestError::Source(e)) if e.reason==biodata::Reason::Syncing)
        );
    } else {
        assert_eq!(result.unwrap().rows, 1);
    }
}
#[test]
fn independent_holders_contend_and_takeover_fences_the_old_publisher() {
    let server = support::Server::<()>::new();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cache.sqlite");
    let _ = cached(&path);
    let mut old = Child::start(&path, server.base, "syncing");
    let old_request = server.next();
    let mut contender = Child::start(&path, server.base, "syncing");
    contender.finish();
    assert!(server.requests.try_recv().is_err());
    let db = rusqlite::Connection::open(&path).unwrap();
    let old_path: String = db
        .query_row("SELECT part FROM bundle_sync", [], |r| r.get(0))
        .unwrap();
    db.execute("UPDATE bundle_sync SET lease_until=0", [])
        .unwrap();
    let mut successor = Child::start(&path, server.base, "ok");
    let next_request = server.next();
    let successor_path: String = db
        .query_row("SELECT part FROM bundle_sync", [], |r| r.get(0))
        .unwrap();
    assert_ne!(old_path, successor_path);
    old_request.respond(200, "ETag: \"old\"\r\n", TSV);
    old.finish();
    assert!(std::path::Path::new(&successor_path).exists());
    assert_eq!(
        db.query_row(
            "SELECT count(*) FROM bundle_sync WHERE holder IS NOT NULL",
            [],
            |r| r.get::<_, u32>(0)
        )
        .unwrap(),
        1
    );
    assert_eq!(
        db.query_row("SELECT count(*) FROM bundle_row", [], |r| r
            .get::<_, u32>(0))
            .unwrap(),
        0
    );
    next_request.respond(200, "ETag: \"new\"\r\n", TSV);
    successor.finish();
    assert_eq!(
        db.query_row("SELECT etag FROM bundle", [], |r| r.get::<_, String>(0))
            .unwrap(),
        "\"new\""
    );
    assert!(!std::path::Path::new(&old_path).exists());
}

#[test]
fn bundle_source_and_storage_boundaries_are_checked_before_requests() {
    let server = support::Server::<()>::new();
    let b = declared(&server);
    let foreign = declared(&server);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cache.sqlite");
    let f = cached(&path);
    assert!(
        matches!(f.leg(foreign.spec,f.deadline(Duration::from_secs(1))).rows(&b,Find::Words("x")),Err(biomcp_fetch::RequestError::Source(e)) if e.reason==biodata::Reason::Rejected)
    );
    assert!(
        matches!(f.leg(b.spec,f.deadline(Duration::ZERO)).rows(&b,Find::Words("x")),Err(biomcp_fetch::RequestError::Source(e)) if e.reason==biodata::Reason::DeadlineExceeded)
    );
    let uncached = Fetch::open(FetchConfig::default()).unwrap();
    assert!(matches!(
        uncached.sync(&b),
        Err(biomcp_fetch::RequestError::Cache(_))
    ));
    let offline = Fetch::open(FetchConfig {
        mode: biomcp_fetch::Mode::Offline,
        ..Default::default()
    })
    .unwrap();
    assert!(
        matches!(offline.sync(&b),Err(biomcp_fetch::RequestError::Source(e)) if e.reason==biodata::Reason::Offline)
    );
    assert!(
        matches!(offline.leg(b.spec,offline.deadline(Duration::from_secs(1))).rows(&b,Find::Words("x")),Err(biomcp_fetch::RequestError::Source(e)) if e.reason==biodata::Reason::Offline)
    );
    assert!(server.requests.try_recv().is_err());
}
#[test]
#[ignore = "Release proof: body larger than the prior recording-file cap"]
fn release_large_bundle_capture_round_trips() {
    let server = support::Server::new();
    let b = declared(&server);
    let dir = tempfile::tempdir().unwrap();
    let capture = dir.path().join("recordings");
    let body = format!(
        "symbol\tband\tname\nBRAF\t9p\t{}\n",
        "x".repeat(17 * 1024 * 1024)
    );
    let record = Fetch::open(FetchConfig {
        cache: Some(dir.path().join("cache.sqlite")),
        mode: biomcp_fetch::Mode::Record {
            dir: capture.clone(),
        },
        max_body: 64,
        ..Default::default()
    })
    .unwrap();
    assert_eq!(
        sync_reply(&record, &b, &server, 200, "", &body)
            .unwrap()
            .bytes,
        body.len() as u64
    );
    drop(record);
    let replay = Fetch::open(FetchConfig {
        mode: biomcp_fetch::Mode::Replay { dir: capture },
        max_body: 64,
        ..Default::default()
    })
    .unwrap();
    assert_eq!(replay.sync(&b).unwrap().bytes, body.len() as u64);
}

#[test]
fn a_stalled_body_keeps_its_partial_bytes_and_resumes_within_a_new_budget() {
    let server = support::Server::new();
    let b = declared(&server);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cache.sqlite");
    let f = Fetch::open(FetchConfig {
        cache: Some(path.clone()),
        sync_budget: Duration::from_secs(1),
        ..Default::default()
    })
    .unwrap();
    let cut = TSV.len() / 2;
    std::thread::scope(|scope| {
        let call = scope.spawn(|| f.sync(&b));
        let request = server.next();
        let (release, wait) = std::sync::mpsc::channel();
        request.reply.send(support::Response{bytes:format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nETag: \"one\"\r\nConnection: close\r\n\r\n{}",TSV.len(),&TSV[..cut]).into_bytes(),continuation:Some(wait)}).unwrap();
        assert!(
            matches!(call.join().unwrap(),Err(biomcp_fetch::RequestError::Source(e)) if e.reason==biodata::Reason::Syncing)
        );
        drop(release);
    });
    let db = rusqlite::Connection::open(&path).unwrap();
    assert_eq!(
        db.query_row("SELECT bytes FROM bundle_sync", [], |r| r.get::<_, u32>(0))
            .unwrap() as usize,
        cut
    );
    std::thread::scope(|scope| {
        let call = scope.spawn(|| f.sync(&b));
        let request = server.next();
        assert!(request.text.contains(&format!("bytes={cut}-")));
        request.respond(
            206,
            &format!(
                "ETag: \"one\"\r\nContent-Range: bytes {cut}-{}/{}\r\n",
                TSV.len() - 1,
                TSV.len()
            ),
            &TSV[cut..],
        );
        assert_eq!(call.join().unwrap().unwrap().rows, 1);
    });
    db.execute("UPDATE bundle SET fresh_until=0", []).unwrap();
    let offline = Fetch::open(FetchConfig {
        cache: Some(path),
        mode: biomcp_fetch::Mode::Offline,
        ..Default::default()
    })
    .unwrap();
    let leg = offline.leg(b.spec, offline.deadline(Duration::from_secs(1)));
    assert_eq!(leg.rows(&b, Find::Words("kinase")).unwrap().len(), 1);
    assert!(leg.finish().stale);
}

fn recording_path(dir: &std::path::Path) -> std::path::PathBuf {
    let source = std::fs::read_dir(dir)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    std::fs::read_dir(source)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path()
}
fn replace_recorded_body(path: &std::path::Path, body: &str) {
    let mut value: serde_json::Value =
        serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    value["exchanges"][0]["body"]["value"] = body.into();
    std::fs::write(path, serde_json::to_vec(&value).unwrap()).unwrap();
}
