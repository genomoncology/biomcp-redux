//! Feature diagnostic-who: exact cards consume a complete explicit stored export.
mod mechanics;
mod search;
use super::{
    configuration::invoke,
    launch::{Environment, fixtures},
};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("diagnostic-who-search/overlapping-phrases", || {
            search::overlapping().map_err(Into::into)
        }),
        transcript_trial(
            "diagnostic-who-search/full-walk-keeps-candidates-and-revision",
            || search::walk().map_err(Into::into),
        ),
        transcript_trial(
            "diagnostic-who-search/replacement-and-policy-admission",
            || search::admission().map_err(Into::into),
        ),
        transcript_trial("diagnostic-who/missing-and-unsupported-never-fetch", || {
            missing().map_err(Into::into)
        }),
        transcript_trial("diagnostic-who/policy-before-stored-data", || {
            policy().map_err(Into::into)
        }),
        transcript_trial("diagnostic-who/failed-replacement-keeps-old-copy", || {
            replacement().map_err(Into::into)
        }),
        transcript_trial(
            "diagnostic-who/two-readers-during-atomic-replacement",
            || mechanics::concurrent().map_err(Into::into),
        ),
        transcript_trial("diagnostic-who/markdown-keeps-native-cells", || {
            mechanics::markdown().map_err(Into::into)
        }),
    ]
}
fn call(env: &Environment, replay: &Path, args: &[&str], exit: i32) -> Result<Value, String> {
    let mut command = env.globals(Some(replay), false);
    command.extend(args.iter().map(|s| (*s).into()));
    invoke(env, command, exit)
}
fn seed(env: &Environment) -> Result<PathBuf, String> {
    let replay = env.root().join("replay");
    fs::create_dir_all(replay.join("who-ivd")).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(fixtures().join("replies/diagnostic/who-synthetic/who-ivd"))
        .map_err(|e| e.to_string())?
    {
        let entry = entry.map_err(|e| e.to_string())?;
        fs::copy(entry.path(), replay.join("who-ivd").join(entry.file_name()))
            .map_err(|e| e.to_string())?;
    }
    Ok(replay)
}
fn missing() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = env.root().join("empty");
    fs::create_dir(&replay).map_err(|e| e.to_string())?;
    let value = call(
        &env,
        &replay,
        &["get", "diagnostic", "0466-046-00", "--source", "who-ivd"],
        4,
    )?;
    if value.pointer("/sources/0/reason") != Some(&json!("syncing"))
        || value.pointer("/next/0/args") != Some(&json!(["sync", "who-ivd"]))
    {
        return Err(format!(
            "missing bundle did not name explicit sync: {value}"
        ));
    }
    for args in [
        vec![
            "get",
            "diagnostic",
            "0466-046-00",
            "--source",
            "who-ivd",
            "genes",
        ],
        vec![
            "get",
            "diagnostic",
            "0466-046-00",
            "--source",
            "who-ivd",
            "--section-limit",
            "1",
        ],
        vec![
            "search",
            "diagnostic",
            "--source",
            "who-ivd",
            "--gene",
            "BRCA1",
        ],
    ] {
        call(&env, &replay, &args, 2)?;
    }
    Ok(())
}
fn policy() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    call(&env, &replay, &["sync", "who-ivd"], 0)?;
    let args = ["get", "diagnostic", "0466-046-00", "--source", "who-ivd"];
    let before = call(&env, &replay, &args, 0)?;
    fs::remove_dir_all(replay.join("who-ivd")).map_err(|e| e.to_string())?;
    let mut fresh = args.to_vec();
    fresh.push("--fresh");
    if call(&env, &replay, &fresh, 0)?.get("result") != before.get("result") {
        return Err("fresh triggered a read-time synchronization".into());
    }
    for setting in ["who-ivd", "noncommercial"] {
        env.config(&json!({"disable_sources":[setting]}))
            .map_err(|e| e.to_string())?;
        for tail in [args.to_vec(), vec!["sync", "who-ivd"]] {
            let value = call(&env, &replay, &tail, 4)?;
            if value.get("result").is_some()
                || value.pointer("/sources/0/reason") != Some(&json!("disabled"))
                || value.pointer("/sources/0/use") != Some(&json!("noncommercial"))
            {
                return Err(format!(
                    "disabled source leaked stored data or terms: {value}"
                ));
            }
        }
    }
    Ok(())
}
fn replacement() -> Check {
    for fault in ["partial", "malformed", "missing-column", "new-column"] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = seed(&env)?;
        call(&env, &replay, &["sync", "who-ivd"], 0)?;
        let args = ["get", "diagnostic", "0466-046-00", "--source", "who-ivd"];
        let before = call(&env, &replay, &args, 0)?;
        for entry in fs::read_dir(replay.join("who-ivd")).map_err(|e| e.to_string())? {
            let path = entry.map_err(|e| e.to_string())?.path();
            let mut value: Value =
                serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            if fault == "partial" {
                *value
                    .pointer_mut("/exchanges/0/status")
                    .ok_or("status missing")? = json!(206);
            } else {
                let cell = value
                    .pointer_mut("/exchanges/0/body/value")
                    .ok_or("body missing")?;
                let text = cell.as_str().ok_or("body not text")?;
                *cell = json!(match fault {
                    "malformed" => format!("{text}\"unclosed"),
                    "missing-column" => text.replace("Product Code", "Missing Code"),
                    _ => text
                        .replace("WHO Product ID,", "WHO Product ID,Extra,")
                        .replace("0466-046-00,", "0466-046-00,,")
                        .replace("12345-12345-00,", "12345-12345-00,,"),
                });
            }
            fs::write(path, value.to_string()).map_err(|e| e.to_string())?;
        }
        let exit = if fault == "new-column" { 0 } else { 4 };
        let synchronized = call(&env, &replay, &["sync", "who-ivd"], exit)?;
        if fault == "new-column"
            && (synchronized.pointer("/result/rows") != Some(&json!(10))
                || synchronized.get("problems").is_some())
        {
            return Err("unknown WHO column changed accepted native rows".into());
        }
        if call(&env, &replay, &args, 0)? != before {
            return Err(format!("{fault} replaced the usable copy"));
        }
    }
    Ok(())
}
