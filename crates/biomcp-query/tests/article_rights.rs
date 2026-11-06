//! Restricted article text is filtered before raw and mapped persistence.
#![expect(
    clippy::unwrap_used,
    clippy::panic,
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "Isolated synthetic transport and SQLite witness"
)]
use biomcp_fetch::{Fetch, FetchConfig, Mode, ReplayCache};
use biomcp_types::{ArticleSearch, ReplyData, Request, Use};
use std::time::Duration;

fn run(fetch: &Fetch) -> biomcp_query::Outcome {
    biomcp_query::run(
        &biomcp_query::Ctx {
            fetch,
            budget: Duration::from_secs(20),
            fresh: false,
            ncbi_email: None,
        },
        &Request::SearchArticle(
            serde_json::from_str::<ArticleSearch>(
                r#"{"text":"rights witness","source":["litsense"],"limit":3}"#,
            )
            .unwrap(),
        ),
    )
}
#[path = "article_rights/fixture.rs"]
mod fixture;
use fixture::record;

#[test]
fn restricted_keeps_every_paper_and_persists_only_permitted_body_text() {
    let input = tempfile::tempdir().unwrap();
    record(input.path());
    let cache = ReplayCache::temporary_with_owners(&[biomcp_query::store::OWNER]).unwrap();
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: input.path().into(),
        },
        disabled_uses: [Use::PerRecord].into(),
        replay_cache: Some(cache),
        cache_owners: vec![biomcp_query::store::OWNER],
        ..FetchConfig::default()
    })
    .unwrap();
    let out = run(&fetch);
    let ReplyData::ArticlePage {
        body: Some(page), ..
    } = out.body
    else {
        panic!("{out:?}")
    };
    assert_eq!(
        page.items
            .iter()
            .filter_map(|row| row.pmid)
            .map(|id| id.to_string())
            .collect::<Vec<_>>(),
        ["1001", "1002", "1003"]
    );
    assert_eq!(
        page.items
            .iter()
            .filter(|row| row.sentence.is_some())
            .count(),
        1,
        "keep all three papers but only the permitted sentence"
    );
    let connection = fetch.cache_access().unwrap().connect().unwrap();
    let disk: String = connection
        .query_row("SELECT group_concat(item) FROM pool_item", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert!(!disk.contains("noncommercial body witness"));
    assert!(!disk.contains("unknown body witness"));
    let raw: Vec<u8> = connection
        .query_row(
            "SELECT body FROM reply WHERE source='litsense'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    let raw = String::from_utf8(raw).unwrap();
    assert!(raw.contains("permitted body witness"));
    assert!(!raw.contains("noncommercial body witness"));
    assert!(!raw.contains("unknown body witness"));
    let query = biomcp_types::ArticleText::part(
        biomcp_types::ArticleInput::Pmcid("PMC1002".parse().unwrap()),
        1,
    )
    .unwrap();
    let fulltext = biomcp_query::run(
        &biomcp_query::Ctx {
            fetch: &fetch,
            budget: Duration::from_secs(20),
            fresh: false,
            ncbi_email: None,
        },
        &Request::ArticleText(query),
    );
    assert_eq!(
        fulltext.status(),
        biomcp_types::Status::Complete,
        "identity and refusal are a complete response: {fulltext:?}"
    );
    let ReplyData::ArticleTextResult {
        body: Some(body), ..
    } = fulltext.body
    else {
        panic!("fulltext identity response")
    };
    assert_eq!(body.pmcid.unwrap().to_string(), "PMC1002");
    assert_eq!(
        body.text_withheld.unwrap().reason,
        biomcp_types::Reason::Disabled
    );
}
#[test]
fn restricted_continuation_keeps_legacy_pool_rank_and_removes_disallowed_text() {
    let input = tempfile::tempdir().unwrap();
    record(input.path());
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: input.path().into(),
        },
        replay_cache: Some(
            ReplayCache::temporary_with_owners(&[biomcp_query::store::OWNER]).unwrap(),
        ),
        cache_owners: vec![biomcp_query::store::OWNER],
        ..FetchConfig::default()
    })
    .unwrap();
    let unrestricted = run(&fetch);
    let pool = unrestricted.page.unwrap().pool.unwrap();
    let fetch = fetch.restrict_uses([Use::PerRecord].into());
    let query = serde_json::from_str::<ArticleSearch>(
        r#"{"text":"rights witness","source":["litsense"],"limit":3}"#,
    )
    .unwrap()
    .page(0, Some(pool.id.clone()));
    let restricted = biomcp_query::run(
        &biomcp_query::Ctx {
            fetch: &fetch,
            budget: Duration::from_secs(20),
            fresh: false,
            ncbi_email: None,
        },
        &Request::SearchArticle(query),
    );
    assert_eq!(restricted.page.unwrap().pool.unwrap(), pool);
    let ReplyData::ArticlePage {
        body: Some(page), ..
    } = restricted.body
    else {
        panic!("legacy article page")
    };
    assert_eq!(page.items.len(), 3);
    assert_eq!(
        page.items
            .iter()
            .filter(|row| row.sentence.is_some())
            .count(),
        1
    );
    let connection = fetch.cache_access().unwrap().connect().unwrap();
    let rows: String = connection
        .query_row("SELECT group_concat(item) FROM pool_item", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert!(!rows.contains("noncommercial body witness"));
    assert!(!rows.contains("unknown body witness"));
}
