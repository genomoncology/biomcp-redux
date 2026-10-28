//! Feature diagnostic-pma: native identity and cardinality guard admission to cache.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
mod metadata;
mod walks;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials: Vec<_> = [
        "wrong-base",
        "wrong-supplement",
        "missing-base-cell",
        "duplicate-base",
        "incomplete",
        "wrong-status",
        "disabled",
    ]
    .into_iter()
    .map(|mode| {
        Trial::test(format!("diagnostic-pma/synthetic-{mode}"), move || {
            boundary(mode).map_err(Into::into)
        })
    })
    .collect();
    trials.push(Trial::test(
        "diagnostic-pma/synthetic-complete-walks",
        || walks::run().map_err(Into::into),
    ));
    trials.push(Trial::test(
        "diagnostic-pma/regulatory-missing-and-empty-statement",
        || metadata::run().map_err(Into::into),
    ));
    trials
}
fn native(mode: &str) -> Result<Value, String> {
    let row = json!({"pma_number":if mode == "wrong-base" {"P200010"} else {"P150013"},
"supplement_number":if mode == "wrong-supplement" {"S021"} else {""},
"trade_name":"Synthetic device",
"ao_statement":"Synthetic statement"});
    let mut rows = vec![row.clone()];
    if mode == "duplicate-base" {
        rows.push(row);
    }
    if mode == "missing-base-cell" {
        rows.first_mut()
            .and_then(Value::as_object_mut)
            .ok_or("missing row")?
            .remove("supplement_number");
    }
    if mode == "incomplete" {
        for n in 1..100 {
            rows.push(json!({"pma_number":"P150013","supplement_number":format!("S{n:03}"),"ao_statement":"Synthetic"}));
        }
    }
    let total = if mode == "incomplete" {
        101
    } else {
        rows.len()
    };
    Ok(json!({"meta":{"results":{"skip":0,"limit":100,"total":total}},"results":rows}))
}
fn boundary(mode: &'static str) -> Check {
    let body = native(mode)?;
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        move |request| {
            if !request.starts_with(&format!(
                "GET /device/pma.json?search=pma_number%3A%22P150013%22{}&limit=100&skip=0 HTTP/1.1",
                if mode == "wrong-supplement" {
                    "%20AND%20supplement_number%3A%22S020%22"
                } else {
                    ""
                }
            )) {
                return Err(format!("wrong exact query: {request}"));
            }
            Ok(if mode == "wrong-status" {
                json!({"error":{"code":"NOT_FOUND","message":"No matches found!"}}).to_string()
            } else {
                body.to_string()
            })
        },
        |_| 200,
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    stub.retain(&env);
    std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    let disabled: Vec<&str> = if mode == "disabled" {
        vec!["fda-pma"]
    } else {
        vec![]
    };
    env.config(&json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address),"disable_sources":disabled}))
        .map_err(|e| e.to_string())?;
    let mut args: Vec<String> = ["get", "diagnostic", "P150013", "--source", "fda-pma"]
        .into_iter()
        .map(String::from)
        .collect();
    if mode == "wrong-supplement" {
        args.extend(["--supplement".into(), "S020".into()]);
    }
    for _ in 0..2 {
        let value = invoke(&env, args.clone(), if mode == "incomplete" { 0 } else { 4 })?;
        if value.get("result").is_some()
            || value.pointer("/sources/0/source") != Some(&json!("fda-pma"))
            || value.pointer("/sources/0/use") != Some(&json!("open"))
            || value.pointer("/sources/0/terms").is_none()
        {
            return Err(format!("failed native boundary: {value}"));
        }
        if mode == "incomplete"
            && (value.get("status") != Some(&json!("partial"))
                || value.pointer("/problems/0/code") != Some(&json!("collection.incomplete")))
        {
            return Err(format!("incomplete: {value}"));
        }
        if matches!(mode, "wrong-base" | "wrong-supplement") {
            identity_problem(mode, &value)?;
        }
        if mode == "disabled" && value.pointer("/sources/0/reason") != Some(&json!("disabled")) {
            return Err(format!("policy: {value}"));
        }
    }
    let requests = stub.finish()?;
    if mode == "disabled" && !requests.is_empty() || mode != "disabled" && requests.len() < 2 {
        return Err(format!(
            "bad reply was cached or disabled source fetched: {}",
            requests.len()
        ));
    }
    Ok(())
}

fn identity_problem(mode: &str, value: &Value) -> Check {
    let baseline: usize = if mode == "wrong-base" { 405 } else { 432 };
    if value.pointer("/problems/0/code") != Some(&json!("identity.conflict"))
        || serde_json::to_vec(value).map_err(|e| e.to_string())?.len()
            > (baseline * 115).div_ceil(100)
    {
        return Err(format!(
            "identity problem missing or exceeds measured ceiling: {value}"
        ));
    }
    Ok(())
}
