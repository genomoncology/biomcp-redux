//! Feature drug-identity: conflicting or absent source identity cannot enter the cache.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = ["wrong-therapy", "missing-join", "conflicting-spl"]
        .into_iter()
        .map(|fault| {
            transcript_trial(
                format!("drug-identity/synthetic-{fault}-before-cache"),
                move || check(fault).map_err(Into::into),
            )
        })
        .collect::<Vec<_>>();
    trials.push(transcript_trial(
        "drug-identity/stale-fallback-keeps-rejected-record-problem",
        || stale().map_err(Into::into),
    ));
    trials
}
fn stale() -> Result<(), String> {
    let mut count = 0;
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        move |_| {
            count += 1;
            let mut value = label();
            if count == 2 {
                corrupt(&mut value, "conflicting-spl")?;
            }
            Ok(value.to_string())
        },
        |_| 200,
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let cert = env.root().join("ca.pem");
    stub.retain(&env);
    fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let args = [
        "get",
        "drug",
        "492dbdb2-077e-4064-bff3-372d6af0a7a2",
        "--source",
        "openfda-label",
    ]
    .map(str::to_owned)
    .to_vec();
    let original = invoke(&env, args.clone(), 0)?;
    let path = fs::read_dir(env.root().join("cache/biomcp"))
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .find(|p| p.extension().is_some_and(|e| e == "sqlite"))
        .ok_or("reply cache")?;
    let cache = rusqlite::Connection::open(path).map_err(|e| e.to_string())?;
    cache
        .execute("UPDATE reply SET fresh_until=0", [])
        .map_err(|e| e.to_string())?;
    let stale = invoke(&env, args.clone(), 0)?;
    if stale.get("status") != Some(&json!("partial"))
        || stale.pointer("/sources/0/state") != Some(&json!("stale"))
        || stale.pointer("/sources/0/reason") != Some(&json!("bad-reply"))
        || stale.get("result") != original.get("result")
        || stale.pointer("/problems/0/field") != Some(&json!("record"))
        || stale.pointer("/problems/0/source") != Some(&json!("openfda-label"))
    {
        return Err(format!(
            "rejected identity lost its record problem during stale fallback: {stale}"
        ));
    }
    let corrected = invoke(&env, args.clone(), 0)?;
    if corrected != original
        || super::mcp::call_with_globals(&env, &args, false, &[])? != corrected
        || stub.finish()?.len() != 3
    {
        return Err("rejected stale identity cached, or corrected identity failed to cache".into());
    }
    let full = serde_json::to_vec(&stale).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(&stale, "")?;
    if full > 897 || comparable > 770 {
        return Err(format!(
            "stale identity refusal exceeds size ceiling: full={full}, comparable={comparable}"
        ));
    }
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "synthetic stale identity: full={full}, comparable={comparable}"
    )
    .map_err(|e| e.to_string())?;
    Ok(())
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
    stub.retain(&env);
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
    let bad = invoke(&env, args.clone(), if civic { 0 } else { 4 })?;
    if civic {
        civic_refusal(&bad, fault)?;
    } else if bad.get("result").is_some()
        || bad.pointer("/sources/0/reason") != Some(&json!("bad-reply"))
        || bad
            .get("next")
            .is_some_and(|v| v.as_array().is_none_or(|a| !a.is_empty()))
    {
        return Err(format!("unsafe identity became a product pick: {bad}"));
    }
    if !civic
        && (bad.pointer("/problems/0/source")
            != Some(&json!(if civic { "civic" } else { "openfda-label" }))
            || bad.pointer("/problems/0/field") != Some(&json!("record")))
    {
        return Err(format!("unsafe identity has no record problem: {bad}"));
    }
    let good = invoke(&env, args.clone(), 0)?;
    if super::mcp::call_with_globals(&env, &args, false, &[])? != good || stub.finish()?.len() != 2
    {
        return Err("unsafe identity cached, or checked identity failed to cache".into());
    }
    refusal_size(&bad, &good, fault, civic)
}
fn refusal_size(bad: &Value, good: &Value, fault: &str, civic: bool) -> Result<(), String> {
    let full = serde_json::to_vec(bad).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(bad, "")?;
    // A partial CIViC reply retains checked sibling records. Bound it against
    // the complete positive control rather than the former whole-reply refusal.
    let (full_bound, comparable_bound): (usize, usize) = if civic {
        (
            serde_json::to_vec(good).map_err(|e| e.to_string())?.len(),
            super::source_terms::size(good, "")?,
        )
    } else {
        (399, 288)
    };
    if full > (full_bound * 115).div_ceil(100)
        || comparable > (comparable_bound * 115).div_ceil(100)
    {
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
        "wrong-therapy" => {
            *value
                .pointer_mut("/data/therapy/id")
                .ok_or("therapy identity")? = json!(84)
        }
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
            *value
                .pointer_mut("/results/0/openfda/spl_set_id")
                .ok_or("SPL identity")? = json!(["b118a40d-6b56-cee3-10f6-ded821a97018"])
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

fn civic_refusal(value: &Value, fault: &str) -> Result<(), String> {
    if value.get("status") != Some(&json!("partial")) {
        return Err("CIViC identity refusal lost partial status".into());
    }
    if fault == "wrong-therapy" {
        if value.get("result").is_some()
            || value.get("next").is_some()
            || value.get("problems")
                != Some(&json!([
                    {"source":"civic","record":"84","field":"id","code":"value.unknown"}
                ]))
        {
            return Err(format!("wrong bound therapy became usable: {value}"));
        }
    } else {
        if value.pointer("/result/identity/id") != Some(&json!("5"))
            || value.pointer("/result/civic/data/evidence/page/returned") != Some(&json!(0))
            || value
                .pointer("/result/civic/data/evidence/records")
                .is_some()
            || value.pointer("/result/civic/data/assertions/records/0/claim/record/id")
                != Some(&json!("AID85"))
        {
            return Err(format!(
                "missing therapy join leaked or lost checked sibling: {value}"
            ));
        }
        if value.get("problems")
            != Some(&json!([
                {"source":"civic","record":"EID259","field":"statement","code":"value.unknown"},
                {"source":"civic","record":"EID259","field":"therapies.id","code":"value.unknown"}
            ]))
        {
            return Err(format!(
                "therapy-join problem lost its named record/field: {value}"
            ));
        }
    }
    Ok(())
}
