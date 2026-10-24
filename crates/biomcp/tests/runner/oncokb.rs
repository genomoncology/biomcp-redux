//! The keyed annotation boundary crosses the actual CLI and synthetic TLS peer.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
const KEY: &str = "synthetic-oncokb-variant-key";
const INPUT: &str = "chr7:g.140753336A>T";
const SECTION: &str = "/result/candidates/0/sections/oncokb";
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("variant/oncokb-key-policy-wire", || {
        wire().map_err(Into::into)
    })]
}
fn args() -> Vec<String> {
    [
        "get",
        "variant",
        INPUT,
        "oncokb",
        "--build",
        "GRCh38",
        "--tumor-type",
        "Synthetic tumor",
        "--section-limit",
        "50",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
fn invoke(env: &Environment, args: &[String], exit: i32) -> Result<Value, String> {
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), args)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) || !output.stderr.is_empty() {
        return Err(format!(
            "wrong annotation exit {:?}, expected {exit}",
            output.status.code()
        ));
    }
    if [output.stdout.as_slice(), output.stderr.as_slice()]
        .concat()
        .windows(KEY.len())
        .any(|b| b == KEY.as_bytes())
    {
        return Err("annotation credential leaked".into());
    }
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}
fn wire() -> Result<(), String> {
    let stub = Stub::for_sources(
        &["myvariant.info", "www.oncokb.org"],
        "application/json",
        reply,
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    let mut config = json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca});
    env.config(&config).map_err(|e| e.to_string())?;
    let skipped = invoke(&env, &args(), 0)?;
    if skipped.pointer(&format!("{SECTION}/state")) != Some(&json!("skipped"))
        || skipped.pointer(&format!("{SECTION}/reason")) != Some(&json!("no_key"))
        || !skipped
            .get("next")
            .and_then(Value::as_array)
            .is_some_and(|rows| rows.iter().any(|r| r.get("kind") == Some(&json!("key"))))
    {
        return Err(format!(
            "no-key annotation did not skip with setup: {skipped}"
        ));
    }
    config["keys"] = json!({"oncokb_api_key":KEY});
    env.config(&config).map_err(|e| e.to_string())?;
    let mut recorded = args();
    let directory = env.root().join("recorded");
    recorded.extend(["--record".into(), directory.to_string_lossy().into()]);
    let data = invoke(&env, &recorded, 0)?;
    if data.pointer(&format!("{SECTION}/state")) != Some(&json!("data")) {
        return Err(format!("keyed annotation missing: {data}"));
    }
    let requests = stub.finish()?;
    let keyed = requests
        .iter()
        .filter(|(_, r)| r.starts_with("GET /api/v1/annotate/"))
        .collect::<Vec<_>>();
    if keyed.len() != 1 {
        return Err("OncoKB request without key or redundant request".into());
    }
    let request = &keyed.first().ok_or("missing annotation request")?.1;
    if !request.starts_with("GET /api/v1/annotate/mutations/byHGVSg?hgvsg=7%3Ag.140753336A%3ET&referenceGenome=GRCh38&tumorType=Synthetic%20tumor HTTP/1.1") || !request.lines().any(|l|l.eq_ignore_ascii_case(&format!("authorization: Bearer {KEY}"))) {return Err(format!("source request context/header mismatch: {}", request.lines().next().unwrap_or_default()));}
    for file in fs::read_dir(directory.join("oncokb")).map_err(|e| e.to_string())? {
        let bytes = fs::read(file.map_err(|e| e.to_string())?.path()).map_err(|e| e.to_string())?;
        if bytes.windows(KEY.len()).any(|b| b == KEY.as_bytes()) {
            return Err("credential in recording".into());
        }
    }
    Ok(())
}
fn reply(request: &str) -> Result<String, String> {
    if request.starts_with("GET /v1/variant/") {
        return Ok(json!({"_id":INPUT,"dbsnp":{"rsid":"rs113488022","gene":{"symbol":"BRAF"}},"clinvar":{"variant_id":13961}}).to_string());
    }
    Ok(json!({"query":{"hgvs":"7:g.140753336A>T","referenceGenome":"GRCh38","tumorType":"Synthetic tumor","hugoSymbol":"BRAF","alteration":"V600E"},"variantExist":false,"alleleExist":false,"oncogenic":"Oncogenic","mutationEffect":{"knownEffect":"Neutral","description":"Synthetic mutation effect.","citations":{"pmids":["17611497"]}},"variantSummary":" Synthetic source text Ω ","dataVersion":"synthetic-v1","lastUpdate":"10/24/2026","treatments":[{"level":"LEVEL_1","alterations":["V600E","V600K"],"drugs":[{"drugName":"Synthetic drug","ncitCode":"C0000"}],"pmids":["17611497"]}],"diagnosticImplications":[],"prognosticImplications":[]}).to_string())
}
