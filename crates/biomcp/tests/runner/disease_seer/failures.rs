use super::{args, invoke, record, replay, setup, wire};
use crate::runner::{launch::Environment, network_stub::Stub};
use serde_json::{Value, json};
use std::fs;
pub(super) fn selectors() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    invalid_forms(&env)?;
    for (selector, extras, valid) in [
        ("--dataset", vec!["--period", "5"], json!(["3", "1", "4"])),
        (
            "--period",
            vec!["--dataset", "4"],
            json!([
                "31", "21", "1", "23", "13", "3", "33", "14", "15", "35", "36", "16", "4", "9",
                "5", "25", "8", "28", "12"
            ]),
        ),
        (
            "--sex",
            vec!["--dataset", "4", "--period", "5", "--sex", "99"],
            json!(["1", "3", "2"]),
        ),
    ] {
        let mut words = vec!["get", "disease", "47", "incidence", "--source", "seer"];
        words.extend(extras);
        replay(
            env.root(),
            json!([{"run":words,"mcp":words,"exit":2,"is_error":true,
            "expect":{"/status":"usage","/error/option":selector,"/error/valid_values":valid,"/error/source":"seer","/next/0/args":["search","disease","--source","seer"]},"max_bytes":if selector=="--period" {406} else if selector=="--dataset" {319} else {314}}]),
        )?;
    }
    Ok(())
}
fn invalid_forms(env: &Environment) -> Result<(), String> {
    for words in [
        vec![
            "get",
            "disease",
            "55",
            "survival",
            "--source",
            "seer",
            "--dataset",
            "4",
        ],
        vec![
            "get",
            "disease",
            "47",
            "incidence",
            "--source",
            "seer",
            "--period",
            "5",
        ],
        vec![
            "get",
            "disease",
            "47",
            "incidence",
            "--source",
            "seer",
            "--dataset",
            "4",
        ],
        vec![
            "get",
            "disease",
            "55",
            "survival",
            "--source",
            "seer",
            "--dataset",
            "4",
            "--interval",
            "5",
            "--period",
            "31",
        ],
        vec![
            "get",
            "disease",
            "MONDO:0007254",
            "incidence",
            "--source",
            "seer",
        ],
    ] {
        let output = invoke(env, &args(&words), 2)?;
        if crate::runner::source_terms::size(&output, "")? > 406 {
            return Err("invalid selector guidance exceeds its measured ceiling".into());
        }
        if output.get("sources").is_some() {
            return Err("usage opened a source leg".into());
        }
    }
    Ok(())
}
pub(super) fn run() -> Result<(), String> {
    for (status, body, reason) in [
        (429, "{}", "rate_limited"),
        (500, "{}", "unavailable"),
        (200, "broken JSON", "bad_reply"),
    ] {
        let stub = Stub::for_source_status(
            "seer.cancer.gov",
            "application/json",
            move |_| Ok(body.into()),
            move |_| status,
        )?;
        let env = setup(&stub)?;
        let directory = env.root().join("failed");
        let words = ["search", "disease", "--source", "seer"];
        record(&env, &words, &directory, 4)?;
        stub.finish()?;
        replay(
            &directory,
            json!([{"run":words,"mcp":words,"exit":4,"is_error":true,"expect":{"/status":"failed","/sources/0/source":"seer","/sources/0/state":"failed","/sources/0/reason":reason,"/sources/0/terms":"SEER/NCI Using This Website"},"max_bytes":match status {429=>197,500=>196,_=>194}}]),
        )?;
    }
    wrong_site()?;
    timeout()?;
    malformed_statistics()?;
    Ok(())
}
fn malformed_statistics() -> Result<(), String> {
    for kind in [
        "object", "double", "broken", "order", "number", "year", "mismatch",
    ] {
        let path = crate::runner::launch::fixtures().join(format!("replies/disease/seer-synthetic/{kind}/seer/6034e658f1d299da7d80b7b51bc8f9751bd9ce674eaa5b4f328a617e4c81c671.json"));
        let raw: Value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        let body = serde_json::to_string(
            raw.pointer("/exchanges/0/body/value")
                .ok_or("missing wire")?,
        )
        .map_err(|e| e.to_string())?;
        let stub = Stub::for_source_status(
            "seer.cancer.gov",
            "application/json",
            move |_| Ok(body.clone()),
            |_| 200,
        )?;
        let env = setup(&stub)?;
        let directory = env.root().join("statistics");
        let words = [
            "get",
            "disease",
            "55",
            "survival",
            "--source",
            "seer",
            "--sex",
            "3",
            "--dataset",
            "4",
            "--interval",
            "5",
        ];
        let exit = if kind == "mismatch" { 0 } else { 4 };
        record(&env, &words, &directory, exit)?;
        let expect = if kind == "mismatch" {
            json!({"/result/items/0/returned_selectors/5/code":"9","/problems/5/field":"returned_selectors.year_range"})
        } else {
            json!({"/status":"failed","/sources/0/reason":"bad_reply","/sources/0/source":"seer"})
        };
        replay(
            &directory,
            json!([{"run":words,"mcp":words,"exit":exit,"is_error":exit!=0,"expect":expect,"max_bytes":if exit==0 {2675} else {327}}]),
        )?;
        if stub.finish()?.len() != 1 {
            return Err("malformed source retried".into());
        }
    }
    Ok(())
}
fn timeout() -> Result<(), String> {
    let stub = Stub::for_source("seer.cancer.gov", "application/json", wire)?;
    let env = setup(&stub)?;
    let directory = env.root().join("timeout");
    let words = ["search", "disease", "--source", "seer"];
    record(&env, &words, &directory, 0)?;
    stub.finish()?;
    for file in fs::read_dir(directory.join("seer")).map_err(|e| e.to_string())? {
        let file = file.map_err(|e| e.to_string())?;
        let mut value: Value =
            serde_json::from_slice(&fs::read(file.path()).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        *value
            .pointer_mut("/exchanges/0/delay_ms")
            .ok_or("missing delay")? = json!(20000);
        fs::write(
            file.path(),
            serde_json::to_vec(&value).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
    }
    replay(
        &directory,
        json!([{"run":words,"mcp":words,"exit":4,"is_error":true,"expect":{"/sources/0/reason":"deadline_exceeded","/sources/0/source":"seer"},"max_bytes":203}]),
    )?;
    Ok(())
}
pub(super) fn disabled() -> Result<(), String> {
    let stub = Stub::for_source("seer.cancer.gov", "application/json", wire)?;
    let env = setup(&stub)?;
    let directory = env.root().join("disabled");
    let words = ["search", "disease", "--source", "seer"];
    record(&env, &words, &directory, 0)?;
    let result = replay(
        &directory,
        json!([
            {"run":words,"expect":{"/page/total":2},"max_bytes":590},
            {"run":["search","disease","--source","seer","--disable-sources","seer"],"mcp":["search","disease","--source","seer","--disable-sources","seer"],"offline":true,"exit":4,"is_error":true,"expect":{"/sources/0/state":"skipped","/sources/0/reason":"disabled"},"max_bytes":194}
        ]),
    )?;
    if result.get("result").is_some() {
        return Err("disabled SEER returned stored rows".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("disabled source requested again".into());
    }
    Ok(())
}

fn wrong_site() -> Result<(), String> {
    let stub = Stub::for_source("seer.cancer.gov", "application/json", |request| {
        let body = super::wire(request)?;
        let mut encoded: Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;
        let mut document: Value =
            serde_json::from_str(encoded.as_str().ok_or("missing encoded document")?)
                .map_err(|e| e.to_string())?;
        let data = document["data"].as_object_mut().ok_or("missing data")?;
        let old = data.keys().next().ok_or("missing series")?.clone();
        let mut parts = old.split('_').collect::<Vec<_>>();
        *parts.last_mut().ok_or("missing site")? = "999";
        let new = parts.join("_");
        let row = data.remove(&old).ok_or("missing original series")?;
        data.insert(new, row);
        encoded = json!(document.to_string());
        Ok(encoded.to_string())
    })?;
    let env = setup(&stub)?;
    let words = [
        "get",
        "disease",
        "55",
        "survival",
        "--source",
        "seer",
        "--dataset",
        "4",
        "--interval",
        "5",
    ];
    for _ in 0..2 {
        let value = invoke(&env, &args(&words), 4)?;
        if value.pointer("/status") != Some(&json!("failed"))
            || value.pointer("/problems/0/field") != Some(&json!("record"))
            || value.pointer("/result/items/0").is_some()
        {
            return Err(format!("wrong SEER site admitted: {value}"));
        }
    }
    if stub.finish()?.len() != 2 {
        return Err("wrong SEER site entered cache".into());
    }
    Ok(())
}
