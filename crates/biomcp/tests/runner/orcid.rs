//! Feature author-orcid: a bounded whole reply fails before any partial group list.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::json;
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "author-orcid/oversized-whole-reply-is-failed",
        || oversized().map_err(Into::into),
    )]
}
fn oversized() -> Result<(), String> {
    let stub = Stub::for_source("pub.orcid.org", "application/json", |_| {
        Ok(json!({"path":"/0000-0002-4299-3018/works","group":[],"ignored-padding":"x".repeat(8 * 1024 * 1024 + 1)}).to_string())
    })?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let args = vec![
        "author".into(),
        "papers".into(),
        "orcid:0000-0002-4299-3018".into(),
    ];
    let cli = invoke(&env, args.clone(), 4)?;
    let mcp = super::mcp::call_with_globals(&env, &args, true, &[])?;
    for value in [&cli, &mcp] {
        if value.get("status") != Some(&json!("failed"))
            || !value.get("result").is_none_or(serde_json::Value::is_null)
            || value.pointer("/sources/0/source") != Some(&json!("orcid"))
            || value.pointer("/sources/0/state") != Some(&json!("failed"))
            || value.pointer("/sources/0/use") != Some(&json!("noncommercial"))
        {
            return Err(
                "oversized ORCID body returned partial groups or lost source policy".into(),
            );
        }
    }
    if cli != mcp {
        return Err("oversized ORCID failure differs through MCP".into());
    }
    if stub.finish()?.len() != 2 {
        return Err("oversized ORCID body retried unexpectedly".into());
    }
    Ok(())
}
