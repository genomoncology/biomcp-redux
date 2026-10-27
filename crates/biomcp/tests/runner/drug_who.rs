//! Feature drug-who-medicines: explicit sync, native cells and exact references.
mod navigation;
use super::{configuration::invoke, launch::Environment, mcp, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("drug-who/native-rows-local-reads-and-replacement", || {
            check().map_err(Into::into)
        }),
        Trial::test("drug-who/all-candidates-keep-continuation", || {
            navigation::check().map_err(Into::into)
        }),
    ]
}
const CSV: &str = "WHO Reference Number,\"INN, Dosage Form and Strength\",Product Type,Therapeutic \
    Area,Applicant,Dosage Form,Basis of Listing,Date of Prequalification\nBT-ON001,Trastuzumab \
    synthetic powder 150 mg,Synthetic biotherapeutic,Synthetic oncology,Synthetic applicant,Synthetic \
    powder,Synthetic basis,\"18  Dec,  2019\"\nBT-ON002,Trastuzumab synthetic solvent kit,Synthetic \
    biotherapeutic,Synthetic oncology,Synthetic applicant,,Synthetic basis,\nP-13399 - \
    test,,,,,,,\nSYN-003,Trastuzumablike synthetic tablet,Synthetic type,Synthetic area,Synthetic \
    applicant,Synthetic tablet,,";
fn check() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    let search = [
        "search",
        "drug",
        "TRASTUZUMAB",
        "--source",
        "who-pq",
        "--limit",
        "1",
    ];
    let first = pair(&env, &replay, &search, 0)?;
    if first.pointer("/result/total") != Some(&json!(2))
        || first.pointer("/result/items/0/reference") != Some(&json!("BT-ON001"))
        || first.pointer("/sources/0/use") != Some(&json!("restricted"))
    {
        return Err(format!(
            "native search lost whole-word matching or terms: {first}"
        ));
    }
    let mut next = search.to_vec();
    next.extend(["--offset", "1"]);
    let second = pair(&env, &replay, &next, 0)?;
    if second.pointer("/result/items/0/dosage_form") != Some(&json!(""))
        || second.pointer("/result/items/0/date_of_prequalification") != Some(&json!(""))
    {
        return Err(format!("empty native cells changed: {second}"));
    }
    for text in ["trastuz", "Herceptin", "not-a-native-name"] {
        let value = pair(
            &env,
            &replay,
            &["search", "drug", text, "--source", "who-pq"],
            1,
        )?;
        if value.pointer("/result/total") != Some(&json!(0)) {
            return Err(format!("search invented substring or alias match: {value}"));
        }
    }
    let get = ["get", "drug", "BT-ON001", "--source", "who-pq"];
    let before = pair(&env, &replay, &get, 0)?;
    if before.pointer("/result/native_rows/0/date_of_prequalification")
        != Some(&json!("18  Dec,  2019"))
    {
        return Err(format!("native date spelling changed: {before}"));
    }
    pair(
        &env,
        &replay,
        &["get", "drug", "bt-on001", "--source", "who-pq"],
        1,
    )?;
    let test = pair(
        &env,
        &replay,
        &["get", "drug", "P-13399 - test", "--source", "who-pq"],
        0,
    )?;
    if test.pointer("/result/native_rows/0/inn_dosage_form_strength") != Some(&json!("")) {
        return Err("test-labelled native reference was discarded or filled".into());
    }
    replacement(&env, &replay, &get, &before)?;
    fs::remove_dir_all(replay.join("who-pq")).map_err(|e| e.to_string())?;
    let mut fresh = get.to_vec();
    fresh.push("--fresh");
    if pair(&env, &replay, &fresh, 0)?.get("result") != before.get("result") {
        return Err("fresh read downloaded or changed native rows".into());
    }
    stale(&env, &replay, &get)?;
    policy(&env, &replay, &get)?;
    Ok(())
}
fn seed(env: &Environment) -> Result<PathBuf, String> {
    seed_csv(env, CSV.into())
}
fn seed_csv(env: &Environment, body: String) -> Result<PathBuf, String> {
    let count = body.lines().count().saturating_sub(1);
    let stub = Stub::for_sources(
        &["extranet.who.int"],
        |_| "text/csv",
        move |_| Ok(body.clone()),
    )?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    let replay = env.root().join("recorded");
    let args = vec![
        "--record".into(),
        replay.to_string_lossy().into(),
        "sync".into(),
        "who-pq".into(),
    ];
    let synced = invoke(env, args, 0).map_err(|e| format!("record sync: {e}"))?;
    if synced.pointer("/result/rows") != Some(&json!(count)) || stub.finish()?.len() != 1 {
        return Err(format!(
            "explicit sync did not preserve all native rows: {synced}"
        ));
    }
    pair(env, &replay, &["sync", "who-pq"], 0)?;
    Ok(replay)
}
fn pair(env: &Environment, replay: &Path, args: &[&str], exit: i32) -> Result<Value, String> {
    let globals = env.globals(Some(replay), false);
    let mut words = globals.clone();
    words.extend(args.iter().map(|s| (*s).into()));
    let value = invoke(env, words, exit).map_err(|e| format!("{}: {e}", args.join(" ")))?;
    let command: Vec<String> = args.iter().map(|s| (*s).into()).collect();
    if mcp::call_with_globals(env, &command, exit == 4, &globals)? != value {
        return Err("WHO CLI/MCP outputs differ".into());
    }
    bounded(&value)?;
    Ok(value)
}
fn replacement(env: &Environment, replay: &Path, get: &[&str], before: &Value) -> Check {
    let path = fs::read_dir(replay.join("who-pq"))
        .map_err(|e| e.to_string())?
        .next()
        .ok_or("missing recording")?
        .map_err(|e| e.to_string())?
        .path();
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
        *changed.pointer_mut("/exchanges/0/body").ok_or("body")? =
            json!({"kind":"text", "value":text});
        fs::write(&path, changed.to_string()).map_err(|e| e.to_string())?;
        let failed = pair(env, replay, &["sync", "who-pq"], 4)?;
        if failed.pointer("/sources/0/reason") != Some(&json!("bad_reply"))
            || pair(env, replay, get, 0)? != *before
        {
            return Err(format!(
                "{fault} replaced last validated snapshot: {failed}"
            ));
        }
    }
    Ok(())
}
fn policy(env: &Environment, replay: &Path, get: &[&str]) -> Check {
    for disabled in ["who-pq", "restricted"] {
        env.config(&json!({"disable_sources":[disabled]}))
            .map_err(|e| e.to_string())?;
        for args in [get, &["sync", "who-pq"]] {
            let value = pair(env, replay, args, 4)?;
            if value.get("result").is_some()
                || value.pointer("/sources/0/reason") != Some(&json!("disabled"))
            {
                return Err(format!("disabled WHO leaked data: {value}"));
            }
        }
    }
    Ok(())
}

