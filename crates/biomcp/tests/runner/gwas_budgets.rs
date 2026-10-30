//! Named full-envelope command phases; native replay budgets remain independently owned.
use super::{configuration, launch::Environment, mcp};
use serde_json::{Value, json};
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
    let mut mcp_args = Vec::new();
    let mut globals = Vec::new();
    let mut words = args.into_iter();
    while let Some(word) = words.next() {
        match word.as_str() {
            "--replay" | "--case-cache" => {
                globals.push(word);
                globals.push(words.next().ok_or("missing process option value")?);
            }
            "--offline" => globals.push(word),
            "--fresh" => {}
            _ => mcp_args.push(word),
        }
    }
    if !globals
        .iter()
        .any(|arg| arg == "--replay" || arg == "--offline")
    {
        globals.push("--offline".into());
    }
    let value = mcp::call_with_globals(env, &mcp_args, !matches!(exit, 0 | 1), &globals)?;
    if cli.get("status") != value.get("status") || cli.get("schema") != value.get("schema") {
        return Err(format!("GWAS phase {phase} MCP command differs: {value}"));
    }
    let cli_bytes = serde_json::to_vec(&cli).map_err(|e| e.to_string())?.len() + 1;
    let mcp_bytes = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
    writeln!(
        std::io::stderr(),
        "GWAS_MEASURE {phase} {cli_bytes} {mcp_bytes}"
    )
    .map_err(|e| e.to_string())?;
    let measured: std::collections::BTreeMap<String, [usize; 2]> = serde_saphyr::from_str(
        &crate::runner::launch::fixture_text("gwas-synthetic-budgets.yaml")?,
    )
    .map_err(|e| e.to_string())?;
    let [cli_limit, mcp_limit] = measured
        .get(phase)
        .ok_or_else(|| format!("GWAS phase lacks measurement: {phase}"))?;
    if cli_bytes > (cli_limit * 115).div_ceil(100) || mcp_bytes > (mcp_limit * 115).div_ceil(100) {
        return Err(format!(
            "GWAS phase {phase} exceeds measured CLI/MCP ceiling: {cli_bytes}/{mcp_bytes} bytes"
        ));
    }
    Ok(cli)
}

pub(super) fn optional_page(value: &Value, field: &str, omitted: &str) -> Result<(), String> {
    if value.get("status") != Some(&json!("partial"))
        || value.pointer("/page/returned") != Some(&json!(3))
        || value.pointer("/page/total") != Some(&json!(6))
        || value
            .pointer("/sources/0/retrieved_count")
            .is_some_and(|count| count != &json!(3))
        || value.pointer("/result/items/0/id") != Some(&json!(6110))
        || value.pointer("/result/items/2/id") != Some(&json!(6112))
        || value.pointer(omitted).is_some()
        || !value
            .get("problems")
            .and_then(Value::as_array)
            .is_some_and(|rows| {
                rows.iter().any(|p| {
                    p.get("field") == Some(&json!(field)) && p.get("record") == Some(&json!("6110"))
                })
            })
    {
        return Err(format!(
            "optional GWAS field lost valid rows, native counts or precise refusal: {value}"
        ));
    }
    let next = super::action(value, "page")?;
    let offset = if value.pointer("/query/operation") == Some(&json!("get")) {
        "--section-offset"
    } else {
        "--offset"
    };
    if !next.windows(2).any(|pair| pair == [offset, "3"]) {
        return Err(format!(
            "optional GWAS field changed consumed native continuation: {next:?}"
        ));
    }
    Ok(())
}

pub(super) fn cached(
    phase: &str,
    env: &Environment,
    words: &[&str],
    original: &Value,
) -> Result<(), String> {
    let mut args = env.globals(None, true);
    args.extend(words.iter().map(|word| (*word).to_owned()));
    let cached = invoke(phase, env, args, 0)?;
    if cached.get("result") != original.get("result")
        || cached.get("status") != original.get("status")
        || cached.get("problems") != original.get("problems")
    {
        return Err(format!(
            "valid GWAS identities with omitted optional metadata did not cache intact: {cached}"
        ));
    }
    Ok(())
}
