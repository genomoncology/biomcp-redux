//! Temporary driver to capture reviewed public source identities once.
#![expect(
    clippy::unwrap_used,
    clippy::disallowed_methods,
    reason = "Temporary transport capture driver"
)]
use biomcp_adapters::{civic, mygene};
use biomcp_fetch::{
    BadReply, Fetch, FetchConfig, KeyUse, Mode, Quota, Rate, Reply, SourceKey, SourceSpec,
};
use std::time::Duration;
static QUOTA: Quota = Quota::new(
    SourceKey::declare("hgnc-rest"),
    Rate::per_second(1),
    None,
    1,
);
static HGNC: SourceSpec = SourceSpec {
    source: biodata::Source::Hgnc,
    base: "https://rest.genenames.org",
    key: KeyUse::None,
    quota: &QUOTA,
    deadline: Duration::from_secs(30),
};
#[derive(serde::Deserialize)]
struct Raw(serde_json::Value);
impl Reply for Raw {
    fn check(&self) -> Result<(), BadReply> {
        if self.0.is_object() {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
#[test]
#[ignore]
fn capture() {
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Record {
            dir: "/tmp/biomcp-0029-civic-capture".into(),
        },
        ..Default::default()
    })
    .unwrap();
    for (symbol, id) in [("VSIR", "64115"), ("ERBB2", "2064"), ("TP53", "7157")] {
        let leg = fetch.leg(&civic::SPEC, fetch.deadline(Duration::from_secs(30)));
        civic::genes(&leg, &[(symbol.parse().unwrap(), id.parse().unwrap())]).unwrap();
    }
    let leg = fetch.leg(&HGNC, fetch.deadline(Duration::from_secs(30)));
    let _: Raw = leg.get(&HGNC.get("/fetch/symbol/VSIR"), &[]).unwrap();
    let leg = fetch.leg(&mygene::SPEC, fetch.deadline(Duration::from_secs(30)));
    for batch in mygene::genes(&leg, &["64115".parse().unwrap()]) {
        batch.result.unwrap();
    }
}
