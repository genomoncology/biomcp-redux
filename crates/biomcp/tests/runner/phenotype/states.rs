//! P24 separates source-local empty, exact absence and source failure.
use super::budgets;
use super::{Environment, Stub, invoke, record, setup};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    let mut rows = vec![transcript_trial(
        "phenotype-identity/P24-synthetic-empty-absent-failure-states",
        || states().map_err(Into::into),
    )];
    rows.push(transcript_trial(
        "phenotype-identity/synthetic-short-limit-keeps-native-page",
        || short_limit().map_err(Into::into),
    ));
    rows.push(transcript_trial(
        "phenotype-identity/refused-native-row-keeps-partial-page",
        || dropped().map_err(Into::into),
    ));
    for source in ["hpo", "monarch"] {
        rows.push(transcript_trial(
            format!("phenotype-identity/synthetic-disabled-{source}-fresh-before-cache"),
            move || disabled(source).map_err(Into::into),
        ));
    }
    rows
}
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
        let exit = match kind {
            "failure" => 4,
            "empty" => 0,
            _ => 1,
        };
        let directory = env.root().join("state");
        let result = record(
            &format!("states/states/record1@kind={kind}"),
            &env,
            &args,
            &directory,
            exit,
        )?;
        budgets::mcp(
            &format!("states/states/mcp1@kind={kind}"),
            &directory,
            &args,
            &result,
            exit,
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
    if empty.0 != Some(json!("complete"))
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

fn short_limit() -> Result<(), String> {
    let stub = Stub::for_source(
        "api-v3.monarchinitiative.org",
        "application/json",
        |request| {
            if !request.contains("&limit=3&offset=0 ") {
                return Err(format!("short limit changed native request: {request}"));
            }
            Ok(json!({"limit":3,"offset":0,"total":0,"items":[]}).to_string())
        },
    )?;
    let env = setup(&stub)?;
    let args = ["search", "phenotype", "synthetic", "-l", "3"];
    let directory = env.root().join("alias");
    let result = record("states/short_limit/record1", &env, &args, &directory, 0)?;
    if result.pointer("/query/limit") != Some(&json!(3)) {
        return Err(format!("short limit lost: {result}"));
    }
    budgets::mcp("states/short_limit/mcp1", &directory, &args, &result, 0)?;
    if stub.finish()?.len() != 1 {
        return Err("short limit request count".into());
    }
    Ok(())
}

fn dropped() -> Result<(), String> {
    let stub = Stub::for_source("api-v3.monarchinitiative.org", "application/json", |_| {
        let mut rejected = super::node();
        *rejected.get_mut("id").ok_or("missing fixture id")? = json!(42);
        Ok(json!({"limit":5,"offset":0,"total":2,"items":[super::node(),rejected]}).to_string())
    })?;
    let env = setup(&stub)?;
    // Reuse the existing two-row search ceiling for this one-row refusal reply.
    let reply = invoke(
        "mod/search/invoke1",
        &env,
        &["search", "phenotype", "synthetic"],
        0,
    )?;
    if reply.get("status") != Some(&json!("partial"))
        || reply.pointer("/sources/0/state") != Some(&json!("ok"))
        || reply.pointer("/page/total") != Some(&json!(2))
        || reply.pointer("/page/returned") != Some(&json!(1))
        || reply.pointer("/result/items/0/id") != Some(&json!("HP:0000001"))
    {
        return Err(format!(
            "refused native row lost partial status or valid sibling: {reply}"
        ));
    }
    if stub.finish()?.len() != 1 {
        return Err("refused row triggered a replacement source page".into());
    }
    Ok(())
}
