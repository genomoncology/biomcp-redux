//! Feature protein-domains-complexes: P24 offers retrieval without analysis execution.
use super::super::CLI_HELP_BYTES;
use super::super::mcp;
use super::*;
// Reviewed output budget 2026-10-26: j15358 compact RPC 7945 bytes plus fifteen percent.
const MCP_TOOLS_BYTES: usize = 9137;
pub(super) fn run() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let help = invoke(&env, vec!["help".into()], 0)?;
    for command in help
        .pointer("/result/commands")
        .and_then(Value::as_array)
        .ok_or("help commands")?
    {
        let words = command
            .get("command")
            .and_then(Value::as_array)
            .ok_or("command words")?;
        if !words.contains(&json!("protein")) {
            continue;
        }
        for word in command
            .get("command")
            .and_then(Value::as_array)
            .ok_or("command words")?
        {
            if word.as_str().is_some_and(analysis) {
                return Err("help offers an analysis-tool command".into());
            }
        }
    }

    let replies = mcp::exchange(&env, &[mcp::request(json!(1), "tools/list", json!({}))])?;
    let result = replies
        .first()
        .and_then(|v| v.get("result"))
        .ok_or("tool list")?;
    for tool in result
        .get("tools")
        .and_then(Value::as_array)
        .ok_or("tools")?
    {
        if tool
            .get("name")
            .and_then(Value::as_str)
            .is_none_or(analysis)
        {
            return Err("MCP offers an analysis tool".into());
        }
    }

    let help_bytes = serde_json::to_vec(&help).map_err(|e| e.to_string())?.len() + 1;
    let tools_bytes = serde_json::to_vec(replies.first().ok_or("tool reply")?)
        .map_err(|e| e.to_string())?
        .len();
    if help_bytes > CLI_HELP_BYTES || tools_bytes > MCP_TOOLS_BYTES {
        return Err(format!(
            "help or MCP discovery exceeded its named ceiling: help={help_bytes}/{CLI_HELP_BYTES}, tools={tools_bytes}/{MCP_TOOLS_BYTES}"
        ));
    }
    Ok(())
}
fn analysis(word: &str) -> bool {
    matches!(
        word.to_ascii_lowercase().as_str(),
        "align"
            | "alignment"
            | "predict"
            | "prediction"
            | "enrich"
            | "enrichment"
            | "interproscan"
            | "install"
            | "run-tool"
    )
}
