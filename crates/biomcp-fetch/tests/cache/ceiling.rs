//! The reply cache stays under `reply_cache_max_bytes` when a reply is written (1001).
//! A row counts its body, its URL and 64 bytes. Fixture rows count 1000 + 24 + 64 = 1088.
use super::*;

const REPLY: &str = r#"{"answer":true}"#;
const ROW: i64 = 1088;

fn ceiling(path: &Path, bytes: u64) -> Fetch {
    Fetch::open(FetchConfig {
        reply_cache_max_bytes: bytes,
        ..config(path)
    })
    .unwrap()
}
fn now() -> i64 {
    i64::try_from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    )
    .unwrap()
}
// Fixture rows with the given key, fetch time and expiry.
fn insert(cx: &Connection, rows: &[(u8, i64, i64)]) {
    for (key, fetched_at, fresh_until) in rows {
        cx.execute(
            "INSERT INTO reply(key,source,url,status,body,fetched_at,fresh_until) VALUES \
            (?1,'pubmed','https://example.test/row',200,zeroblob(1000),?2,?3)",
            rusqlite::params![vec![*key], fetched_at, fresh_until],
        )
        .unwrap();
    }
}
fn keys(cx: &Connection) -> Vec<Vec<u8>> {
    cx.prepare("SELECT key FROM reply WHERE length(key)=1 ORDER BY key")
        .unwrap()
        .query_map([], |r| r.get(0))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap()
}
// The running total and the bytes it should equal.
fn totals(cx: &Connection) -> (i64, i64) {
    cx.query_row(
        "SELECT (SELECT total FROM reply_bytes), \
        (SELECT coalesce(sum(length(body)+length(url)+64),0) FROM reply)",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )
    .unwrap()
}
fn served(cx: &Connection) -> u32 {
    cx.query_row("SELECT count(*) FROM reply WHERE length(key)=32", [], |r| {
        r.get(0)
    })
    .unwrap()
}

#[test]
fn a_write_past_the_ceiling_removes_expired_rows_then_the_oldest() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let spec = spec(&server, KeyUse::None);
    let fetch = ceiling(&path, 3400);
    let cx = Connection::open(&path).unwrap();
    let now = now();
    // Key 2 is expired though fetched last; keys 1, 3 and 4 are fresh, oldest first.
    insert(
        &cx,
        &[
            (1, now - 300, i64::MAX),
            (2, now - 10, now - 5),
            (3, now - 200, i64::MAX),
            (4, now - 100, i64::MAX),
        ],
    );
    assert_eq!(totals(&cx), (4 * ROW, 4 * ROW));
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    exchange(&server, 200, REPLY, || get(&fetch, spec, &endpoint, &[])).unwrap();
    // Past 3400, removal runs down to 3060: the expired row goes first, then the oldest.
    assert_eq!(keys(&cx), vec![vec![3], vec![4]]);
    assert_eq!(served(&cx), 1);
    let (total, actual) = totals(&cx);
    assert_eq!(total, actual);
    assert!(total <= 3060, "{total}");
}

#[test]
fn a_write_under_the_ceiling_reads_only_the_running_total() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let spec = spec(&server, KeyUse::None);
    let fetch = ceiling(&path, 3400);
    let cx = Connection::open(&path).unwrap();
    let now = now();
    insert(
        &cx,
        &[
            (1, now - 300, i64::MAX),
            (2, now - 10, now - 5),
            (3, now - 200, i64::MAX),
            (4, now - 100, i64::MAX),
        ],
    );
    // The rows are past the ceiling, but the running total says they are not. A write
    // that scanned the rows would remove some; a write that reads the total removes none.
    cx.execute("UPDATE reply_bytes SET total=0", []).unwrap();
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    exchange(&server, 200, REPLY, || get(&fetch, spec, &endpoint, &[])).unwrap();
    assert_eq!(keys(&cx).len(), 4);
    assert_eq!(served(&cx), 1);
}

#[test]
fn the_reply_being_served_is_never_removed_even_when_it_is_the_oldest() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let spec = spec(&server, KeyUse::None);
    let fetch = ceiling(&path, 300);
    let cx = Connection::open(&path).unwrap();
    // A fresh row fetched after the reply about to be written.
    insert(&cx, &[(1, now() + 3600, i64::MAX)]);
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    assert!(
        exchange(&server, 200, REPLY, || get(&fetch, spec, &endpoint, &[]))
            .unwrap()
            .answer
    );
    assert!(keys(&cx).is_empty());
    assert_eq!(served(&cx), 1);
    // The stored reply answers the next call with no request.
    assert!(get(&fetch, spec, &endpoint, &[]).unwrap().answer);
    assert!(server.requests.try_recv().is_err());
}

#[test]
fn a_reply_larger_than_the_ceiling_is_answered_but_not_stored() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let spec = spec(&server, KeyUse::None);
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    let roomy = ceiling(&path, 10_000);
    exchange(&server, 200, REPLY, || get(&roomy, spec, &endpoint, &[])).unwrap();
    let cx = Connection::open(&path).unwrap();
    insert(&cx, &[(1, now(), i64::MAX)]);
    // 64 bytes is less than the 15-byte reply, its URL and the 64-byte overhead.
    let tight = Fetch::open(FetchConfig {
        mode: Mode::Live { fresh: true },
        reply_cache_max_bytes: 64,
        ..config(&path)
    })
    .unwrap();
    assert!(
        exchange(&server, 200, REPLY, || get(&tight, spec, &endpoint, &[]))
            .unwrap()
            .answer
    );
    // The older copy of the same reply goes too; other rows stay.
    assert_eq!(served(&cx), 0);
    assert_eq!(keys(&cx), vec![vec![1]]);
    assert_eq!(totals(&cx), (ROW, ROW));
}

#[test]
fn a_zero_ceiling_stores_no_replies_and_no_absences() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let spec = spec(&server, KeyUse::None);
    let fetch = ceiling(&path, 0);
    let cx = Connection::open(&path).unwrap();
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    for _ in 0..2 {
        exchange(&server, 200, REPLY, || get(&fetch, spec, &endpoint, &[])).unwrap();
    }
    // A not-found reply has an empty body, and still counts its URL and overhead.
    let absent = spec.get("/answer").fresh(Freshness::Minutes(30));
    let error = exchange(&server, 404, "", || {
        fetch
            .leg(spec, fetch.deadline(Duration::from_secs(10)))
            .call(&absent, Input::Params(&[]), Read::<Answer>::json())
            .map(|received| received.value)
    })
    .unwrap_err();
    assert_eq!(source(error).reason, Reason::NotFound);
    assert_eq!(served(&cx), 0);
    assert_eq!(totals(&cx), (0, 0));
}

#[test]
fn a_version_five_cache_gains_its_running_total_on_open() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    drop(Fetch::open(config(&path)).unwrap());
    let cx = Connection::open(&path).unwrap();
    cx.execute_batch(
        "DROP TRIGGER reply_bytes_insert; DROP TRIGGER reply_bytes_delete; \
        DROP TRIGGER reply_bytes_update; DROP TABLE reply_bytes; \
        UPDATE meta SET version=5 WHERE owner='biomcp-fetch';",
    )
    .unwrap();
    insert(&cx, &[(1, now(), i64::MAX), (2, now(), i64::MAX)]);
    drop(Fetch::open(config(&path)).unwrap());
    assert_eq!(totals(&cx), (2 * ROW, 2 * ROW));
    cx.execute("DELETE FROM reply WHERE key=x'01'", []).unwrap();
    assert_eq!(totals(&cx), (ROW, ROW));
}
