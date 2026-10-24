//! Named source filters intersect declared selectors without interpreting source labels.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("variant-search/literal-filters-preserve-page-and-cache", || {
        filters().map_err(Into::into)
    })]
}
fn filters() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", body)?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("synthetic-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    let mut args: Vec<String> = ["search", "variant", "--region", "chr7:140453136-140453136", "--build", "GRCh37", "--significance", "Pathogenic", "--review-status", "no assertion criteria provided", "--condition", "Carcinoma of colon (CRC)", "--has", "clinvar", "--limit", "1"]
        .into_iter().map(Into::into).collect();
    for offset in [0, 1, 1] {
        let value = command(&env, &args)?;
        if value.pointer("/status") != Some(&json!("complete"))
            || value.pointer("/page/offset") != Some(&json!(offset))
            || value.pointer("/query/condition") != Some(&json!("Carcinoma of colon (CRC)"))
            || value.pointer("/result/source_query") != Some(&json!("chr7:140453136-140453136 AND clinvar.rcv.clinical_significance:\"Pathogenic\" AND clinvar.rcv.review_status:\"no assertion criteria provided\" AND clinvar.rcv.conditions.name:\"Carcinoma of colon \\(CRC\\)\" AND _exists_:clinvar.variant_id"))
        { return Err(format!("literal filter or page lost: {value}")); }
        if offset == 0 {
            let action = value["next"].as_array().ok_or("missing actions")?.iter()
                .find(|v| v["kind"] == "page").ok_or("missing page")?;
            args = serde_json::from_value(action["args"].clone()).map_err(|e| e.to_string())?;
        }
    }
    for extra in [vec!["--missing", "clinvar"], vec!["--significance", "x\" OR *"], vec!["--has", "cadd"]] {
        let mut args = vec!["search", "variant", "--query", "dbsnp.rsid:rs113488022", "--source", "myvariant"];
        args.extend(extra);
        if command(&env, &args.into_iter().map(Into::into).collect::<Vec<_>>())?["status"] != "usage" {
            return Err("raw query/filter mix or unsupported component was accepted".into());
        }
    }
    if stub.finish()?.len() != 2 { return Err("warm page or invalid arguments requested source".into()); }
    Ok(())
}
fn command(env: &Environment, args: &[String]) -> Result<Value, String> {
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), args).output().map_err(|e| e.to_string())?;
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}
fn body(request: &str) -> Result<String, String> {
    if !request.contains("q=chr7%3A140453136-140453136%20AND%20clinvar.rcv.clinical_significance%3A%22Pathogenic%22%20AND%20clinvar.rcv.review_status%3A%22no%20assertion%20criteria%20provided%22%20AND%20clinvar.rcv.conditions.name%3A%22Carcinoma%20of%20colon%20%5C%28CRC%5C%29%22%20AND%20_exists_%3Aclinvar.variant_id&") {
        return Err(format!("incorrect literal filter query: {request}"));
    }
    Ok(json!({"total":2,"hits":[{"_id":"chr7:g.140453136A>T"}]}).to_string())
}
