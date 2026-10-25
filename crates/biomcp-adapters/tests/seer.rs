//! Temporary public source contracts before disease command integration.
use biomcp_adapters::seer::{self, Selection, Statistic};
use biomcp_fetch::{Fetch, FetchConfig, Mode};
use std::time::Duration;

fn fetch() -> Fetch {
    Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../tests/replies/disease/seer-synthetic"),
        },
        ..Default::default()
    })
    .unwrap()
}

fn selected(statistic: Statistic) -> Selection {
    Selection {
        site: "55".parse().unwrap(),
        sex: "3".parse().unwrap(),
        race: "1".parse().unwrap(),
        age_range: "1".parse().unwrap(),
        dataset: "4".parse().unwrap(),
        statistic,
    }
}

#[test]
fn catalog_pages_preserve_native_order_labels_and_active_state() {
    let fetch = fetch();
    let leg = fetch.leg(&seer::SPEC, fetch.deadline(Duration::from_secs(20)));
    let first = seer::catalog(&leg, 1, 0).unwrap();
    assert_eq!(first.total, 2);
    assert_eq!(first.rows[0].code.as_str(), "55");
    assert_eq!(first.rows[0].label.as_str(), "Example site");
    let next = seer::catalog(&leg, 1, 1).unwrap();
    assert_eq!(next.rows[0].code.as_str(), "47");
    assert!(!next.rows[0].active);
}

#[test]
fn annual_pages_keep_model_only_values_and_original_fetch_metadata() {
    let fetch = fetch();
    let leg = fetch.leg(&seer::SPEC, fetch.deadline(Duration::from_secs(20)));
    let query = selected(Statistic::Survival {
        interval: "5".parse().unwrap(),
    });
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
