//! Feature maude-reports: synthetic source-native device reports through CLI and MCP.
use super::{configuration::invoke as raw_invoke, launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) type Check = Result<(), String>;
mod filters;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = filters::trials();
    trials.extend(vec![
        transcript_trial("maude-reports/synthetic-snapshot-survives-refresh", || {
            super::maude_snapshot::run().map_err(Into::into)
        }),
        transcript_trial(
            "maude-reports/component-admission-before-cache-replay-http",
            || super::maude_policy::run().map_err(Into::into),
        ),
        transcript_trial(
            "maude-reports/retained-d03-no-match-adapter-fixture",
            || absence().map_err(Into::into),
        ),
        transcript_trial("maude-reports/synthetic-search-and-card", || {
            search_card().map_err(Into::into)
        }),
        transcript_trial(crate::runner::required_contract::MAUDE_SECTIONS.0, || {
            sections().map_err(Into::into)
        }),
        transcript_trial("maude-reports/synthetic-policy-before-io", || {
            policy().map_err(Into::into)
        }),
        transcript_trial("maude-reports/synthetic-wrong-key-never-cached", || {
            wrong_key().map_err(Into::into)
        }),
        transcript_trial("maude-reports/synthetic-oversized-narrative", || {
            oversized().map_err(Into::into)
        }),
    ]);
    trials
}
pub(super) fn row() -> Value {
    json!(
        {"mdr_report_key":"160","report_number":"Synthetic-160","event_type":"Other","date_of_event":"20240101","date_received":"20240202",
        "report_date":"20240303","date_report":"","number_devices_in_event":"9","number_patients_in_event":"2",
        "device":[{"device_sequence_number":"1","generic_name":"Synthetic device","brand_name":"","manufacturer_d_name":"Synthetic maker",
        "manufacturer_d_address_1":"Excluded"},{"device_sequence_number":"2","model_number":"MODEL2"}],"mdr_text":[{"mdr_text_key":"A",
        "patient_sequence_number":"1","text_type_code":"D","text":"Synthetic complete first narrative."},{"mdr_text_key":"B","text":""},
        {"mdr_text_key":"C"}]}
    )
}
pub(super) fn response(row: Value) -> String {
    json!({"meta":{"last_updated":"2024-09-01","results":{"skip":0,"limit":1,"total":1}},"results":[row]}).to_string()
}
pub(super) fn environment(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    stub.retain(&env);
    std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
pub(super) fn invoke(env: &Environment, args: Vec<String>, exit: i32) -> Result<Value, String> {
    let value = raw_invoke(env, args, exit)?;
    super::maude_budget::check(&value)?;
    Ok(value)
}
pub(super) fn both(env: &Environment, words: &[&str], exit: i32) -> Result<Value, String> {
    let args: Vec<String> = words.iter().map(|v| (*v).into()).collect();
    let cli = invoke(env, args.clone(), exit)?;
    super::schema::Registry::embedded()?.validate(&cli, &args, true)?;
    let mcp = super::mcp::call_with_globals(env, &args, exit >= 2, &[])?;
    if !super::reply_comparison::same_reply(&cli, &mcp)? {
        return Err(format!("MAUDE CLI/MCP differ: {cli} / {mcp}"));
    }
    Ok(cli)
}
fn search_card() -> Check {
    let stub = Stub::for_source("api.fda.gov", "application/json", |req| {
        let first = req.lines().next().ok_or("request")?;
        if first
            == "GET /device/event.json?search=device.generic_name%3A%22Synthetic%20device%22&limit=5&skip=0&sort=date_received%3Adesc HTTP/1.1"
        {
            return Ok(json!({"meta":{"last_updated":"2024-09-01","results":{"skip":0,"limit":5,"total":1}},"results":[row()]}).to_string());
        }
        if first != "GET /device/event.json?search=mdr_report_key%3A%22160%22&limit=1 HTTP/1.1" {
            return Err(format!("unexpected query {first}"));
        }
        Ok(response(row()))
    })?;
    let env = environment(&stub)?;
    let search = both(
        &env,
        &[
            "search",
            "adverse-event",
            "Synthetic device",
            "--source",
            "maude",
        ],
        0,
    )?;
    if search.pointer("/result/source_query")
        != Some(&json!("device.generic_name:\"Synthetic device\""))
        || search.pointer("/result/items/0/id") != Some(&json!("160"))
        || search.pointer("/result/unit") != Some(&json!("reports"))
        || search.pointer("/page/total") != Some(&json!(1))
    {
        return Err(format!("report search lost native facts: {search}"));
    }
    let card = both(
        &env,
        &["get", "adverse-event", "160", "--source", "maude"],
        0,
    )?;
    for (key, expected) in [
        ("report_number", json!("Synthetic-160")),
        ("event_date", json!("20240101")),
        ("received", json!("20240202")),
        ("report_date", json!("20240303")),
        ("date_report", json!("")),
        ("number_devices_in_event", json!("9")),
        ("number_patients_in_event", json!("2")),
        ("device_count", json!(2)),
        ("narrative_count", json!(3)),
    ] {
        if card.pointer(&format!("/result/{key}")) != Some(&expected) {
            return Err(format!("lost {key}: {card}"));
        }
    }
    if card.pointer("/result/date_report_to_fda").is_some()
        || card.pointer("/result/snapshot").is_none()
        || card.pointer("/result/report_url")
            != Some(&json!(
                "https://api.fda.gov/device/event.json?search=mdr_report_key%3A%22160%22&limit=1"
            ))
    {
        return Err(format!("card fabricated or lost identity: {card}"));
    }
    if stub.finish()?.len() != 2 {
        return Err("warm search/card reached HTTP".into());
    }
    Ok(())
}
fn sections() -> Check {
    let stub = Stub::for_source("api.fda.gov", "application/json", |_| Ok(response(row())))?;
    let env = environment(&stub)?;
    let devices = both(
        &env,
        &[
            "get",
            "adverse-event",
            "160",
            "devices",
            "--source",
            "maude",
            "--section-limit",
            "1",
        ],
        0,
    )?;
    if devices.pointer("/result/devices/total") != Some(&json!(2))
        || devices.pointer("/result/devices/records/0/brand_name") != Some(&json!(""))
        || devices.to_string().contains("Excluded")
    {
        return Err(format!("device projection changed: {devices}"));
    }
    let next = devices
        .get("next")
        .and_then(Value::as_array)
        .and_then(|v| v.iter().find(|v| v.get("kind") == Some(&json!("page"))))
        .and_then(|v| v.get("args"))
        .ok_or("device next page")?;
    let args: Vec<String> = serde_json::from_value(next.clone()).map_err(|e| e.to_string())?;
    let second = invoke(&env, args, 0)?;
    if second.pointer("/result/devices/records/0/index") != Some(&json!(1))
        || second.pointer("/result/devices/records/0/model") != Some(&json!("MODEL2"))
    {
        return Err("device page lost index/model".into());
    }
    let narrative = both(
        &env,
        &[
            "get",
            "adverse-event",
            "160",
            "narrative",
            "--source",
            "maude",
        ],
        0,
    )?;
    if narrative.pointer("/result/narrative/records/0/text")
        != Some(&json!("Synthetic complete first narrative."))
        || narrative.pointer("/result/narrative/records/1/text") != Some(&json!(""))
        || narrative
            .pointer("/result/narrative/records/2/text")
            .is_some()
    {
        return Err(format!("whole narrative/missingness lost: {narrative}"));
    }
    if devices.pointer("/result/snapshot") != narrative.pointer("/result/snapshot")
        || stub.finish()?.len() != 1
    {
        return Err("report sections changed snapshot or fetched twice".into());
    }
    Ok(())
}
fn policy() -> Check {
    let stub = Stub::for_source("api.fda.gov", "application/json", |_| {
        Err("invalid input reached HTTP".into())
    })?;
    let env = environment(&stub)?;
    for flags in [
        ["--reaction", "term"],
        ["--serious", "death"],
        ["--received-from", "2024-01-01"],
        ["--received-to", "2024-12-31"],
    ] {
        both(
            &env,
            &[
                "search",
                "adverse-event",
                "Synthetic device",
                "--source",
                "maude",
                flags[0],
                flags[1],
            ],
            2,
        )?;
    }
    both(
        &env,
        &["get", "adverse-event", "not-numeric", "--source", "maude"],
        2,
    )?;
    if !stub.finish()?.is_empty() {
        return Err("invalid MAUDE selector reached source".into());
    }
    Ok(())
}
fn wrong_key() -> Check {
    let stub = Stub::for_source("api.fda.gov", "application/json", |_| {
        let mut r = row();
        *r.get_mut("mdr_report_key").ok_or("synthetic key")? = json!("999");
        Ok(response(r))
    })?;
    let env = environment(&stub)?;
    let value = both(
        &env,
        &["get", "adverse-event", "160", "--source", "maude"],
        4,
    )?;
    if value.get("result").is_some()
        || value.pointer("/problems/0/field") != Some(&json!("report"))
        || value.pointer("/sources/0/reason") != Some(&json!("bad-reply"))
        || stub.finish()?.len() != 2
    {
        return Err("wrong report entered cache or result".into());
    }
    Ok(())
}
fn oversized() -> Check {
    let stub = Stub::for_source("api.fda.gov", "application/json", |_| {
        let mut r = row();
        *r.get_mut("mdr_text").ok_or("synthetic narrative")? =
            json!([{"text":"X".repeat(100_000)}]);
        Ok(response(r))
    })?;
    let env = environment(&stub)?;
    let value = both(
        &env,
        &[
            "get",
            "adverse-event",
            "160",
            "narrative",
            "--source",
            "maude",
        ],
        4,
    )?;
    if value.get("result").is_some()
        || !value.to_string().contains("too-large")
        || value.to_string().contains("XXXXX")
    {
        return Err(format!("oversized narrative was cut or returned: {value}"));
    }
    Ok(())
}

fn absence() -> Check {
    // The unchanged D03 error payload is an adapter fixture, not a positive SCC CLI query.
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        |_| Ok(r#"{"error":{"code":"NOT_FOUND","message":"No matches found!"}}"#.into()),
        |_| 404,
    )?;
    let env = environment(&stub)?;
    let value = both(
        &env,
        &[
            "search",
            "adverse-event",
            "Synthetic empty device",
            "--source",
            "maude",
        ],
        1,
    )?;
    if value.get("status") != Some(&json!("absent"))
        || value.pointer("/result/items").is_some()
        || value.pointer("/page/total") != Some(&json!(0))
        || value.pointer("/sources/0/state") != Some(&json!("ok"))
    {
        return Err(format!("native absence became failure: {value}"));
    }
    stub.finish()?;
    Ok(())
}
