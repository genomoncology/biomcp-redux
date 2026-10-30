use super::{Environment, Stub, association, invoke, record, replay, setup};
use serde_json::{Value, json};
use std::fs;

fn reply(_: &str) -> Result<String, String> {
    Ok(json!({"limit":5,"offset":0,"total":1,"items":[association()]}).to_string())
}
const WORDS: &[&str] = &[
    "get",
    "disease",
    "MONDO:0019003",
    "genes",
    "--source",
    "monarch",
    "--section-limit",
    "5",
];
fn disabled() -> Result<(), String> {
    let stub = Stub::for_source("api-v3.monarchinitiative.org", "application/json", reply)?;
    let env = setup(&stub)?;
    let recording = env.root().join("recording");
    record(&env, WORDS, &recording, 0)?;
    for component in ["monarch", "hpo", "orphanet"] {
        let mut words = WORDS.to_vec();
        words.extend(["--disable-sources", component]);
        let value = invoke(&env, &words, 4)?;
        if value.pointer("/sources/0/state") != Some(&json!("skipped"))
            || value.get("result").is_some()
        {
            return Err(format!("disabled component leaked cached rows: {value}"));
        }
        replay(
            &recording,
            json!([{"run":words,"mcp":words,"exit":4,"is_error":true,"expect":{"/sources/0/state":"skipped","/sources/0/reason":"disabled"},"max_bytes":264}]),
        )?;
    }
    let mut words = WORDS.to_vec();
    words.extend(["--disable-sources", "restricted"]);
    let value = invoke(&env, &words, 0)?;
    if value.pointer("/result/items/0/association/object/id") != Some(&json!("MONDO:0007958")) {
        return Err("restricted group removed cited component".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("disabled section made a network request".into());
    }
    Ok(())
}
fn failures() -> Result<(), String> {
    for (status, body) in [(429, "{}"), (500, "{}"), (200, "broken JSON")] {
        let stub = Stub::for_source_status(
            "api-v3.monarchinitiative.org",
            "application/json",
            move |_| Ok(body.into()),
            move |_| status,
        )?;
        let env = setup(&stub)?;
        let directory = env.root().join("failure");
        record(&env, WORDS, &directory, 4)?;
        stub.finish()?;
        replay(
            &directory,
            json!(
                [{"run":WORDS,"mcp":WORDS,"exit":4,"is_error":true,"expect":{"/status":"failed","/sources/0/source":"monarch",
                "/sources/0/state":"failed","/sources/0/terms":"Monarch source-specific licensing"},"max_bytes":if status==200 {379} else {267}}]
            ),
        )?;
    }
    Ok(())
}
fn timeout() -> Result<(), String> {
    let stub = Stub::for_source("api-v3.monarchinitiative.org", "application/json", reply)?;
    let env = setup(&stub)?;
    let directory = env.root().join("timeout");
    record(&env, WORDS, &directory, 0)?;
    stub.finish()?;
    for file in fs::read_dir(directory.join("monarch")).map_err(|e| e.to_string())? {
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
        json!(
            [{"run":WORDS,"mcp":WORDS,"exit":4,"is_error":true,"expect":{"/sources/0/reason":"deadline-exceeded",
            "/sources/0/terms":"Monarch source-specific licensing"},"max_bytes":273}]
        ),
    )?;
    Ok(())
}
fn unsupported() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    for section in ["synonyms", "hierarchy", "xrefs", ""] {
        let mut words = vec!["get", "disease", "MONDO:0019003"];
        if !section.is_empty() {
            words.push(section);
        }
        words.extend(["--source", "monarch"]);
        let value = invoke(&env, &words, 2)?;
        if value.get("sources").is_some() {
            return Err("unsupported section reached source".into());
        }
    }
    Ok(())
}

fn wrong() -> Result<(), String> {
    let stub = Stub::for_source("api-v3.monarchinitiative.org", "application/json", |_| {
        let mut row = association();
        *row.get_mut("object_closure").ok_or("missing closure")? = json!(["MONDO:9999999"]);
        Ok(json!({"limit":5,"offset":0,"total":1,"items":[row]}).to_string())
    })?;
    let env = setup(&stub)?;
    for _ in 0..2 {
        let value = invoke(&env, WORDS, 4)?;
        if value.pointer("/status") != Some(&json!("failed"))
            || value.pointer("/problems/0/field") != Some(&json!("record"))
            || value.pointer("/result/items/0").is_some()
        {
            return Err(format!("wrong Monarch identity admitted: {value}"));
        }
        if super::super::source_terms::size(&value, "")? > 379 {
            return Err("wrong identity exceeds measured budget".into());
        }
    }
    if stub.finish()?.len() != 2 {
        return Err("wrong Monarch identity entered cache".into());
    }
    Ok(())
}
