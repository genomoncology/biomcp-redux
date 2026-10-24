//! Public stored article searches retain snapshots, headers and local failures.
#![expect(
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "Public store and concurrency contracts own isolated fixture I/O and abort on invalid fixture setup"
)]
use biodata::{ArticlePage, ArticleSearch, QueryBody, Request, Status};
use biomcp_fetch::{CacheError, Fetch, FetchConfig, Mode, ReplayCache, RequestError};
use biomcp_query::{Ctx, Outcome};
use std::{path::PathBuf, time::Duration};
fn recordings(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/replies/article/search")
        .join(name)
}
fn fetch(name: &str, cache: ReplayCache) -> Fetch {
    Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: recordings(name),
        },
        replay_cache: Some(cache),
        cache_owners: vec![biomcp_query::store::OWNER],
        ..FetchConfig::default()
    })
    .unwrap()
}
fn query(pool: Option<&str>, offset: u32) -> ArticleSearch {
    let mut v = serde_json::json!({"text":"AKT1 E17K","limit":2,"offset":offset});
    if let Some(pool) = pool {
        v["pool"] = pool.into();
    }
    serde_json::from_value(v).unwrap()
}
fn run(fetch: &Fetch, query: ArticleSearch, fresh: bool) -> Outcome {
    biomcp_query::run(
        &Ctx {
            fetch,
            budget: Duration::from_secs(20),
            fresh,
            ncbi_email: None,
        },
        &Request::SearchArticle(query),
    )
}
fn page(out: Outcome) -> (biodata::PageInfo, ArticlePage) {
    assert_eq!(out.local, None);
    assert_eq!(out.status, Status::Complete);
    let QueryBody::ArticlePage(page) = out.body else {
        panic!("article page")
    };
    (out.page.unwrap(), page)
}
#[test]
fn fixed_snapshots_survive_replacement_cursor_changes_and_expiry() {
    let cache = ReplayCache::temporary().unwrap();
    let fetch = fetch("synthetic-small", cache);
    let (first, items) = page(run(&fetch, query(None, 0), false));
    let old = first.pool.unwrap();
    let (second, _) = page(run(&fetch, query(None, 0), true));
    let new = second.pool.unwrap();
    assert_ne!(old.id, new.id);
    assert_eq!(old.expires - old.created, 86400);
    let (again, retained) = page(run(&fetch, query(Some(old.id.as_str()), 0), false));
    assert_eq!(again.pool, Some(old.clone()));
    assert_eq!(items, retained);
    let (_, current) = page(run(&fetch, query(None, 0), false));
    assert_eq!(current, items);
    let (_, empty) = page(run(&fetch, query(Some(old.id.as_str()), u32::MAX), false));
    assert!(empty.items.is_empty());
    let mut mismatch = serde_json::to_value(query(Some(old.id.as_str()), 0)).unwrap();
    mismatch["text"] = "different selectors".into();
    assert!(matches!(
        run(&fetch, serde_json::from_value(mismatch).unwrap(), false).body,
        QueryBody::PoolError { mismatch: true }
    ));
    let connection = fetch.cache_access().unwrap().connect().unwrap();
    connection
        .execute("UPDATE pool SET expires=0 WHERE id=?1", [old.id.as_str()])
        .unwrap();
    assert!(matches!(
        run(&fetch, query(Some(old.id.as_str()), 0), false).body,
        QueryBody::PoolError { mismatch: false }
    ));
    assert_eq!(
        page(run(&fetch, query(Some(new.id.as_str()), 0), false))
            .0
            .pool,
        Some(new)
    );
}
#[test]
fn store_corruption_and_exhausted_budget_remain_typed_local_failures() {
    let cache = ReplayCache::temporary().unwrap();
    let fetch = fetch("synthetic-small", cache);
    let (first, _) = page(run(&fetch, query(None, 0), false));
    let id = first.pool.unwrap().id;
    fetch
        .cache_access()
        .unwrap()
        .connect()
        .unwrap()
        .execute(
            "UPDATE pool SET outcomes='invalid' WHERE id=?1",
            [id.as_str()],
        )
        .unwrap();
    let out = run(&fetch, query(Some(id.as_str()), 0), false);
    assert_eq!(out.status, Status::Failed);
    assert_eq!(out.local, Some(RequestError::Cache(CacheError::Database)));
    assert!(out.sources.is_empty());
    let out = biomcp_query::run(
        &Ctx {
            fetch: &fetch,
            budget: Duration::ZERO,
            fresh: false,
            ncbi_email: None,
        },
        &Request::SearchArticle(query(None, 0)),
    );
    assert_eq!(out.local, Some(RequestError::Cache(CacheError::Busy)));
}
#[test]
fn concurrent_commands_publish_distinct_complete_snapshots() {
    let cache = ReplayCache::temporary().unwrap();
    let fetch = fetch("synthetic-small", cache);
    page(run(&fetch, query(None, 0), false));
    let outcomes = std::thread::scope(|scope| {
        let one = scope.spawn(|| run(&fetch, query(None, 0), true));
        let two = scope.spawn(|| run(&fetch, query(None, 0), true));
        [one.join().unwrap(), two.join().unwrap()]
    });
    let mut ids = std::collections::BTreeSet::new();
    for outcome in outcomes {
        let (p, items) = page(outcome);
        let id = p.pool.unwrap().id;
        assert!(ids.insert(id.to_string()));
        assert_eq!(items.items.len(), 2);
        assert_ne!(items.items[0].pmid, items.items[1].pmid);
    }
    for id in ids {
        assert_eq!(
            page(run(&fetch, query(Some(&id), 0), false)).1.items.len(),
            2
        );
    }
}

