use super::{record, run};
use biomcp_fetch::{Fetch, FetchConfig, Mode, ReplayCache};
use biomcp_types::{ReplyData, Use};
use std::time::Duration;

#[test]
fn restricted_abstracts_and_passages_follow_the_same_record_policy_before_cache_writes() {
    let input = tempfile::tempdir().unwrap();
    record(input.path());
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: input.path().into(),
        },
        disabled_uses: [Use::PerRecord].into(),
        replay_cache: Some(ReplayCache::temporary().unwrap()),
        ..FetchConfig::default()
    })
    .unwrap();
    let leg = fetch.leg(
        &biomcp_adapters::pubmed::SPEC,
        fetch.deadline(Duration::from_secs(20)),
    );
    let (articles, _) =
        biomcp_adapters::pubmed::fetch_articles(&leg, &["1002".parse().unwrap()], None).unwrap();
    assert_eq!(articles.len(), 1);
    assert!(articles[0].article.abstract_parts.is_empty());
    assert!(articles[0].article.text_withheld.is_some());
    let leg = fetch.leg(
        &biomcp_adapters::pubtator::SPEC,
        fetch.deadline(Duration::from_secs(20)),
    );
    let annotations = biomcp_adapters::pubtator::annotations(
        &leg,
        &[
            "1001".parse().unwrap(),
            "1002".parse().unwrap(),
            "1003".parse().unwrap(),
        ],
    )
    .unwrap();
    assert_eq!(
        annotations
            .iter()
            .map(|row| row.id.to_string())
            .collect::<Vec<_>>(),
        ["1001", "1002", "1003"]
    );
    let leg = fetch.leg(
        &biomcp_adapters::europepmc::SPEC,
        fetch.deadline(Duration::from_secs(20)),
    );
    let (noncommercial, _) =
        biomcp_adapters::europepmc::core(&leg, &"PPR1002".parse().unwrap()).unwrap();
    let (unknown, _) = biomcp_adapters::europepmc::core(&leg, &"PPR1003".parse().unwrap()).unwrap();
    assert!(noncommercial.value.abstract_parts.is_empty());
    assert!(noncommercial.value.text_withheld.is_some());
    assert_eq!(
        unknown.value.abstract_parts.len(),
        1,
        "unknown abstract licenses retain the source's terms"
    );
    let connection = fetch.cache_access().unwrap().connect().unwrap();
    let stored: String = connection
        .query_row(
            "SELECT group_concat(CAST(body AS TEXT)) FROM reply",
            [],
            |row| row.get(0),
        )
        .unwrap();
    for forbidden in [
        "noncommercial abstract witness",
        "noncommercial passage witness",
        "unknown passage witness",
        "PPR1002 abstract witness",
    ] {
        assert!(
            !stored.contains(forbidden),
            "restricted cache retained {forbidden}"
        );
    }
    assert!(stored.contains("permitted passage witness"));
    assert!(stored.contains("unknown abstract passage retained"));
    assert!(stored.contains("PPR1003 abstract witness"));
}

#[test]
fn exhausted_license_budget_returns_papers_without_text_or_license_requests() {
    let original = tempfile::tempdir().unwrap();
    record(original.path());
    let delayed = tempfile::tempdir().unwrap();
    biomcp_test_support::recordings::copy_capture(
        original.path(),
        delayed.path(),
        |source, capture| {
            if source == "litsense" {
                for exchange in capture["exchanges"].as_array_mut().unwrap() {
                    exchange["delay_ms"] = 14_950.into();
                }
            }
        },
    )
    .unwrap();
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: delayed.path().into(),
        },
        disabled_uses: [Use::PerRecord].into(),
        replay_cache: Some(
            ReplayCache::temporary_with_owners(&[biomcp_query::store::OWNER]).unwrap(),
        ),
        cache_owners: vec![biomcp_query::store::OWNER],
        ..FetchConfig::default()
    })
    .unwrap();
    let out = run(&fetch);
    let ReplyData::ArticlePage {
        body: Some(page), ..
    } = out.body
    else {
        panic!("delayed article page: {out:?}")
    };
    assert_eq!(page.items.len(), 3);
    assert!(page.items.iter().all(|row| row.sentence.is_none()));
    let connection = fetch.cache_access().unwrap().connect().unwrap();
    let lookups: u64 = connection
        .query_row(
            "SELECT count(*) FROM reply WHERE source='pmc-oa'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(
        lookups, 0,
        "the remaining source budget must reserve time for paper identities"
    );
}
