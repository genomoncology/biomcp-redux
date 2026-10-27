use super::super::{
    configuration::invoke,
    launch::{Environment, fixtures},
};
use serde_json::{Value, json};
use std::{fs, path::Path};
type Check = Result<(), String>;
pub(super) fn call(
    env: &Environment,
    replay: &Path,
    args: &[&str],
    exit: i32,
) -> Result<Value, String> {
    let mut command = env.globals(Some(replay), false);
    command.extend(args.iter().map(|s| (*s).into()));
    invoke(env, command, exit)
}
pub(super) fn seed(env: &Environment) -> Result<std::path::PathBuf, String> {
    let source = fixtures().join("replies/diagnostic/gtr-synthetic/gtr");
    let replay = env.root().join("replay");
    fs::create_dir_all(replay.join("gtr")).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(source).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        fs::copy(entry.path(), replay.join("gtr").join(entry.file_name()))
            .map_err(|e| e.to_string())?;
    }
    Ok(replay)
}
pub(super) fn replacement() -> Check {
    for fault in [
        "truncated-gzip",
        "wrong-version-member",
        "malformed-relation",
        "second-file-failure",
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = seed(&env)?;
        let receipt = call(&env, &replay, &["sync", "gtr"], 0)?;
        if receipt.pointer("/result/duplicate_identities") != Some(&json!(1)) {
            return Err("duplicate count excluded an old version".into());
        }
        let args = ["get", "diagnostic", "GTR000006692.3", "--source", "gtr"];
        let before = call(&env, &replay, &args, 0)?;
        let entries = fs::read_dir(replay.join("gtr")).map_err(|e| e.to_string())?;
        for entry in entries {
            let path = entry.map_err(|e| e.to_string())?.path();
            if fault == "truncated-gzip" && path.extension().is_some_and(|v| v == "bin") {
                fs::write(&path, [0x1f, 0x8b, 8]).map_err(|e| e.to_string())?;
            } else if fault == "wrong-version-member"
                && path.extension().is_some_and(|v| v == "bin")
            {
                fs::copy(
                    fixtures().join("replies/diagnostic/gtr-wrong-version-member.bin"),
                    &path,
                )
                .map_err(|e| e.to_string())?;
            } else if path.extension().is_some_and(|v| v == "json") {
                let mut value: Value =
                    serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
                        .map_err(|e| e.to_string())?;
                if value
                    .get("url")
                    .and_then(Value::as_str)
                    .is_some_and(|u| u.ends_with("test_condition_gene.txt"))
                {
                    if fault == "malformed-relation" {
                        *value
                            .pointer_mut("/exchanges/0/body/value")
                            .ok_or("missing body cell")? =
                            json!("#accession_version\ttest_type\ninvalid\n");
                    }
                    if fault == "second-file-failure" {
                        *value
                            .pointer_mut("/exchanges/0/status")
                            .ok_or("missing status cell")? = json!(403);
                    }
                    fs::write(&path, value.to_string()).map_err(|e| e.to_string())?;
                }
            }
        }
        let failed = call(&env, &replay, &["sync", "gtr"], 4)?;
        if fault == "wrong-version-member"
            && failed.pointer("/sources/0/reason") != Some(&json!("bad_reply"))
        {
            return Err(format!("wrong native member was not rejected: {failed}"));
        }
        if call(&env, &replay, &args, 0)? != before {
            return Err(format!("{fault} replaced usable data"));
        }
    }
    Ok(())
}
pub(super) fn policy() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    call(&env, &replay, &["sync", "gtr"], 0)?;
    let args = ["get", "diagnostic", "GTR000006692.3", "--source", "gtr"];
    let before = call(&env, &replay, &args, 0)?;
    fs::remove_dir_all(replay.join("gtr")).map_err(|e| e.to_string())?;
    let mut fresh = args.to_vec();
    fresh.push("--fresh");
    let after = call(&env, &replay, &fresh, 0)?;
    if after.get("result") != before.get("result") || after.get("status") != before.get("status") {
        return Err("stored read requested a refresh".into());
    }
    env.config(&json!({"disable_sources":["gtr"]}))
        .map_err(|e| e.to_string())?;
    let skipped = call(&env, &replay, &args, 4)?;
    if skipped.get("result").is_some()
        || skipped.pointer("/sources/0/state") != Some(&json!("skipped"))
        || skipped.pointer("/sources/0/reason") != Some(&json!("disabled"))
    {
        return Err(format!("disabled source leaked stored data: {skipped}"));
    }
    Ok(())
}
pub(super) fn conflict() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    call(&env, &replay, &["sync", "gtr"], 0)?;
    let value = call(
        &env,
        &replay,
        &["get", "diagnostic", "GTR000551465.2", "--source", "gtr"],
        4,
    )?;
    if value.get("result").is_some()
        || value.pointer("/problems/0/code") != Some(&json!("identity.conflict"))
    {
        return Err(format!("conflicting identity selected a card: {value}"));
    }
    Ok(())
}

pub(super) fn markdown() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    call(&env, &replay, &["sync", "gtr"], 0)?;
    let mut args = env.globals(Some(&replay), false);
    args.extend(
        [
            "get",
            "diagnostic",
            "GTR000006692.3",
            "--source",
            "gtr",
            "--format",
            "md",
        ]
        .map(str::to_owned),
    );
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() || !output.stderr.is_empty() {
        return Err("diagnostic Markdown command failed".into());
    }
    // Full synthetic Markdown stdout measured by j13400, plus fifteen percent.
    if output.stdout.len() > 1761 {
        return Err("diagnostic Markdown exceeds its measured ceiling".into());
    }
    let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    for expected in [
        "GTR000006692&#46;3",
        "Synthetic test",
        r#"Synthetic "quoted" laboratory"#,
        "2026-10-22",
        "get diagnostic GTR000006692.3 genes",
        "NCBI usage policies",
    ] {
        if !text.contains(expected) {
            return Err(format!("diagnostic Markdown omitted {expected}"));
        }
    }
    Ok(())
}