#[test]
fn checked_missing_headers_stay_warm_and_fresh_fills_only_the_hole() {
    let cache = ReplayCache::temporary().unwrap();
    let fetch = fetch("synthetic-retry", cache);
    let first = run(&fetch, query(None, 0), false);
    assert_eq!(first.status, Status::Partial);
    assert_eq!(first.local, None);
    let id = first.page.unwrap().pool.unwrap().id;
    let QueryBody::ArticlePage(before) = first.body else {
        panic!("article page")
    };
    assert_eq!(before.items.len(), 2);
    assert!(before.items[0].header.is_none());
    assert!(before.items[1].header.is_some());
    let warm = run(&fetch, query(Some(id.as_str()), 0), false);
    assert_eq!(warm.status, Status::Partial);
    let QueryBody::ArticlePage(warm) = warm.body else {
        panic!("article page")
    };
    assert_eq!(warm, before);
    let (after, filled) = page(run(&fetch, query(Some(id.as_str()), 0), true));
    assert_eq!(after.pool.unwrap().id, id);
    assert!(filled.items[0].header.is_some());
    assert_eq!(before.items[1], filled.items[1]);
    assert_eq!(
        before.items.iter().map(|i| i.pmid).collect::<Vec<_>>(),
        filled.items.iter().map(|i| i.pmid).collect::<Vec<_>>()
    );
}
#[test]
fn search_and_hydration_share_one_budget_while_pool_age_uses_wall_time() {
    let cache = ReplayCache::temporary().unwrap();
    let fetch = fetch("synthetic-deadline", cache);
    let before = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let first = biomcp_query::run(
        &Ctx {
            fetch: &fetch,
            budget: Duration::from_millis(300),
            fresh: false,
            ncbi_email: None,
        },
        &Request::SearchArticle(query(None, 0)),
    );
    assert_eq!(first.status, Status::Failed);
    assert_eq!(first.local, None);
    assert_eq!(
        first.sources[0].reason,
        Some(biodata::Reason::DeadlineExceeded)
    );
    let biodata::StepKind::Fresh = first.next[0].0 else {
        panic!("fresh action")
    };
    let biomcp_query::Action::Run {
        request: Request::SearchArticle(retry),
        ..
    } = first.next[0].1.clone()
    else {
        panic!("retry retains pool")
    };
    assert!(retry.pool().is_some());
    let (p, _) = page(run(&fetch, retry, false));
    let pool = p.pool.unwrap();
    let after = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    assert!((before..=after).contains(&pool.created));
    assert_eq!(pool.expires - pool.created, 86400);
}

#[test]
fn concurrent_hydration_retains_accepted_headers_and_snapshot_metadata() {
    let cache = ReplayCache::temporary().unwrap();
    let fetch = fetch("synthetic-retry", cache);
    let first = run(&fetch, query(None, 0), false);
    let info = first.page.unwrap().pool.unwrap();
    let QueryBody::ArticlePage(before) = first.body else {
        panic!("article page")
    };
    assert!(before.items[0].header.is_none());
    let outcomes = std::thread::scope(|scope| {
        let one = scope.spawn(|| run(&fetch, query(Some(info.id.as_str()), 0), true));
        let two = scope.spawn(|| run(&fetch, query(Some(info.id.as_str()), 0), true));
        [one.join().unwrap(), two.join().unwrap()]
    });
    for out in outcomes {
        let (page, items) = page(out);
        assert_eq!(page.pool, Some(info.clone()));
        assert!(items.items[0].header.is_some());
        assert_eq!(items.items[1], before.items[1]);
    }
    let (_, items) = page(run(&fetch, query(Some(info.id.as_str()), 0), false));
    assert!(items.items.iter().all(|i| i.header.is_some()));
}
