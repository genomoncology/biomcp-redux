//! The reply cache stays under `reply_cache_max_bytes` when a reply is written (1001).
use super::*;

const REPLY: &str = r#"{"answer":true}"#;

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
// Ten-byte rows with the given key, fetch time and expiry.
fn insert(cx: &Connection, rows: &[(u8, i64, i64)]) {
    for (key, fetched_at, fresh_until) in rows {
        cx.execute(
            "INSERT INTO reply(key,source,url,status,body,fetched_at,fresh_until) VALUES \
            (?1,'pubmed','https://example.test/row',200,zeroblob(10),?2,?3)",
            rusqlite::params![vec![*key], fetched_at, fresh_until],
        )
        .unwrap();
    }
}
fn stored(cx: &Connection) -> (Vec<Vec<u8>>, i64) {
    let keys = cx
        .prepare("SELECT key FROM reply WHERE length(key)=1 ORDER BY key")
        .unwrap()
        .query_map([], |r| r.get(0))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    let bytes = cx
        .query_row("SELECT coalesce(sum(length(body)),0) FROM reply", [], |r| {
            r.get(0)
        })
        .unwrap();
    (keys, bytes)
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
    let fetch = ceiling(&path, 40);
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
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    exchange(&server, 200, REPLY, || get(&fetch, spec, &endpoint, &[])).unwrap();
    // 40 stored + 15 new = 55: the expired row goes first, then the oldest fresh one.
    let (keys, bytes) = stored(&cx);
    assert_eq!(keys, vec![vec![3], vec![4]]);
    assert_eq!(served(&cx), 1);
    assert_eq!(bytes, 35);
    assert!(bytes <= 40);
}

#[test]
fn the_reply_being_served_is_never_removed_even_when_it_is_the_oldest() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let spec = spec(&server, KeyUse::None);
    let fetch = ceiling(&path, 20);
    let cx = Connection::open(&path).unwrap();
    // A fresh row fetched after the reply about to be written.
    insert(&cx, &[(1, now() + 3600, i64::MAX)]);
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    assert!(
        exchange(&server, 200, REPLY, || get(&fetch, spec, &endpoint, &[]))
            .unwrap()
            .answer
    );
    let (keys, bytes) = stored(&cx);
    assert!(keys.is_empty());
    assert_eq!((served(&cx), bytes), (1, 15));
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
    let roomy = ceiling(&path, 1000);
    exchange(&server, 200, REPLY, || get(&roomy, spec, &endpoint, &[])).unwrap();
    let cx = Connection::open(&path).unwrap();
    insert(&cx, &[(1, now(), i64::MAX)]);
    let tight = Fetch::open(FetchConfig {
        mode: Mode::Live { fresh: true },
        reply_cache_max_bytes: 14,
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
    assert_eq!(stored(&cx), (vec![vec![1]], 10));
}
