//! Exact released registry plus source-grounded endpoint ownership.
use super::{Selection, recording::Route};
use serde::Deserialize;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

const RELEASE: &str = "0.9.3";
const REVISION: &str = "5641de7d7b79b53872f8599dbb4dba22e611dcaf";

#[derive(Deserialize)]
pub(super) struct Registry {
    release: String,
    source_revision: String,
    catalog: Vec<Value>,
    #[serde(skip)]
    declarations: Declarations,
}
#[derive(Default, Deserialize)]
struct Declarations {
    release: String,
    source_revision: String,
    sources: BTreeMap<String, Source>,
    endpoints: BTreeMap<String, Endpoint>,
    fixed_endpoints: BTreeMap<String, FixedEndpoint>,
}
#[derive(Deserialize)]
struct Source {
    recording_source: String,
    variables: Vec<String>,
    delivery: Delivery,
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Delivery {
    DeclarationOnly,
    ReviewedJson,
}
#[derive(Deserialize)]
struct Proof {
    source_path: String,
    declaration: String,
}
#[derive(Deserialize)]
struct FixedEndpoint {
    source_path: String,
    urls: Vec<String>,
    reason: String,
}
#[derive(Deserialize)]
pub(super) struct Endpoint {
    pub(super) variable: String,
    pub(super) host: String,
    pub(super) base_path: String,
    default_url: String,
    provenance: Vec<Proof>,
}
pub(super) fn load() -> Result<Registry, String> {
    let root = super::super::launch::fixtures().join("parity");
    let mut registry: Registry = serde_json::from_slice(
        &fs::read(root.join("released-sources.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    registry.declarations = serde_json::from_slice(
        &fs::read(root.join("released-endpoints.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    if registry.release != RELEASE
        || registry.source_revision != REVISION
        || registry.declarations.release != RELEASE
        || registry.declarations.source_revision != REVISION
    {
        return Err("expected pinned released source registry".into());
    }
    audit(&registry)?;
    Ok(registry)
}
fn audit(registry: &Registry) -> Result<(), String> {
    let declarations = &registry.declarations;
    let direct = registry
        .catalog
        .iter()
        .filter(|row| row.get("integration_mode").and_then(Value::as_str) == Some("direct_api"))
        .map(|row| {
            row.get("id")
                .and_then(Value::as_str)
                .ok_or("source ID missing")
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    let declared = declarations
        .sources
        .keys()
        .chain(declarations.fixed_endpoints.keys())
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if direct != declared {
        return Err("released direct source endpoint coverage differs from registry".into());
    }
    for source in declarations.sources.values() {
        if source.variables.is_empty()
            || source.recording_source.is_empty()
            || source.variables.iter().collect::<BTreeSet<_>>().len() != source.variables.len()
            || source
                .variables
                .iter()
                .any(|v| !declarations.endpoints.contains_key(v))
        {
            return Err("source has missing or duplicate endpoint ownership".into());
        }
    }
    for (variable, endpoint) in &declarations.endpoints {
        if variable != &endpoint.variable
            || endpoint.host.is_empty()
            || endpoint.provenance.is_empty()
            || !endpoint
                .default_url
                .starts_with(&format!("https://{}", endpoint.host))
            || endpoint.provenance.iter().any(|proof| {
                !proof.source_path.starts_with("src/")
                    || !proof.declaration.contains(variable)
                    || !proof.declaration.contains(&endpoint.default_url)
            })
        {
            return Err("endpoint lacks released source declaration".into());
        }
    }
    for endpoint in declarations.fixed_endpoints.values() {
        if endpoint.urls.is_empty()
            || endpoint.reason.trim().is_empty()
            || !endpoint.source_path.starts_with("src/sources/")
            || endpoint.urls.iter().any(|url| !url.starts_with("https://"))
        {
            return Err("fixed endpoint lacks released source URL or isolation reason".into());
        }
    }
    Ok(())
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
        registry.endpoint(route)?;
        if !registry
            .catalog
            .iter()
            .any(|row| row.get("id").and_then(Value::as_str) == Some(&route.source))
        {
            return Err("routed source missing from released registry".into());
        }
        let source = registry.source(route)?;
        if !matches!(source.delivery, Delivery::ReviewedJson) {
            return Err(format!(
                "assessment gap: {} delivery containment is not reviewed",
                route.source
            ));
        }
    }
    Ok(())
}
impl Registry {
    fn source(&self, route: &Route) -> Result<&Source, String> {
        self.declarations.sources.get(&route.source).ok_or_else(|| {
            format!(
                "assessment gap: no reviewed endpoint declaration for {}",
                route.source
            )
        })
    }
    pub(super) fn recording_source(&self, route: &Route) -> Result<&str, String> {
        self.source(route)
            .map(|source| source.recording_source.as_str())
    }
    pub(super) fn endpoint(&self, route: &Route) -> Result<&Endpoint, String> {
        let source = self.source(route)?;
        let endpoint = self
            .declarations
            .endpoints
            .get(&route.endpoint_env)
            .filter(|endpoint| {
                source.variables.contains(&endpoint.variable) && route.host == endpoint.host
            })
            .ok_or_else(|| {
                format!(
                    "endpoint variable or host does not belong to source {}",
                    route.source
                )
            })?;
        Ok(endpoint)
    }
    pub(super) fn active_endpoints(&self, routes: &[Route]) -> Result<Vec<&Endpoint>, String> {
        let mut variables = BTreeSet::new();
        for route in routes {
            variables.extend(self.source(route)?.variables.iter());
        }
        variables
            .into_iter()
            .map(|variable| {
                self.declarations
                    .endpoints
                    .get(variable)
                    .ok_or("enabled source endpoint missing".into())
            })
            .collect()
    }
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

impl Endpoint {
    pub(super) fn local_url(&self, address: std::net::SocketAddr) -> String {
        let query = self
            .default_url
            .split_once('?')
            .map_or("", |(_, query)| query);
        let separator = if query.is_empty() { "" } else { "?" };
        format!("http://{address}{}{separator}{query}", self.base_path)
    }
}
