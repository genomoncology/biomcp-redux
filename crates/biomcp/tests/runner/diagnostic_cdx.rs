//! Feature diagnostic-cdx: synthetic stored companion rows retain owned identities.
use super::{
    configuration::invoke,
    launch::{Environment, fixtures},
};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("diagnostic-cdx/synthetic-owned-identities", || {
            identities().map_err(Into::into)
        }),
        Trial::test("diagnostic-cdx/synthetic-header-replacement", || {
            replacement().map_err(Into::into)
        }),
        Trial::test("diagnostic-cdx/stored-only-policy", || {
            stored().map_err(Into::into)
        }),
    ]
}
fn call(env: &Environment, replay: &Path, tail: &[&str], exit: i32) -> Result<Value, String> {
    let mut args = env.globals(Some(replay), false);
    args.extend(tail.iter().map(|s| (*s).to_owned()));
    invoke(env, args, exit)
}
fn seed(env: &Environment) -> Result<PathBuf, String> {
    let source = fixtures().join("replies/diagnostic/fda-cdx-synthetic/fda-cdx");
    let replay = env.root().join("replay");
    fs::create_dir_all(replay.join("fda-cdx")).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(source).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        fs::copy(entry.path(), replay.join("fda-cdx").join(entry.file_name()))
            .map_err(|e| e.to_string())?;
    }
    Ok(replay)
}
fn identities() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    let receipt = call(&env, &replay, &["sync", "fda-cdx"], 0)?;
    if receipt.pointer("/result/rows") != Some(&json!(4)) {
        return Err(format!("wrong physical row total: {receipt}"));
    }
    let value = call(
        &env,
        &replay,
        &[
            "search",
            "diagnostic",
            "--source",
            "fda-cdx",
            "--device-id",
            "P200010/S026",
        ],
        0,
    )?;
    if value.pointer("/page/total") != Some(&json!(1))
        || value.pointer("/result/items/0/drug_applications/1/id") != Some(&json!("BLA125084"))
    {
        return Err(format!("combination identities lost: {value}"));
    }
    let args = value
        .get("next")
        .and_then(Value::as_array)
        .ok_or("missing pivots")?;
    for id in ["NDA210496", "BLA125084"] {
        if !args
            .iter()
            .any(|v| v.get("args") == Some(&json!(["get", "drug", id, "--source", "drugsfda"])))
        {
            return Err(format!("missing drug pivot {id}: {value}"));
        }
    }
    let mismatch = call(
        &env,
        &replay,
        &[
            "search",
            "diagnostic",
            "--source",
            "fda-cdx",
            "--device-id",
            "P150013/S020",
        ],
        0,
    )?;
    if mismatch.pointer("/result/items/0/drug_applications") != Some(&json!([]))
        || mismatch.pointer("/problems/0/code") != Some(&json!("identity.conflict"))
    {
        return Err(format!("mismatched cell supplied identity: {mismatch}"));
    }
    let biomarker = call(
        &env,
        &replay,
        &[
            "search",
            "diagnostic",
            "--source",
            "fda-cdx",
            "--biomarker",
            "KRAS",
        ],
        0,
    )?;
    if biomarker.pointer("/page/total") != Some(&json!(3))
        || !biomarker
            .get("problems")
            .and_then(Value::as_array)
            .is_some_and(|a| {
                a.iter()
                    .any(|v| v.get("field") == Some(&json!("table_1.biomarker")))
            })
    {
        return Err(format!("group biomarker unsupported missing: {biomarker}"));
    }
    Ok(())
}
fn replacement() -> Check {
    for header in ["Biomarker(s)", "PMA (Approval Date)"] {
        for fault in ["missing", "extra", "changed", "reordered"] {
            let env = Environment::new().map_err(|e| e.to_string())?;
            let replay = seed(&env)?;
            call(&env, &replay, &["sync", "fda-cdx"], 0)?;
            let args = [
                "search",
                "diagnostic",
                "--source",
                "fda-cdx",
                "--device-id",
                "P200010/S026",
            ];
            let before = call(&env, &replay, &args, 0)?;
            for entry in fs::read_dir(replay.join("fda-cdx")).map_err(|e| e.to_string())? {
                let path = entry.map_err(|e| e.to_string())?.path();
                let mut value: Value =
                    serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
                        .map_err(|e| e.to_string())?;
                let cell = value
                    .pointer_mut("/exchanges/0/body/value")
                    .ok_or("missing body")?;
                let body = cell.as_str().ok_or("non-text body")?;
                let original = format!("<th>{header}</th>");
                let replacement = match fault {
                    "missing" => String::new(),
                    "extra" => format!("{original}<th>Extra</th>"),
                    "reordered" => original.clone(),
                    _ => format!("<th>{header} changed</th>"),
                };
                *cell = json!(if fault == "reordered" {
                    let previous = if header == "Biomarker(s)" {
                        "Drug Trade Name (Generic) NDA / BLA"
                    } else {
                        "Indication(s) - Sample Type"
                    };
                    body.replace(
                        &format!("<th>{previous}</th>{original}"),
                        &format!("{original}<th>{previous}</th>"),
                    )
                } else {
                    body.replace(&original, &replacement)
                });
                fs::write(path, value.to_string()).map_err(|e| e.to_string())?;
            }
            let failed = call(&env, &replay, &["sync", "fda-cdx"], 4)?;
            if failed.pointer("/sources/0/reason") != Some(&json!("bad_reply")) {
                return Err(format!("{header}/{fault} accepted: {failed}"));
            }
            if call(&env, &replay, &args, 0)? != before {
                return Err(format!("{header}/{fault} replaced prior snapshot"));
            }
        }
    }
    Ok(())
}
fn stored() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    let args = [
        "search",
        "diagnostic",
        "--source",
        "fda-cdx",
        "--device-id",
        "P200010/S026",
    ];
    let missing = call(&env, &replay, &args, 4)?;
    if missing.pointer("/sources/0/reason") != Some(&json!("syncing")) {
        return Err(format!("read synchronized: {missing}"));
    }
    call(&env, &replay, &["sync", "fda-cdx"], 0)?;
    let before = call(&env, &replay, &args, 0)?;
    fs::remove_dir_all(replay.join("fda-cdx")).map_err(|e| e.to_string())?;
    let mut fresh = args.to_vec();
    fresh.push("--fresh");
    let after = call(&env, &replay, &fresh, 0)?;
    if after.get("result") != before.get("result") {
        return Err("fresh read synchronized".into());
    }
    env.config(&json!({"disable_sources":["fda-cdx"]}))
        .map_err(|e| e.to_string())?;
    let disabled = call(&env, &replay, &args, 4)?;
    if disabled.get("result").is_some()
        || disabled.pointer("/sources/0/reason") != Some(&json!("disabled"))
    {
        return Err(format!("disabled source leaked data: {disabled}"));
    }
    Ok(())
}
