//! Exact source request spelling crosses the real command and TLS transport.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("variant/native-candidate-walk-is-warm", || {
            candidate_walk().map_err(Into::into)
        }),
        Trial::test("variant/car-projection-wire", || car().map_err(Into::into)),
        Trial::test("variant/myvariant-exact-build-wire", || {
            myvariant().map_err(Into::into)
        }),
    ]
}
fn car() -> Result<(), String> {
    let (output, requests) = call(
        "reg.genome.network",
        car_body,
        &[
            "get",
            "variant",
            "NM_004333.4:c.1799T>A",
            "--disable-sources",
            "myvariant",
        ],
    )?;
    let [(_, request)] = requests.as_slice() else {
        return Err("expected one CAR request".into());
    };
    if !request.starts_with("GET /allele?hgvs=NM_004333.4%3Ac.1799T%3EA&fields=none%2B%40id%2BcommunityStandardTitle%2BgenomicAlleles%2BtranscriptAlleles.MANE%2BexternalRecords.dbSNP%2BexternalRecords.ClinVarVariations HTTP/1.1") {
        return Err(format!("CAR projection must preserve literal plus modifiers: {request}"));
    }
    if output.pointer("/result/candidates/0/identity/ids/0") != Some(&json!("CA123643")) {
        return Err(format!("source identity lost: {output}"));
    }
    Ok(())
}
fn myvariant() -> Result<(), String> {
    let (output, requests) = call(
        "myvariant.info",
        myvariant_body,
        &["get", "variant", "chr7:g.140753336A>T", "--build", "GRCh38"],
    )?;
    let [(_, request)] = requests.as_slice() else {
        return Err("expected one exact MyVariant request".into());
    };
    if !request.starts_with("GET /v1/variant/chr7%3Ag.140753336A%3ET?fields=_id%2Cdbsnp.rsid%2Cdbsnp.gene.symbol%2Cclinvar.variant_id&assembly=hg38 HTTP/1.1") { return Err(format!("exact annotation endpoint/build was not sent: {request}")); }
    if output.pointer("/result/candidates/0/identity/representations/0/build")
        != Some(&json!("GRCh38"))
    {
        return Err(format!("source assembly lost: {output}"));
    }
    Ok(())
}
type Requests = Vec<(String, String)>;
fn call(
    host: &'static str,
    body: fn(&str) -> Result<String, String>,
    args: &[&str],
) -> Result<(Value, Requests), String> {
    let stub = Stub::for_source(host, "application/json", body)?;
    let environment = Environment::new().map_err(|e| e.to_string())?;
    let ca = environment.root().join("synthetic-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    environment
        .config(&json!({"proxy":format!("http://{}", stub.address), "ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    let args = args.iter().map(|s| (*s).into()).collect::<Vec<_>>();
    let output = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    Ok((value, stub.finish()?))
}
fn car_body(_: &str) -> Result<String, String> {
    Ok(json!({"@id":"http://reg.genome.network/allele/CA123643","genomicAlleles":[{"chromosome":"7","referenceGenome":"GRCh38","referenceSequence":"http://reg.genome.network/refseq/RS000055","hgvs":["NC_000007.14:g.140753336A>T"],"coordinates":[{"start":140753335,"end":140753336,"referenceAllele":"A","allele":"T"}]}]}).to_string())
}
fn myvariant_body(_: &str) -> Result<String, String> {
    Ok(json!({"_id":"chr7:g.140753336A>T","dbsnp":{"rsid":"rs113488022","gene":{"symbol":"BRAF"}},"clinvar":{"variant_id":13961}}).to_string())
}
fn candidate_walk() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", candidates_body)?;
    let environment = Environment::new().map_err(|e| e.to_string())?;
    let ca = environment.root().join("synthetic-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    environment
        .config(&json!({"proxy":format!("http://{}", stub.address), "ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    let mut args = ["get", "variant", "rs113488022", "--limit", "1"]
        .iter()
        .map(|s| (*s).into())
        .collect::<Vec<_>>();
    let mut ids = Vec::new();
    for offset in 0..3 {
        let output = environment
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err("candidate page command failed".into());
        }
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if value.pointer("/page/offset") != Some(&json!(offset))
            || value.pointer("/page/returned") != Some(&json!(1))
            || value.pointer("/page/total") != Some(&json!(3))
        {
            return Err(format!("candidate page lost its bounds: {value}"));
        }
        ids.push(
            value
                .pointer("/result/candidates/0/record/id")
                .cloned()
                .ok_or("candidate ID absent")?,
        );
        let next = value.get("next")
            .and_then(Value::as_array)
            .ok_or("candidate selection absent")?;
        if !next.iter().any(|n| n.get("kind").and_then(Value::as_str) == Some("disambiguate")) {
            return Err("candidate selection action absent".into());
        }
        if offset < 2 {
            let page = next
                .iter()
                .find(|n| n.get("kind").and_then(Value::as_str) == Some("page"))
                .ok_or("next candidate page absent")?;
            args = serde_json::from_value(page.get("args").cloned().ok_or("page args absent")?).map_err(|e| e.to_string())?;
        } else if next.iter().any(|n| n.get("kind").and_then(Value::as_str) == Some("page")) {
            return Err("unexpected fourth page".into());
        }
    }
    if ids
        != [
            json!("chr7:g.140453136A>G"),
            json!("chr7:g.140453136A>T"),
            json!("chr7:g.140453136A>C"),
        ]
    {
        return Err("candidate walk changed native order or dropped an allele".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("warm candidate pages made another HTTP request".into());
    }
    Ok(())
}
fn candidates_body(_: &str) -> Result<String, String> {
    Ok(json!({"total":3,"hits":[{"_id":"chr7:g.140453136A>G","dbsnp":{"rsid":"rs113488022"}},{"_id":"chr7:g.140453136A>T","dbsnp":{"rsid":"rs113488022"}},{"_id":"chr7:g.140453136A>C","dbsnp":{"rsid":"rs113488022"}}]}).to_string())
}
