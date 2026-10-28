//! Feature diagnostic-device-recall: exact native recall fields through CLI and MCP.
use super::super::{configuration::invoke, maude::environment, mcp, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::json;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("diagnostic-device-recall/native-firm-search", || {
        firm().map_err(Into::into)
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
