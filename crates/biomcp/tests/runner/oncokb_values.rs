//! Synthetic field mutations preserve the checked genomic request and native group.
use super::{KEY, SECTION, args, invoke, reply};
use crate::runner::{launch::Environment, network_stub::Stub};
use serde_json::{Value, json};
use std::fs;

pub(super) fn optional_values() -> Result<(), String> {
    let positive = check(None)?;
    let base = format!("{SECTION}/data");
    if positive.pointer(&format!("{base}/hgvsg")) != Some(&json!("7:g.140753336A>T"))
        || positive.pointer(&format!("{base}/variant_exist")) != Some(&json!(false))
    {
        return Err(format!(
            "original annotation positive control failed: {positive}"
        ));
    }
    for (path, invalid, field, absent) in [
        (
            "/variantExist",
            json!("false"),
            "variantExist",
            "variant_exist",
        ),
        (
            "/query/hugoSymbol",
            json!(42),
            "query.hugoSymbol",
            "hugo_symbol",
        ),
        ("/dataVersion", json!([]), "dataVersion", "data_version"),
    ] {
        let value = check(Some((path, invalid)))?;
        if value.pointer(&format!("{base}/{absent}")).is_some()
            || value.pointer(&format!("{base}/alteration")) != Some(&json!("V600E"))
            || value.pointer(&format!("{base}/hgvsg")) != Some(&json!("7:g.140753336A>T"))
        {
            return Err(format!("bad {field} erased annotation context: {value}"));
        }
        problem(&value, field)?;
        if field == "query.hugoSymbol"
            && value
                .pointer(&format!("{base}/records"))
                .and_then(Value::as_array)
                .is_none_or(|rows| rows.iter().any(|row| row.get("statement").is_some()))
        {
            return Err(format!(
                "bad gene context created a protein statement: {value}"
            ));
        }
    }
    Ok(())
}
pub(super) fn group_values() -> Result<(), String> {
    let positive = check(None)?;
    let original = group(&positive)?;
    if original.get("ordinal") != Some(&json!(0))
        || original.pointer("/drugs/0/label") != Some(&json!("Synthetic drug"))
    {
        return Err("original group positive control failed".into());
    }
    for (path, invalid, field, absent) in [
        (
            "/treatments/0/level",
            json!(42),
            "treatments[0].level",
            "level",
        ),
        (
            "/treatments/0/drugs/0/drugName",
            json!(42),
            "treatments[0].drugs.drugName",
            "drugs/0/label",
        ),
    ] {
        let value = check(Some((path, invalid)))?;
        let row = group(&value)?;
        if row.pointer(&format!("/{absent}")).is_some()
            || row.get("ordinal") != Some(&json!(0))
            || row.pointer("/drugs/0/id") != Some(&json!("NCIT:C0000"))
            || row.get("alterations") != Some(&json!(["V600E", "V600K"]))
            || row.get("statement").is_some()
        {
            return Err(format!(
                "bad {field} erased or broadened native group: {value}"
            ));
        }
        problem(&value, field)?;
    }
    Ok(())
}
fn group(value: &Value) -> Result<&Value, String> {
    value
        .pointer(&format!("{SECTION}/data/records"))
        .and_then(Value::as_array)
        .and_then(|rows| {
            rows.iter()
                .find(|row| row.get("field") == Some(&json!("treatments")))
        })
        .ok_or_else(|| format!("native treatment group missing: {value}"))
}
fn problem(value: &Value, field: &str) -> Result<(), String> {
    let problems = value
        .get("problems")
        .and_then(Value::as_array)
        .ok_or("missing field Problem")?;
    if problems.len() != 1 || problems.first().and_then(|p| p.get("field")) != Some(&json!(field)) {
        return Err(format!("bad {field} needs one named Problem: {value}"));
    }
    Ok(())
}
fn check(mutation: Option<(&'static str, Value)>) -> Result<Value, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_sources(
        &["myvariant.info", "www.oncokb.org"],
        |_| "application/json",
        move |request| {
            let mut body: Value =
                serde_json::from_str(&reply(request)?).map_err(|e| e.to_string())?;
            if request.starts_with("GET /api/v1/annotate/")
                && let Some((path, value)) = &mutation
            {
                *body.pointer_mut(path).ok_or("missing mutation field")? = value.clone();
            }
            Ok(body.to_string())
        },
    )?;
    let ca = env.root().join("synthetic-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    stub.retain(&env);
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca,"keys":{"oncokb_api_key":KEY}})).map_err(|e| e.to_string())?;
    let value = invoke(&env, &args(), 0)?;
    stub.finish()?;
    Ok(value)
}
