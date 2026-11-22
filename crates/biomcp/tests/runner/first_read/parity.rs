//! Fresh CLI and MCP processes read the same publisher recordings through a local stub.
use super::{COMPANION, Check, SEARCH, ema, invoke, live, words};
use crate::runner::{
    launch::Environment, mcp, network_stub::Stub, protocol::recent_parity, transcript,
};
use libtest_mimic::Trial;
use serde_json::json;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript::trial("mcp-parity/recent/1055/cdx-first-read", || {
            compare(SEARCH, 0, || live("www.fda.gov", COMPANION, json!({}))).map_err(Into::into)
        }),
        transcript::trial("mcp-parity/recent/1070/cdx-size-refusal", || {
            compare(SEARCH, 4, || {
                live(
                    "www.fda.gov",
                    COMPANION,
                    json!({"auto_sync_max_bytes":1000}),
                )
            })
            .map_err(Into::into)
        }),
        transcript::trial("mcp-parity/recent/1119/ema-first-read", || {
            compare(ema::GET, 0, || ema::recorded_server(200)).map_err(Into::into)
        }),
    ]
}

fn compare(
    args: &[&str],
    exit: i32,
    prepare: impl Fn() -> Result<(Environment, Stub), String>,
) -> Check {
    let (env, stub) = prepare()?;
    let cli = invoke(&env, words(args), exit)?;
    let cli_requests = stub.finish()?;
    let (env, stub) = prepare()?;
    let mcp = mcp::call_with_globals(&env, &words(args), exit >= 2, &[])?;
    let mcp_requests = stub.finish()?;
    if cli_requests.len() != mcp_requests.len() || mcp_requests.is_empty() {
        return Err("CLI and MCP first reads sent a different number of requests".into());
    }
    recent_parity::compare(&cli, &mcp)
}
