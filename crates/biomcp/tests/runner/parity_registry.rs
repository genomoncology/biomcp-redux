//! Exact released registry plus reviewed source endpoint declarations.
use super::{Selection, recording::Route};
use serde::Deserialize;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};
#[derive(Deserialize)]
pub(super) struct Registry {
    release: String,
    source_revision: String,
    catalog: Vec<Value>,
    pub(super) endpoints: BTreeMap<String, Endpoint>,
}
#[derive(Deserialize)]
pub(super) struct Endpoint {
    pub(super) variable: String,
    pub(super) host: String,
    pub(super) base_path: String,
    pub(super) recording_source: String,
}
pub(super) fn load() -> Result<Registry, String> {
    let path = super::super::launch::fixtures().join("parity/released-sources.json");
    let registry: Registry = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if registry.release != "0.9.3"
        || registry.source_revision != "5641de7d7b79b53872f8599dbb4dba22e611dcaf"
    {
        return Err("expected pinned released source registry".into());
    }
    Ok(registry)
}
pub(super) fn validate(selection: &Selection) -> Result<(), String> {
    let registry = load()?;
    if selection
        .baseline_globals
        .iter()
        .any(|arg| arg == "--disable-sources" || arg.starts_with("--disable-sources="))
    {
        return Err(
            "baseline source disables are derived from routes; omit --disable-sources".into(),
        );
    }
    if selection.recordings.is_empty() {
        return Err("assessment gap: baseline endpoint routes missing".into());
    }
    for route in &selection.recordings {
        let endpoint = registry.endpoints.get(&route.source).ok_or_else(|| {
            format!(
                "assessment gap: no reviewed endpoint declaration for {}",
                route.source
            )
        })?;
        if route.endpoint_env != endpoint.variable || route.host != endpoint.host {
            return Err(format!(
                "endpoint variable or host does not belong to source {}",
                route.source
            ));
        }
        if !registry
            .catalog
            .iter()
            .any(|row| row.get("id").and_then(Value::as_str) == Some(&route.source))
        {
            return Err("routed source missing from released registry".into());
        }
    }
    Ok(())
}
impl Registry {
    pub(super) fn disabled(&self, routes: &[Route]) -> Result<String, String> {
        let active = routes
            .iter()
            .map(|r| r.source.as_str())
            .collect::<BTreeSet<_>>();
        self.catalog
            .iter()
            .map(|row| {
                row.get("id")
                    .and_then(Value::as_str)
                    .ok_or("source registry ID missing".into())
            })
            .collect::<Result<Vec<_>, String>>()
            .map(|ids| {
                ids.into_iter()
                    .filter(|id| !active.contains(id))
                    .collect::<Vec<_>>()
                    .join(",")
            })
    }
}
