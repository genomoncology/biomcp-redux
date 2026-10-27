//! Feature source-controls: health reads one exact term and replays without HTTP.
use super::{Check, Environment};
use biomcp_adapters::registry;
use serde_json::{Value, json};
pub(super) fn run() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    // A proof job may supply the retained native body outside Git. Routine runs
    // use a synthetic exact identity and never request a provider.
    let body = if let Some(path) = std::env::var_os("HPO_HEALTH_CAPTURE") {
        let bytes = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_str(&bytes).map_err(|e| e.to_string())?;
        if value.get("id") != Some(&json!("HP:0001250")) {
            return Err("wrong retained term".into());
        }
        bytes
    } else {
        json!({"id":"HP:0001250","name":"Synthetic health term"}).to_string()
    };
    let stub = crate::runner::network_stub::Stub::for_sources(
        &["ontology.jax.org"],
        |_| "application/json",
        move |request| {
            if !request.starts_with("GET /api/hp/terms/HP:0001250 HTTP/") {
                return Err(format!("unbounded or wrong HPO health request: {request}"));
            }
            Ok(body.clone())
        },
    )?;
    let cert = env.root().join("private-ca.pem");
    std::fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let disabled = registry::SOURCES
        .iter()
        .filter(|s| s.source != biomcp_types::Source::Hpo)
        .map(|s| s.source.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let recording = env.root().join("term-recording");
    for mode in ["--record", "--replay"] {
        let health = crate::runner::configuration::invoke(
            &env,
            vec![
                "health".into(),
                "--disable-sources".into(),
                disabled.clone(),
                mode.into(),
                recording.to_string_lossy().into(),
            ],
            0,
        )?;
        let row = health
            .pointer("/result/sources")
            .and_then(Value::as_array)
            .and_then(|rows| rows.iter().find(|r| r["source"] == "hpo"))
            .ok_or("missing HPO")?;
        if row["state"] != "ok"
            || row["use"] != "attribution"
            || row["disabled"] != false
            || row["terms_url"] != "https://human-phenotype-ontology.github.io/license.html"
        {
            return Err(format!("wrong exact-term health: {row}"));
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("health replay made another request".into());
    }
    Ok(())
}
