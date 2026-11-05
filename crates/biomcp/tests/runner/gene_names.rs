//! Typed gene search uses the HGNC word index shared with CLI and single MCP.
use super::{
    launch::{self, Environment},
    mcp::{exchange_with_globals, request},
};
use libtest_mimic::Trial;
use serde_json::json;
use std::io::Write;

pub(super) fn trials() -> Vec<Trial> {
    vec![super::transcript_trial(
        "gene-search/gene/names-typed",
        || check().map_err(Into::into),
    )]
}

fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"mcp_tool_mode":"typed"}))
        .map_err(|e| e.to_string())?;
    let replay = launch::fixtures().join("replies/gene/main");
    // The CLI/single-tool case owns name matching and rank assertions. This
    // bridge checks only that typed query mapping reaches the same word index.
    for (query, total, symbol) in [("kirsten rat sarcoma", 1, Some("KRAS"))] {
        let replies = exchange_with_globals(
            &env,
            &[request(
                json!(1),
                "tools/call",
                json!({"name":"search_gene","arguments":{"query":query}}),
            )],
            &env.globals(Some(&replay), false),
        )?;
        let value = replies
            .first()
            .and_then(|v| v.pointer("/result/structuredContent"))
            .ok_or_else(|| format!("missing typed gene result: {replies:?}"))?;
        if value.pointer("/status") != Some(&json!("complete"))
            || value.pointer("/page/total") != Some(&json!(total))
            || symbol.is_some_and(|symbol| {
                value.pointer("/result/items/0/symbol") != Some(&json!(symbol))
            })
        {
            return Err(format!("typed gene search {query:?}: {value}"));
        }
        writeln!(
            std::io::stderr(),
            "typed gene query={query:?} total={total} first={symbol:?}"
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
