//! Exact source request spelling crosses the real command and TLS transport.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("variant/car-projection-wire", || car().map_err(Into::into)),
        Trial::test("variant/myvariant-exact-build-wire", || myvariant().map_err(Into::into)),
    ]
}
fn car() -> Result<(), String> {
    let (output, requests) = call("reg.genome.network", car_body, &["get","variant","NM_004333.4:c.1799T>A","--disable-sources","myvariant"])?;
    let [(_, request)] = requests.as_slice() else { return Err("expected one CAR request".into()); };
    if !request.starts_with("GET /allele?hgvs=NM_004333.4%3Ac.1799T%3EA&fields=none%2B%40id%2BcommunityStandardTitle%2BgenomicAlleles%2BtranscriptAlleles.MANE%2BexternalRecords.dbSNP%2BexternalRecords.ClinVarVariations HTTP/1.1") {
        return Err(format!("CAR projection must preserve literal plus modifiers: {request}"));
    }
    if output.pointer("/result/candidates/0/identity/ids/0") != Some(&json!("CA123643")) { return Err(format!("source identity lost: {output}")); }
    Ok(())
}
fn myvariant() -> Result<(), String> {
    let (output, requests) = call("myvariant.info", myvariant_body, &["get","variant","chr7:g.140753336A>T","--build","GRCh38"])?;
    let [(_, request)] = requests.as_slice() else { return Err("expected one exact MyVariant request".into()); };
    if !request.starts_with("GET /v1/variant/chr7%3Ag.140753336A%3ET?fields=_id%2Cdbsnp.rsid%2Cdbsnp.gene.symbol%2Cclinvar.variant_id&assembly=hg38 HTTP/1.1") { return Err(format!("exact annotation endpoint/build was not sent: {request}")); }
    if output.pointer("/result/candidates/0/identity/representations/0/build") != Some(&json!("GRCh38")) { return Err(format!("source assembly lost: {output}")); }
    Ok(())
}
type Requests = Vec<(String, String)>;
fn call(host: &'static str, body: fn(&str) -> Result<String,String>, args: &[&str]) -> Result<(Value, Requests), String> {
    let stub = Stub::for_source(host, "application/json", body)?;
    let environment = Environment::new().map_err(|e| e.to_string())?;
    let ca = environment.root().join("synthetic-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    environment.config(&json!({"proxy":format!("http://{}", stub.address), "ca_bundle":ca})).map_err(|e| e.to_string())?;
    let args = args.iter().map(|s| (*s).into()).collect::<Vec<_>>();
    let output = environment.command(env!("CARGO_BIN_EXE_biomcp"), &args).output().map_err(|e| e.to_string())?;
    let value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    Ok((value, stub.finish()?))
}
fn car_body(_: &str) -> Result<String,String> {
    Ok(json!({"@id":"http://reg.genome.network/allele/CA123643","genomicAlleles":[{"chromosome":"7","referenceGenome":"GRCh38","referenceSequence":"http://reg.genome.network/refseq/RS000055","hgvs":["NC_000007.14:g.140753336A>T"],"coordinates":[{"start":140753335,"end":140753336,"referenceAllele":"A","allele":"T"}]}]}).to_string())
}
fn myvariant_body(_: &str) -> Result<String,String> {
    Ok(json!({"_id":"chr7:g.140753336A>T","dbsnp":{"rsid":"rs113488022","gene":{"symbol":"BRAF"}},"clinvar":{"variant_id":13961}}).to_string())
}
