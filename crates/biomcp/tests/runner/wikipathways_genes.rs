//! Feature pathway-wikipathways-genes: native data-node membership without identifier guesses.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("wikipathways-genes/native-kinds-and-retained-paging", || {
        paging().map_err(Into::into)
    })]
}
fn catalog(search: bool) -> String {
    let mut row = json!({"id":"WP382","url":"https://www.wikipathways.org/instance/WP382",
        "name":"Synthetic pathway","species":"Homo sapiens","revision":"2025-08-29",
        "authors":"Synthetic author","description":"Native description","citedIn":""});
    if search { row["datanodes"] = json!(""); row["annotations"] = json!(""); }
    json!({"pathwayInfo":[row]}).to_string()
}
fn gpml() -> String {
    let mut body = String::from("<Pathway xmlns=\"http://pathvisio.org/GPML/2013a\" Name=\"Synthetic pathway\" Data-Source=\"WikiPathways\" Version=\"WP382_r140476\" Organism=\"Homo sapiens\">");
    for n in 0..12 {
        let namespace = if n == 10 { "" } else { " Database=\"\"" };
        let xref = if n < 10 { format!("Database=\"Synthetic\" ID=\"{}\"", 7189 + n) }
            else { format!("{namespace} ID=\"7189\"") };
        body.push_str(&format!("<DataNode TextLabel=\"Node {n}\" GraphId=\"n{n}\" Type=\"GeneProduct\"><Xref {xref}/></DataNode>"));
    }
    body.push_str("<DataNode TextLabel=\"Not a gene\" GraphId=\"p1\" Type=\"Pathway\"><Xref Database=\"\" ID=\"\"/></DataNode><DataNode TextLabel=\"Metabolite\" GraphId=\"m1\" Type=\"Metabolite\"><Xref Database=\"ChEBI\" ID=\"CHEBI:1\"/></DataNode></Pathway>");
    body
}
fn seed() -> Result<(Environment, std::path::PathBuf, Value), String> {
    let stub = Stub::for_sources(&["www.wikipathways.org"], |request| {
        if request.contains(".gpml") { "application/xml" } else { "application/json" }
    }, |request| {
        if request.contains("findPathwaysByText.json") { Ok(catalog(true)) }
        else if request.contains("getPathwayInfo.json") { Ok(catalog(false)) }
        else if request.contains("/WP382/WP382.gpml") { Ok(gpml()) }
        else { Err(format!("Unexpected source request: {request}")) }
    })?;
    let env = super::variant::source_environment(&stub)?;
    let replay = env.root().join("replay");
    let record = replay.to_str().ok_or("record path")?;
    let cache = env.root().join("case-cache");
    let cache = cache.to_str().ok_or("cache path")?;
    invoke(&env, ["sync", "wikipathways", "--record", record, "--case-cache", cache].map(str::to_owned).to_vec(), 0)?;
    let first = invoke(&env, ["get", "pathway", "WP382", "genes", "--record", record, "--case-cache", cache].map(str::to_owned).to_vec(), 0)?;
    let requests = stub.finish()?;
    if requests.len() != 3 { return Err(format!("Expected two catalog requests and one GPML request: {}", requests.len())); }
    Ok((env, replay, first))
}
fn paging() -> Check {
    let (env, replay, first) = seed()?;
    if first.pointer("/result/genes/data/page/total") != Some(&json!(12)) {
        return Err(format!("GeneProduct data-node count changed: {first}"));
    }
    let mut next = first;
    let mut seen = Vec::new();
    loop {
        let rows = next.pointer("/result/genes/data/items")
            .or_else(|| next.pointer("/result/items")).and_then(Value::as_array).ok_or("members")?;
        for row in rows {
            if row.get("gene").is_some() { return Err("Display label resolved an unsupported namespace".into()); }
            let id = row.get("node_id").and_then(Value::as_str).ok_or("node identity")?;
            if seen.contains(&id.to_owned()) { return Err("Duplicate retained data node".into()); }
            seen.push(id.to_owned());
            if id == "n10" && row.pointer("/reference/namespace").is_some() { return Err("Missing namespace became present".into()); }
            if id == "n11" && row.pointer("/reference/namespace") != Some(&json!("")) { return Err("Empty namespace was lost".into()); }
        }
        let step = next.get("next").and_then(Value::as_array).ok_or("next")?
            .iter().find(|s| s.get("kind") == Some(&json!("page")));
        let Some(step) = step else { break; };
        let mut args: Vec<String> = serde_json::from_value(step["args"].clone()).map_err(|e| e.to_string())?;
        args.extend(env.globals(Some(&replay), true));
        next = invoke(&env, args, 0)?;
    }
    if seen != (0..12).map(|n| format!("n{n}")).collect::<Vec<_>>() { return Err(format!("Retained walk changed: {seen:?}")); }
    Ok(())
}
