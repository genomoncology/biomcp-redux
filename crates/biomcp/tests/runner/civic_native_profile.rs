//! Feature drug-civic: native profile context crosses the CLI in a synthetic claim envelope.
//! Native response SHA256 0b3ad5ceecc72a9799d7256f20c63cc3f7bae4dbdcd8ee03972c71da771caee9.
use super::{configure, run};
use crate::runner::{launch::Environment, network_stub::Stub};
use serde_json::{Value, json};

pub(super) fn context() -> Result<(), String> {
    for malformed in [false, true] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let stub = Stub::for_source("civicdb.org", "application/json", move |request| {
            let mut body: Value =
                serde_json::from_str(&crate::runner::drug_civic_fixtures::reply(request)?)
                    .map_err(|e| e.to_string())?;
            let mut profile = native()?;
            if malformed {
                *profile.get_mut("rawName").ok_or("native rawName missing")? = json!(42);
            }
            *body
                .pointer_mut("/data/evidenceItems/nodes/0/molecularProfile")
                .ok_or("synthetic claim profile missing")? = profile;
            Ok(body.to_string())
        })?;
        configure(&env, &stub)?;
        let value = run(&env, &["get", "drug", "5", "civic", "--source", "civic"])?;
        let profile = value
            .pointer("/result/civic/data/evidence/records/0/profile")
            .ok_or_else(|| format!("native context missing: {value}"))?;
        if profile.pointer("/record/id") != Some(&json!("6153")) {
            return Err(format!("native profile identity lost: {value}"));
        }
        if malformed {
            if profile.get("subject").is_some()
                || !value
                    .get("problems")
                    .and_then(Value::as_array)
                    .is_some_and(|rows| {
                        rows.iter()
                            .filter(|p| p.get("field") == Some(&json!("rawName")))
                            .count()
                            == 1
                    })
            {
                return Err(format!(
                    "malformed expression invented context or lost Problem: {value}"
                ));
            }
        } else {
            for (path, expected) in [
                (
                    "/raw_name",
                    json!("#VID276 AND NOT #VID442 AND NOT #VID499"),
                ),
                ("/subject/expression/tree/operator", json!("and")),
                ("/subject/expression/tree/operands/1/operator", json!("not")),
                (
                    "/subject/expression/tree/operands/1/operand/record",
                    json!("civic:442"),
                ),
                ("/subject/members/2/record", json!("civic:499")),
                (
                    "/subject/members/2/feature_record",
                    json!("civic-feature:62024"),
                ),
                ("/subject/members/2/feature", json!("v::ALK")),
            ] {
                if profile.pointer(path) != Some(&expected) {
                    return Err(format!("native {path} lost: {value}"));
                }
            }
        }
        stub.finish()?;
    }
    Ok(())
}
fn native() -> Result<Value, String> {
    let body: Value = serde_json::from_str(include_str!(
        "../../../biomcp-adapters/tests/fixtures/civic-profile-members.json"
    ))
    .map_err(|e| e.to_string())?;
    body.pointer("/data/molecularProfiles/nodes")
        .and_then(Value::as_array)
        .ok_or("native nodes missing")?
        .iter()
        .find(|row| row.get("id") == Some(&json!(6153)))
        .cloned()
        .ok_or_else(|| "native profile missing".into())
}
