//! Process settings and persistent case-cache ownership through the shipped binary.
use super::{launch::Environment, mcp};
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};
type Check = Result<(), String>;
pub(super) fn root() -> PathBuf {
    crate::runner::launch::fixtures().join("replies/gene/main")
}
pub(super) fn invoke(env: &Environment, args: Vec<String>, exit: i32) -> Result<Value, String> {
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) || !output.stderr.is_empty() {
        return Err(format!(
            "expected exit {exit}: {:?}; stderr: {}; stdout: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
                .chars()
                .take(2000)
                .collect::<String>()
        ));
    }
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if let Some(kind) = value.get("local_error") {
        use std::io::Write;
        writeln!(
            std::io::stdout(),
            "LOCAL_ERROR_CLI_BYTES {kind} {}",
            output.stdout.len()
        )
        .map_err(|e| e.to_string())?;
    }
    if value.get("schema").and_then(Value::as_str)
        != Some(<biomcp_types::Help as biomcp_types::ResponseBody>::SCHEMA)
        && (args.iter().any(|word| word == "diagnostic")
            || args.windows(2).any(|words| {
                words.first().is_some_and(|word| word == "sync")
                    && words
                        .get(1)
                        .is_some_and(|word| matches!(word.as_str(), "gtr" | "who-ivd" | "fda-cdx"))
            }))
    {
        super::diagnostic_retirements::bounded(&value)?;
    }
    Ok(value)
}
pub(super) fn cached(args: &[&str], case: &Path, recording: Option<&Path>) -> Vec<String> {
    let mut args: Vec<String> = args.iter().map(|arg| (*arg).into()).collect();
    if let Some(recording) = recording {
        args.extend(["--replay".into(), recording.to_string_lossy().into()]);
    } else {
        args.push("--offline".into());
    }
    args.extend(["--case-cache".into(), case.to_string_lossy().into()]);
    args
}

