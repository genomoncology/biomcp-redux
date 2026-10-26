//! Feature study-catalog: stored literal search never synchronizes implicitly.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};
type Check = Result<(), String>;
const URL: &str = "https://www.cbioportal.org/api/studies?projection=DETAILED";
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("study-catalog/replacement-binds-pool-revision", || {
            revision().map_err(Into::into)
        }),
        Trial::test("study-catalog/expiry-stale-and-zero-network", || {
            lifetimes().map_err(Into::into)
        }),
        Trial::test("study-catalog/mcp-markdown-help", || {
            surfaces().map_err(Into::into)
        }),
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
    let key = "6a22562d8dcf66701258cac4800e9ad4d5275fdfd1ac5eee038c0e4c9d651085";
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
    for fault in [
        "malformed",
        "conflict",
        "429",
        "500",
        "oversized",
        "partial",
        "timeout",
    ] {
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
        alter_fault(&replay, fault)?;
        let failure = call(&env, &replay, &["sync", "cbioportal-studies"], 4)?;
        if failure.pointer("/sources/0/state") != Some(&json!("failed"))
            || failure.pointer("/sources/0/use") != Some(&json!("restricted"))
            || failure.pointer("/sources/0/terms").is_none()
        {
            return Err(format!(
                "sync fault lost terms or source outcome: {failure}"
            ));
        }
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
fn lifetimes() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env, rows(), 200)?;
    call(&env, &replay, &["sync", "cbioportal-studies"], 0)?;
    let first = call(&env, &replay, &["search", "study", "atlas"], 0)?;
    let id = first
        .pointer("/page/pool/id")
        .and_then(Value::as_str)
        .ok_or("pool")?;
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute("UPDATE pool SET expires=0 WHERE id=?1", [id])
        .map_err(|e| e.to_string())?;
    fs::remove_dir_all(replay.join("cbioportal")).map_err(|e| e.to_string())?;
    let expired = call(
        &env,
        &replay,
        &["search", "study", "atlas", "--pool", id, "--offset", "5"],
        4,
    )?;
    if expired.get("result").is_some() || expired.pointer("/next/0/kind") != Some(&json!("restart"))
    {
        return Err(format!("expired snapshot: {expired}"));
    }
    db.execute(
        "UPDATE bundle SET fresh_until=0 WHERE name='cbioportal-studies'",
        [],
    )
    .map_err(|e| e.to_string())?;
    // Bundle mutation invalidates provenance through the ordinary store trigger.
    db.execute(
        "INSERT INTO bundle_content VALUES ('cbioportal-studies','sha256:synthetic-stale',1)",
        [],
    )
    .map_err(|e| e.to_string())?;
    let stale = call(&env, &replay, &["search", "study", "atlas", "--fresh"], 0)?;
    if stale.get("status") != Some(&json!("partial"))
        || stale.pointer("/sources/0/state") != Some(&json!("stale"))
        || !stale
            .get("next")
            .and_then(Value::as_array)
            .is_some_and(|a| a.iter().any(|v| v.get("kind") == Some(&json!("sync"))))
    {
        return Err(format!("stale stored catalog: {stale}"));
    }
    Ok(())
}
fn surfaces() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env, rows(), 200)?;
    call(&env, &replay, &["sync", "cbioportal-studies"], 0)?;
    let args = vec!["search".into(), "study".into(), "atlas".into()];
    let mcp =
        super::mcp::call_with_globals(&env, &args, false, &env.globals(Some(&replay), false))?;
    if mcp.pointer("/result/items/0/studyId") != Some(&json!("synthetic_0")) {
        return Err("MCP study argument mapping".into());
    }
    let mut args = env.globals(Some(&replay), false);
    args.extend(["search", "study", "atlas", "--format", "md"].map(str::to_owned));
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    if !output.status.success()
        || !text.contains("Synthetic Atlas 0")
        || !text.contains("get study synthetic_0")
        || !text.contains("cBioPortal study-specific terms")
    {
        return Err("study Markdown lost native rows, actions or terms".into());
    }
    let help = invoke(
        &env,
        vec!["help".into(), "search".into(), "study".into()],
        0,
    )?;
    if !help.to_string().contains("cancer_type") {
        return Err("study executable help omitted selector".into());
    }
    Ok(())
}

fn alter_fault(replay: &Path, fault: &str) -> Check {
    let path = fs::read_dir(replay.join("cbioportal"))
        .map_err(|e| e.to_string())?
        .next()
        .ok_or("recording")?
        .map_err(|e| e.to_string())?
        .path();
    let mut value: Value = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    match fault {
        "oversized" => {
            *value.pointer_mut("/exchanges/0/body/value").ok_or("body")? = json!({
                let mut valid = rows();
                *valid.pointer_mut("/0/description").ok_or("description")? =
                    json!("x".repeat(1024 * 1024));
                valid.to_string()
            })
        }
        "partial" => {
            *value.pointer_mut("/exchanges/0/body/value").ok_or("body")? =
                json!("[{\"studyId\":\"synthetic_0\"")
        }
        "timeout" => *value.pointer_mut("/exchanges/0/delay_ms").ok_or("delay")? = json!(120001),
        "429" => {
            *value
                .pointer_mut("/exchanges/0/retry_after_ms")
                .ok_or("retry")? = json!(1000)
        }
        _ => {}
    }
    fs::write(path, value.to_string()).map_err(|e| e.to_string())
}
fn revision() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env, rows(), 200)?;
    call(&env, &replay, &["sync", "cbioportal-studies"], 0)?;
    let first = call(&env, &replay, &["search", "study", "atlas"], 0)?;
    let id = first
        .pointer("/page/pool/id")
        .and_then(Value::as_str)
        .ok_or("pool")?;
    let mut changed = rows();
    *changed.pointer_mut("/0/name").ok_or("name")? = json!("Changed native title");
    seed(&env, changed, 200)?;
    call(&env, &replay, &["sync", "cbioportal-studies"], 0)?;
    let after = call(
        &env,
        &replay,
        &["search", "study", "atlas", "--pool", id, "--offset", "5"],
        4,
    )?;
    if after.get("result").is_some() || after.pointer("/next/0/kind") != Some(&json!("restart")) {
        return Err("continuation mixed catalog revisions".into());
    }
    let fresh = call(
        &env,
        &replay,
        &["search", "study", "Changed native title"],
        0,
    )?;
    if fresh.pointer("/result/items/0/studyId") != Some(&json!("synthetic_0"))
        || fresh.pointer("/result/snapshot/revision") == first.pointer("/result/snapshot/revision")
    {
        return Err("replacement revision or rows did not change".into());
    }
    Ok(())
}
