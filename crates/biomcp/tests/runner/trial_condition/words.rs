//! Registry condition words support a factual notice, never local relevance filtering.
use super::{Stub, invoke, setup};
use serde_json::{Value, json};
use std::fs;

pub(super) fn mismatch() -> Result<(), String> {
    let stub = Stub::for_source("clinicaltrials.gov", "application/json", |request| {
        if request.starts_with("GET /api/v2/studies?")
            && request.contains("query.cond=%28Snyder-Robinson%20syndrome%29")
        {
            if request.contains("filter.advanced=%28%28AREA%5BCondition%5DEXPANSION%5BNone%5D") {
                return fs::read_to_string(
                    super::super::launch::fixtures()
                        .join("replies/trial/condition-words/exact.body"),
                )
                .map_err(|e| e.to_string());
            }
            return fs::read_to_string(
                super::super::launch::fixtures().join("replies/trial/condition-words/broad.body"),
            )
            .map_err(|e| e.to_string());
        }
        if request.starts_with("GET /api/v2/version ") {
            return Ok(json!({"dataTimestamp":"2026-10-27T00:00:00"}).to_string());
        }
        Err(format!("undeclared source request: {request}"))
    })?;
    let env = setup(&stub)?;
    let recording = env.root().join("recording");
    let words = [
        "search",
        "trial",
        "--condition",
        "Snyder-Robinson syndrome",
        "--limit",
        "25",
    ];
    let mut args: Vec<String> = words.iter().map(|v| (*v).into()).collect();
    args.extend(["--record".into(), recording.to_string_lossy().into()]);
    let value = invoke(&env, &args, 0)?;
    check(&value)?;
    exact(&env, &recording, &words, &value)?;
    let requests = stub.finish()?;
    if requests.len() != 4 {
        return Err("condition notice introduced source requests".into());
    }
    let restriction = "filter.advanced=%28%28AREA%5BCondition%5DEXPANSION%5BNone%5D%22Snyder-Robinson%20syndrome%22%29%29";
    if requests
        .iter()
        .filter(|(connect, request)| {
            connect.starts_with("CONNECT clinicaltrials.gov:443 ")
                && request.starts_with("GET /api/v2/studies?")
                && request.contains(restriction)
        })
        .count()
        != 1
    {
        return Err("missing captured condition field restriction in the source request".into());
    }
    replay(
        &env,
        &recording,
        &words,
        json!({"/result/items":{"count":6},"/sources/0/notices/0/kind":"condition-word-mismatch"}),
    )?;
    Ok(())
}

fn check(value: &Value) -> Result<(), String> {
    if value.pointer("/sources/0/notices/0/kind") != Some(&json!("condition-word-mismatch")) {
        return Err(format!("missing factual condition-word notice: {value}"));
    }
    let expected = [
        "NCT07668869",
        "NCT03303365",
        "NCT03285932",
        "NCT01345552",
        "NCT01345539",
        "NCT03297788",
    ];
    let actual: Vec<_> = value
        .pointer("/result/items")
        .and_then(Value::as_array)
        .ok_or("missing trial items")?
        .iter()
        .filter_map(|v| v.get("nct_id").and_then(Value::as_str))
        .collect();
    if actual != expected || value.pointer("/page/total") != Some(&json!(6)) {
        return Err("condition notice dropped, reordered or recounted source rows".into());
    }
    Ok(())
}

fn exact(
    env: &super::Environment,
    recording: &std::path::Path,
    words: &[&str],
    broad: &Value,
) -> Result<(), String> {
    let mut args: Vec<String> = words.iter().map(|v| (*v).into()).collect();
    args.push("--exact-condition".into());
    let next = broad
        .pointer("/next/0/args")
        .ok_or("missing exact-condition next command")?;
    let next: Vec<String> = serde_json::from_value(next.clone()).map_err(|e| e.to_string())?;
    if !next.iter().any(|arg| arg == "--exact-condition")
        || next
            .iter()
            .filter(|arg| arg.as_str() != "--exact-condition")
            .map(String::as_str)
            .collect::<Vec<_>>()
            != words
    {
        return Err(format!("incorrect exact-condition step: {next:?}"));
    }
    let recording = recording.with_file_name("exact-recording");
    let exact_args = args.clone();
    args.extend(["--record".into(), recording.to_string_lossy().into()]);
    let result = invoke(env, &args, 0)?;
    if result.pointer("/page/total") != Some(&json!(0))
        || result.pointer("/page/returned") != Some(&json!(0))
        || result.pointer("/result/items").is_some()
    {
        return Err(format!(
            "exact condition did not retain the captured zero: {result}"
        ));
    }
    let words: Vec<_> = exact_args.iter().map(String::as_str).collect();
    replay(
        env,
        &recording,
        &words,
        json!({"/page/total":0,"/page/returned":0}),
    )?;
    Ok(())
}

fn replay(
    env: &super::Environment,
    recording: &std::path::Path,
    words: &[&str],
    expect: Value,
) -> Result<(), String> {
    let case = env.root().join("case.json");
    fs::write(&case, json!({"feature":"trial-search",
        "story":"A condition search preserves source rows and offers an explicit registered-phrase restriction",
        "replay":recording,"steps":[{"run":words,"mcp":words,"expect":expect}]
    }).to_string()).map_err(|e| e.to_string())?;
    super::super::run_case(&case, super::super::schema::Registry::embedded()?.as_ref())?;
    Ok(())
}
