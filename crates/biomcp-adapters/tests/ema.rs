//! Native EMA boundaries using a deliberately synthetic local document.

use biomcp_adapters::ema;
use biomcp_fetch::{Fetch, FetchConfig, Mode, ReplayCache};
use std::{path::PathBuf, time::Duration};
#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "Mutate isolated synthetic recordings through the public sync boundary"
)]
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
