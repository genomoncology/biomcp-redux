//! Temporary public source contracts before disease command integration.
use biomcp_adapters::seer::{self, Selection, Statistic};
use biomcp_fetch::{Fetch, FetchConfig, Mode};
use std::time::Duration;

fn fetch() -> Result<Fetch, biomcp_fetch::FetchError> {
    Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../tests/replies/disease/seer-synthetic"),
        },
        ..Default::default()
    })
}
fn replay_and_offline() -> Result<(Fetch, Fetch), Box<dyn std::error::Error>> {
    let cache = biomcp_fetch::ReplayCache::temporary()?;
    let replay = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../tests/replies/disease/seer-synthetic"),
        },
        replay_cache: Some(cache.clone()),
        ..Default::default()
    })?;
    let offline = Fetch::open(FetchConfig {
        mode: Mode::Offline,
        replay_cache: Some(cache),
        ..Default::default()
    })?;
    Ok((replay, offline))
}
fn changed(kind: &str) -> Result<Fetch, biomcp_fetch::FetchError> {
    Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../tests/replies/disease/seer-synthetic")
                .join(kind),
        },
        ..Default::default()
    })
}

fn selected(statistic: Statistic) -> Result<Selection, biodata::Error> {
    Ok(Selection {
        site: "55".parse()?,
        sex: "3".parse()?,
        race: "1".parse()?,
        age_range: "1".parse()?,
        dataset: "4".parse()?,
        statistic,
    })
}

#[test]
fn catalog_pages_preserve_native_order_labels_and_active_state() {
    let (fetch, offline) = replay_and_offline().unwrap();
    let leg = fetch.leg(&seer::SPEC, fetch.deadline(Duration::from_secs(20)));
    let first = seer::catalog(&leg, 1, 0).unwrap();
    assert_eq!(first.total, 2);
    assert_eq!(first.rows[0].code.as_str(), "55");
    assert_eq!(first.rows[0].label.as_str(), "Example site");
    let offline_leg = offline.leg(&seer::SPEC, offline.deadline(Duration::from_secs(20)));
    let next = seer::catalog(&offline_leg, 1, 1).unwrap();
    assert_eq!(next.rows[0].code.as_str(), "47");
    assert!(!next.rows[0].active);
}

#[test]
fn annual_pages_keep_model_only_values_and_original_fetch_metadata() {
    let fetch = fetch().unwrap();
    let leg = fetch.leg(&seer::SPEC, fetch.deadline(Duration::from_secs(20)));
    let query = selected(Statistic::Survival {
        interval: "5".parse().unwrap(),
    })
    .unwrap();
    let page = seer::statistics(&leg, &query, 1, 1).unwrap();
    assert_eq!(page.total, 2);
    assert_eq!(page.rows[0].year, Some(2022));
    assert!(page.rows[0].observed.is_none());
    assert_eq!(
        serde_json::to_value(&page.rows[0].modeled).unwrap()["value"],
        93.5
    );
    assert_eq!(page.metadata.fetched, 1234);
    assert_eq!(
        page.rows[0]
            .returned_selectors
            .last()
            .unwrap()
            .code
            .as_str(),
        "31"
    );
}

#[test]
fn malformed_wrapper_field_order_and_numbers_fail_the_source() {
    for kind in ["object", "double", "broken", "order", "number", "year"] {
        let fetch = changed(kind).unwrap();
        let leg = fetch.leg(&seer::SPEC, fetch.deadline(Duration::from_secs(20)));
        let query = selected(Statistic::Survival {
            interval: "5".parse().unwrap(),
        })
        .unwrap();
        assert!(
            matches!(seer::statistics(&leg, &query, 5, 0),
            Err(biomcp_fetch::RequestError::Source(e)) if e.source == biomcp_types::Source::Seer && e.reason == biomcp_types::Reason::BadReply),
            "{kind}"
        );
    }
}

#[test]
fn contradictory_returned_period_is_retained_as_a_problem() {
    let fetch = changed("mismatch").unwrap();
    let leg = fetch.leg(&seer::SPEC, fetch.deadline(Duration::from_secs(20)));
    let query = selected(Statistic::Survival {
        interval: "5".parse().unwrap(),
    })
    .unwrap();
    let page = seer::statistics(&leg, &query, 5, 0).unwrap();
    assert_eq!(
        page.rows[0]
            .returned_selectors
            .last()
            .unwrap()
            .code
            .as_str(),
        "9"
    );
    assert!(
        page.problems
            .iter()
            .any(|p| p.field.as_ref() == "returned_selectors.year_range")
    );
    assert!(
        page.problems
            .iter()
            .any(|p| p.field.as_ref() == "dataset_membership")
    );
}

#[test]
fn incidence_preserves_stage_order_and_native_counts_without_aggregating() {
    let (fetch, offline) = replay_and_offline().unwrap();
    let leg = fetch.leg(&seer::SPEC, fetch.deadline(Duration::from_secs(20)));
    let query = Selection {
        site: "47".parse().unwrap(),
        sex: "1".parse().unwrap(),
        race: "1".parse().unwrap(),
        age_range: "1".parse().unwrap(),
        dataset: "4".parse().unwrap(),
        statistic: Statistic::Incidence {
            period: "5".parse().unwrap(),
        },
    };
    let page = seer::statistics(&leg, &query, 1, 0).unwrap();
    assert_eq!(page.total, 2);
    assert_eq!(page.rows[0].observed.as_ref().unwrap().count, Some(100));
    assert!(page.rows[0].modeled.is_none());
    assert!(page.rows[0].year.is_none());
    assert_eq!(
        page.rows[0]
            .returned_selectors
            .iter()
            .find(|s| s.name.as_str() == "stage")
            .unwrap()
            .code
            .as_str(),
        "104"
    );
    let offline_leg = offline.leg(&seer::SPEC, offline.deadline(Duration::from_secs(20)));
    let terminal = seer::statistics(&offline_leg, &query, 1, 2).unwrap();
    assert_eq!(terminal.total, 2);
    assert!(terminal.rows.is_empty());
}
