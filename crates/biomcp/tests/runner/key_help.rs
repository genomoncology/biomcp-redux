//! Feature key-registration: each source's help names its own credential settings.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    [
        (
            "openfda",
            "openfda-label",
            "openfda_api_key",
            "OPENFDA_API_KEY",
        ),
        ("orcid", "orcid", "orcid_access_token", "ORCID_ACCESS_TOKEN"),
    ]
    .into_iter()
    .map(|(source, identity, config, environment)| {
        Trial::test(format!("keys/{source}/configuration-help"), move || {
            check(source, identity, config, environment).map_err(Into::into)
        })
    })
    .collect()
}
fn check(source: &str, identity: &str, config: &str, environment: &str) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let output = env
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &["help".into(), "key".into(), source.into()],
        )
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if !output.status.success()
        || value.pointer("/result/source") != Some(&json!(identity))
        || value.pointer("/result/config") != Some(&json!(format!("keys.{config}")))
        || value.pointer("/result/environment") != Some(&json!(environment))
        || value.pointer("/result/guidance") != Some(&json!(format!("biomcp key set {source}")))
    {
        return Err("key help omitted source-owned registration or settings".into());
    }
    if source == "openfda" {
        measure(&env)?;
    }
    Ok(())
}
fn measure(env: &Environment) -> Result<(), String> {
    let help = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &["help".into()])
        .output()
        .map_err(|e| e.to_string())?;
    if !help.status.success() {
        return Err("whole help failed".into());
    }
    use std::io::Write;
    writeln!(
        std::io::stderr(),
        "OPENFDA_CLI_HELP_BYTES {}",
        help.stdout.len()
    )
    .map_err(|e| e.to_string())
}
