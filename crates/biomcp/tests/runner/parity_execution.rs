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
        "stdout":stdout_path.file_name().and_then(|name| name.to_str()).ok_or("stdout filename must be UTF-8")?,
        "stderr":stderr_path.file_name().and_then(|name| name.to_str()).ok_or("stderr filename must be UTF-8")?});
    if let Ok(parsed) = serde_json::from_slice::<Value>(&stdout) {
        field(&mut result, "parsed", parsed)?;
    }
    Ok(result)
}

pub(super) fn assess_exit(output: &mut Value, expected: i32) -> Result<(), String> {
    if output.get("exit").and_then(Value::as_i64) != Some(i64::from(expected)) {
        let reason = json!(format!(
            "command exit {:?}, expected {expected}",
            output.get("exit")
        ));
        field(output, "assessment_error", reason)?;
    }
    Ok(())
}
pub(super) fn fingerprint(binary: &Path) -> Result<Value, String> {
    use sha2::{Digest, Sha256};
    let binary = super::super::launch::executable(binary);
    let bytes = fs::read(&binary).map_err(|e| e.to_string())?;
    let digest = Sha256::digest(&bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    Ok(json!({"path":binary,"sha256":digest}))
}

pub(super) fn candidate(
    case: &super::Case,
    step: &super::Step,
    args: &[String],
    replay: &Path,
    out: &Path,
    index: usize,
) -> Result<Value, String> {
    let guard = super::super::network_stub::Stub::for_source(
        "replay-only.invalid",
        "application/json",
        |_| Err("candidate attempted a provider connection".into()),
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let mut config = case.config.clone().unwrap_or(json!({}));
    field(
        &mut config,
        "proxy",
        json!(format!("http://{}", guard.address)),
    )?;
    env.config(&config).map_err(|e| e.to_string())?;
    let globals = env.globals(Some(replay), step.offline);
    let args = super::super::launch::arguments(args, &globals);
    let mut result = invoke(
        &env,
        Path::new(env!("CARGO_BIN_EXE_biomcp")),
        &args,
        &out.join(format!("{index}-candidate")),
    )?;
    match guard.finish() {
        Ok(requests) if requests.is_empty() => {
            field(&mut result, "provider_connections", json!(0))?
        }
        other => field(
            &mut result,
            "assessment_error",
            json!(format!("offline replay guard: {other:?}")),
        )?,
    }
    Ok(result)
}

pub(super) fn field(object: &mut Value, key: &str, value: Value) -> Result<(), String> {
    object
        .as_object_mut()
        .ok_or("expected report object")?
        .insert(key.into(), value);
    Ok(())
}
