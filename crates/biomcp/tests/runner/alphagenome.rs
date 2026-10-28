//! Feature variant-alphagenome: required keys stop before variant resolution or any provider request.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
const SECTION: &str = "/result/candidates/0/sections/alphagenome";
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "variant/alphagenome-keyless-zero-network",
        || keyless().map_err(Into::into),
    )]
}
fn keyless() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_sources(
        &[
            "myvariant.info",
            "reg.genome.network",
            "gdmscience.googleapis.com",
        ],
        |_| "application/json",
        |_| Err("keyless request reached a provider fixture".into()),
    )?;
    let ca = env.root().join("ca.pem");
    stub.retain(&env);
    std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    for id in ["rs113488022", "clinvar:13961", "CA123643"] {
        let args = ["get", "variant", id, "alphagenome"].map(str::to_owned);
        let value = invoke(&env, &args, 0)?;
        if value.pointer(&format!("{SECTION}/state")) != Some(&json!("skipped"))
            || value.pointer(&format!("{SECTION}/reason")) != Some(&json!("no_key"))
            || !value
                .get("next")
                .and_then(Value::as_array)
                .is_some_and(|steps| {
                    steps.iter().any(|step| {
                        step.get("args") == Some(&json!(["help", "key", "alphagenome"]))
                    })
                })
        {
            return Err("missing AlphaGenome key did not skip before identity resolution".into());
        }
        let mcp = super::mcp::call_with_globals(&env, &args, false, &[])?;
        if value.get("result") != mcp.get("result") {
            return Err("keyless CLI and MCP differ".into());
        }
    }
    let help = invoke(&env, &["help", "key", "alphagenome"].map(str::to_owned), 0)?;
    if help.pointer("/result/environment") != Some(&json!("ALPHAGENOME_API_KEY")) {
        return Err("missing-key guidance names the wrong environment variable".into());
    }
    if !stub.finish()?.is_empty() {
        return Err("keyless command made a network request".into());
    }
    Ok(())
}
pub(super) fn invoke(env: &Environment, args: &[String], exit: i32) -> Result<Value, String> {
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), args)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) || !output.stderr.is_empty() {
        return Err(format!(
            "AlphaGenome command failed: {:?}; {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}
