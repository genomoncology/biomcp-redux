//! Key registration crosses the real CLI boundary without real credentials.
use super::launch::Environment;
use serde_json::Value;
use std::{io::Write, process::Stdio};
type Check = Result<(), String>;
const FIRST: &str = "synthetic-registration-first";
const SECOND: &str = "synthetic-registration-second";

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
