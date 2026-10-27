//! Named source filters intersect declared selectors without interpreting source labels.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test(
            "variant-search/literal-filters-preserve-page-and-cache",
            || filters().map_err(Into::into),
        ),
        Trial::test(
            "variant-search/component-groups-survive-page-action",
            || restricted_page().map_err(Into::into),
        ),
    ]
}
fn restricted_page() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", restricted_body)?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("synthetic-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca, "disable_sources":["noncommercial"]}))
        .map_err(|e| e.to_string())?;
    let initial = command(
        &env,
        &[
            "search",
            "variant",
            "--region",
            "chr7:140453136-140453136",
            "--build",
            "GRCh37",
            "--limit",
            "1",
            "--disable-sources",
            "restricted,clinvar",
        ]
        .map(Into::into),
    )?;
    let args: Vec<String> = serde_json::from_value(
        initial
            .pointer("/next/0/args")
            .cloned()
            .ok_or("missing page action")?,
    )
    .map_err(|e| e.to_string())?;
    let page = command(&env, &args)?;
    let disabled = args
        .windows(2)
        .find_map(|pair| match pair {
            [flag, value] if flag == "--disable-sources" => Some(value),
            _ => None,
        })
        .map(|value| value.split(',').collect::<Vec<_>>())
        .unwrap_or_default();
    if !disabled.contains(&"restricted")
        || disabled.contains(&"noncommercial")
        || !disabled.contains(&"clinvar")
        || page.pointer("/page/offset") != Some(&json!(1))
        || page.pointer("/result/items/0/identity/ids").is_some()
        || stub.finish()?.len() != 2
    {
        return Err(format!(
            "page action lost component groups: {args:?} {page}"
        ));
    }
    Ok(())
}
fn restricted_body(request: &str) -> Result<String, String> {
    if request.contains("clinvar.variant_id") {
        return Err(format!(
            "page action reenabled ClinVar projection: {request}"
        ));
    }
    Ok(
        json!({"total":2,"hits":[{"_id":"chr7:g.140453136A>T","clinvar":{"variant_id":123}}]})
            .to_string(),
    )
}
fn filters() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", body)?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("synthetic-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    let mut args: Vec<String> = [
        "search",
        "variant",
        "--region",
        "chr7:140453136-140453136",
        "--build",
        "GRCh37",
        "--review-status",
        "no assertion criteria provided",
        "--condition",
        "Carcinoma of colon (CRC)",
        "--has",
        "clinvar",
        "--limit",
        "1",
    ]
    .into_iter()
    .map(Into::into)
    .collect();
    for offset in [0, 1, 1] {
        let value = command(&env, &args)?;
        if value.pointer("/status") != Some(&json!("complete"))
            || value.pointer("/page/offset") != Some(&json!(offset))
            || value.pointer("/query/condition") != Some(&json!("Carcinoma of colon (CRC)"))
            || value.pointer("/result/source_query")
                != Some(&json!(
                    "chr7:140453136-140453136 AND clinvar.rcv.review_status:\"no assertion criteria provided\" AND \
                        clinvar.rcv.conditions.name:\"Carcinoma of colon \\(CRC\\)\" AND _exists_:clinvar.variant_id"
                ))
        {
            return Err(format!("literal filter or page lost: {value}"));
        }
        if offset == 0 {
            let action = value
                .get("next")
                .and_then(Value::as_array)
                .ok_or("missing actions")?
                .iter()
                .find(|v| v.get("kind") == Some(&json!("page")))
                .ok_or("missing page")?;
            args = serde_json::from_value(action.get("args").cloned().ok_or("missing page args")?)
                .map_err(|e| e.to_string())?;
        }
    }
    for extra in [
        vec!["--missing", "clinvar"],
        vec!["--review-status", "x\" OR *"],
        vec!["--has", "cadd"],
    ] {
        let mut args = vec![
            "search",
            "variant",
            "--query",
            "dbsnp.rsid:rs113488022",
            "--source",
            "myvariant",
        ];
        args.extend(extra);
        if command(&env, &args.into_iter().map(Into::into).collect::<Vec<_>>())?
            .pointer("/error/code")
            != Some(&json!("usage.arguments"))
        {
            return Err("raw query/filter mix or unsupported component was accepted".into());
        }
    }
    if stub.finish()?.len() != 2 {
        return Err("warm page or invalid arguments requested source".into());
    }
    Ok(())
}
fn command(env: &Environment, args: &[String]) -> Result<Value, String> {
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), args)
        .output()
        .map_err(|e| e.to_string())?;
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}
fn body(request: &str) -> Result<String, String> {
    if !request.contains(
        "q=chr7%3A140453136-140453136%20AND%20clinvar.rcv.review_status%3A%22no%20assertion%20criteria%20prov\
        ided%22%20AND%20clinvar.rcv.conditions.name%3A%22Carcinoma%20of%20colon%20%5C%28CRC%5C%29%22%20AND%2\
        0_exists_%3Aclinvar.variant_id&",
    ) {
        return Err(format!("incorrect literal filter query: {request}"));
    }
    Ok(json!({"total":2,"hits":[{"_id":"chr7:g.140453136A>T","chrom":"7","hg19":{"start":140453136,"end":140453136}}]}).to_string())
}
