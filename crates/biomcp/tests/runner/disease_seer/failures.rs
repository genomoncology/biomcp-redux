use super::{args, invoke, record, replay, setup, wire};
use crate::runner::{launch::Environment, network_stub::Stub};
use serde_json::{Value, json};
use std::fs;
pub(super) fn selectors() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
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
        let output = invoke(&env, &args(&words), 2)?;
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
            json!([{"run":words,"mcp":words,"exit":4,"is_error":true,"expect":{"/status":"failed","/sources/0/source":"seer","/sources/0/state":"failed","/sources/0/reason":reason,"/sources/0/terms":"SEER/NCI Using This Website"},"max_bytes":20000}]),
        )?;
    }
    timeout()?;
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
        json!([{"run":words,"mcp":words,"exit":4,"is_error":true,"expect":{"/sources/0/reason":"deadline_exceeded","/sources/0/source":"seer"},"max_bytes":20000}]),
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
            {"run":words,"expect":{"/page/total":2}},
            {"run":["search","disease","--source","seer","--disable-sources","seer"],"mcp":["search","disease","--source","seer","--disable-sources","seer"],"offline":true,"exit":4,"is_error":true,"expect":{"/sources/0/state":"skipped","/sources/0/reason":"disabled"},"max_bytes":20000}
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
