//! Full-envelope command phases; CLI includes its newline and MCP counts compact JSON.
use super::launch::Environment;
use serde_json::Value;
pub(super) fn invoke(
    phase: &str,
    env: &Environment,
    args: Vec<String>,
    exit: i32,
) -> Result<Value, String> {
    let value = super::configuration::invoke(env, args, exit)?;
    check(&value, phase, "cli")?;
    Ok(value)
}
pub(super) fn mcp(
    phase: &str,
    env: &Environment,
    args: &[String],
    error: bool,
    globals: &[String],
) -> Result<Value, String> {
    let value = super::mcp::call_with_globals(env, args, error, globals)?;
    check(&value, phase, "mcp")?;
    Ok(value)
}
fn check(value: &Value, phase: &str, surface: &str) -> Result<(), String> {
    use std::io::Write;
    let bytes =
        serde_json::to_vec(value).map_err(|e| e.to_string())?.len() + usize::from(surface == "cli");
    writeln!(
        std::io::stdout(),
        "PATHWAY_PHASE {}",
        serde_json::json!({"phase":phase,"surface":surface,"bytes":bytes})
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
