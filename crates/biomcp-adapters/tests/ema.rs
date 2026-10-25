//! Native EMA boundaries using a deliberately synthetic local document.
#![expect(clippy::unwrap_used, reason = "Test setup and contract assertions")]
use biomcp_adapters::ema;
use biomcp_fetch::{Fetch, FetchConfig, Mode, ReplayCache};
use std::{path::PathBuf, time::Duration};
#[test]
fn explicit_sync_keeps_tibsovo_candidates_and_native_regulatory_roles_separate() {
    let cache = ReplayCache::temporary().unwrap();
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay { dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies/drug/ema-synthetic") },
        replay_cache: Some(cache), ..Default::default()
    }).unwrap();
    let leg = fetch.leg(&ema::SPEC, fetch.deadline(Duration::from_secs(10))).fresh(true);
    assert!(ema::search(&leg, "Tibsovo").is_err());
    assert_eq!(ema::sync(&fetch).unwrap().rows, 6);
    let candidates = ema::search(&leg, "Tibsovo").unwrap();
    assert_eq!(candidates.len(), 2);
    assert_ne!(candidates[0].id, candidates[1].id);
    let id = "EMEA/H/C/004124".parse().unwrap();
    let card = serde_json::to_value(ema::card(&leg, &id).unwrap().unwrap()).unwrap();
    assert!(card.get("status").is_none());
    assert!(card.get("dates").is_none());
    let records = ema::regulatory(&leg, &id).unwrap().unwrap();
    assert_eq!(records.version.unwrap().as_str(), "2026-10-24T18:22:32Z");
    let event = &records.records[0];
    assert!(event.jurisdiction.is_none());
    assert!(event.dates.iter().any(|date| date.role.as_str() == "marketing_authorisation_date" && date.date.to_string() == "2016-02-01"));
    let withdrawn = ema::regulatory(&leg, &"EMEA/H/C/005621".parse().unwrap()).unwrap().unwrap();
    assert_eq!(withdrawn.records[0].status.as_ref().unwrap().as_str(), "Application withdrawn");
    assert_eq!(withdrawn.records[0].record.to_string(), "EMEA/H/C/005621");
    assert!(ema::card(&leg, &"EMEA/H/C/999999".parse().unwrap()).unwrap().is_none());
}
