//! Feature study-catalog: stored literal search never synchronizes implicitly.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};
type Check = Result<(), String>;
const URL: &str = "https://www.cbioportal.org/api/studies?projection=DETAILED";
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("study-catalog/literal-pages-and-selectors", || {
            search().map_err(Into::into)
        }),
        Trial::test("study-catalog/atomic-replacement-and-policy", || {
            replacement().map_err(Into::into)
        }),
        Trial::test("study-catalog/missing-bundle-explicit-sync", || {
            missing().map_err(Into::into)
        }),
    ]
}
fn rows() -> Value {
    json!((0..12).map(|n| json!({"studyId":format!("synthetic_{n}"),"name":format!("Synthetic Atlas {n}"),"description":"Literal mixed metadata","cancerTypeId":if n < 8 {"brca"} else {"mixed"},"allSampleCount":n,"referenceGenome":"hg19","sequencedSampleCount":n,"cnaSampleCount":0,"structuralVariantCount":0,"treatmentCount":0,"resourceCounts":[],"importDate":"2026-08-20 21:46:46","groups":"","readPermission":true,"publicStudy":true})).collect::<Vec<_>>())
}
fn seed(env: &Environment, body: Value, status: u16) -> Result<PathBuf, String> {
    let dir = env.root().join("replay/cbioportal");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let hash = Sha256::digest([b"GET\0".as_slice(), URL.as_bytes()].concat());
    let key = hash.iter().map(|b| format!("{b:02x}")).collect::<String>();
    let recording = json!({"source":"cbioportal","method":"GET","url":URL,"exchanges":[{"status":status,"content_type":"application/json","delay_ms":0,"retry_after_ms":null,"body":{"kind":"text","value":body.to_string()}}]});
    fs::write(dir.join(format!("{key}.json")), recording.to_string()).map_err(|e| e.to_string())?;
    Ok(env.root().join("replay"))
}
fn call(env: &Environment, replay: &Path, args: &[&str], exit: i32) -> Result<Value, String> {
    let mut command = env.globals(Some(replay), false);
    command.extend(args.iter().map(|s| (*s).into()));
    invoke(env, command, exit)
}
fn missing() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env, rows(), 200)?;
    let v = call(&env, &replay, &["search", "study", "atlas"], 4)?;
    if v.pointer("/sources/0/reason") != Some(&json!("syncing"))
        || v.pointer("/next/0/args") != Some(&json!(["sync", "cbioportal-studies"]))
    {
        return Err(format!("missing bundle: {v}"));
    }
    call(&env, &replay, &["search", "study"], 2)?;
    Ok(())
}
fn search() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env, rows(), 200)?;
    let receipt = call(&env, &replay, &["sync", "cbioportal-studies"], 0)?;
    if receipt.pointer("/result/rows") != Some(&json!(12))
        || receipt
            .pointer("/result/snapshot/acquired")
            .and_then(Value::as_u64)
            .is_none()
    {
        return Err(format!("receipt: {receipt}"));
    }
    fs::remove_dir_all(replay.join("cbioportal")).map_err(|e| e.to_string())?;
    let first = call(&env, &replay, &["search", "study", "ATLAS"], 0)?;
    if first.pointer("/result/items/0/studyId") != Some(&json!("synthetic_0"))
        || first.pointer("/page/returned") != Some(&json!(5))
        || first.pointer("/page/total") != Some(&json!(12))
    {
        return Err(format!("literal search: {first}"));
    }
    let mut current = first.clone();
    let mut ids = Vec::new();
    loop {
        ids.extend(
            current
                .pointer("/result/items")
                .and_then(Value::as_array)
                .ok_or("items")?
                .iter()
                .filter_map(|v| v.get("studyId").and_then(Value::as_str))
                .map(str::to_owned),
        );
        let next = current
            .get("next")
            .and_then(Value::as_array)
            .and_then(|a| a.iter().find(|v| v.get("kind") == Some(&json!("page"))));
        let Some(next) = next else { break };
        let args = next
            .get("args")
            .and_then(Value::as_array)
            .ok_or("args")?
            .iter()
            .map(|v| v.as_str().ok_or("arg"))
            .collect::<Result<Vec<_>, _>>()?;
        current = call(&env, &replay, &args, 0)?;
    }
    if ids
        != (0..12)
            .map(|n| format!("synthetic_{n}"))
            .collect::<Vec<_>>()
    {
        return Err("page walk changed order or lost rows".into());
    }
    for (args, total) in [
        (vec!["search", "study", "--cancer-type", "brca"], 8),
        (vec!["search", "study", "--cancer-type", "BRCA"], 0),
        (
            vec!["search", "study", "synthetic_11", "--cancer-type", "brca"],
            0,
        ),
        (
            vec![
                "search",
                "study",
                "atlas",
                "--cancer-type",
                "mixed",
                "--limit",
                "50",
            ],
            4,
        ),
        (vec!["search", "study", "fabricated"], 0),
        (vec!["search", "study", "synthetic_11", "--limit", "1"], 1),
    ] {
        let value = call(&env, &replay, &args, 0)?;
        if value.pointer("/page/total") != Some(&json!(total)) {
            return Err(format!("selectors: {value}"));
        }
    }
    let pool = first
        .pointer("/page/pool/id")
        .and_then(Value::as_str)
        .ok_or("pool")?;
    let changed = call(
        &env,
        &replay,
        &["search", "study", "other", "--pool", pool],
        4,
    )?;
    if changed.get("result").is_some() {
        return Err("changed selectors reused a pool".into());
    }
    Ok(())
}
fn replacement() -> Check {
    for fault in ["malformed", "conflict", "429", "500"] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = seed(&env, rows(), 200)?;
        call(&env, &replay, &["sync", "cbioportal-studies"], 0)?;
        let mut body = rows();
        if fault == "malformed" {
            body = json!({"unexpected":true});
        }
        if fault == "conflict" {
            let row = body.get(0).ok_or("row")?.clone();
            body.as_array_mut().ok_or("rows")?.push(row);
        }
        let status = fault.parse().unwrap_or(200);
        seed(&env, body, status)?;
        call(&env, &replay, &["sync", "cbioportal-studies"], 4)?;
        let found = call(&env, &replay, &["search", "study", "atlas"], 0)?;
        if found.pointer("/page/total") != Some(&json!(12)) {
            return Err(format!("failed replacement lost old rows: {found}"));
        }
        for group in ["cbioportal", "restricted"] {
            let skipped = call(
                &env,
                &replay,
                &["search", "study", "atlas", "--disable-sources", group],
                4,
            )?;
            if skipped.get("result").is_some()
                || skipped.pointer("/sources/0/reason") != Some(&json!("disabled"))
            {
                return Err(format!("disabled read: {skipped}"));
            }
            call(
                &env,
                &replay,
                &["sync", "cbioportal-studies", "--disable-sources", group],
                4,
            )?;
        }
    }
    Ok(())
}
