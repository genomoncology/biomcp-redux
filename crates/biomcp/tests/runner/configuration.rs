//! Process settings and persistent case-cache ownership through the shipped binary.
use super::launch::Environment;
use serde_json::Value;
use std::path::{Path, PathBuf};

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
