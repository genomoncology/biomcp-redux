//! P24 separates source-local empty, exact absence and source failure.
use super::budgets;
use super::{Environment, Stub, invoke, record, setup};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};

fn states() -> Result<(), String> {
    let mut observed = Vec::new();
    for kind in ["empty", "absent", "failure"] {
        let stub = Stub::for_source_status(
            "api-v3.monarchinitiative.org",
            "application/json",
            move |_| {
                Ok(if kind == "empty" {
                    json!({"limit":5,"offset":0,"total":0,"items":[]})
                } else {
                    json!({"error":"synthetic source error"})
                }
                .to_string())
            },
            move |_| {
                if kind == "absent" {
                    404
                } else if kind == "failure" {
                    500
                } else {
                    200
                }
            },
        )?;
        let env = setup(&stub)?;
        let args = if kind == "empty" {
            vec!["search", "phenotype", "synthetic empty"]
        } else {
            vec!["get", "phenotype", "HP:9999999", "--source", "monarch"]
        };
        let directory = env.root().join("state");
        let result = record(
            &format!("states/states/record1@kind={kind}"),
            &env,
            &args,
            &directory,
            if kind == "failure" { 4 } else { 1 },
        )?;
        budgets::mcp(
            &format!("states/states/mcp1@kind={kind}"),
            &directory,
            &args,
            &result,
            if kind == "failure" { 4 } else { 1 },
        )?;
        if kind == "empty" {
            if result.pointer("/page/total") != Some(&json!(0))
                || result.pointer("/result/items").is_some()
            {
                return Err(format!("successful empty search lost: {result}"));
            }
        } else if result.get("result").is_some() {
            return Err(format!("absence or failure invented a result: {result}"));
        }
        observed.push((
            result.get("status").cloned(),
            result.pointer("/sources/0/state").cloned(),
            result.pointer("/sources/0/reason").cloned(),
            result.get("result").is_some(),
        ));
        if stub.finish()?.is_empty() {
            return Err("synthetic state made no request".into());
        }
    }
    let [empty, absent, failure] = observed.as_slice() else {
        return Err("P24 state collection incomplete".into());
    };
    if empty == absent || empty == failure || absent == failure {
        return Err(format!("P24 states collapsed: {observed:?}"));
    }
    if empty.0 != Some(json!("absent"))
        || absent.0 != Some(json!("absent"))
        || failure.0 != Some(json!("failed"))
    {
        return Err(format!("P24 statuses changed: {observed:?}"));
    }
    Ok(())
}
fn disabled(source: &str) -> Result<(), String> {
    let stub = Stub::for_source(
        if source == "hpo" {
            "ontology.jax.org"
        } else {
            "api-v3.monarchinitiative.org"
        },
        "application/json",
        |_| Ok(super::node().to_string()),
    )?;
    let env = setup(&stub)?;
    let words = ["get", "phenotype", "HP:0000001", "--source", source];
    let directory = env.root().join("warm");
    record(
        &format!("states/disabled/record1@source={source}"),
        &env,
        &words,
        &directory,
        0,
    )?;
    let mut args = words.to_vec();
    args.extend(["--fresh", "--disable-sources", source]);
    let result = invoke(
        &format!("states/disabled/invoke1@source={source}"),
        &env,
        &args,
        4,
    )?;
    if result.pointer("/sources/0/state") != Some(&json!("skipped"))
        || result.pointer("/sources/0/reason") != Some(&json!("disabled"))
        || result.get("result").is_some()
    {
        return Err(format!("disabled fresh leaked cache: {result}"));
    }
    budgets::mcp(
        &format!("states/disabled/mcp1@source={source}"),
        &directory,
        &args,
        &result,
        4,
    )?;
    if stub.finish()?.len() != 1 {
        return Err("disabled fresh reached source".into());
    }
    // A new isolated environment also skips without requiring any populated cache.
    let cold = Environment::new().map_err(|e| e.to_string())?;
    let cold_result: Value = invoke(
        &format!("states/disabled/invoke2@source={source}"),
        &cold,
        &args,
        4,
    )?;
    if cold_result.pointer("/sources/0/reason") != Some(&json!("disabled")) {
        return Err("cold disabled source touched cache".into());
    }
    Ok(())
}
