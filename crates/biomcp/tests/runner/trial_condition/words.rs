//! Registry condition words support a factual notice, never local relevance filtering.
use super::{Stub, invoke, setup};
use serde_json::{Value, json};
use std::fs;

pub(super) fn mismatch() -> Result<(), String> {
    let stub = Stub::for_source("clinicaltrials.gov", "application/json", |request| {
        if request.starts_with("GET /api/v2/studies?")
            && request.contains("query.cond=Snyder-Robinson%20syndrome")
        {
            return fs::read_to_string(super::super::launch::fixtures()
                .join("replies/trial/condition-words/broad.body")).map_err(|e| e.to_string());
        }
        if request.starts_with("GET /api/v2/version ") {
            return Ok(json!({"dataTimestamp":"2026-10-27T00:00:00"}).to_string());
        }
        Err(format!("undeclared source request: {request}"))
    })?;
    let env = setup(&stub)?;
    let recording = env.root().join("recording");
    let words = ["search", "trial", "--condition", "Snyder-Robinson syndrome", "--limit", "25"];
    let mut args: Vec<String> = words.iter().map(|v| (*v).into()).collect();
    args.extend(["--record".into(), recording.to_string_lossy().into()]);
    let value = invoke(&env, &args, 0)?;
    check(&value)?;
    if stub.finish()?.len() != 2 {
        return Err("condition notice introduced source requests".into());
    }
    let case = env.root().join("case.json");
    fs::write(&case, json!({"feature":"trial-search",
        "story":"A condition search reports missing literal words and preserves every source trial",
        "replay":recording,"steps":[{"run":words,"mcp":words,
        "expect":{"/result/items":{"count":6},"/sources/0/notices/0/kind":"condition_word_mismatch"}}]
    }).to_string()).map_err(|e| e.to_string())?;
    super::super::run_case(&case, &super::super::schema::Registry::embedded()?)?;
    Ok(())
}

fn check(value: &Value) -> Result<(), String> {
    if value.pointer("/sources/0/notices/0/kind") != Some(&json!("condition_word_mismatch")) {
        return Err(format!("missing factual condition-word notice: {value}"));
    }
    let expected = ["NCT07668869", "NCT03303365", "NCT03285932", "NCT01345552", "NCT01345539", "NCT03297788"];
    let actual: Vec<_> = value.pointer("/result/items").and_then(Value::as_array)
        .ok_or("missing trial items")?.iter().filter_map(|v| v.get("nct_id").and_then(Value::as_str)).collect();
    if actual != expected || value.pointer("/page/total") != Some(&json!(6)) {
        return Err("condition notice dropped, reordered or recounted source rows".into());
    }
    Ok(())
}
