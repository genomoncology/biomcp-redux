//! Feature diagnostic-pma: native identity and cardinality guard admission to cache.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    ["wrong-base", "wrong-supplement", "missing-base-cell", "duplicate-base", "incomplete", "wrong-status", "broken", "500", "429", "disabled"]
        .into_iter().map(|mode| Trial::test(format!("diagnostic-pma/synthetic-{mode}"), move || boundary(mode).map_err(Into::into))).collect()
}
fn native(mode: &str) -> Result<Value, String> {
    let row = json!({"pma_number":if mode == "wrong-base" {"P200010"} else {"P150013"},"supplement_number":if mode == "wrong-supplement" {"S021"} else {""},"trade_name":"Synthetic device","ao_statement":"Synthetic statement"});
    let mut rows = vec![row.clone()];
    if mode == "duplicate-base" {rows.push(row);}
    if mode == "missing-base-cell" {
        rows.first_mut().and_then(Value::as_object_mut).ok_or("missing row")?.remove("supplement_number");
    }
    let total = if mode == "incomplete" {101} else {rows.len()};
    Ok(json!({"meta":{"results":{"skip":0,"limit":100,"total":total}},"results":rows}))
}
fn boundary(mode: &'static str) -> Check {
    let body = native(mode)?;
    let stub = Stub::for_source_status("api.fda.gov", "application/json", move |request| {
        if !request.starts_with("GET /device/pma.json?search=pma_number%3A%22P150013%22&limit=100&skip=0 HTTP/1.1") {return Err(format!("wrong exact query: {request}"));}
        Ok(if mode == "broken" {"{".into()} else if mode == "wrong-status" {json!({"error":{"code":"NOT_FOUND","message":"No matches found!"}}).to_string()} else {body.to_string()})
    }, move |_| match mode {"500"=>500,"429"=>429,_=>200})?;
    let env = Environment::new().map_err(|e|e.to_string())?;
    let ca = env.root().join("ca.pem");
    std::fs::write(&ca, &stub.certificate).map_err(|e|e.to_string())?;
    let disabled: Vec<&str> = if mode == "disabled" {vec!["fda-pma"]} else {vec![]};
    env.config(&json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address),"disable_sources":disabled})).map_err(|e|e.to_string())?;
    let args: Vec<String> = ["get","diagnostic","P150013","--source","fda-pma"].into_iter().map(String::from).collect();
    for _ in 0..2 {
        let value = invoke(&env,args.clone(),4)?;
        if value.get("result").is_some() || value.pointer("/sources/0/source") != Some(&json!("fda-pma")) || value.pointer("/sources/0/use") != Some(&json!("open")) || value.pointer("/sources/0/terms").is_none() {return Err(format!("failed native boundary: {value}"));}
        if mode == "disabled" && value.pointer("/sources/0/reason") != Some(&json!("disabled")) {return Err(format!("policy: {value}"));}
    }
    let requests = stub.finish()?;
    if mode == "disabled" && !requests.is_empty() || mode != "disabled" && requests.len()<2 {return Err(format!("bad reply was cached or disabled source fetched: {}",requests.len()));}
    Ok(())
}
