//! Capture reviewed public Reactome mapping identities once through shared transport.
use biomcp_adapters::reactome;
use biomcp_fetch::{Fetch, FetchConfig, Mode};
use std::time::Duration;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fetch = Fetch::open(FetchConfig { mode: Mode::Record { dir: "tests/replies/gene/pathways-live".into() }, ..Default::default() })?;
    for accession in ["P15056", "P04637", "A0A000Z9Z9"] {
        let leg = fetch.leg(&reactome::SPEC, fetch.deadline(Duration::from_secs(30)));
        let mapped = reactome::pathways(&leg, &accession.parse()?)?;
        println!("{accession}: {} mappings, {} problems", mapped.value.len(), mapped.problems.len());
    }
    Ok(())
}
