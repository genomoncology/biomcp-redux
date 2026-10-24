//! Temporary capture driver for the ClinGen command identities.
use biodata::Source;
use biomcp_fetch::{
    BadReply, Fetch, FetchConfig, KeyUse, Mode, Quota, Rate, Reply, SourceKey, SourceSpec,
};
use serde::Deserialize;
use std::time::Duration;
#[derive(Deserialize)]
struct Root {
    response: Response,
}
#[derive(Deserialize)]
struct Response {
    docs: Vec<serde_json::Value>,
}
impl Reply for Root {
    fn check(&self) -> Result<(), BadReply> {
        if self.response.docs.len() == 1 {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
static QUOTA: Quota = Quota::new(
    SourceKey::declare("hgnc-rest"),
    Rate::per_second(1),
    None,
    1,
);
static SPEC: SourceSpec = SourceSpec {
    source: Source::Hgnc,
    base: "https://rest.genenames.org",
    key: KeyUse::None,
    quota: &QUOTA,
    deadline: Duration::from_secs(20),
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Record {
            dir: "/tmp/biomcp-0030-identity-capture".into(),
        },
        ..Default::default()
    })?;
    let mut ids = Vec::new();
    for name in ["MLH1", "MSH2", "MSH6", "PMS2", "EPCAM"] {
        let path = Box::leak(format!("/fetch/symbol/{name}").into_boxed_str());
        let leg = fetch.leg(&SPEC, fetch.deadline(Duration::from_secs(20)));
        let root = leg.json::<Root>(&SPEC.get(path), &[])?;
        let id = root.response.docs[0]["entrez_id"]
            .as_str()
            .ok_or("no Entrez")?
            .parse()?;
        ids.push(id);
    }
    let leg = fetch.leg(
        &biomcp_adapters::mygene::SPEC,
        fetch.deadline(Duration::from_secs(20)),
    );
    let batches = biomcp_adapters::mygene::genes(&leg, &ids);
    for batch in batches {
        let rows = batch.result?;
        println!("captured {} identities", rows.len());
    }
    Ok(())
}
