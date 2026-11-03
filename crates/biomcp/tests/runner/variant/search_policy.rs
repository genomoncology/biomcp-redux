//! Open selectors use existing public BRAF identity facts through recorded transport.
use super::super::{launch, mcp, network_stub::Stub};
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    for protein in ["V600E", "Val600Glu", "p.V600E", "p.Val600Glu"] {
        for policy in ["noncommercial", "restricted"] {
            check_policy(protein, policy)?;
        }
    }
    Ok(())
}

fn check_policy(protein: &str, policy: &str) -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", body)?;
    let env = super::source_environment(&stub)?;
    let replay = env.root().join("recorded");
    let words = ["search", "variant", "--gene", "BRAF", "--protein", protein,
        "--disable-sources", policy];
    let mut args: Vec<String> = words.into_iter().map(str::to_owned).collect();
    args.extend(["--record".into(), replay.to_string_lossy().into()]);
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output().map_err(|e| e.to_string())?;
    let cli: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    check_reply(&cli)?;
    let replay_env = launch::Environment::new().map_err(|e| e.to_string())?;
    let args: Vec<String> = words.into_iter().map(str::to_owned).collect();
    let mut globals = replay_env.globals(Some(&replay), false);
    globals.push("--hosted".into());
    let hosted = mcp::call_with_globals(&replay_env, &args, false, &globals)?;
    check_reply(&hosted)?;
    let requests = stub.finish()?;
    let [(_, request)] = requests.as_slice() else {
        return Err(format!("expected exactly one source request: {requests:?}"));
    };
    for forbidden in ["dbnsfp", "cadd", "cosmic", "snpedia"] {
        if request.to_lowercase().contains(forbidden) || cli.to_string().to_lowercase().contains(forbidden) {
            return Err(format!("disabled component leaked: {forbidden}"));
        }
    }
    if !request.contains("snpeff.ann.genename%3ABRAF")
        || !request.contains("snpeff.ann.hgvs_p%3A") {
        return Err(format!("open fields missing from request: {request}"));
    }
    Ok(())
}

fn check_reply(value: &Value) -> Result<(), String> {
    if value.pointer("/result/items/0/record/id") != Some(&json!("chr7:g.140753336A>T"))
        || value.pointer("/result/selector_terms/name") != Some(&json!("SnpEff MIT"))
        || value.pointer("/result/items/0/transcripts/0/matched") != Some(&json!(true)) {
        return Err(format!("open protein selector did not return the recorded candidate: {value}"));
    }
    Ok(())
}

fn body(_: &str) -> Result<String, String> {
    // Synthetic page and transcript projection of existing common.json BRAF facts;
    // no new assertion or current live-source result is claimed.
    Ok(json!({"total":1,"hits":[{"_id":"chr7:g.140753336A>T",
        "dbsnp":{"rsid":"rs113488022","gene":{"symbol":"BRAF"}},
        "clinvar":{"variant_id":13961},"snpeff":{"ann":[{
            "feature_id":"NM_004333.4","feature_type":"transcript","genename":"BRAF",
            "hgvs_c":"c.1799T>A","hgvs_p":"p.Val600Glu"}]}}]}).to_string())
}
