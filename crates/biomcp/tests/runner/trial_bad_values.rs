//! Features trial-search and trial-get: malformed nonidentity fields retain their sibling facts.
use super::response_cases::checked_steps;
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), Box<dyn std::error::Error>>;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("trial-search/bad-values-retain-siblings", || search().map_err(Into::into)),
        Trial::test("trial-get/bad-values-retain-siblings", || detail().map_err(Into::into)),
    ]
}
fn replace(record: &mut Value, path: &str, invalid: &Value) {
    if let Some((parent, name)) = path.rsplit_once('/')
        && let Some(object) = record.pointer_mut(parent).and_then(Value::as_object_mut)
    {
        object.insert(name.into(), invalid.clone());
    }
}
fn search() -> Check {
    for (path, invalid, field) in [
        ("identificationModule/briefTitle", json!(42), "briefTitle"),
        ("conditionsModule/conditions", json!(["valid", 42]), "conditions"),
        ("armsInterventionsModule/interventions", json!([{"name":"valid"},42]), "interventions"),
        ("statusModule", json!(42), "statusModule"),
        ("designModule", json!(42), "designModule"),
        ("identificationModule/nctId", json!("bad"), "nctId"),
    ] {
        let output = checked_steps("trial/status", |_, record| {
            replace(record, &format!("/exchanges/0/body/value/studies/0/protocolSection/{path}"), &invalid);
        }, json!([{"run":["search","trial","--condition","melanoma","--status","RECRUITING"],
            "expect":{"/status":"partial","/result/items/*/nct_id":{"includes":"NCT07381972"}}}]))?;
        problem(&output, field)?;
        if field == "nctId" && output.pointer("/result/items").and_then(Value::as_array).map(Vec::len) != Some(9) {
            return Err("bad identity did not remove exactly its own study".into());
        }
    }
    Ok(())
}
fn detail() -> Check {
    for (path, invalid, field) in [
        ("identificationModule/briefTitle", json!(42), "briefTitle"),
        ("statusModule/startDateStruct/type", json!("FUTURE"), "startDateStruct.type"),
        ("eligibilityModule/healthyVolunteers", json!(42), "healthyVolunteers"),
        ("eligibilityModule/eligibilityCriteria", json!(42), "eligibilityCriteria"),
        ("conditionsModule/conditions", json!(["valid",42]), "conditions"),
        ("armsInterventionsModule/interventions/0/type", json!("FUTURE"), "interventions.type"),
        ("armsInterventionsModule/interventions/0/name", json!(42), "interventions.name"),
        ("contactsLocationsModule/locations/0/status", json!("FUTURE"), "locations.status"),
        ("referencesModule/references/0/pmid", json!("bad"), "references.pmid"),
        ("statusModule", json!(42), "statusModule"),
        ("designModule", json!(42), "designModule"),
    ] {
        let output = checked_steps("trial/get/NCT03511664", |_, record| {
            replace(record, &format!("/exchanges/0/body/value/protocolSection/{path}"), &invalid);
        }, json!([{"run":["get","trial","NCT03511664"],"expect":{
            "/status":"partial","/result/items/0/card/nct_id":"NCT03511664",
            "/result/items/0/card/brief_summary":{"includes":"PSMA"}}}]))?;
        problem(&output, field)?;
    }
    Ok(())
}
fn problem(output: &Value, field: &str) -> Check {
    if output.get("problems").and_then(Value::as_array).into_iter().flatten().any(|p| {
        p.get("source").and_then(Value::as_str) == Some("ctgov")
            && p.get("field").and_then(Value::as_str) == Some(field)
    }) {
        Ok(())
    } else {
        Err(format!("missing source field problem for {field}: {output}").into())
    }
}
