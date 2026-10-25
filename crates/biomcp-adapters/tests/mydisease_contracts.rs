//! Exact annotation identity is checked before transport admission.
use biomcp_adapters::mydisease;
use biomcp_fetch::{Fetch, FetchConfig, Mode, RequestError};
use std::time::Duration;

#[test]
fn unsupported_namespaces_fail_before_replay_or_cache() {
    let directory = tempfile::tempdir().unwrap();
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay { dir: directory.path().into() },
        ..Default::default()
    }).unwrap();
    let leg = fetch.leg(&mydisease::SPEC, fetch.deadline(Duration::from_secs(20)));
    for id in ["NCIT:C3512", "EFO:0000571", "HP:0002861", "MONDO:bad"] {
        let id = id.parse().unwrap();
        assert!(matches!(mydisease::get(&leg, &id), Err(RequestError::Input(_))));
    }
}
