//! Temporary driver for bounded public source captures.
use biomcp_adapters::{gnomad, mygene};
use biomcp_fetch::{Fetch, FetchConfig, Mode, Quota, Rate, SourceKey, SourceSpec, KeyUse, Reply, BadReply};
use biodata::{Source, EntrezGeneId};
use std::time::Duration;
static QUOTA: Quota = Quota::new(SourceKey::declare("hgnc-rest-capture"), Rate::per_second(1), None, 1);
static REST: SourceSpec = SourceSpec { source: Source::Hgnc, base:"https://rest.genenames.org", key: KeyUse::None, quota:&QUOTA, deadline:Duration::from_secs(20) };
#[derive(Debug, serde::Deserialize)]
struct Raw(serde_json::Value);
impl Reply for Raw { fn check(&self) -> Result<(), BadReply> { if self.0["response"]["numFound"] == 1 { Ok(()) } else { Err(BadReply) } } }
#[test]
#[ignore]
fn capture_missing_public_identities() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies/gene/constraint/captured");
    let fetch = Fetch::open(FetchConfig { mode:Mode::Record { dir }, ..Default::default() }).unwrap();
    for symbol in ["TP53","BCR"] {
        let leg = fetch.leg(&REST, fetch.deadline(Duration::from_secs(30)));
        // HGNC's fetch route requires the symbol in the path.
        let ep = if symbol == "TP53" { REST.get("/fetch/symbol/TP53") } else { REST.get("/fetch/symbol/BCR") };
        let reply = leg.json::<Raw>(&ep, &[]).unwrap();
        assert_eq!(reply.0["response"]["numFound"], 1);
    }
    for id in [7157, 406991, 613] {
        let leg = fetch.leg(&mygene::SPEC, fetch.deadline(Duration::from_secs(30)));
        let batches = mygene::genes(&leg, &[EntrezGeneId::new(id).unwrap()]);
        assert!(batches[0].result.is_ok(), "{:?}", batches);
    }
    let leg = fetch.leg(&gnomad::SPEC, fetch.deadline(Duration::from_secs(30)));
    let result = gnomad::constraints(&leg, &["ENSG00000141510".parse().unwrap()]).unwrap();
    assert!(result[0].constraint.value.is_some());
}
