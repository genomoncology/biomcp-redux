//! Feature diagnostic-510k: exact clearance identity, native fields and cache admission.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    let mut trials: Vec<_> = [
        "card",
        "foreign",
        "absent",
        "disabled",
        "invalid",
        "wrong-status",
    ]
    .into_iter()
    .map(|mode| {
        transcript_trial(format!("diagnostic-510k/synthetic-{mode}"), move || {
            run(mode).map_err(Into::into)
        })
    })
    .collect();
    trials.push(transcript_trial(
        "diagnostic-510k/synthetic-companion-pivots",
        || companion().map_err(Into::into),
    ));
    trials
}
fn run(mode: &'static str) -> Result<(), String> {
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        move |request| {
            if !request.starts_with(
                "GET /device/510k.json?search=k_number%3A%22K173492%22&limit=1&skip=0 HTTP/1.1",
            ) {
                return Err(format!("unexpected query: {request}"));
            }
            Ok(if matches!(mode, "absent" | "wrong-status") {
                json!({"error":{"code":"NOT_FOUND","message":"No matches found!"}})
            } else {
                json!(
                    {"meta":{"results":{"skip":0,"limit":1,"total":1}},"results":[{ "k_number":if mode == "foreign" {"K080653"} else {"K173492"},
                    "device_name":"Synthetic device","applicant":"Synthetic applicant","decision_code":"SESD","decision_description":"Unknown",
                    "decision_date":"2017-12-22","date_received":"2017-11-13","product_code":"OYX","openfda":{"device_name":"Unrelated harmonized name",
                    "gmdn":"Do not project"} }]}
                )
            }
            .to_string())
        },
        move |_| if mode == "absent" { 404 } else { 200 },
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    stub.retain(&env);
    std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(
        &json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address),
        "disable_sources":if mode == "disabled" {vec!["fda-510k"]} else {vec![]}}),
    )
    .map_err(|e| e.to_string())?;
    let exit = match mode {
        "invalid" => 2,
        "absent" => 1,
        "foreign" | "disabled" | "wrong-status" => 4,
        _ => 0,
    };
    for _ in 0..2 {
        let args = [
            "get",
            "diagnostic",
            if mode == "invalid" {
                "DEN170088"
            } else {
                "K173492"
            },
            "--source",
            "fda-510k",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        let v = invoke(&env, args, exit)?;
        if mode == "invalid" {
            continue;
        }
        if v.pointer("/sources/0/use") != Some(&json!("open"))
            || v.pointer("/sources/0/terms").is_none()
        {
            return Err(format!("missing source policy: {v}"));
        }
        validate(mode, &v)?;
    }
    let requests = stub.finish()?;
    let expected = match mode {
        "invalid" | "disabled" => 0,
        "foreign" | "absent" | "wrong-status" => 2,
        _ => 1,
    };
    if requests.len() != expected {
        return Err(format!(
            "cache admission: {} requests, expected {expected}",
            requests.len()
        ));
    }
    Ok(())
}

fn validate(mode: &str, v: &Value) -> Result<(), String> {
    let full = serde_json::to_vec(v).map_err(|e| e.to_string())?.len() + 1;
    if mode == "wrong-status" && full > 354 {
        return Err("HTTP 200 error exceeds its measured ceiling".into());
    }
    if mode == "card" && full > 873 || mode == "absent" && full > 325 {
        return Err(format!(
            "clearance envelope exceeds measured budget: {full}"
        ));
    }
    if mode == "wrong-status"
        && (v.get("status") != Some(&json!("failed"))
            || v.pointer("/sources/0/reason") != Some(&json!("bad-reply")))
    {
        return Err(format!("HTTP 200 error became absence: {v}"));
    }
    if mode == "card" {
        for (field, expected) in [
            ("k_number", "K173492"),
            ("decision_description", "Unknown"),
            ("device_name", "Synthetic device"),
            ("applicant", "Synthetic applicant"),
            ("decision_code", "SESD"),
            ("decision_date", "2017-12-22"),
            ("date_received", "2017-11-13"),
            ("product_code", "OYX"),
        ] {
            if v.pointer(&format!("/result/{field}")) != Some(&json!(expected)) {
                return Err(format!("lost native {field}: {v}"));
            }
        }
        if v.pointer("/result/kind") != Some(&json!("clearance"))
            || v.pointer("/result/openfda").is_some()
        {
            return Err(format!("clearance projection: {v}"));
        }
        let actions = v
            .get("next")
            .and_then(Value::as_array)
            .ok_or("missing association action")?;
        if !actions.iter().any(|a| {
            a.get("args")
                == Some(&json!([
                    "search",
                    "diagnostic",
                    "--device-id",
                    "K173492",
                    "--source",
                    "fda-cdx"
                ]))
        }) {
            return Err(format!("missing exact device association: {v}"));
        }
    } else if v.get("result").is_some()
        || mode == "absent" && v.get("status") != Some(&json!("absent"))
        || mode == "foreign" && v.pointer("/problems/0/field") != Some(&json!("k_number"))
    {
        return Err(format!("identity or absence: {v}"));
    }
    Ok(())
}

fn companion() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = super::diagnostic_cdx::seed(&env)?;
    for entry in std::fs::read_dir(replay.join("fda-cdx")).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        let mut receipt: Value =
            serde_json::from_slice(&std::fs::read(&path).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        let body = receipt
            .pointer_mut("/exchanges/0/body/value")
            .ok_or("missing synthetic HTML")?;
        let text = body
            .as_str()
            .ok_or("non-text HTML")?
            .replace("P200010/S026", "K173492")
            .replace("P200010S026", "K173492");
        *body = json!(text);
        std::fs::write(path, receipt.to_string()).map_err(|e| e.to_string())?;
    }
    let mut sync = env.globals(Some(&replay), false);
    sync.extend(["sync".into(), "fda-cdx".into()]);
    invoke(&env, sync, 0)?;
    let mut args = env.globals(Some(&replay), false);
    args.extend(
        [
            "search",
            "diagnostic",
            "--source",
            "fda-cdx",
            "--device-id",
            "K173492",
        ]
        .into_iter()
        .map(String::from),
    );
    let value = invoke(&env, args, 0)?;
    let steps = value
        .get("next")
        .and_then(Value::as_array)
        .ok_or("missing K pivot")?;
    if value.pointer("/page/total") != Some(&json!(1))
        || steps
            .iter()
            .filter(|step| {
                step.get("args")
                    == Some(&json!([
                        "get",
                        "diagnostic",
                        "K173492",
                        "--source",
                        "fda-510k"
                    ]))
            })
            .count()
            != 1
    {
        return Err(format!("exact K row or pivot lost: {value}"));
    }
    Ok(())
}
