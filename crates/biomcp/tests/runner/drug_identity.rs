//! Feature drug-identity: conflicting or absent source identity cannot enter the cache.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    ["wrong-therapy", "missing-join", "conflicting-spl"]
        .into_iter()
        .map(|fault| {
            Trial::test(
                format!("drug-identity/synthetic-{fault}-before-cache"),
                move || check(fault).map_err(Into::into),
            )
        })
        .collect()
}
fn check(fault: &'static str) -> Result<(), String> {
    let civic = fault != "conflicting-spl";
    let host = if civic { "civicdb.org" } else { "api.fda.gov" };
    let mut first = true;
    let stub = Stub::for_source_status(
        host,
        "application/json",
        move |request| {
            let mut value: Value = if civic {
                serde_json::from_str(&super::drug_civic_fixtures::reply(request)?)
                    .map_err(|e| e.to_string())?
            } else {
                label()
            };
            if first {
                corrupt(&mut value, fault)?;
                first = false;
            }
            Ok(value.to_string())
        },
        |_| 200,
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let cert = env.root().join("ca.pem");
    fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let words = if civic {
        vec!["get", "drug", "5", "civic", "--source", "civic"]
    } else {
        vec![
            "get",
            "drug",
            "492dbdb2-077e-4064-bff3-372d6af0a7a2",
            "--source",
            "openfda-label",
        ]
    };
    let args = words.into_iter().map(str::to_owned).collect::<Vec<_>>();
    let bad = invoke(&env, args.clone(), 4)?;
    if bad.get("result").is_some()
        || bad.pointer("/sources/0/reason") != Some(&json!("bad_reply"))
        || bad
            .get("next")
            .is_some_and(|v| v.as_array().is_none_or(|a| !a.is_empty()))
    {
        return Err(format!("unsafe identity became a product pick: {bad}"));
    }
    if bad.pointer("/problems/0/source")
        != Some(&json!(if civic { "civic" } else { "openfda-label" }))
        || bad.pointer("/problems/0/field") != Some(&json!("record"))
    {
        return Err(format!("unsafe identity has no record problem: {bad}"));
    }
    let good = invoke(&env, args.clone(), 0)?;
    if super::mcp::call_with_globals(&env, &args, false, &[])? != good || stub.finish()?.len() != 2
    {
        return Err("unsafe identity cached, or checked identity failed to cache".into());
    }
    let full = serde_json::to_vec(&bad).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(&bad, "")?;
    if full > 8192 || comparable > 8192 {
        return Err("identity refusal exceeds size ceiling".into());
    }
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "synthetic {fault}: full={full} comparable={comparable}"
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
fn corrupt(value: &mut Value, fault: &str) -> Result<(), String> {
    match fault {
        "wrong-therapy" => value["data"]["therapy"]["id"] = json!(84),
        "missing-join" => {
            let therapies = value
                .pointer_mut("/data/evidenceItems/nodes/0/therapies")
                .and_then(Value::as_array_mut)
                .ok_or("claim therapies")?;
            for therapy in therapies {
                therapy.as_object_mut().ok_or("therapy")?.remove("id");
            }
        }
        _ => {
            value["results"][0]["openfda"]["spl_set_id"] =
                json!(["b118a40d-6b56-cee3-10f6-ded821a97018"])
        }
    }
    Ok(())
}
fn label() -> Value {
    json!({"meta":{"results":{"skip":0,"limit":1,"total":1}},"results":[{
        "id":"492dbdb2-077e-4064-bff3-372d6af0a7a2","set_id":"492dbdb2-077e-4064-bff3-372d6af0a7a2",
        "version":"1","effective_time":"20261024","openfda":{"brand_name":["Synthetic brand"],
        "spl_set_id":["492dbdb2-077e-4064-bff3-372d6af0a7a2"]}}]})
}
