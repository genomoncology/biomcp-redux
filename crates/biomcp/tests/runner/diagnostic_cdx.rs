//! Feature diagnostic-cdx: synthetic stored companion rows retain owned identities.
use super::{configuration::invoke, launch::{Environment, fixtures}};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::{Path, PathBuf}};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("diagnostic-cdx/synthetic-owned-identities", || identities().map_err(Into::into))]
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
        fs::copy(entry.path(), replay.join("fda-cdx").join(entry.file_name())).map_err(|e| e.to_string())?;
    }
    Ok(replay)
}
fn identities() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    let receipt = call(&env, &replay, &["sync", "fda-cdx"], 0)?;
    if receipt.pointer("/result/rows") != Some(&json!(4)) { return Err(format!("wrong physical row total: {receipt}")); }
    let value = call(&env, &replay, &["search", "diagnostic", "--source", "fda-cdx", "--device-id", "P200010/S026"], 0)?;
    if value.pointer("/page/total") != Some(&json!(1)) || value.pointer("/result/items/0/drug_applications/1/id") != Some(&json!("BLA125084")) { return Err(format!("combination identities lost: {value}")); }
    let args = value.get("next").and_then(Value::as_array).ok_or("missing pivots")?;
    for id in ["NDA210496", "BLA125084"] {
        if !args.iter().any(|v| v.get("args") == Some(&json!(["get", "drug", id, "--source", "drugsfda"]))) { return Err(format!("missing drug pivot {id}: {value}")); }
    }
    let mismatch = call(&env, &replay, &["search", "diagnostic", "--source", "fda-cdx", "--device-id", "P150013/S020"], 0)?;
    if mismatch.pointer("/result/items/0/drug_applications") != Some(&json!([])) || mismatch.pointer("/problems/0/code") != Some(&json!("identity.conflict")) { return Err(format!("mismatched cell supplied identity: {mismatch}")); }
    let biomarker = call(&env, &replay, &["search", "diagnostic", "--source", "fda-cdx", "--biomarker", "KRAS"], 0)?;
    if biomarker.pointer("/page/total") != Some(&json!(3)) || !biomarker.get("problems").and_then(Value::as_array).is_some_and(|a| a.iter().any(|v|v.get("code")==Some(&json!("query.unsupported")))) { return Err(format!("group biomarker unsupported missing: {biomarker}")); }
    Ok(())
}
