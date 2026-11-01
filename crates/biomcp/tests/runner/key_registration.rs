//! Persistent MCP key snapshots and real filesystem ownership require process/file inspection.
use super::launch::Environment;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::Value;
use std::{io::Write, process::Stdio};
const FIRST: &str = "synthetic-registration-first";
const SECOND: &str = "synthetic-registration-second";
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("key-registration/input-before-lock", || {
            input_before_lock().map_err(Into::into)
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

// Keep one stdin open while a second process registers the same key. The first
// process must recheck replacement after input, preserving the second writer.
fn input_before_lock() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = ["key", "set", "semantic-scholar"].map(str::to_owned);
    let mut command = env.command(env!("CARGO_BIN_EXE_biomcp"), &args);
    let first = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let mut first = super::process::Process::new(first)?;
    let mut second = env.command(env!("CARGO_BIN_EXE_biomcp"), &args);
    let child = second
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let mut child = super::process::Process::new(child)?;
    child
        .child
        .stdin
        .take()
        .ok_or("missing second stdin")?
        .write_all(format!("{SECOND}\n").as_bytes())
        .map_err(|e| e.to_string())?;
    let result = child.finish();
    // Always release and reap the waiting caller, even if the second writer hung.
    first
        .child
        .stdin
        .take()
        .ok_or("missing first stdin")?
        .write_all(format!("{FIRST}\n").as_bytes())
        .map_err(|e| e.to_string())?;
    let (first_status, first_out, _) = first.finish()?;
    let (status, stdout, _) = result?;
    second
        .record(status, &stdout, "cli")
        .map_err(|e| e.to_string())?;
    command
        .record(first_status, &first_out, "cli")
        .map_err(|e| e.to_string())?;
    if !status.success() || first_status.code() != Some(2) {
        return Err(format!(
            "second writer={status}; waiting caller={first_status}"
        ));
    }
    let value: Value = serde_json::from_slice(&first_out).map_err(|e| e.to_string())?;
    if value.pointer("/error/code").and_then(Value::as_str) != Some("usage.arguments") {
        return Err(format!(
            "waiting caller did not require replacement: {value}"
        ));
    }
    let config =
        std::fs::read(env.root().join("config/biomcp/config.json")).map_err(|e| e.to_string())?;
    let config: Value = serde_json::from_slice(&config).map_err(|e| e.to_string())?;
    if config
        .pointer("/keys/semantic_scholar_api_key")
        .and_then(Value::as_str)
        != Some(SECOND)
    {
        return Err("waiting caller overwrote the concurrent credential".into());
    }
    Ok(())
}
