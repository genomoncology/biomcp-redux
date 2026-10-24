//! Population source requests cross the public command and real TLS transport.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("variant/gwas-v2-valid-zero", || gwas_zero().map_err(Into::into))]
}
fn gwas_zero() -> Result<(), String> {
    let stub = Stub::for_source("www.ebi.ac.uk", "application/json", empty_gwas)?;
    let env = Environment::new().map_err(|e|e.to_string())?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca,&stub.certificate).map_err(|e|e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca})).map_err(|e|e.to_string())?;
    let args = ["get","variant","rs113488022","gwas"].map(Into::into);
    let out = env.command(env!("CARGO_BIN_EXE_biomcp"),&args).output().map_err(|e|e.to_string())?;
    let value:Value = serde_json::from_slice(&out.stdout).map_err(|e|e.to_string())?;
    if !out.status.success() || value.pointer("/result/sections/gwas/state")!=Some(&json!("empty")) {
        return Err(format!("valid v2 catalog zero was not source-local empty: {value}"));
    }
    let requests=stub.finish()?;
    if requests.len()!=1 || !requests[0].1.starts_with("GET /gwas/rest/api/v2/associations?rs_id=rs113488022&page=0&size=10 HTTP/1.1") {
        return Err(format!("expected exact rsID v2 request only: {requests:?}"));
    }
    Ok(())
}
fn empty_gwas(_: &str)->Result<String,String> {
    Ok(json!({"_links":{"self":{"href":"https://www.ebi.ac.uk/gwas/rest/api/v2/associations?rs_id=rs113488022&page=0&size=10"}},"page":{"size":10,"totalElements":0,"totalPages":0,"number":0}}).to_string())
}
