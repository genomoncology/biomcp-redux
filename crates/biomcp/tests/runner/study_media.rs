//! Feature study-catalog: synchronize the publisher's JSON representation.
use super::{rows, super::{configuration::invoke, launch::Environment, network_stub::Stub}};
use libtest_mimic::Trial;
use serde_json::json;
use std::fs;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("study-catalog/sync-negotiates-json", || {
        sync().map_err(Into::into)
    })]
}
fn accepts_json(request: &str) -> bool {
    request.lines().any(|line| line.eq_ignore_ascii_case("accept: application/json"))
}
fn sync() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_source_status(
        "www.cbioportal.org",
        "application/json",
        |_| Ok(rows().to_string()),
        |request| if accepts_json(request) { 200 } else { 406 },
    )?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    let result = invoke(&env, vec!["sync".into(), "cbioportal-studies".into()], 0);
    let requests = stub.finish()?;
    if requests.len() != 1 {
        return Err(format!("catalog sync made {} physical requests", requests.len()));
    }
    let request = &requests.first().ok_or("missing catalog request")?.1;
    if !accepts_json(request) {
        return Err(format!("catalog requested a rejected representation: {request}; result: {result:?}"));
    }
    let value = result?;
    if value.pointer("/result/rows") != Some(&json!(12)) {
        return Err(format!("catalog sync did not publish the JSON rows: {value}"));
    }
    let value = invoke(&env, vec!["search".into(), "study".into(), "atlas".into(), "--offline".into()], 0)?;
    if value.pointer("/page/total") != Some(&json!(12)) {
        return Err(format!("synchronized catalog is unavailable offline: {value}"));
    }
    Ok(())
}
