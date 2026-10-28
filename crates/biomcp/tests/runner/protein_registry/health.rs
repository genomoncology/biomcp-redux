//! Feature source-controls: health reuses the measured integrated and human requests.
use super::{Check, Environment, invoke, registry, rows};
use serde_json::json;
pub(super) fn run() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = crate::runner::network_stub::Stub::for_sources(
        &["www.ebi.ac.uk"],
        |_| "application/json",
        |request| {
            let path = request
                .split_whitespace()
                .nth(1)
                .ok_or("missing request path")?;
            if path == "/interpro/api/entry/interpro/protein/uniprot/P00533/?page_size=5" {
                return Ok(
                    json!({"count":1,"results":[{"metadata":{"accession":"IPR000719"}}]})
                        .to_string(),
                );
            }
            if path
                == "/intact/complex-ws/search/P15056?first=0&number=5&filters=species_f%3A%28%22Homo%20sapiens%22%29"
            {
                return Ok(
                    json!({"totalNumberOfResults":1,"elements":[{"complexAc":"CPX-715"}]})
                        .to_string(),
                );
            }
            Err(format!("unexpected health request: {path}"))
        },
    )?;
    let cert = env.root().join("private-ca.pem");
    stub.retain(&env);
    std::fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let disabled = registry::SOURCES
        .iter()
        .filter(|s| !["interpro", "complex-portal"].contains(&s.source.to_string().as_str()))
        .map(|s| s.source.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let record = env.root().join("health-recording");
    for mode in ["--record", "--replay"] {
        let health = invoke(
            &env,
            vec![
                "health".into(),
                "--disable-sources".into(),
                disabled.clone(),
                mode.into(),
                record.to_string_lossy().into(),
            ],
            0,
        )?;
        for source in ["interpro", "complex-portal"] {
            let row = rows(&health)?
                .iter()
                .find(|r| r["source"] == source)
                .ok_or("missing health")?;
            if row["state"] != "ok" || row["use"] != "open" || row["terms"] != "CC0" {
                return Err(format!("wrong health: {row}"));
            }
        }
    }
    if stub.finish()?.len() != 2 {
        return Err("health replay reached HTTP".into());
    }
    Ok(())
}
