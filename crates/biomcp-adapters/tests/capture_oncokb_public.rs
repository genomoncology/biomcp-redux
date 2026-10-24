//! Temporary public-source capture driver, removed after recording.
#[test]
#[ignore]
#[expect(clippy::unwrap_used, reason = "Capture driver fails visibly")]
fn capture_public() {
    use biomcp_fetch::{BadReply, Fetch, FetchConfig, KeyUse, Mode, Quota, Rate, Reply, SourceKey, SourceSpec};
    use std::time::Duration;
    #[derive(serde::Deserialize)]
    struct Public(serde_json::Value);
    impl Reply for Public {
        fn check(&self) -> Result<(), BadReply> {
            if self.0.pointer("/response/numFound").and_then(serde_json::Value::as_u64) == Some(1) { Ok(()) } else { Err(BadReply) }
        }
    }
    static QUOTA: Quota = Quota::new(SourceKey::declare("hgnc-rest-capture"), Rate::per_second(1), None, 1);
    static HGNC: SourceSpec = SourceSpec { source: biodata::Source::Hgnc, base: "https://rest.genenames.org", key: KeyUse::None, quota: &QUOTA, deadline: Duration::from_secs(20) };
    let fetch = Fetch::open(FetchConfig { mode: Mode::Record { dir: "/tmp/biomcp-0032-public-not".into() }, ..FetchConfig::default() }).unwrap();
    let leg = fetch.leg(&HGNC, fetch.deadline(Duration::from_secs(20)));
    let reply: Public = leg.json(&HGNC.get("/fetch/symbol/NOTCH1"), &[]).unwrap();
    let id = reply.0.pointer("/response/docs/0/entrez_id").and_then(serde_json::Value::as_str).unwrap().parse().unwrap();
    let leg = fetch.leg(&biomcp_adapters::mygene::SPEC, fetch.deadline(Duration::from_secs(20)));
    assert!(biomcp_adapters::mygene::genes(&leg, &[id]).into_iter().all(|batch| batch.result.is_ok()));
}
