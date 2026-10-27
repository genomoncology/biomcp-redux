//! Synthetic replies are recorded through the real transport before fault substitution.
use super::{Check, Environment};
use biomcp_adapters::reactome;
use biomcp_fetch::{Certificates, Fetch, FetchConfig, Mode, Network, Secret};
use serde_json::{Value, json};
use std::{fs, path::Path};
pub(super) const ID: &str = "R-HSA-5673001";
pub(super) fn card() -> Value {
    json!({"stId":ID,"displayName":"Synthetic pathway","speciesName":"Homo sapiens","schemaClass":"Pathway","stIdVersion":format!("{ID}.1")})
}
pub(super) fn search() -> Value {
    json!({"results":[{"entries":[{"stId":ID,"name":"Synthetic pathway","species":["Homo sapiens"],"exactType":"Pathway"}]}],"rowCount":1,"numberOfMatches":1})
}
pub(super) fn body(request: &str) -> Result<String, String> {
    Ok(if request.contains("/json/") {
        catalog(request.contains("findPathwaysByText"))
    } else if request.contains("/search/query?") {
        search()
    } else if request.contains("/database/version") {
        json!(97)
    } else if request.contains("/participants/") {
        json!([])
    } else if request.contains("/containedEvents") {
        json!([123])
    } else if request.contains("/mapping/UniProt/") {
        json!([{"stId":ID, "displayName":"Synthetic pathway", "speciesName":"Homo sapiens", "schemaClass":"Pathway"}])
    } else {
        card()
    }
    .to_string())
}
pub(super) fn seed(env: &Environment) -> Result<std::path::PathBuf, String> {
    let stub = crate::runner::network_stub::Stub::for_sources(
        &["reactome.org", "www.wikipathways.org"],
        |_| "application/json",
        body,
    )?;
    let root = env.root().join("replay");
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Record { dir: root.clone() },
        cache: Some(env.root().join("synthetic-source.sqlite")),
        network: Network::new(
            None,
            Some(Secret::new(format!("http://{}", stub.address))),
            "",
            Certificates::from_pem(stub.certificate.as_bytes()).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?,
        ..Default::default()
    })
    .map_err(|e| e.to_string())?;
    let leg = fetch.leg(&reactome::SPEC, fetch.deadline(reactome::SPEC.deadline));
    let id = ID
        .parse::<biodata::ReactomeId>()
        .map_err(|e| e.to_string())?;
    let query = serde_json::from_value(json!({"query":"Synthetic"})).map_err(|e| e.to_string())?;
    reactome::search(&leg, &query).map_err(|e| e.to_string())?;
    reactome::card(&leg, &id, false).map_err(|e| e.to_string())?;
    reactome::release(&leg).map_err(|e| e.to_string())?;
    reactome::members(&leg, &id).map_err(|e| e.to_string())?;
    reactome::events(&leg, &id).map_err(|e| e.to_string())?;
    reactome::pathways(
        &leg,
        &"P15056"
            .parse::<biodata::UniProtAcc>()
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    biomcp_adapters::wikipathways::sync(&fetch).map_err(|e| e.to_string())?;
    drop(fetch);
    if stub.finish()?.len() != 8 {
        return Err("synthetic recording changed request count".into());
    }
    let source = crate::runner::launch::fixtures().join("replies/pathway/native/hgnc");
    fs::create_dir(root.join("hgnc")).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(source).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        fs::copy(entry.path(), root.join("hgnc").join(entry.file_name()))
            .map_err(|e| e.to_string())?;
    }
    Ok(root)
}
pub(super) fn replace(root: &Path, route: &str, body: &str, status: u16, delay: u64) -> Check {
    replace_source(root, "reactome", route, body, status, delay)
}
pub(super) fn replace_source(
    root: &Path,
    source: &str,
    route: &str,
    body: &str,
    status: u16,
    delay: u64,
) -> Check {
    for entry in fs::read_dir(root.join(source)).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        let mut wire: Value = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        if wire
            .get("url")
            .and_then(Value::as_str)
            .is_some_and(|v| v.contains(route))
        {
            let exchange = json!(
                {"status":status,"content_type":"application/json","delay_ms":delay,"retry_after_ms":null,"body":{"kind":"text","value":body}}
            );
            *wire.get_mut("exchanges").ok_or("exchanges")? =
                json!(vec![
                    exchange;
                    if matches!(status, 429 | 500) { 3 } else { 1 }
                ]);
            fs::write(path, wire.to_string()).map_err(|e| e.to_string())?;
            return Ok(());
        }
    }
    Err(format!("missing synthetic recorded route {route}"))
}

pub(super) fn catalog(search: bool) -> Value {
    let mut row = json!(
        {"id":"WP382","url":"https://www.wikipathways.org/instance/WP382","name":"Synthetic pathway","species":"Homo sapiens",
        "revision":"2025-05-22","authors":"Synthetic Author","description":"Synthetic description","citedIn":""}
    );
    if search && let Some(fields) = row.as_object_mut() {
        fields.insert("datanodes".into(), json!(""));
        fields.insert("annotations".into(), json!(""));
    }
    json!({"pathwayInfo":[row]})
}
