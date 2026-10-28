//! Feature source-controls: health preserves the distinction between annotation and payload policy.
use super::{Check, Environment};
use biomcp_adapters::registry;
use serde_json::{Value, json};
pub(super) fn run() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = crate::runner::network_stub::Stub::for_sources(
        &["api-v3.monarchinitiative.org", "ontology.jax.org"],
        |_| "application/json",
        |_| Ok("{}".into()),
    )?;
    let cert = env.root().join("private-ca.pem");
    stub.retain(&env);
    std::fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    let disabled = registry::SOURCES
        .iter()
        .filter(|s| {
            !matches!(
                s.source,
                biomcp_types::Source::Monarch | biomcp_types::Source::Hpo
            )
        })
        .map(|s| s.source.to_string())
        .collect::<Vec<_>>()
        .join(",");
    for switch in ["hpo", "orphanet", "omim", "restricted"] {
        env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address),"disable_sources":[switch]}))
            .map_err(|e| e.to_string())?;
        let health = crate::runner::configuration::invoke(
            &env,
            vec![
                "health".into(),
                "--disable-sources".into(),
                disabled.clone(),
            ],
            0,
        )?;
        let row = health
            .pointer("/result/sources")
            .and_then(Value::as_array)
            .and_then(|rows| rows.iter().find(|r| r["source"] == "monarch"))
            .ok_or("missing Monarch")?;
        let blocked = matches!(switch, "hpo" | "orphanet");
        if row["state"] != if blocked { "skipped" } else { "ok" }
            || (blocked && row["reason"] != "disabled")
            || row["disabled"] != false
        {
            return Err(format!("{switch} changed annotation policy: {row}"));
        }
        let components = row["components"].as_array().ok_or("missing components")?;
        for component in components {
            if component["disabled"] != (component["name"] == switch)
                || component.pointer("/terms/usage") != Some(&json!("attribution"))
            {
                return Err(format!("{switch} mislabeled component: {component}"));
            }
        }
    }
    if stub.finish()?.len() != 5 {
        return Err("a prohibited Monarch health query ran".into());
    }
    Ok(())
}
