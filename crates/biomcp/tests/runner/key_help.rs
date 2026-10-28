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
        (
            "alphagenome",
            "alphagenome",
            "alphagenome_api_key",
            "ALPHAGENOME_API_KEY",
        ),
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
    if source == "alphagenome" {
        alphagenome_precedence(&env)?;
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

fn alphagenome_precedence(env: &Environment) -> Result<(), String> {
    for (stored, fallback, expected) in [
        (true, true, "stored"),
        (false, true, "environment"),
        (false, false, "none"),
    ] {
        env.config(&if stored {
            json!({"keys":{"alphagenome_api_key":"synthetic-alphagenome-config"}})
        } else {
            json!({})
        })
        .map_err(|e| e.to_string())?;
        let mut command = env.command(env!("CARGO_BIN_EXE_biomcp"), &["key".into(), "list".into()]);
        if fallback {
            command.env("ALPHAGENOME_API_KEY", "synthetic-alphagenome-env");
        }
        let output = command.output().map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        let state = value
            .pointer("/result/entries")
            .and_then(Value::as_array)
            .and_then(|rows| rows.iter().find(|row| row["source"] == "alphagenome"))
            .and_then(|row| row.get("state"))
            .and_then(Value::as_str);
        if !output.status.success()
            || state != Some(expected)
            || value.to_string().contains("synthetic-alphagenome")
        {
            return Err("AlphaGenome credential precedence or secrecy failed".into());
        }
    }
    Ok(())
}
