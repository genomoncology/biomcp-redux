//! Features trial-nci and cell-line-identity: new publisher values retain their native text.
use super::{launch, response_cases, run_case, schema};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("trial-nci/unknown-phase-keeps-trial", || check(true).map_err(Into::into)),
        Trial::test("cell-line-identity/unknown-sex-keeps-cell", || check(false).map_err(Into::into))]
}
fn replace(value: &mut Value, field: &str) {
    match value {
        Value::Object(map) => {
            if map.contains_key(field) { map.insert(field.into(), json!(" new source value ")); }
            for value in map.values_mut() { replace(value, field); }
        }
        Value::Array(rows) => for value in rows { replace(value, field); },
        _ => {}
    }
}
fn check(nci: bool) -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e|e.to_string())?;
    let (capture, source, field, path, args) = if nci {
        ("trial/nci/get", "nci", "phase", "/result/items/0/card/phase", vec!["get","trial","NCT05136196","--source","nci"])
    } else {
        ("cell-line/examples", "cellosaurus", "sex", "/result/sex", vec!["get","cell-line","CVCL_2119"])
    };
    let recordings = temp.path().join("recordings");
    response_cases::copy_capture(capture, &recordings, |name, record| {
        if name != source { return; }
        if let Some(exchanges) = record.get_mut("exchanges").and_then(Value::as_array_mut) {
            for exchange in exchanges {
                if let Some(body) = exchange.pointer_mut("/body/value") {
                    if let Some(text) = body.as_str() {
                        if let Ok(mut data) = serde_json::from_str::<Value>(text) {
                            replace(&mut data, field); *body = json!(data.to_string());
                        }
                    } else { replace(body, field); }
                }
            }
        }
    }).map_err(|e|e.to_string())?;
    let case = temp.path().join("case.yaml");
    fs::write(&case, serde_json::to_vec(&json!({
        "feature":if nci {"trial-nci"} else {"cell-line-identity"}, "story":"New source values retain their item",
        "replay":recordings, "config":{"keys":{"nci_api_key":"synthetic-nci-replay"}},
        "steps":[{"run":args,"expect":{path:" new source value "},"exit":0}]
    })).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    let result = run_case(&case, &schema::Registry::embedded()?).map_err(|e|e.to_string())?;
    if !result.get("problems").and_then(Value::as_array).is_some_and(|rows|rows.iter().any(|p|p.get("field")==Some(&json!(field)))) {
        return Err(format!("new {field} lacks a problem: {result}"));
    }
    let _ = launch::fixtures();
    Ok(())
}
