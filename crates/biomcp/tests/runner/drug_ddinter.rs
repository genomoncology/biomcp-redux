//! Feature drug-ddinter: reads never fetch, and failed parts preserve the complete set.
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
        Trial::test(
            "drug-ddinter/failed-parts-preserve-stored-set-and-policy",
            || check().map_err(Into::into),
        ),
        Trial::test(
            "drug-ddinter/missing-local-set-offers-explicit-sync",
            || missing().map_err(Into::into),
        ),
    ]
}
fn missing() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = env.root().join("empty-replay");
    fs::create_dir(&replay).map_err(|e| e.to_string())?;
    for args in [
        vec!["search", "drug", "Synthetic", "--source", "ddinter"],
        vec![
            "get",
            "drug",
            "DDInter1",
            "interactions",
            "--source",
            "ddinter",
        ],
    ] {
        let value = call(&env, &replay, &args, 4)?;
        if value.pointer("/sources/0/state") != Some(&json!("syncing"))
            || value.pointer("/next/0/args") != Some(&json!(["sync", "ddinter"]))
        {
            return Err(format!(
                "read fetched or hid the missing local set: {value}"
            ));
        }
    }
    Ok(())
}

fn call(env: &Environment, replay: &Path, args: &[&str], exit: i32) -> Result<Value, String> {
    let mut command = env.globals(Some(replay), false);
    command.extend(args.iter().map(|s| (*s).into()));
    invoke(env, command, exit)
}
fn seed(env: &Environment) -> Result<PathBuf, String> {
    let source = fixtures().join("replies/drug/ddinter-synthetic/ddinter");
    let replay = env.root().join("replay");
    fs::create_dir_all(replay.join("ddinter")).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(source).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        fs::copy(entry.path(), replay.join("ddinter").join(entry.file_name()))
            .map_err(|e| e.to_string())?;
    }
    Ok(replay)
}
fn check() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    call(&env, &replay, &["sync", "ddinter"], 0)?;
    let args = [
        "get",
        "drug",
        "DDInter3",
        "interactions",
        "--source",
        "ddinter",
    ];
    let before = call(&env, &replay, &args, 0)?;
    let paths: Vec<_> = fs::read_dir(replay.join("ddinter"))
        .map_err(|e| e.to_string())?
        .map(|r| r.map(|e| e.path()).map_err(|e| e.to_string()))
        .collect::<Result<_, _>>()?;
    let (path, original) = paths
        .iter()
        .filter_map(|path| fs::read(path).ok().map(|bytes| (path, bytes)))
        .find(|(_, bytes)| String::from_utf8_lossy(bytes).contains("code_L.csv"))
        .ok_or("missing L partition")?;
    corrupted(&env, &replay, &args, &before, path, &original)?;
    fs::remove_file(path).map_err(|e| e.to_string())?;
    call(&env, &replay, &["sync", "ddinter"], 5)?;
    if call(&env, &replay, &args, 0)? != before {
        return Err("missing partition changed the complete set".into());
    }
    // No recording remains. All subsequent successful commands must be stored-only.
    for path in paths {
        if path.exists() {
            fs::remove_file(path).map_err(|e| e.to_string())?;
        }
    }
    let mut fresh = args.to_vec();
    fresh.push("--fresh");
    if call(&env, &replay, &fresh, 0)? != before {
        return Err("fresh read changed or fetched stored rows".into());
    }
    env.config(&json!({"disable_sources":["noncommercial"]}))
        .map_err(|e| e.to_string())?;
    for query in [
        &args[..],
        &["search", "drug", "Synthetic", "--source", "ddinter"][..],
        &["sync", "ddinter"][..],
    ] {
        let disabled = call(&env, &replay, query, 4)?;
        if disabled.pointer("/sources/0/reason") != Some(&json!("disabled")) {
            return Err(format!("policy admitted stored or replay data: {disabled}"));
        }
    }
    Ok(())
}

fn corrupted(
    env: &Environment,
    replay: &Path,
    args: &[&str],
    before: &Value,
    path: &Path,
    original: &[u8],
) -> Check {
    let header = "DDInterID_A,Drug_A,DDInterID_B,Drug_B,Level\n";
    for row in [
        "wrong,Synthetic A,DDInter1,Synthetic B,Major\n",
        "DDInter1,Synthetic A,DDInter,Synthetic B,Major\n",
        "DDInter1,,DDInter2,Synthetic B,Major\n",
        "DDInter1,Synthetic A,DDInter2, ,Major\n",
        "DDInter1,Synthetic A,DDInter2,Synthetic B,\n",
        "+++ ,+++,+++,+++,+++\n",
        "+++,+++,+++,+++,+++\n",
        "DDInter1,Synthetic A,DDInter2,\"unterminated\n",
        "DDInter1,Synthetic A,DDInter2,Synthetic B\n",
        "",
    ] {
        let mut bad: Value = serde_json::from_slice(original).map_err(|e| e.to_string())?;
        *bad.pointer_mut("/exchanges/0/body/value")
            .ok_or("missing synthetic body")? = format!("{header}{row}").into();
        fs::write(path, bad.to_string()).map_err(|e| e.to_string())?;
        let failed = call(env, replay, &["sync", "ddinter"], 4)?;
        if failed.pointer("/sources/0/reason") != Some(&json!("bad_reply"))
            || call(env, replay, args, 0)? != *before
        {
            return Err(format!(
                "malformed partition changed the complete set: {failed}"
            ));
        }
    }
    Ok(())
}
