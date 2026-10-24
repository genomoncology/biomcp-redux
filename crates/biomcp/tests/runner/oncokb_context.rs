//! Resolve keyed source selectors while keyless calls stop before identity I/O.
use super::{launch::Environment, network_stub::Stub, oncokb};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
const SECTION: &str = "/result/candidates/0/sections/oncokb";
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("variant/oncokb-resolved-id-and-all-context", || {
        run().map_err(Into::into)
    })]
}
fn run() -> Result<(), String> {
    let stub = Stub::for_sources(
        &["myvariant.info", "www.oncokb.org"],
        |_| "application/json",
        reply,
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    let config = json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca});
    env.config(&config).map_err(|e| e.to_string())?;
    let mut args = oncokb::args();
    *args.get_mut(2).ok_or("input")? = "rs113488022".into();
    let skipped = oncokb::invoke(&env, &args, 0)?;
    if skipped.pointer(&format!("{SECTION}/reason")) != Some(&json!("no_key")) {
        return Err("rsID did not stop before resolution without a key".into());
    }
    let mut keyed = config;
    keyed.as_object_mut().ok_or("config")?.insert(
        "keys".into(), json!({"oncokb_api_key":"synthetic-oncokb-variant-key"})
    );
    env.config(&keyed).map_err(|e| e.to_string())?;
    let resolved = oncokb::invoke(&env, &args, 0)?;
    if resolved.pointer(&format!("{SECTION}/state")) != Some(&json!("data")) {
        return Err(format!("keyed rsID was not resolved for annotation: {resolved}"));
    }
    let mcp = super::mcp::call_with_globals(&env, &args, false, &[])?;
    if resolved.get("result") != mcp.get("result") {
        return Err("resolved annotation CLI/MCP differs".into());
    }
    let all = ["get", "variant", "rs113488022", "all"].map(str::to_owned);
    let missing = oncokb::invoke(&env, &all, 0)?;
    if missing.pointer(&format!("{SECTION}/state")) != Some(&json!("skipped"))
        || !missing.get("next").and_then(Value::as_array).is_some_and(|steps| {
            steps.iter().any(|s| s.get("kind") == Some(&json!("configure"))
                && s.get("args") == Some(&json!(["help","get","variant"])))
        }) {
        return Err(format!("all omitted the explicit context help step: {missing}"));
    }
    let mcp = super::mcp::call_with_globals(&env, &all, false, &[])?;
    if missing.get("result") != mcp.get("result") || missing.get("next") != mcp.get("next") {
        return Err("all context CLI/MCP differs".into());
    }
    let requests = stub.finish()?;
    if requests.iter().filter(|(_, r)| r.starts_with("GET /api/v1/annotate/")).count() != 1
        || requests.len() != 3 {
        return Err(format!("keyless or contextless request escaped, or warm cache missed: {}", requests.len()));
    }
    Ok(())
}
fn reply(request: &str) -> Result<String, String> {
    if request.starts_with("GET /v1/query?") {
        return Ok(json!({"total":1,"hits":[{"_id":"chr7:g.140753336A>T","dbsnp":{"rsid":"rs113488022","gene":{"symbol":"BRAF"}}}]}).to_string());
    }
    oncokb::reply(request)
}