fn stale(env: &Environment, replay: &Path, get: &[&str]) -> Check {
    let cache = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    // Preserve content-addressed old snapshot bytes while expiring current bundle metadata.
    cache
        .execute_batch(
            "BEGIN; CREATE TEMP TABLE saved_content AS SELECT * FROM bundle_content; UPDATE bundle SET \
        fresh_until=0; INSERT INTO bundle_content SELECT * FROM saved_content; COMMIT;",
        )
        .map_err(|e| e.to_string())?;
    drop(cache);
    for args in [
        get.to_vec(),
        vec!["search", "drug", "trastuzumab", "--source", "who-pq"],
    ] {
        let value = pair(env, replay, &args, 0)?;
        if value.get("status") != Some(&json!("partial"))
            || value.pointer("/sources/0/state") != Some(&json!("stale"))
            || !value
                .get("next")
                .and_then(Value::as_array)
                .is_some_and(|steps| {
                    steps
                        .iter()
                        .any(|step| step.get("args") == Some(&json!(["sync", "who-pq"])))
                })
        {
            return Err(format!(
                "stale WHO data omitted outcome or explicit sync: {value}"
            ));
        }
    }
    Ok(())
}

fn bounded(value: &Value) -> Check {
    let schema = value
        .get("schema")
        .and_then(Value::as_str)
        .ok_or("schema")?;
    let status = value
        .get("status")
        .and_then(Value::as_str)
        .ok_or("status")?;
    // Compact synthetic maxima measured by j13484; every cap adds fifteen percent.
    let baseline: usize = match (schema, status) {
        ("biomcp.drug-who-page/1", "complete") => 937,
        ("biomcp.drug-who-page/1", "partial") => 1242,
        ("biomcp.drug-who-page/1", "absent") => 433,
        ("biomcp.drug-who-card/1", "complete") => 677,
        ("biomcp.drug-who-card/1", "partial") => 729,
        ("biomcp.drug-who-card/1", "absent") => 305,
        ("biomcp.drug-who-card/1", "failed") => 330,
        ("biomcp.sync/5", "complete") => 311,
        ("biomcp.sync/5", "failed") => 302,
        _ => return Err(format!("unmeasured WHO output {schema}/{status}")),
    };
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    if bytes > (baseline * 115).div_ceil(100) {
        return Err(format!("WHO output exceeds measured ceiling: {bytes}"));
    }
    Ok(())
}
