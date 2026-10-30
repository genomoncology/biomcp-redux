//! Key registration crosses the real CLI boundary without real credentials.
use super::launch::Environment;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, io::Write, process::Stdio};
type Check = Result<(), String>;
const FIRST: &str = "synthetic-registration-first";
const SECOND: &str = "synthetic-registration-second";
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("key-registration/nci/key-reload", || {
            super::key_reload::nci().map_err(Into::into)
        }),
        transcript_trial("local-file-persistence/local-file-modes", || {
            super::local_files::modes().map_err(Into::into)
        }),
        transcript_trial("local-file-persistence/key-file-order", || {
            super::local_files::ordered(false).map_err(Into::into)
        }),
        #[cfg(unix)]
        transcript_trial("local-file-persistence/key-file-symlink", || {
            super::local_files::ordered(true).map_err(Into::into)
        }),
        transcript_trial("key-registration/key-registration", || {
            registration().map_err(Into::into)
        }),
        transcript_trial("key-registration/key-reload-stdio", || {
            super::key_reload::check(false, false).map_err(Into::into)
        }),
        transcript_trial("key-registration/key-reload-stdio-environment", || {
            super::key_reload::check(false, true).map_err(Into::into)
        }),
        transcript_trial("key-registration/key-reload-http", || {
            super::key_reload::check(true, true).map_err(Into::into)
        }),
    ]
}
pub(super) fn invoke(
    env: &Environment,
    args: &[&str],
    input: &str,
    exit: i32,
) -> Result<Value, String> {
    let args = args.iter().map(|s| (*s).into()).collect::<Vec<_>>();
    let mut command = env.command(env!("CARGO_BIN_EXE_biomcp"), &args);
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    child
        .stdin
        .take()
        .ok_or("missing stdin")?
        .write_all(input.as_bytes())
        .map_err(|e| e.to_string())?;
    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    command
        .record(output.status, &output.stdout, "cli")
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) {
        return Err(format!("wrong exit: {:?}", output.status));
    }
    for marker in [FIRST, SECOND] {
        if output
            .stdout
            .windows(marker.len())
            .chain(output.stderr.windows(marker.len()))
            .any(|s| s == marker.as_bytes())
        {
            return Err("credential leaked in process output".into());
        }
    }
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}
fn presence(env: &Environment, expected: &str, fallback: bool) -> Check {
    let mut command = env.command(env!("CARGO_BIN_EXE_biomcp"), &["key".into(), "list".into()]);
    if fallback {
        command.env("S2_API_KEY", SECOND);
    }
    let output = command.output().map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if !output.status.success()
        || !output.stderr.is_empty()
        || value.to_string().contains(SECOND)
        || value.pointer("/result/entries/1/state") != Some(&json!(expected))
    {
        return Err("key presence precedence or secrecy failed".into());
    }
    Ok(())
}
fn registration() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"format":"json","disable_sources":["reactome"],"ncbi_email":"synthetic@example.invalid"}))
        .map_err(|e| e.to_string())?;
    invoke(
        &env,
        &["key", "set", "semantic-scholar"],
        &format!("{FIRST}\n"),
        0,
    )?;
    presence(&env, "stored", true)?;
    let path = env.root().join("config/biomcp/config.json");
    let stored: Value = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if stored.pointer("/keys/semantic_scholar_api_key") != Some(&json!(FIRST))
        || stored.get("disable_sources") != Some(&json!(["reactome"]))
        || stored.get("ncbi_email") != Some(&json!("synthetic@example.invalid"))
    {
        return Err("set lost configuration".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if fs::metadata(&path)
            .map_err(|e| e.to_string())?
            .permissions()
            .mode()
            & 0o777
            != 0o600
        {
            return Err("config permissions are not owner-only".into());
        }
    }
    let existing = invoke(
        &env,
        &["key", "set", "semantic-scholar"],
        &format!("{SECOND}\n"),
        2,
    )?;
    if existing.pointer("/error/option") != Some(&json!("--replace")) {
        return Err("existing key omitted --replace guidance".into());
    }
    invoke(
        &env,
        &["key", "set", "semantic-scholar", "--replace"],
        &format!("{SECOND}\n"),
        0,
    )?;
    let listed = invoke(&env, &["key", "list"], "", 0)?;
    if listed.pointer("/result/entries/1/state") != Some(&json!("stored")) {
        return Err(format!("stored state missing: {listed}"));
    }
    invoke(&env, &["key", "remove", "semantic-scholar"], "", 0)?;
    let value = invoke(&env, &["key", "remove", "semantic-scholar"], "", 0)?;
    if value.pointer("/result/entries/0/state") != Some(&json!("none")) {
        return Err("absent remove did not report none".into());
    }
    presence(&env, "environment", true)?;
    presence(&env, "none", false)?;
    verify_preserved(&path)?;
    invoke(&env, &["key", "set", "semantic-scholar"], "\n", 2)?;
    invoke(&env, &["key", "set", "reactome"], "", 2)?;
    invoke(&env, &["key", "set", "semantic-scholar", FIRST], "", 2)?;
    override_config(&env)
}
fn override_config(env: &Environment) -> Check {
    let override_path = env.root().join("override.json");
    invoke(
        env,
        &[
            "--config",
            override_path.to_str().ok_or("path")?,
            "key",
            "set",
            "ncbi",
        ],
        &format!("{FIRST}\n"),
        0,
    )?;
    let value: Value = serde_json::from_slice(&fs::read(override_path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if value.pointer("/keys/ncbi_api_key") != Some(&json!(FIRST)) {
        return Err("override not used".into());
    }
    Ok(())
}

fn verify_preserved(path: &std::path::Path) -> Check {
    let value: Value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if value.get("disable_sources") != Some(&json!(["reactome"]))
        || value.get("ncbi_email") != Some(&json!("synthetic@example.invalid"))
        || value.pointer("/keys/semantic_scholar_api_key").is_some()
    {
        return Err("remove damaged configuration".into());
    }
    Ok(())
}
