//! Declared MyVariant query construction and native paging cross the real TLS boundary.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "variant-search/declared-protein-native-walk-and-warm-page",
        || walk().map_err(Into::into),
    )]
}
fn walk() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", body)?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("synthetic-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}", stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    let mut args: Vec<String> = [
        "search",
        "variant",
        "--gene",
        "EGFR",
        "--protein",
        "L858R",
        "--limit",
        "1",
    ]
    .into_iter()
    .map(Into::into)
    .collect();
    let mut ids = Vec::new();
    for offset in 0..2 {
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if !output.status.success()
            || value.pointer("/page/offset") != Some(&json!(offset))
            || value.pointer("/page/total") != Some(&json!(2))
            || value.pointer("/result/source_query")
                != Some(&json!("dbnsfp.genename:EGFR AND dbnsfp.hgvsp:\"p.L858R\""))
        {
            return Err(format!(
                "declared source query or native page lost: {value}"
            ));
        }
        ids.push(
            value
                .pointer("/result/items/0/record/id")
                .cloned()
                .ok_or("candidate missing")?,
        );
        if offset == 0 {
            let page = value
                .get("next")
                .and_then(Value::as_array)
                .and_then(|n| n.iter().find(|v| v.get("kind") == Some(&json!("page"))))
                .ok_or("page action absent")?;
            args = serde_json::from_value(page.get("args").cloned().ok_or("page args absent")?)
                .map_err(|e| e.to_string())?;
        }
    }
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success()
        || ids != [json!("chr7:g.55259515T>G"), json!("chr7:g.55259516T>G")]
        || stub.finish()?.len() != 2
    {
        return Err("walk reordered, lost candidates, or warm page requested again".into());
    }
    Ok(())
}
fn body(request: &str) -> Result<String, String> {
    if !request.starts_with(
        "GET /v1/query?q=dbnsfp.genename%3AEGFR%20AND%20dbnsfp.hgvsp%3A%22p.L858R%22&size=1&from=",
    ) || !request.contains("&assembly=hg19 HTTP/1.1")
    {
        return Err(format!("incorrect query request: {request}"));
    }
    let id = if request.contains("&from=1&") {
        "chr7:g.55259516T>G"
    } else {
        "chr7:g.55259515T>G"
    };
    Ok(json!({"total":2,"hits":[{"_id":id,"dbsnp":{"gene":{"symbol":"EGFR"}}}]}).to_string())
}
