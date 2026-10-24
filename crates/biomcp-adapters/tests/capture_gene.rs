//! Temporary transport capture driver, removed after collecting missing request identities.
#![expect(
    clippy::unwrap_used,
    clippy::disallowed_methods,
    reason = "One-time public fixture capture"
)]
#[test]
fn capture_missing_gene_requests() {
    use biomcp_adapters::mygene;
    use biomcp_fetch::{Fetch, FetchConfig, Mode};
    use std::time::Duration;
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/replies/gene/missing");
    std::fs::create_dir_all(dir.parent().unwrap()).unwrap();
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Record { dir },
        ..Default::default()
    })
    .unwrap();
    for id in [673, 3845, 2064, 4297, 54855] {
        let leg = fetch.leg(&mygene::SPEC, fetch.deadline(Duration::from_secs(20)));
        let values = mygene::genes(&leg, &[biodata::EntrezGeneId::new(id).unwrap()]);
        for batch in values {
            batch.result.unwrap();
        }
    }
}
