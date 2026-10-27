//! Feature: variant-search. Source-paired rows expose numbering differences through the CLI.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test(
            "variant-search/source-transcripts-and-matched-notation",
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
    for (protein, matched) in [("His540Arg", 1), ("His515Arg", 0)] {
        let args = [
            "search",
            "variant",
            "--gene",
            "HSD17B4",
            "--protein",
            protein,
        ];
        let value = run(&env, &args)?;
        let rows = value
            .pointer("/result/items/0/transcripts")
            .ok_or("transcripts missing")?;
        if rows.as_array().map(Vec::len) != Some(2)
            || rows[0]["transcript"] != "NM_000414.3"
            || rows[0]["hgvs_c"] != "c.1544A>G"
            || rows[0]["hgvs_p"] != "p.His515Arg"
            || rows[1]["transcript"] != "NM_001199291.2"
            || rows[1]["hgvs_c"] != "c.1619A>G"
            || rows[1]["hgvs_p"] != "p.His540Arg"
            || rows[matched]["matched"] != true
            || rows[1 - matched]["matched"] != false
            || rows[0].get("mane_select").is_some()
        {
            return Err(format!(
                "source transcript pairing or match changed: {rows}"
            ));
        }
    }
    let card = run(
        &env,
        &["get", "variant", "chr5:g.118860951A>G", "--build", "GRCh37"],
    )?;
    if card.pointer("/result/candidates/0/transcripts/1/hgvs_p") != Some(&json!("p.His540Arg")) {
        return Err(format!("card lost transcript annotations: {card}"));
    }
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
    if !markdown.contains("NM\\_001199291&#46;2") || !markdown.contains("p&#46;His515Arg") {
        return Err(format!("readable card lost transcript rows: {markdown}"));
    }
    let excluded = run(
        &env,
        &[
            "--disable-sources",
            "snpeff",
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
    stub.finish()?;
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
        ("NM_001199291.3:c.1619A>G", false),
    ] {
        let value = run(&env, &["get", "variant", notation])?;
        if value.pointer("/result/candidates/0/transcripts/1/matched") != Some(&json!(expected)) {
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
        return Err(String::from_utf8_lossy(&out.stderr).into());
    }
    serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())
}
fn reply(request: &str) -> Result<String, String> {
    if request.starts_with("GET /allele?") {
        // Synthetic resolver identity; genomic and transcript strings below come from the public witness.
        return Ok(json!({"@id":"http://reg.genome.network/allele/CA99999999",
            "genomicAlleles":[{"chromosome":"5","referenceGenome":"GRCh37",
                "hgvs":["NC_000005.9:g.118860951A>G"],
                "coordinates":[{"start":118860950,"end":118860951,"referenceAllele":"A","allele":"G"}]}]}).to_string());
    }
    // Source-paired values retained by the public 0.9.3 HSD17B4 witness.
    let hit = json!({"_id":"chr5:g.118860951A>G","chrom":"5",
    "hg19":{"start":118860951,"end":118860951},
    "snpeff":{"ann":[
        {"feature_id":"NM_000414.3","feature_type":"transcript","genename":"HSD17B4","hgvs_c":"c.1544A>G","hgvs_p":"p.His515Arg"},
        {"feature_id":"NM_001199291.2","feature_type":"transcript","genename":"HSD17B4","hgvs_c":"c.1619A>G","hgvs_p":"p.His540Arg"}
    ]}});
    Ok(if request.starts_with("GET /v1/query?") {
        json!({"total":1,"hits":[hit]})
    } else {
        hit
    }
    .to_string())
}
