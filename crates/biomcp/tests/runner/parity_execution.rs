use super::Environment;
use serde_json::{Value, json};
use std::{fs, path::Path, process::Stdio, time::Instant};

pub(super) fn invoke(
    env: &Environment,
    binary: &Path,
    args: &[String],
    out: &Path,
) -> Result<Value, String> {
    invoke_command(env.command(binary, args), args, out)
}
pub(super) fn invoke_command(
    mut command: std::process::Command,
    args: &[String],
    out: &Path,
) -> Result<Value, String> {
    let started = Instant::now();
    let child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let (status, stdout, stderr) = super::super::process::Process::new(child)?.finish()?;
    let elapsed = started.elapsed().as_micros();
    let stdout_path = out.with_extension("stdout");
    let stderr_path = out.with_extension("stderr");
    fs::write(&stdout_path, &stdout).map_err(|e| e.to_string())?;
    fs::write(&stderr_path, &stderr).map_err(|e| e.to_string())?;
    let mut result = json!({"command":args,"exit":status.code(),"stdout_bytes":stdout.len(),
        "stderr_bytes":stderr.len(),"elapsed_us":elapsed,
        "stdout":stdout_path.file_name(),"stderr":stderr_path.file_name()});
    if let Ok(parsed) = serde_json::from_slice::<Value>(&stdout) {
        result["parsed"] = parsed;
    }
    Ok(result)
}

pub(super) fn assess_exit(output: &mut Value, expected: i32) {
    if output.get("exit").and_then(Value::as_i64) != Some(i64::from(expected)) {
        output["assessment_error"] = json!(format!(
            "command exit {:?}, expected {expected}",
            output.get("exit")
        ));
    }
}
pub(super) fn fingerprint(binary: &Path) -> Result<Value, String> {
    use sha2::{Digest, Sha256};
    let bytes = fs::read(binary).map_err(|e| e.to_string())?;
    let digest = Sha256::digest(&bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    Ok(json!({"path":binary,"sha256":digest}))
}
