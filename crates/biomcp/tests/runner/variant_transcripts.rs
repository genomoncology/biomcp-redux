//! Feature: variant-search. Source-paired rows expose numbering differences through the CLI.
use super::super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test(
            "variant-search/transcript-projection-and-component-policy",
            || transcripts().map_err(Into::into),
        ),
        Trial::test("variant/source-coding-transcript-match", || {
            coding().map_err(Into::into)
        }),
    ]
}
fn transcripts() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", reply)?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("synthetic-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    presentation_and_policy(&env)?;
    let requests = stub.finish()?;
    if !requests.iter().any(|(_, request)| {
        request.starts_with("GET /v1/variant/") && request.contains("snpeff.ann")
    }) || !requests.iter().any(|(_, request)| {
        request.starts_with("GET /v1/query?") && !request.contains("snpeff.ann")
    }) {
        return Err("transcript projection did not follow component policy".into());
    }
    Ok(())
}
fn presentation_and_policy(env: &Environment) -> Result<(), String> {
    let args = [
        "--format",
        "md",
        "get",
        "variant",
        "chr5:g.118860951A>G",
        "--build",
        "GRCh37",
    ]
    .into_iter()
    .map(Into::into)
    .collect::<Vec<_>>();
    let markdown = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let markdown = String::from_utf8_lossy(&markdown.stdout);
    if !markdown.contains("001199291&#46;2") || !markdown.contains("p&#46;His515Arg") {
        return Err(format!("readable card lost transcript rows: {markdown}"));
    }
    let excluded = run(
        env,
        &[
            "--disable-sources",
            "restricted",
            "search",
            "variant",
            "--gene",
            "HSD17B4",
            "--protein",
            "His540Arg",
        ],
    )?;
    if excluded.pointer("/result/items/0/transcripts").is_some() {
        return Err(format!("excluded annotation leaked: {excluded}"));
    }
    Ok(())
}
// Feature: variant-identity. A coding request matches only the identical source transcript version.
fn coding() -> Result<(), String> {
    let stub = Stub::for_sources(
        &["myvariant.info", "reg.genome.network"],
        |_| "application/json",
        reply,
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("synthetic-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    for (notation, expected) in [
        ("NM_001199291.2:c.1619A>G", true),
        ("NM_001199291.2(HSD17B4):c.1619A>G", true),
        ("NM_001199291.3:c.1619A>G", false),
    ] {
        let value = run(&env, &["get", "variant", notation])?;
        if value.pointer("/result/candidates/0/component_terms/1/component")
            != Some(&json!("snpeff"))
            || value.pointer("/result/candidates/0/transcripts/0/matched") != Some(&json!(expected))
        {
            return Err(format!(
                "coding source match ignored transcript version: {value}"
            ));
        }
    }
    stub.finish()?;
    Ok(())
}
fn run(env: &Environment, args: &[&str]) -> Result<Value, String> {
    let args = args.iter().map(|v| (*v).into()).collect::<Vec<_>>();
    let out = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!(
            "command {args:?} failed: {} {}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())
}
fn reply(request: &str) -> Result<String, String> {
    if request.starts_with("GET /allele?") {
        // Synthetic resolver reply; the annotation rows below use the public source capture.
        return Ok(json!({"@id":"http://reg.genome.network/allele/CA99999999",
            "genomicAlleles":[{"chromosome":"5","referenceGenome":"GRCh37",
                "hgvs":["NC_000005.9:g.118860951A>G"],
                "coordinates":[{"start":118860950,"end":118860951,"referenceAllele":"A","allele":"G"}]}]}).to_string());
    }
    let fixture = fs::read(
        super::super::launch::fixtures()
            .join("fixtures/variant/transcripts/myvariant-hsd17b4.json"),
    )
    .map_err(|e| e.to_string())?;
    let hit: Value = serde_json::from_slice(&fixture).map_err(|e| e.to_string())?;
    Ok(if request.starts_with("GET /v1/query?") {
        json!({"total":1,"hits":[hit]})
    } else {
        hit
    }
    .to_string())
}
