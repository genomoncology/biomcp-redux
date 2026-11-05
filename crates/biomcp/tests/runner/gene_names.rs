//! Typed gene search uses the HGNC word index shared with CLI and single MCP.
use super::{launch::{self, Environment}, mcp::{exchange_with_globals, request}};
use libtest_mimic::Trial;
use serde_json::json;

pub(super) fn trials() -> Vec<Trial> {
    vec![super::transcript_trial("gene-search/gene/names-typed", || {
        check().map_err(Into::into)
    })]
}

fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"mcp_tool_mode":"typed"})).map_err(|e| e.to_string())?;
    let replay = launch::fixtures().join("replies/gene/main");
    for (query, total, symbol) in [
        ("kirsten rat sarcoma", 0, None),
        ("aconitate hydratase", 0, None),
        ("aconitase 1", 1, Some("ACO1")),
        ("IRP1", 1, Some("ACO1")),
        ("KRAS", 1, Some("KRAS")),
    ] {
        let replies = exchange_with_globals(&env, &[request(json!(1), "tools/call",
            json!({"name":"search_gene","arguments":{"query":query}}))],
            &env.globals(Some(&replay), false))?;
        let value = replies.first().and_then(|v| v.pointer("/result/structuredContent"))
            .ok_or_else(|| format!("missing typed gene result: {replies:?}"))?;
        if value.pointer("/status") != Some(&json!("complete"))
            || value.pointer("/page/total") != Some(&json!(total))
            || symbol.is_some_and(|symbol| value.pointer("/result/items/0/symbol") != Some(&json!(symbol)))
        {
            return Err(format!("typed gene search {query:?}: {value}"));
        }
        eprintln!("typed gene query={query:?} total={total} first={symbol:?}");
    }
    Ok(())
}
