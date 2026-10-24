//! One-shot public source capture, removed after harvesting.
#![expect(clippy::unwrap_used, clippy::disallowed_methods, reason = "One-shot explicit source capture outside product code")]
use biomcp_fetch::{BadReply, Fetch, FetchConfig, KeyUse, Mode, Quota, Rate, Reply, SourceKey, SourceSpec};
use biodata::Source;
use serde::Deserialize;
use std::{collections::BTreeMap, path::PathBuf, time::Duration};
#[derive(Debug, Deserialize)]
struct Object { #[serde(flatten)] _fields: BTreeMap<String, serde_json::Value> }
impl Reply for Object { fn check(&self) -> Result<(), BadReply> { Ok(()) } }
static QUOTA: Quota = Quota::new(SourceKey::declare("hpa"), Rate::per_second(1), None, 1);
static HPA: SourceSpec = SourceSpec { source: Source::Hpa, base: "https://www.proteinatlas.org", key: KeyUse::None, quota: &QUOTA, deadline: Duration::from_secs(30) };
static HQ: Quota = Quota::new(SourceKey::declare("hgnc"), Rate::per_second(1), None, 1);
static HGNC: SourceSpec = SourceSpec { source: Source::Hgnc, base: "https://rest.genenames.org", key: KeyUse::None, quota: &HQ, deadline: Duration::from_secs(30) };
#[test]
#[ignore = "Record each missing public identity once"]
fn capture() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies/gene/expression/live");
    let fetch = Fetch::open(FetchConfig { mode: Mode::Record { dir }, ..Default::default() }).unwrap();
    let leg = fetch.leg(&HPA, fetch.deadline(Duration::from_secs(120)));
    for path in ["/ENSG00000066405.json", "/ENSG00000284190.json", "/ENSG00000141510.json"] {
        let _reply = leg.json_opt::<Object>(&HPA.get(path), &[]).unwrap();
    }
    let leg = fetch.leg(&HGNC, fetch.deadline(Duration::from_secs(60)));
    let _reply = leg.json::<Object>(&HGNC.get("/fetch/symbol/CLDN18"), &[]).unwrap();
    let leg = fetch.leg(&biomcp_adapters::mygene::SPEC, fetch.deadline(Duration::from_secs(60)));
    for batch in biomcp_adapters::mygene::genes(&leg, &["51208".parse().unwrap()]) { batch.result.unwrap(); }
    println!("CAPTURE_ROOT={}", env!("CARGO_MANIFEST_DIR"));
}
