//! Feature drug-who-medicines: explicit sync, native cells and exact references.
use super::{configuration::invoke, launch::Environment, mcp, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::{Path, PathBuf}};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("drug-who/native-rows-local-reads-and-replacement", || check().map_err(Into::into))]
}
const CSV: &str = "WHO Reference Number,\"INN, Dosage Form and Strength\",Product Type,Therapeutic Area,Applicant,Dosage Form,Basis of Listing,Date of Prequalification\nBT-ON001,Trastuzumab synthetic powder 150 mg,Synthetic biotherapeutic,Synthetic oncology,Synthetic applicant,Synthetic powder,Synthetic basis,\"18  Dec,  2019\"\nBT-ON002,Trastuzumab synthetic solvent kit,Synthetic biotherapeutic,Synthetic oncology,Synthetic applicant,,Synthetic basis,\nP-13399 - test,,,,,,,\nSYN-003,Trastuzumablike synthetic tablet,Synthetic type,Synthetic area,Synthetic applicant,Synthetic tablet,,";
fn check() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    let search = ["search", "drug", "TRASTUZUMAB", "--source", "who-pq", "--limit", "1"];
    let first = pair(&env, &replay, &search, 0)?;
    if first.pointer("/result/total") != Some(&json!(2))
        || first.pointer("/result/items/0/reference") != Some(&json!("BT-ON001"))
        || first.pointer("/sources/0/use") != Some(&json!("restricted")) {
        return Err(format!("native search lost whole-word matching or terms: {first}"));
    }
    let mut next = search.to_vec();
    next.extend(["--offset", "1"]);
    let second = pair(&env, &replay, &next, 0)?;
    if second.pointer("/result/items/0/dosage_form") != Some(&json!(""))
        || second.pointer("/result/items/0/date_of_prequalification") != Some(&json!("")) {
        return Err(format!("empty native cells changed: {second}"));
    }
    for text in ["trastuz", "Herceptin", "not-a-native-name"] {
        let value = pair(&env, &replay, &["search", "drug", text, "--source", "who-pq"], 1)?;
        if value.pointer("/result/total") != Some(&json!(0)) {
            return Err(format!("search invented substring or alias match: {value}"));
        }
    }
    let get = ["get", "drug", "BT-ON001", "--source", "who-pq"];
    let before = pair(&env, &replay, &get, 0)?;
    if before.pointer("/result/native_rows/0/date_of_prequalification") != Some(&json!("18  Dec,  2019")) {
        return Err(format!("native date spelling changed: {before}"));
    }
    pair(&env, &replay, &["get", "drug", "bt-on001", "--source", "who-pq"], 1)?;
    let test = pair(&env, &replay, &["get", "drug", "P-13399 - test", "--source", "who-pq"], 0)?;
    if test.pointer("/result/native_rows/0/inn_dosage_form_strength") != Some(&json!("")) {
        return Err("test-labelled native reference was discarded or filled".into());
    }
    replacement(&env, &replay, &get, &before)?;
    fs::remove_dir_all(replay.join("who-pq")).map_err(|e| e.to_string())?;
    let mut fresh = get.to_vec(); fresh.push("--fresh");
    if pair(&env, &replay, &fresh, 0)?.get("result") != before.get("result") {
        return Err("fresh read downloaded or changed native rows".into());
    }
    policy(&env, &replay, &get)?;
    Ok(())
}
fn seed(env: &Environment) -> Result<PathBuf, String> {
    let stub = Stub::for_source("extranet.who.int", "text/csv", |_| Ok(CSV.into()))?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    let replay = env.root().join("recorded");
    let args = vec!["--record".into(), replay.to_string_lossy().into(), "--case-cache".into(),
        env.root().join("case-cache").to_string_lossy().into(), "sync".into(), "who-pq".into()];
    let synced = invoke(env, args, 0)?;
    if synced.pointer("/result/rows") != Some(&json!(4)) || stub.finish()?.len() != 1 {
        return Err(format!("explicit sync did not preserve all four native rows: {synced}"));
    }
    Ok(replay)
}
fn pair(env: &Environment, replay: &Path, args: &[&str], exit: i32) -> Result<Value, String> {
    let globals = env.globals(Some(replay), false);
    let mut words = globals.clone(); words.extend(args.iter().map(|s| (*s).into()));
    let value = invoke(env, words, exit)?;
    let command: Vec<String> = args.iter().map(|s| (*s).into()).collect();
    if mcp::call_with_globals(env, &command, exit == 4, &globals)? != value {
        return Err("WHO CLI/MCP outputs differ".into());
    }
    println!("WHO_PQ {} bytes={}", args.join(" "), serde_json::to_vec(&value).map_err(|e| e.to_string())?.len());
    Ok(value)
}
fn replacement(env: &Environment, replay: &Path, get: &[&str], before: &Value) -> Check {
    let path = fs::read_dir(replay.join("who-pq")).map_err(|e| e.to_string())?
        .next().ok_or("missing recording")?.map_err(|e| e.to_string())?.path();
    let original: Value = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    for fault in ["header", "extra", "malformed", "empty"] {
        let mut changed = original.clone();
        let text = match fault {
            "header" => CSV.replace("Applicant", "Changed Applicant"),
            "extra" => CSV.replace("WHO Reference Number", "Extra,WHO Reference Number"),
            "malformed" => format!("{CSV}\n\"unclosed"),
            _ => CSV.lines().next().ok_or("header")?.to_owned(),
        };
        *changed.pointer_mut("/exchanges/0/body/value").ok_or("body")? = json!(text);
        fs::write(&path, changed.to_string()).map_err(|e| e.to_string())?;
        let failed = pair(env, replay, &["sync", "who-pq"], 4)?;
        if failed.pointer("/sources/0/reason") != Some(&json!("bad_reply"))
            || pair(env, replay, get, 0)? != *before {
            return Err(format!("{fault} replaced last validated snapshot: {failed}"));
        }
    }
    Ok(())
}
fn policy(env: &Environment, replay: &Path, get: &[&str]) -> Check {
    for disabled in ["who-pq", "restricted"] {
        env.config(&json!({"disable_sources":[disabled]})).map_err(|e| e.to_string())?;
        for args in [get, &["sync", "who-pq"]] {
            let value = pair(env, replay, args, 4)?;
            if value.get("result").is_some()
                || value.pointer("/sources/0/reason") != Some(&json!("disabled")) {
                return Err(format!("disabled WHO leaked data: {value}"));
            }
        }
    }
    Ok(())
}
