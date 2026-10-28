//! Feature diagnostic-device-recall: exact native recall fields through CLI and MCP.
use super::super::{configuration::invoke, maude::environment, mcp, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("diagnostic-device-recall/native-firm-search", || {
        firm().map_err(Into::into)
    }), Trial::test("diagnostic-device-recall/exact-card-native-ids-and-classification", || {
        cards().map_err(Into::into)
    })]
}
fn firm() -> Result<(), String> {
    let stub = Stub::for_source("api.fda.gov", "application/json", |request| {
        if request.lines().next() != Some(concat!(
            "GET /device/recall.json?search=recalling_firm%3A%22Synthetic%20maker%22",
            "&limit=5&skip=0&sort=event_date_initiated%3Adesc HTTP/1.1"
        )) {
            return Err(format!("wrong native recall query: {request}"));
        }
        Ok(json!({"meta":{"results":{"skip":0,"limit":5,"total":1}},"results":[{
            "product_res_number":"Z-0001-2024","res_event_number":"101",
            "recalling_firm":"Synthetic maker","product_code":"ABC",
            "recall_status":"Ongoing","event_date_initiated":"2024-01-02",
            "openfda":{"k_number":["K999999"],"pma_number":["P999999"]}
        }]}).to_string())
    })?;
    let env = environment(&stub)?;
    let args = ["search", "diagnostic", "--source", "fda-device-recall", "--manufacturer", "Synthetic maker"]
        .map(str::to_owned).to_vec();
    let cli = invoke(&env, args.clone(), 0)?;
    let tool = mcp::call_with_globals(&env, &args, false, &[])?;
    if cli != tool || cli.pointer("/result/items/0/header/recall_number") != Some(&json!("Z-0001-2024"))
        || cli.pointer("/result/count_unit") != Some(&json!("recall_records"))
        || cli.pointer("/page/total") != Some(&json!(1))
        || cli.to_string().contains("K999999") || cli.to_string().contains("P999999")
    {
        return Err(format!("recall fields or count unit changed: {cli} / {tool}"));
    }
    if stub.finish()?.len() != 1 { return Err("warm recall search made extra HTTP requests".into()); }
    Ok(())
}

fn cards() -> Result<(), String> {
    for matched in [true, false] {
        let stub = Stub::for_source_status("api.fda.gov", "application/json", move |request| {
            let line = request.lines().next().ok_or("missing request")?;
            if line == concat!("GET /device/recall.json?search=product_res_number.exact%3A%22Z-0001-2024%22",
                "&limit=1&skip=0 HTTP/1.1") {
                return Ok(json!({"meta":{"results":{"skip":0,"limit":1,"total":1}},"results":[{
                    "product_res_number":"Z-0001-2024","res_event_number":"101",
                    "recalling_firm":"Synthetic maker","product_code":"ABC","recall_status":"Ongoing",
                    "event_date_initiated":"2024-01-02","reason_for_recall":"Synthetic reason",
                    "root_cause_description":"Synthetic cause","product_description":"Synthetic description",
                    "distribution_pattern":"Synthetic distribution","k_numbers":["K000001"],"pma_numbers":["P000001"],
                    "openfda":{"k_number":["K999999"],"pma_number":["P999999"]}
                }]}).to_string());
            }
            if line != concat!("GET /device/enforcement.json?search=recall_number.exact%3A%22Z-0001-2024%22",
                "&limit=1&skip=0 HTTP/1.1") {
                return Err(format!("wrong exact recall request: {line}"));
            }
            Ok(if matched {
                json!({"meta":{"results":{"skip":0,"limit":1,"total":1}},"results":[{
                    "recall_number":"Z-0001-2024","classification":"Class II"
                }]}).to_string()
            } else { json!({"error":{"code":"NOT_FOUND","message":"No matches found!"}}).to_string() })
        }, move |request| {
            if !matched && request.starts_with("GET /device/enforcement.json?") { 404 } else { 200 }
        })?;
        let env = environment(&stub)?;
        let args = ["get", "diagnostic", "Z-0001-2024", "--source", "fda-device-recall"]
            .map(str::to_owned).to_vec();
        let recording = env.root().join("recording");
        let globals = ["--record".into(), recording.to_string_lossy().into_owned()];
        let card = invoke(&env, super::super::launch::arguments(&args, &globals), 0)?;
        for (field, value) in [
            ("kind",json!("device_recall")), ("reason",json!("Synthetic reason")),
            ("root_cause",json!("Synthetic cause")), ("product_description",json!("Synthetic description")),
            ("distribution_pattern",json!("Synthetic distribution")), ("k_numbers",json!(["K000001"])),
            ("pma_numbers",json!(["P000001"]))
        ] {
            if card.pointer(&format!("/result/{field}")) != Some(&value) {
                return Err(format!("missing native recall {field}: {card}"));
            }
        }
        if card.pointer("/result/classification") != matched.then_some(json!("Class II")).as_ref()
            || card.to_string().contains("K999999") || card.to_string().contains("P999999") {
            return Err(format!("classification or harmonized IDs changed: {card}"));
        }
        let pivot = card.get("next").and_then(Value::as_array).and_then(|steps| steps.iter()
            .find(|step| step.get("kind") == Some(&json!("pivot"))))
            .and_then(|step| step.get("args")).ok_or("missing MAUDE pivot")?;
        let words: Vec<String> = serde_json::from_value(pivot.clone()).map_err(|e| e.to_string())?;
        if !words.windows(2).any(|v| v == ["--product-code", "ABC"])
            || !words.windows(2).any(|v| v == ["--source", "maude"])
            || words.iter().any(|v| v == "--manufacturer") { return Err(format!("wrong pivot: {words:?}")); }
        let replay = super::super::launch::Environment::new().map_err(|e| e.to_string())?;
        let globals = replay.globals(Some(&recording), false);
        let cli = invoke(&replay, super::super::launch::arguments(&args, &globals), 0)?;
        let tool = mcp::call_with_globals(&replay, &args, false, &globals)?;
        if cli != tool || cli.get("result") != card.get("result") { return Err("recall replay changed result".into()); }
        if stub.finish()?.len() != 2 { return Err("recall get did not make exactly two source calls".into()); }
    }
    Ok(())
}
