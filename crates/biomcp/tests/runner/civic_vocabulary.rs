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

pub(super) fn drug_status() -> Result<(), String> {
    for status in [
        " new curation state ",
        " ACCEPTED ",
        "",
        "SUBMITTED",
        "REJECTED",
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let stub = Stub::for_sources(
            &["civicdb.org"],
            |_| "application/json",
            move |request| {
                crate::runner::drug_civic_fixtures::reply(request).map(|body| {
                    body.replace(
                        "\"status\":\"ACCEPTED\"",
                        &format!("\"status\":\"{status}\""),
                    )
                })
            },
        )?;
        configure(&env, &stub)?;
        let result = run(&env, &["get", "drug", "5", "civic", "--source", "civic"])?;
        if matches!(status, "SUBMITTED" | "REJECTED") {
            if result
                .pointer("/result/civic/data/evidence/records")
                .is_some()
                || result
                    .pointer("/result/civic/data/assertions/records")
                    .is_some()
                || result.pointer("/result/identity/id") != Some(&json!("5"))
                || !result
                    .get("problems")
                    .and_then(|v| v.as_array())
                    .is_some_and(|rows| {
                        rows.iter()
                            .any(|p| p.get("field") == Some(&json!("status")))
                    })
            {
                return Err(format!(
                    "known nonaccepted curation status passed the requested source filter: {result}"
                ));
            }
        } else {
            for (stream, record) in [("evidence", "EID259"), ("assertions", "AID85")] {
                let base = format!("/result/civic/data/{stream}/records/0/claim");
                if result.pointer(&format!("{base}/status")) != Some(&json!(status))
                    || result.pointer(&format!("{base}/statement")).is_some()
                    || result.pointer(&format!("{base}/native")).is_none()
                    || !result
                        .get("problems")
                        .and_then(|v| v.as_array())
                        .is_some_and(|rows| {
                            rows.iter().any(|p| {
                                p.get("field") == Some(&json!("status"))
                                    && p.get("record") == Some(&json!(record))
                            })
                        })
                {
                    return Err(format!(
                        "unknown drug claim status lost native facts, gained a common claim, or lacks its field problem: {result}"
                    ));
                }
            }
        }
        stub.finish()?;
    }
    Ok(())
}
