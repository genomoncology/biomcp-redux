//! Measure the bytes the command and MCP actually emit.
use super::*;
pub(super) fn invoke_measured(env: &Environment, words: &[&str], exit: i32) -> Result<(Value, usize), String> {
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &words.iter().map(|s| s.to_string()).collect::<Vec<_>>())
        .output().map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) {
        return Err(format!("{words:?}: exit {:?}; stdout {}; stderr {}", output.status.code(), String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr)));
    }
    let value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    Ok((value, output.stdout.len()))
}
pub(super) fn parity(phase: &str, env: &Environment, words: &[&str], exit: i32, measure: bool) -> Result<Value, String> {
    let recording = env.root().join("recordings").join(phase.replace(':', "_"));
    fs::create_dir_all(recording.parent().ok_or("record parent")?).map_err(|e| e.to_string())?;
    let mut args = words.to_vec();
    args.extend(["--record", recording.to_str().ok_or("recording")?]);
    let (value, bytes) = invoke_measured(env, &args, exit)?;
    compared(phase, env, words, exit, measure, &recording, (value, bytes))
}
pub(super) fn compared(phase: &str, env: &Environment, words: &[&str], exit: i32, measure: bool, recording: &std::path::Path, output: (Value, usize)) -> Result<Value, String> {
    let (value, bytes) = output;
    let retained = env.root().join("mcp-raw").join(phase);
    fs::create_dir_all(&retained).map_err(|e| e.to_string())?;
    let replay = vec!["--replay".to_string(), recording.to_str().ok_or("recording")?.into()];
    let args = words.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let mcp = super::super::super::mcp::call_retained(env, &args, exit >= 2, &replay, Some(&retained))?;
    if value != mcp { return Err(format!("{phase}: CLI/MCP mismatch")); }
    measure_bytes(phase, "cli", bytes, measure)?;
    measure_mcp(phase, &retained, measure)?;
    let registry = super::super::super::schema::Registry::embedded()?;
    registry.validate(&value, &args, exit == 0)?;
    registry.validate(&mcp, &args, exit == 0)?;
    Ok(value)
}
pub(super) fn measure_bytes(phase: &str, surface: &str, bytes: usize, measure: bool) -> Result<(), String> {
    let _ = writeln!(std::io::stderr(), "CELL_LINE_MEASURE {phase} [{surface}] {bytes}");
    if !measure && bytes > ceiling(phase, surface)? { return Err(format!("{phase} [{surface}] exceeds measured cap")); }
    Ok(())
}
pub(super) fn measure_mcp(phase: &str, directory: &std::path::Path, measure: bool) -> Result<(), String> {
    let bytes = fs::read(directory.join("mcp.stdout")).map_err(|e| e.to_string())?;
    let reply: Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let text = reply.pointer("/result/content/0/text").and_then(Value::as_str).ok_or("MCP text")?;
    let _ = writeln!(std::io::stderr(), "CHEMBL_MCP_RPC_BYTES {phase} {}", bytes.len());
    measure_bytes(phase, "mcp", text.len(), measure)
}
