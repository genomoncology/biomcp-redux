//! Registry condition words support a factual notice, never local relevance filtering.
use super::{Stub, invoke, setup};
use serde_json::{Value, json};
use std::fs;

pub(super) fn mismatch() -> Result<(), String> {
    let stub = Stub::for_source("clinicaltrials.gov", "application/json", |request| {
        if request.starts_with("GET /api/v2/studies?")
            && request.contains("query.cond=Snyder-Robinson%20syndrome")
        {
            if request.contains("filter.advanced=AREA%5BCondition%5DEXPANSION%5BNone%5D") {
                return fs::read_to_string(super::super::launch::fixtures()
                    .join("replies/trial/condition-words/exact.body")).map_err(|e| e.to_string());
            }
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
    exact(&env, &recording, &words, &value)?;
    if stub.finish()?.len() != 3 {
        return Err("condition notice introduced source requests".into());
    }
    let case = env.root().join("case.json");
    fs::write(&case, json!({"feature":"trial-search",
        "story":"A condition search reports missing literal words and preserves every source trial",
        "replay":recording,"steps":[{"run":words,"mcp":words,
        "expect":{"/result/items":{"count":6},"/sources/0/notices/0/kind":"condition_word_mismatch"}},
        {"run":["search","trial","--condition","Snyder-Robinson syndrome","--limit","25","--exact-condition"],
         "mcp":["search","trial","--condition","Snyder-Robinson syndrome","--limit","25","--exact-condition"],
         "expect":{"/page/total":0,"/result/items":[]}}]
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

fn exact(env: &super::Environment, recording: &std::path::Path, words: &[&str], broad: &Value) -> Result<(), String> {
    let mut args: Vec<String> = words.iter().map(|v| (*v).into()).collect();
    args.push("--exact-condition".into());
    let next = broad.pointer("/next/0/args").ok_or("missing exact-condition next command")?;
    let next: Vec<String> = serde_json::from_value(next.clone()).map_err(|e| e.to_string())?;
    if !next.iter().any(|arg| arg == "--exact-condition") || next.iter().filter(|arg| arg.as_str() != "--exact-condition").map(String::as_str).collect::<Vec<_>>() != words {
        return Err(format!("incorrect exact-condition step: {next:?}"));
    }
    args.extend(["--record".into(), recording.to_string_lossy().into()]);
    let result = invoke(env, &args, 0)?;
    if result.pointer("/page/total") != Some(&json!(0)) || result.pointer("/result/items") != Some(&json!([])) {
        return Err(format!("exact condition did not retain the captured zero: {result}"));
    }
    let expected = json!({"name":"filter.advanced","value":r#"AREA[Condition]EXPANSION[None]"Snyder-Robinson syndrome""#});
    if !result.pointer("/result/request").and_then(Value::as_array).is_some_and(|values| values.contains(&expected)) {
        return Err(format!("missing captured condition field restriction: {result}"));
    }
    Ok(())
}

pub(super) fn complete_conditions() -> Result<(), String> {
    let stub = Stub::for_source("clinicaltrials.gov", "application/json", |request| {
        if request.starts_with("GET /api/v2/version ") {
            return Ok(json!({"dataTimestamp":"2026-10-27T00:00:00"}).to_string());
        }
        if !request.starts_with("GET /api/v2/studies?") { return Err("unexpected request".into()); }
        let mut body: Value = serde_json::from_slice(&fs::read(super::super::launch::fixtures()
            .join("replies/trial/condition-words/broad.body")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        for (index, study) in body.get_mut("studies").and_then(Value::as_array_mut).ok_or("missing studies")?.iter_mut().enumerate() {
            let conditions = study.pointer_mut("/protocolSection/conditionsModule/conditions").ok_or("missing conditions")?;
            *conditions = if index % 2 == 0 {
                json!(["unrelated first", "unrelated second", "unrelated third", "SNYDER Robinson SYNDROME"])
            } else { json!(["Snyder-Robinson", "syndrome"]) };
        }
        Ok(body.to_string())
    })?;
    let env = setup(&stub)?;
    let args = ["search", "trial", "--condition", "Snyder-Robinson syndrome"];
    let args: Vec<String> = args.iter().map(|v| (*v).into()).collect();
    let value = invoke(&env, &args, 0)?;
    if value.pointer("/sources/0/notices").is_some() || value.get("next").is_some() {
        return Err(format!("complete native condition words produced a mismatch: {value}"));
    }
    stub.finish()?;
    Ok(())
}

pub(super) fn exact_boundary() -> Result<(), String> {
    for value in [json!({"exact_condition":true}), json!({"source":"nci","condition":"C3224","exact_condition":true})] {
        if serde_json::from_value::<biomcp_types::TrialSearch>(value).is_ok() {
            return Err("exact-condition accepted an unsupported source or absent condition".into());
        }
    }
    let query: biomcp_types::TrialSearch = serde_json::from_value(json!({
        "condition":"Snyder-Robinson syndrome", "criteria":"retained eligibility", "phase":"2", "next_page":"opaque"
    })).map_err(|e| e.to_string())?;
    let exact = query.exact_condition_search().ok_or("missing exact alternative")?;
    if exact.next_page().is_some() || !exact.exact_condition() || exact.criteria() != query.criteria() || exact.phase() != query.phase() {
        return Err("exact condition changed filters or retained cursor".into());
    }
    let params = biomcp_adapters::ctgov::parameters(&exact, None);
    if params.get("query.cond").map(String::as_str) != Some("Snyder-Robinson syndrome") ||
        params.get("filter.advanced").map(String::as_str) != Some(r#"AREA[Condition]EXPANSION[None]"Snyder-Robinson syndrome" AND AREA[Phase]PHASE2 AND AREA[EligibilityCriteria]"retained eligibility""#) {
        return Err(format!("exact condition lost existing advanced conjuncts: {params:?}"));
    }
    Ok(())
}