fn ownership() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let user_cache = env.root().join("cache/biomcp");
    fs::create_dir(&user_cache).map_err(|e| e.to_string())?;
    let file = user_cache.join("replies.sqlite");
    fs::write(&file, b"prepopulated user cache sentinel").map_err(|e| e.to_string())?;
    let case = env.root().join("case");
    let records = env.root().join("known-recordings");
    super::response_cases::cache_capture(
        "gene/main",
        &records,
        super::response_cases::CACHE_ACQUIRED,
    )?;
    let replay = invoke(
        &env,
        cached(&["get", "gene", "BRAF"], &case, Some(&records)),
        0,
    )?;
    let offline = invoke(&env, cached(&["get", "gene", "BRAF"], &case, None), 0)?;
    if replay != offline {
        return Err("offline lost the checked replay reply".into());
    }
    let missing = env.root().join("missing");
    fs::create_dir(&missing).map_err(|e| e.to_string())?;
    let failed = invoke(
        &env,
        cached(&["get", "gene", "BRAF"], &case, Some(&missing)),
        5,
    )?;
    if failed.get("status") != Some(&json!("failed"))
        || failed.pointer("/result/items/0/answer").is_some()
    {
        return Err("warm rows masked a missing recording".into());
    }
    if fs::read(&file).map_err(|e| e.to_string())? != b"prepopulated user cache sentinel" {
        return Err("replay wrote the user cache".into());
    }
    let sync_case = env.root().join("sync-case");
    let value = invoke(
        &env,
        cached(&["sync", "hgnc"], &sync_case, Some(&root())),
        0,
    )?;
    if value.get("schema") != Some(&json!("biomcp.sync/7"))
        || value.pointer("/result/rows") != Some(&json!(176))
        || value.pointer("/query/source") != Some(&json!("hgnc"))
    {
        return Err("source sync lost metadata".into());
    }
    let withdrawn = invoke(&env, cached(&["get", "gene", "BRCA3"], &sync_case, None), 0)?;
    if withdrawn.pointer("/result/items/0/answer/resolution") != Some(&json!("withdrawn")) {
        return Err("HGNC sync omitted withdrawn identity".into());
    }
    Ok(())
}
fn refusals() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let case = env.root().join("owned");
    invoke(
        &env,
        cached(&["get", "gene", "MLL2"], &case, Some(&root())),
        0,
    )?;
    for (name, marker, bytes) in [
        ("unmarked", None, b"foreign database".as_slice()),
        (
            "foreign",
            Some(b"another owner".as_slice()),
            b"foreign database".as_slice(),
        ),
        (
            "bad-database",
            Some(b"biomcp.replay-case/1\n".as_slice()),
            b"foreign database".as_slice(),
        ),
    ] {
        let path = env.root().join(name);
        fs::create_dir(&path).map_err(|e| e.to_string())?;
        if let Some(marker) = marker {
            fs::write(path.join(".biomcp-case"), marker).map_err(|e| e.to_string())?;
        }
        fs::write(path.join("replay.sqlite"), bytes).map_err(|e| e.to_string())?;
        invoke(&env, cached(&["get", "gene", "BRAF"], &path, None), 5)?;
        if fs::read(path.join("replay.sqlite")).map_err(|e| e.to_string())? != bytes {
            return Err("refusal changed foreign files".into());
        }
    }
    #[cfg(unix)]
    {
        let link = env.root().join("linked-root");
        std::os::unix::fs::symlink(&case, &link).map_err(|e| e.to_string())?;
        invoke(&env, cached(&["get", "gene", "BRAF"], &link, None), 5)?;
        let path = env.root().join("linked-file");
        fs::create_dir(&path).map_err(|e| e.to_string())?;
        fs::copy(case.join(".biomcp-case"), path.join(".biomcp-case"))
            .map_err(|e| e.to_string())?;
        std::os::unix::fs::symlink(case.join("replay.sqlite"), path.join("replay.sqlite"))
            .map_err(|e| e.to_string())?;
        invoke(&env, cached(&["get", "gene", "BRAF"], &path, None), 5)?;
    }
    Ok(())
}
fn boundaries() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    for args in [
        vec!["sync", "hgnc-withdrawn"],
        vec!["get", "gene"],
        vec!["search", "gene"],
        vec!["search", "gene", "BRAF", "--limit", "0"],
        vec!["search", "gene", "--band", "invalid"],
        vec!["get", "gene", "BRAF", "--case-cache", "sensitive"],
        vec!["get", "gene", "BRAF", "--offline", "--replay", "sensitive"],
    ] {
        let value = invoke(&env, args.iter().map(|arg| (*arg).into()).collect(), 2)?;
        if value.to_string().contains("sensitive") || value.get("local_error").is_some() {
            return Err("usage echoed caller input".into());
        }
    }
    let args = vec!["get".into(), "gene".into(), "NOTAGENE".into()];
    let globals = vec!["--replay".into(), root().to_string_lossy().into()];
    mcp::call_with_globals(&env, &args, false, &globals)?;
    for flag in [
        "--record",
        "--replay",
        "--case-cache",
        "--case-cache=sensitive",
    ] {
        mcp::call_with_globals(
            &env,
            &["get".into(), "gene".into(), "BRAF".into(), flag.into()],
            true,
            &[],
        )?;
    }
    let directory = env.root().join("config/biomcp");
    fs::create_dir(&directory).map_err(|e| e.to_string())?;
    for config in [
        json!({"unknown":"sensitive"}),
        json!({"keys":{"unknown":"sensitive"}}),
        json!({"keys":{"oncokb_api_key":9}}),
        json!({"proxy":9}),
        json!({"proxy":"http://[invalid"}),
    ] {
        fs::write(directory.join("config.json"), config.to_string()).map_err(|e| e.to_string())?;
        let value = invoke(
            &env,
            vec![
                "get".into(),
                "gene".into(),
                "BRAF".into(),
                "--offline".into(),
            ],
            5,
        )?;
        if value.to_string().contains("sensitive") || value.get("local_error").is_some() {
            return Err("configuration echoed input".into());
        }
    }
    key_boundaries()
}
fn key_boundaries() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"unknown":"sensitive"}))
        .map_err(|e| e.to_string())?;
    let help = invoke(&env, vec!["help".into(), "key".into(), "oncokb".into()], 0)?;
    if help.pointer("/result/config") != Some(&json!("keys.oncokb_api_key")) {
        return Err("key help read malformed settings".into());
    }
    if env.root().join("cache/biomcp").exists() {
        return Err("key help opened a cache".into());
    }
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"keys":{"oncokb_api_key":""}}))
        .map_err(|e| e.to_string())?;
    let recording = root()
        .parent()
        .ok_or("fixture root")?
        .join("oncokb-synthetic/data");
    let args = vec![
        "--replay".into(),
        recording.to_string_lossy().into(),
        "get".into(),
        "gene".into(),
        "TP53".into(),
    ];
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .env("ONCOKB_TOKEN", "synthetic-override")
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if !output.status.success()
        || value.pointer("/result/items/0/oncokb/version") != Some(&json!("synthetic-1"))
    {
        return Err("environment key did not override the empty file key".into());
    }
    Ok(())
}
