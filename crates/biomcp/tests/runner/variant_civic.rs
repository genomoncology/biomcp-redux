//! CIViC exact membership crosses the real CLI, source policy, POST body and TLS cache.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "variant/civic-exact-profile-membership",
        || exact().map_err(Into::into),
    )]
}
fn configure(environment: &Environment, stub: &Stub) -> Result<(), String> {
    let ca = environment.root().join("synthetic-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    environment
        .config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())
}
fn run(environment: &Environment, args: &[&str]) -> Result<Value, String> {
    let args = args.iter().map(|a| (*a).into()).collect::<Vec<_>>();
    let out = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    serde_json::from_slice(&out.stdout)
        .map_err(|e| format!("{e}: {}", String::from_utf8_lossy(&out.stdout)))
}
fn exact() -> Result<(), String> {
    let environment = Environment::new().map_err(|e| e.to_string())?;
    let car = Stub::for_source("reg.genome.network", "application/json", car_body)?;
    configure(&environment, &car)?;
    let value = run(
        &environment,
        &[
            "get",
            "variant",
            "CA123643",
            "--disable-sources",
            "myvariant",
        ],
    )?;
    if value.pointer("/result/candidates/0/identity/ids/0") != Some(&json!("CA123643")) {
        return Err(format!("identity prewarm failed: {value}"));
    }
    if car.finish()?.len() != 1 {
        return Err("identity prewarm request count".into());
    }
    let civic = Stub::for_source("civicdb.org", "application/json", civic_body)?;
    configure(&environment, &civic)?;
    let value = run(
        &environment,
        &[
            "get",
            "variant",
            "CA123643",
            "civic",
            "--disable-sources",
            "myvariant",
        ],
    )?;
    if value.pointer("/result/candidates/0/sections/civic/data/profiles/0/membership")
        != Some(&json!("exact"))
    {
        return Err(format!("exact source profile not retained: {value}"));
    }
    let base = "/result/candidates/0/sections/civic/data/profiles/0";
    for (path, expected) in [
        ("/statements/0/statement/subject/level", json!("protein")),
        (
            "/statements/0/statement/proposition",
            json!("therapeutic_response"),
        ),
        (
            "/statements/0/statement/predicate",
            json!("predicts_resistance_to"),
        ),
        ("/statements/0/statement/direction", json!("disputes")),
        ("/statements/0/direction", json!("DOES_NOT_SUPPORT")),
        (
            "/statements/1/statement/classification/coding/code",
            json!("tier_i"),
        ),
        (
            "/statements/1/statement/strength/coding/code",
            json!("level_a"),
        ),
        ("/evidence_page/next/civic_profile", json!(12)),
        ("/evidence_page/next/civic_kind", json!("evidence")),
        ("/assertion_page/next/civic_kind", json!("assertions")),
    ] {
        if value.pointer(&format!("{base}{path}")) != Some(&expected) {
            return Err(format!("source claim changed at {path}: {value}"));
        }
    }
    let requests = civic.finish()?;
    let [(_, request)] = requests.as_slice() else {
        return Err("expected one native CIViC request".into());
    };
    let (_, body) = request.split_once("\r\n\r\n").ok_or("missing POST body")?;
    let body: Value = serde_json::from_str(body).map_err(|e| e.to_string())?;
    if !request.starts_with("POST /api/graphql HTTP/1.1")
        || body.pointer("/variables/allele") != Some(&json!("CA123643"))
        || !body
            .get("query")
            .and_then(Value::as_str)
            .is_some_and(|q| q.contains("alleleRegistryId"))
    {
        return Err(format!(
            "CIViC request did not use checked allele membership: {body}"
        ));
    }
    Ok(())
}
fn car_body(_: &str) -> Result<String, String> {
    Ok(json!({"@id":"http://reg.genome.network/allele/CA123643","genomicAlleles":[{"chromosome":"7","referenceGenome":"GRCh38","hgvs":["NC_000007.14:g.140753336A>T"],"coordinates":[{"start":140753335,"end":140753336,"referenceAllele":"A","allele":"T"}]}]}).to_string())
}
fn civic_body(_: &str) -> Result<String, String> {
    let evidence = json!({"totalCount":2,"nodes":[{"id":1409,"status":"ACCEPTED","evidenceType":"PREDICTIVE","evidenceLevel":"A","evidenceDirection":"DOES_NOT_SUPPORT","significance":"RESISTANCE","disease":{"name":"melanoma","doid":"1909"},"therapies":[{"name":"dabrafenib"}],"source":{"citationId":"25741868","sourceType":"PUBMED"}}],"pageInfo":{"hasNextPage":true,"endCursor":"synthetic-e1"}});
    let assertions = json!({"totalCount":2,"nodes":[{"id":7,"status":"ACCEPTED","assertionType":"PREDICTIVE","assertionDirection":"SUPPORTS","significance":"SENSITIVITYRESPONSE","ampLevel":"TIER_I_LEVEL_A","disease":{"name":"melanoma","doid":"1909"},"therapies":[{"name":"dabrafenib"},{"name":"trametinib"}],"variantOrigin":"SOMATIC"}],"pageInfo":{"hasNextPage":true,"endCursor":"synthetic-a1"}});
    Ok(json!({"data":{"molecularProfiles":{"totalCount":1,"nodes":[{"id":12,"name":"BRAF V600E","link":"/molecular-profiles/12","isComplex":false,"variants":[{"id":12,"name":"V600E","link":"/variants/12","alleleRegistryId":"CA123643","hgvsDescriptions":["NP_004324.2:p.Val600Glu"],"feature":{"name":"BRAF"}}],"evidenceItems":evidence,"assertions":assertions}],"pageInfo":{"hasNextPage":false,"endCursor":null}}}}).to_string())
}
