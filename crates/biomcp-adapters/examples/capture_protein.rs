//! Capture missing source identities once through shared transport.
use biomcp_adapters::{uniprot, mygene};
use biomcp_fetch::{Fetch, FetchConfig, Mode};
use std::time::Duration;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fetch = Fetch::open(FetchConfig { mode: Mode::Record { dir: "protein-capture".into() }, ..Default::default() })?;
    for ids in [vec!["P42771", "Q8N726"], vec!["P04637"]] {
        let ids = ids.into_iter().map(str::parse).collect::<Result<Vec<_>,_>>()?;
        let leg = fetch.leg(&uniprot::SPEC, fetch.deadline(Duration::from_secs(30)));
        let proteins = uniprot::proteins(&leg, &ids)?;
        println!("{} proteins, {} problems", proteins.value.len(), proteins.problems.len());
    }
    let leg = fetch.leg(&mygene::SPEC, fetch.deadline(Duration::from_secs(30)));
    for batch in mygene::genes(&leg, &[biodata::EntrezGeneId::new(1029)?]) { batch.result?; }
    Ok(())
}
