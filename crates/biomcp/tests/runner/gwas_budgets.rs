//! Named full-envelope command phases; native replay budgets remain independently owned.
use super::{configuration, launch::Environment, mcp};
use serde_json::Value;
use std::io::Write;

pub(super) fn invoke(
    phase: &str,
    env: &Environment,
    args: Vec<String>,
    exit: i32,
) -> Result<Value, String> {
    let cli = configuration::invoke(env, args.clone(), exit)?;
    // Reuse checked replies for the second surface so cache/fresh request-count
    // behavior stays owned by its existing CLI test. Replay has no provider I/O.
    let mut mcp_args = args;
    mcp_args.retain(|arg| arg != "--fresh");
    if !mcp_args.iter().any(|arg| arg == "--replay" || arg == "--offline") {
        mcp_args.push("--offline".into());
    }
    let value = mcp::call_with_globals(env, &mcp_args, !matches!(exit, 0 | 3), &[])?;
    let cli_bytes = serde_json::to_vec(&cli).map_err(|e| e.to_string())?.len() + 1;
    let mcp_bytes = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
    writeln!(std::io::stdout(), "GWAS_OUTPUT {}", serde_json::json!({"phase":phase,"cli":cli_bytes,"mcp":mcp_bytes}))
        .map_err(|e| e.to_string())?;
    Ok(cli)
}
