//! Population source requests cross the public command and real TLS transport.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

#[path = "variant_population_ids.rs"]
mod ids;
pub(super) fn trials() -> Vec<Trial> {
    ids::trials()
}

pub(super) fn population_body(request: &str) -> Result<String, String> {
    if request.starts_with("GET /allele/") {
        return Ok(json!(
            {"@id":"http://reg.genome.network/allele/CA123643","genomicAlleles":[{"chromosome":"7","referenceGenome":"GRCh38",
            "referenceSequence":"http://reg.genome.network/refseq/RS000055","hgvs":["NC_000007.14:g.140753336A>T"],
            "coordinates":[{"start":140753335,"end":140753336,"referenceAllele":"A","allele":"T"}]}]}
        )
        .to_string());
    }
    Ok(json!(
        {"data":{"variant":{"variant_id":"7-140753336-A-T","exome":{"ac":2,"an":1460618,"filters":[],"populations":[{"id":"nfe","ac":1,
        "an":1111098},{"id":"sas","ac":1,"an":86244}]},"genome":null}}}
    )
    .to_string())
}

pub(super) fn environment(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    stub.retain(&env);
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
fn execute(env: &Environment, args: &[&str]) -> Result<Value, String> {
    let args = args.iter().map(|s| (*s).into()).collect::<Vec<_>>();
    let out = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())
}
