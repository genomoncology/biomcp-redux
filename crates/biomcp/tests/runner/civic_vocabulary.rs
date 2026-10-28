//! Feature variant: exact source vocabulary keeps each claim and names its field problem.
use super::{civic_body, configure, run};
use crate::runner::{launch::Environment, network_stub::Stub};
use serde_json::json;

pub(super) fn unknown() -> Result<(), String> {
    for (native, known, output) in [
        ("status", "ACCEPTED", "status"),
        ("evidenceType", "PREDICTIVE", "kind"),
        ("significance", "RESISTANCE", "significance"),
    ] {
        for value in [
            "new native value".to_owned(),
            format!(" {known} "),
            String::new(),
        ] {
            check(native, known, output, value, 0)?;
        }
    }
    Ok(())
}
fn check(
    native: &'static str,
    known: &'static str,
    output: &str,
    value: String,
    index: usize,
) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replacement = value.clone();
    let stub = Stub::for_sources(
        &["civicdb.org"],
        |_| "application/json",
        move |request| {
            civic_body(request).map(|body| {
                body.replace(
                    &format!("\"{native}\":\"{known}\""),
                    &format!("\"{native}\":\"{replacement}\""),
                )
            })
        },
    )?;
    configure(&env, &stub)?;
    let result = run(
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
    let base = format!("/result/candidates/0/sections/civic/data/profiles/0/statements/{index}");
    let record = if index == 0 { "EID1409" } else { "AID7" };
    if result.pointer(&format!("{base}/{output}")) != Some(&json!(value)) {
        return Err(format!(
            "unknown {native} lost its original claim: {result}"
        ));
    }
    let disclosed = result
        .get("problems")
        .and_then(|v| v.as_array())
        .is_some_and(|rows| {
            rows.iter().any(|p| {
                p.get("field") == Some(&json!(native)) && p.get("record") == Some(&json!(record))
            })
        });
    if !disclosed {
        return Err(format!("unknown {native} lost its field problem: {result}"));
    }
    if native == "significance"
        && value.is_empty()
        && (result.pointer(&format!("{base}/statement")).is_some()
            || result.pointer(&format!("{base}/native")).is_none())
    {
        return Err(format!(
            "empty significance invented a common classification or lost its native claim: {result}"
        ));
    }
    stub.finish()?;
    Ok(())
}

pub(super) fn stream_context() -> Result<(), String> {
    for (native, known, output, value, index) in [
        ("assertionType", "PREDICTIVE", "kind", "FUNCTIONAL", 2),
        (
            "significance",
            "SENSITIVITYRESPONSE",
            "significance",
            "GAIN_OF_FUNCTION",
            2,
        ),
        ("significance", "RESISTANCE", "significance", "ONCOGENIC", 0),
    ] {
        check(native, known, output, value.into(), index)?;
    }
    Ok(())
}
