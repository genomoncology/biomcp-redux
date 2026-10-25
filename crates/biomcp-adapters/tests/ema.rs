//! Native EMA boundaries using a deliberately synthetic local document.

use biomcp_adapters::ema;
use biomcp_fetch::{Fetch, FetchConfig, Mode, ReplayCache};
use std::{path::PathBuf, time::Duration};
#[test]
fn explicit_sync_keeps_tibsovo_candidates_and_native_regulatory_roles_separate() {
    let cache = ReplayCache::temporary().unwrap();
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../tests/replies/drug/ema-synthetic"),
        },
        replay_cache: Some(cache),
        ..Default::default()
    })
    .unwrap();
    let leg = fetch
        .leg(&ema::SPEC, fetch.deadline(Duration::from_secs(10)))
        .fresh(true);
    assert!(ema::search(&leg, "Tibsovo").is_err());
    assert_eq!(ema::sync(&fetch).unwrap().rows, 6);
    let candidates = ema::search(&leg, "Tibsovo").unwrap();
    assert_eq!(candidates.len(), 2);
    assert_ne!(candidates[0].id, candidates[1].id);
    let id = "EMEA/H/C/004124".parse().unwrap();
    let card = serde_json::to_value(ema::card(&leg, &id).unwrap().unwrap()).unwrap();
    assert!(card.get("status").is_none());
    assert!(card.get("dates").is_none());
    let section = ema::regulatory(&leg, &id).unwrap().unwrap();
    assert_eq!(section.identity.id, id);
    let biomcp_types::Section::Data { data: records, .. } = section.regulatory else { panic!("missing regulatory facts") };
    assert_eq!(records.version.unwrap().as_str(), "2026-10-24T18:22:32Z");
    let event = &records.records[0];
    assert!(event.jurisdiction.is_none());
    assert!(
        event
            .dates
            .iter()
            .any(|date| date.role.as_str() == "marketing_authorisation_date"
                && date.date.to_string() == "2016-02-01")
    );
    let withdrawn = ema::regulatory(&leg, &"EMEA/H/C/005621".parse().unwrap())
        .unwrap()
        .unwrap();
    let biomcp_types::Section::Data { data: withdrawn, .. } = withdrawn.regulatory else { panic!("missing withdrawn application") };
    assert_eq!(
        withdrawn.records[0].status.as_ref().unwrap().as_str(),
        "Application withdrawn"
    );
    assert_eq!(withdrawn.records[0].record.to_string(), "EMEA/H/C/005621");
    assert!(
        ema::card(&leg, &"EMEA/H/C/999999".parse().unwrap())
            .unwrap()
            .is_none()
    );
}
#[test]
#[expect(clippy::disallowed_methods, reason = "Mutate isolated synthetic recordings through the public sync boundary")]
fn malformed_native_metadata_dates_and_duplicate_ids_never_replace_usable_medicines() {
    let original = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/replies/drug/ema-synthetic/ema");
    let fixture = std::fs::read_dir(original)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let capture: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&fixture).unwrap()).unwrap();
    let body: serde_json::Value =
        serde_json::from_str(capture["exchanges"][0]["body"]["value"].as_str().unwrap()).unwrap();
    for fault in ["total", "duplicate", "date", "metadata"] {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("ema");
        std::fs::create_dir(&source).unwrap();
        let path = source.join(fixture.file_name().unwrap());
        std::fs::copy(&fixture, &path).unwrap();
        let cache = ReplayCache::temporary().unwrap();
        let open = || {
            Fetch::open(FetchConfig {
                mode: Mode::Replay {
                    dir: dir.path().into(),
                },
                replay_cache: Some(cache.clone()),
                ..Default::default()
            })
            .unwrap()
        };
        let initial = open();
        ema::sync(&initial).unwrap();
        let mut bad = body.clone();
        match fault {
            "total" => bad["meta"]["total_records"] = 5.into(),
            "duplicate" => {
                bad["data"][1]["ema_product_number"] = bad["data"][0]["ema_product_number"].clone()
            }
            "date" => bad["data"][1]["marketing_authorisation_date"] = "31/02/2016".into(),
            _ => bad["meta"]["timestamp"] = "".into(),
        }
        let mut changed = capture.clone();
        changed["exchanges"][0]["body"]["value"] = bad.to_string().into();
        std::fs::write(path, serde_json::to_vec(&changed).unwrap()).unwrap();
        assert!(ema::sync(&open()).is_err(), "accepted {fault}");
        let leg = initial.leg(&ema::SPEC, initial.deadline(Duration::from_secs(10)));
        assert_eq!(ema::search(&leg, "Tibsovo").unwrap().len(), 2);
    }
}
