//! Population source requests cross the public command and real TLS transport.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("variant/population-source-given-grch38-snv", || {
            population().map_err(Into::into)
        }),
        Trial::test("variant/gwas-v2-valid-zero", || {
            gwas_zero().map_err(Into::into)
        }),
    ]
}
fn gwas_zero() -> Result<(), String> {
    let stub = Stub::for_source("www.ebi.ac.uk", "application/json", empty_gwas)?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    let args = ["get", "variant", "rs113488022", "gwas"].map(Into::into);
    let out = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
    if !out.status.success()
        || value.pointer("/result/sections/gwas/state") != Some(&json!("empty"))
    {
        return Err(format!(
            "valid v2 catalog zero was not source-local empty: {value}"
        ));
    }
    let requests = stub.finish()?;
    if requests.len() != 1
        || !requests[0].1.starts_with(
            "GET /gwas/rest/api/v2/associations?rs_id=rs113488022&page=0&size=3 HTTP/1.1",
        )
    {
        return Err(format!("expected exact rsID v2 request only: {requests:?}"));
    }
    Ok(())
}
fn empty_gwas(_: &str) -> Result<String, String> {
    Ok(json!({"_links":{"self":{"href":"https://www.ebi.ac.uk/gwas/rest/api/v2/associations?rs_id=rs113488022&page=0&size=3"}},"page":{"size":3,"totalElements":0,"totalPages":0,"number":0}}).to_string())
}

fn population() -> Result<(), String> {
    let stub = Stub::for_sources(
        &["reg.genome.network", "gnomad.broadinstitute.org"],
        "application/json",
        population_body,
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    let args = [
        "variant",
        "population",
        "CA123643",
        "--detail",
        "--limit",
        "1",
    ]
    .map(Into::into);
    let out = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
    if !out.status.success()
        || value.pointer("/result/population/data/exome/allele_count") != Some(&json!(2))
        || value.pointer("/result/population/data/exome/allele_number") != Some(&json!(1460618))
        || value.pointer("/result/population/data/genome").is_some()
        || value
            .pointer("/result/population/data/exome/frequency")
            .is_some()
    {
        return Err(format!(
            "source counts/null dataset/no-derived-frequency lost: {value}"
        ));
    }
    let requests = stub.finish()?;
    if requests.len() != 2
        || !requests[1].1.starts_with("POST /api HTTP/1.1")
        || !requests[1].1.contains("gnomad_r4")
        || !requests[1].1.contains("7-140753336-A-T")
        || requests.iter().any(|(_, r)| r.contains("myvariant"))
    {
        return Err(format!(
            "source-given exact SNV wire projection was not used: {requests:?}"
        ));
    }
    Ok(())
}
fn population_body(request: &str) -> Result<String, String> {
    if request.starts_with("GET /allele/") {
        return Ok(json!({"@id":"http://reg.genome.network/allele/CA123643","genomicAlleles":[{"chromosome":"7","referenceGenome":"GRCh38","referenceSequence":"http://reg.genome.network/refseq/RS000055","hgvs":["NC_000007.14:g.140753336A>T"],"coordinates":[{"start":140753335,"end":140753336,"referenceAllele":"A","allele":"T"}]}]}).to_string());
    }
    Ok(json!({"data":{"variant":{"variant_id":"7-140753336-A-T","exome":{"ac":2,"an":1460618,"filters":[],"populations":[{"id":"nfe","ac":1,"an":1111098},{"id":"sas","ac":1,"an":86244}]},"genome":null}}}).to_string())
}
