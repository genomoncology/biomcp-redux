//! Exact registry protocol integrity through real CLI/MCP and immutable page walks.
#![expect(
    clippy::indexing_slicing,
    reason = "Captured source fixtures and checked envelopes fail directly on shape drift"
)]
use super::{
    launch::Environment,
    response_cases::{checked_steps, copy_capture},
};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
type Check = Result<(), Box<dyn std::error::Error>>;
const IDS: [&str; 8] = [
    "NCT02296125",
    "NCT02578680",
    "NCT03734029",
    "NCT04303780",
    "NCT02032823",
    "NCT02987543",
    "NCT03511664",
    "NCT02563002",
];
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("trial/get/absent-fields-and-version-failure", || {
            optional_source().map_err(|e| e.to_string().into())
        }),
        transcript_trial("trial/get/exact-source-section-walks", || {
            walks().map_err(|e| e.to_string().into())
        }),
        transcript_trial("trial/get/identity-and-failure-boundaries", || {
            failures().map_err(|e| e.to_string().into())
        }),
        transcript_trial("trial/get/independent-input-states", || {
            batch().map_err(|e| e.to_string().into())
        }),
        transcript_trial("trial/get/checked-source-continuations", || {
            continuations().map_err(|e| e.to_string().into())
        }),
    ]
}
pub(super) fn original(id: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let directory = super::launch::fixtures()
        .join("replies/trial/get")
        .join(id)
        .join("ctgov");
    for file in fs::read_dir(directory)? {
        let record: Value = serde_json::from_slice(&fs::read(file?.path())?)?;
        if record["url"].as_str().is_some_and(|url| url.ends_with(id)) {
            return Ok(record["exchanges"][0]["body"]["value"]["protocolSection"].clone());
        }
    }
    Err("detail recording missing".into())
}
pub(super) fn command(
    env: &Environment,
    args: &[String],
    globals: &[String],
    exit: i32,
) -> Result<Value, String> {
    let mut all = args.to_vec();
    all.extend_from_slice(globals);
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &all)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) || !output.stderr.is_empty() {
        return Err(format!(
            "trial command exit {:?}, expected {exit}: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    super::schema::Registry::embedded()?.validate(&value, args, exit == 0)?;
    Ok(value)
}
fn walks() -> Check {
    for id in IDS {
        let raw = original(id)?;
        for section in ["eligibility", "outcomes", "arms"] {
            walk(id, section, &raw)?;
        }
    }
    Ok(())
}
fn walk(id: &str, section: &str, raw: &Value) -> Check {
    let env = Environment::new()?;
    let replay = super::launch::fixtures().join("replies/trial/get").join(id);
    let globals = env.globals(Some(&replay), false);
    let mut args = vec!["get".into(), "trial".into(), id.into(), section.into()];
    let mut joined = Vec::new();
    let mut text = String::new();
    let mut first = None;
    let mut finished = false;
    for page in 0..20 {
        let output = command(&env, &args, &globals, 0)?;
        if page == 0 {
            let mcp = super::mcp::call_with_globals(&env, &args, false, &globals)?;
            if mcp != output {
                return Err(format!("{id} {section} CLI/MCP page parity differs").into());
            }
            first = Some(output["result"]["items"][0]["card"].clone());
        }
        if first.as_ref() != output.pointer("/result/items/0/card") {
            return Err("page changed the captured card".into());
        }
        let data = &output["result"]["items"][0]["sections"][section]["data"];
        if section == "eligibility" {
            if data["offset"] != json!(text.chars().count()) {
                return Err("eligibility skipped or repeated characters".into());
            }
            text.push_str(data["criteria"].as_str().ok_or("criteria missing")?);
        } else {
            if data["offset"] != json!(joined.len()) {
                return Err("record page skipped or repeated rows".into());
            }
            joined.extend(
                data["items"]
                    .as_array()
                    .ok_or("whole records missing")?
                    .iter()
                    .cloned(),
            );
        }
        match super::action(&output, "page") {
            Ok(next) => args = next,
            Err(_) => {
                finished = true;
                break;
            }
        }
    }
    if !finished {
        return Err("bounded trial walk did not finish".into());
    }
    if section == "eligibility" {
        if raw["eligibilityModule"]["eligibilityCriteria"].as_str() != Some(text.as_str()) {
            return Err(format!("{id} lost original eligibility text").into());
        }
    } else if joined != expected_records(raw, section) {
        return Err(format!("{id} {section} lost a source record or attached field").into());
    }
    Ok(())
}
fn expected_records(raw: &Value, section: &str) -> Vec<Value> {
    let mut rows = Vec::new();
    if section == "outcomes" {
        for (collection, kind) in [
            ("primaryOutcomes", "primary"),
            ("secondaryOutcomes", "secondary"),
            ("otherOutcomes", "other"),
        ] {
            for row in raw["outcomesModule"][collection]
                .as_array()
                .into_iter()
                .flatten()
            {
                rows.push(json!({"kind":kind,"measure":row["measure"],"description":row["description"],"time_frame":row["timeFrame"]}));
            }
        }
    } else {
        for row in raw["armsInterventionsModule"]["armGroups"]
            .as_array()
            .into_iter()
            .flatten()
        {
            rows.push(json!(
                {"record":"arm","data":{"label":row["label"],"kind":row["type"],"description":row["description"],
                "intervention_names":row.get("interventionNames").cloned().unwrap_or(json!([]))}}
            ));
        }
        for row in raw["armsInterventionsModule"]["interventions"]
            .as_array()
            .into_iter()
            .flatten()
        {
            rows.push(json!(
                {"record":"intervention","data":{"name":row["name"],"kind":row["type"],"description":row["description"],
                "arm_group_labels":row.get("armGroupLabels").cloned().unwrap_or(json!([])),
                "other_names":row.get("otherNames").cloned().unwrap_or(json!([]))}}
            ));
        }
    }
    for row in &mut rows {
        super::assertions::omit_empty_lists(row);
    }
    rows
}
fn failures() -> Check {
    let args = ["get", "trial", "NCT02296125"];
    for (kind, reason) in [
        ("429", "rate_limited"),
        ("500", "unavailable"),
        ("cut", "bad_reply"),
        ("error", "bad_reply"),
        ("deadline", "deadline_exceeded"),
        ("identity", "bad_reply"),
    ] {
        checked_steps(
            "trial/get/NCT02296125",
            |_, record| {
                if !record["url"]
                    .as_str()
                    .is_some_and(|u| u.ends_with("NCT02296125"))
                {
                    return;
                }
                let mut exchange = record["exchanges"][0].clone();
                exchange["delay_ms"] = json!(0);
                match kind {
                    "429" => {
                        exchange["status"] = json!(429);
                        exchange["retry_after_ms"] = json!(1);
                    }
                    "500" => exchange["status"] = json!(500),
                    "cut" => {
                        exchange["body"] = json!({"kind":"text","value":"{\"protocolSection\":"})
                    }
                    "error" => {
                        exchange["body"] =
                            json!({"kind":"json","value":{"error":"synthetic provider failure"}})
                    }
                    "identity" => {
                        exchange["body"]["value"]["protocolSection"]["identificationModule"]["nctId"] =
                            json!("NCT02578680")
                    }
                    _ => exchange["delay_ms"] = json!(30001),
                }
                record["exchanges"] = json!(vec![
                    exchange;
                    if matches!(kind, "429" | "500") { 3 } else { 1 }
                ]);
            },
            json!([
                {"run":args,"mcp":args,"exit":4,"is_error":true,"expect":{"/status":"failed","/sources/0/reason":reason,"/result/items/0/status":"failed"}},
                {"run":args,"offline":true,"exit":4,"expect":{"/status":"failed","/sources/0/reason":"offline"}}
            ]),
        )?;
    }
    Ok(())
}
fn batch() -> Check {
    let env = Environment::new()?;
    let replay = env.root().join("recordings");
    for id in ["NCT02296125", "NCT01811710", "NCT02578680"] {
        copy_capture(&format!("trial/get/{id}"), &replay, |_, record| {
            if record["url"]
                .as_str()
                .is_some_and(|u| u.ends_with("NCT02578680"))
            {
                let mut exchange = record["exchanges"][0].clone();
                exchange["status"] = json!(500);
                record["exchanges"] = json!([exchange.clone(), exchange.clone(), exchange]);
            }
        })?;
    }
    let args = ["get", "trial", "NCT02296125", "NCT01811710", "NCT02578680"].map(str::to_owned);
    let out = command(&env, &args, &env.globals(Some(&replay), false), 0)?;
    let items = out["result"]["items"]
        .as_array()
        .ok_or("batch items missing")?;
    if out["status"] != "partial"
        || items.len() != 3
        || items[0]["status"] != "complete"
        || items[1]["status"] != "absent"
        || items[2]["status"] != "failed"
    {
        return Err(format!("batch did not retain independent input states: {out}").into());
    }
    Ok(())
}
fn continuations() -> Check {
    let env = Environment::new()?;
    let replay = env.root().join("recordings");
    let original = format!(" \n{} Ω\n", "x  \n".repeat(3500));
    copy_capture("trial/get/NCT02296125", &replay, |_, record| {
        if record["url"]
            .as_str()
            .is_some_and(|u| u.ends_with("NCT02296125"))
        {
            let e = &mut record["exchanges"][0]["body"]["value"]["protocolSection"]["eligibilityModule"];
            e["eligibilityCriteria"] = json!(original);
            e.as_object_mut().map(|e| e.remove("maximumAge"));
        }
    })?;
    let args = ["get", "trial", "NCT02296125", "eligibility"].map(str::to_owned);
    let globals = env.globals(Some(&replay), false);
    let first = command(&env, &args, &globals, 0)?;
    if first.pointer("/result/items/0/sections/eligibility/data/maximum_age/state")
        != Some(&json!("no_limit"))
    {
        return Err("missing maximum age invented a number".into());
    }
    let mut next = super::action(&first, "page")?;
    let mut joined = first
        .pointer("/result/items/0/sections/eligibility/data/criteria")
        .and_then(Value::as_str)
        .ok_or("missing text")?
        .to_owned();
    // No recording is available to continuations: only the admitted immutable snapshot can answer.
    let offline = env.globals(None, true);
    for _ in 0..10 {
        let out = command(&env, &next, &offline, 0)?;
        let data = &out["result"]["items"][0]["sections"]["eligibility"]["data"];
        if data["offset"] != json!(joined.chars().count()) {
            return Err("whitespace fragment offset changed".into());
        }
        joined.push_str(data["criteria"].as_str().ok_or("fragment missing")?);
        match super::action(&out, "page") {
            Ok(args) => next = args,
            Err(_) => break,
        }
    }
    if joined != original {
        return Err("continuation changed source Unicode or whitespace".into());
    }
    let mut disabled = next.clone();
    disabled.extend(["--disable-sources".into(), "ctgov".into()]);
    let out = command(&env, &disabled, &offline, 4)?;
    if out["sources"][0]["reason"] != "disabled" || out.pointer("/result/items/0/card").is_some() {
        return Err("disabled source restored a stored protocol".into());
    }
    altered_pool(&env, &next, &offline)?;
    Ok(())
}
fn altered_pool(env: &Environment, args: &[String], globals: &[String]) -> Check {
    for selector in ["NCT02578680", "offset", "pool"] {
        let mut changed = args.to_vec();
        if selector.starts_with("NCT") {
            changed[2] = selector.into();
        } else {
            let value = changed
                .iter()
                .position(|s| s == &format!("--{selector}"))
                .ok_or("missing continuation selector")?
                + 1;
            changed[value] = if selector == "offset" {
                "4294967295".into()
            } else {
                "unknown-snapshot".into()
            };
        }
        let exit = if selector == "offset" { 2 } else { 3 };
        let out = command(env, &changed, globals, exit)?;
        if out["status"]
            != if selector == "offset" {
                "usage"
            } else {
                "expired"
            }
        {
            return Err("changed continuation was accepted".into());
        }
    }
    Ok(())
}

fn optional_source() -> Check {
    checked_steps(
        "trial/get/NCT02296125",
        |_, record| {
            if record["url"]
                .as_str()
                .is_some_and(|u| u.ends_with("NCT02296125"))
            {
                let protocol = &mut record["exchanges"][0]["body"]["value"]["protocolSection"];
                if let Some(fields) = protocol.as_object_mut() {
                    for key in [
                        "conditionsModule",
                        "descriptionModule",
                        "sponsorCollaboratorsModule",
                        "eligibilityModule",
                        "outcomesModule",
                        "armsInterventionsModule",
                    ] {
                        fields.remove(key);
                    }
                }
                if let Some(design) = protocol["designModule"].as_object_mut() {
                    design.remove("phases");
                }
                if let Some(status) = protocol["statusModule"].as_object_mut() {
                    for key in [
                        "startDateStruct",
                        "primaryCompletionDateStruct",
                        "completionDateStruct",
                    ] {
                        status.remove(key);
                    }
                }
            }
        },
        json!([{"run":["get","trial","NCT02296125","all"],"expect":{
            "/status":"complete",
            "/result/items/0/card/start":null,"/result/items/0/card/brief_summary":null,"/result/items/0/card/sponsor":null,
            "/result/items/0/sections/eligibility/state":"empty","/result/items/0/sections/outcomes/state":"empty","/result/items/0/sections/arms/state":"empty"
        }}]),
    )?;
    checked_steps(
        "trial/get/NCT02296125",
        |_, record| {
            if record["url"]
                .as_str()
                .is_some_and(|u| u.ends_with("/version"))
            {
                let mut exchange = record["exchanges"][0].clone();
                exchange["status"] = json!(500);
                record["exchanges"] = json!([exchange.clone(), exchange.clone(), exchange]);
            }
        },
        json!(
            [{"run":["get","trial","NCT02296125"],"expect":{ "/status":"partial","/result/items/0/status":"partial",
            "/result/items/0/card/nct_id":"NCT02296125","/sources/0/reason":"unavailable" }}]
        ),
    )?;
    Ok(())
}
