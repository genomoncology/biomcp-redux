//! Feature variant: new source vocabulary keeps each claim and names its field problem.
use super::{civic_body, configure, run};
use crate::runner::{launch::Environment, network_stub::Stub};
use serde_json::json;

pub(super) fn unknown() -> Result<(), String> {
    for (native, reply, field, output) in [
        (
            "status",
            status_body as fn(&str) -> Result<String, String>,
            "status",
            "status",
        ),
        ("evidenceType", kind_body, "evidenceType", "kind"),
        (
            "significance",
            significance_body,
            "significance",
            "significance",
        ),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let stub = Stub::for_source("civicdb.org", "application/json", reply)?;
        configure(&env, &stub)?;
        let value = run(
            &env,
            &[
                "get",
                "variant",
                "CA123643",
                "civic",
                "--disable-sources",
                "myvariant",
            ],
        )?;
        let path =
            format!("/result/candidates/0/sections/civic/data/profiles/0/statements/0/{output}");
        if value.pointer(&path) != Some(&json!("new native value")) {
            return Err(format!("unknown {native} lost its original claim: {value}"));
        }
        let disclosed = value
            .get("problems")
            .and_then(|v| v.as_array())
            .is_some_and(|rows| {
                rows.iter().any(|p| {
                    p.get("field") == Some(&json!(field))
                        && p.get("record") == Some(&json!("EID1409"))
                })
            });
        if !disclosed {
            return Err(format!("unknown {native} lost its field problem: {value}"));
        }
        stub.finish()?;
    }
    Ok(())
}

fn changed(request: &str, field: &str, known: &str) -> Result<String, String> {
    civic_body(request).map(|body| {
        body.replace(
            &format!("\"{field}\":\"{known}\""),
            &format!("\"{field}\":\"new native value\""),
        )
    })
}
fn status_body(request: &str) -> Result<String, String> {
    changed(request, "status", "ACCEPTED")
}
fn kind_body(request: &str) -> Result<String, String> {
    changed(request, "evidenceType", "PREDICTIVE")
}
fn significance_body(request: &str) -> Result<String, String> {
    changed(request, "significance", "RESISTANCE")
}
