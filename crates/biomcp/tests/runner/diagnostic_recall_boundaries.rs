//! Feature diagnostic-device-recall: exact identity admission and visible enforcement failures.
use super::super::super::{configuration::invoke, maude::environment, mcp, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::json;
pub(super) fn trials() -> Vec<Trial> {
    ["wrong-recall", "wrong-enforcement", "enforcement-error"]
        .into_iter()
        .map(|mode| {
            Trial::test(format!("diagnostic-device-recall/{mode}"), move || {
                run(mode).map_err(Into::into)
            })
        })
        .collect()
}
fn run(mode: &'static str) -> Result<(), String> {
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        move |req| {
            let line = req.lines().next().ok_or("missing request")?;
            let row = if line
                == concat!(
                    "GET /device/recall.json?search=product_res_number.exact%3A%22Z-0001-2024%22",
                    "&limit=1&skip=0 HTTP/1.1"
                ) {
                json!({"product_res_number":if mode=="wrong-recall" {"Z-0002-2024"}else{"Z-0001-2024"},
                "product_code":"ABC","reason_for_recall":"Synthetic reason"})
            } else if line
                == concat!(
                    "GET /device/enforcement.json?search=recall_number.exact%3A%22Z-0001-2024%22",
                    "&limit=1&skip=0 HTTP/1.1"
                )
                && mode != "wrong-recall"
            {
                if mode == "enforcement-error" {
                    return Ok("{}".into());
                }
                json!({"recall_number":"Z-0002-2024","classification":"Class II"})
            } else {
                return Err(format!("wrong recall boundary request: {line}"));
            };
            Ok(
                json!({"meta":{"results":{"skip":0,"limit":1,"total":1}},"results":[row]})
                    .to_string(),
            )
        },
        move |req| {
            if mode == "enforcement-error" && req.starts_with("GET /device/enforcement.json?") {
                400
            } else {
                200
            }
        },
    )?;
    let env = environment(&stub)?;
    let args = [
        "get",
        "diagnostic",
        "Z-0001-2024",
        "--source",
        "fda-device-recall",
    ]
    .map(str::to_owned)
    .to_vec();
    let exit = if mode == "wrong-recall" { 4 } else { 0 };
    let cli = invoke(&env, args.clone(), exit)?;
    let tool = mcp::call_with_globals(&env, &args, exit >= 2, &[])?;
    if cli != tool || cli.pointer("/result/classification").is_some() {
        return Err(format!("wrong boundary result: {cli} / {tool}"));
    }
    if mode == "wrong-recall" {
        if cli.get("result").is_some()
            || cli.pointer("/problems/0/field") != Some(&json!("product_res_number"))
        {
            return Err(format!("wrong recall identity entered result: {cli}"));
        }
    } else if cli.get("status") != Some(&json!("partial"))
        || cli.pointer("/result/header/recall_number") != Some(&json!("Z-0001-2024"))
        || cli.pointer("/result/reason") != Some(&json!("Synthetic reason"))
    {
        return Err(format!("enforcement failure hid the native recall: {cli}"));
    }
    let reason = if mode == "enforcement-error" {
        "rejected"
    } else {
        "bad_reply"
    };
    if cli.pointer("/sources/0/reason") != Some(&json!(reason))
        || cli.pointer("/sources/0/state") != Some(&json!("failed"))
    {
        return Err(format!("enforcement failure looked absent: {cli}"));
    }
    if mode == "wrong-enforcement"
        && cli.pointer("/problems/0/field") != Some(&json!("recall_number"))
    {
        return Err(format!(
            "wrong enforcement identity lacked disclosure: {cli}"
        ));
    }
    if stub.finish()?.len() != if mode == "wrong-recall" { 2 } else { 3 } {
        return Err("wrong identity entered cache or exact get opened extra requests".into());
    }
    Ok(())
}
