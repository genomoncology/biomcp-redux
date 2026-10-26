//! Replay the two retained publisher bodies with the exact approved HGNC rows.
use super::{Check, Environment, Stub, Value, invoke, json};
use std::{fs, io::Write};

pub(super) fn run() -> Check {
    let input = super::super::launch::fixtures().join("wikipathways-genes-native");
    let expected: std::collections::BTreeMap<String, Vec<String>> = serde_json::from_slice(
        &fs::read(input.join("nodes.json")).map_err(|e| e.to_string())?,
    ).map_err(|e| e.to_string())?;
    let search = fs::read_to_string(input.join("search.json")).map_err(|e| e.to_string())?;
    let get = fs::read_to_string(input.join("get.json")).map_err(|e| e.to_string())?;
    let hgnc = fs::read_to_string(input.join("hgnc.tsv")).map_err(|e| e.to_string())?;
    let wp382 = fs::read_to_string(input.join("WP382.gpml")).map_err(|e| e.to_string())?;
    let wp100 = fs::read_to_string(input.join("WP100.gpml")).map_err(|e| e.to_string())?;
    let stub = Stub::for_sources(
        &["www.wikipathways.org", "storage.googleapis.com"],
        |r| {
            if r.contains(".gpml") {
                "application/octet-stream"
            } else if r.contains(".txt") {
                "text/tab-separated-values"
            } else {
                "application/json"
            }
        },
        move |r| {
            if r.contains("findPathwaysByText.json") {
                Ok(search.clone())
            } else if r.contains("getPathwayInfo.json") {
                Ok(get.clone())
            } else if r.contains("hgnc_complete_set.txt") {
                Ok(hgnc.clone())
            } else if r.contains("/WP382/WP382.gpml") {
                Ok(wp382.clone())
            } else if r.contains("/WP100/WP100.gpml") {
                Ok(wp100.clone())
            } else {
                Err(format!("Unexpected source request {r}"))
            }
        },
    )?;
    let env = super::super::variant::source_environment(&stub)?;
    for args in [vec!["sync", "wikipathways"], vec!["sync", "hgnc"]] {
        invoke(&env, args.into_iter().map(str::to_owned).collect(), 0)?;
    }
    let mut outputs = Vec::new();
    for (id, count) in [("WP382", 252), ("WP100", 43)] {
        let first = invoke(
            &env,
            ["pathway", "genes", id, "--limit", "50"]
                .map(str::to_owned)
                .to_vec(),
            0,
        )?;
        walk(&env, first, count, id, expected.get(id).ok_or("native source nodes")?)?;
        measure(&env, id, &mut outputs)?;
    }
    if stub.finish()?.len() != 5 {
        return Err("A retained page or section refetched GPML".into());
    }
    fs::write(
        input.join("outputs.json"),
        serde_json::to_vec(&outputs).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
fn walk(env: &Environment, mut value: Value, count: usize, id: &str, expected: &[String]) -> Check {
    let mut ids = Vec::new();
    let mut joins = Vec::new();
    loop {
        if value.pointer("/result/page/total") != Some(&json!(count))
            || value.pointer("/result/page/total_kind") != Some(&json!("data_nodes"))
        {
            return Err(format!("Native count provenance changed: {value}"));
        }
        for row in value
            .pointer("/result/items")
            .and_then(Value::as_array)
            .ok_or("native nodes")?
        {
            let node = row
                .get("node_id")
                .and_then(Value::as_str)
                .ok_or("native node ID")?;
            if ids.contains(&node.to_owned()) {
                return Err("Duplicated native data node".into());
            }
            ids.push(node.to_owned());
            if row.get("schema_class") != Some(&json!("GeneProduct")) {
                return Err("Non-gene node admitted".into());
            }
            if let Some(gene) = row.get("gene") {
                joins.push((row["reference"].clone(), gene.clone()));
            }
        }
        let step = value
            .get("next")
            .and_then(Value::as_array)
            .ok_or("next")?
            .iter()
            .find(|s| s["kind"] == "page");
        let Some(step) = step else {
            break;
        };
        let mut args: Vec<String> =
            serde_json::from_value(step["args"].clone()).map_err(|e| e.to_string())?;
        args.push("--offline".into());
        value = invoke(env, args, 0)?;
    }
    if ids.len() != count || ids != expected {
        return Err(format!("Native walk lost or reordered source nodes: {}", ids.len()));
    }
    if id == "WP382" {
        for (namespace, native) in [
            ("Entrez Gene", "7189"),
            ("Ensembl", "ENSG00000158711"),
            ("HGNC", "CACNG3"),
        ] {
            if !joins.iter().any(|(reference, _)| {
                reference["namespace"] == namespace && reference["value"] == native
            }) {
                return Err(format!("Missing exact native join {namespace}:{native}"));
            }
        }
    }
    Ok(())
}

fn measure(env: &Environment, id: &str, outputs: &mut Vec<Value>) -> Check {
    for section in ["genes", "all"] {
        let args = ["get", "pathway", id, section].map(str::to_owned).to_vec();
        let cli = invoke(&env, args.clone(), 0)?;
        let mcp =
            super::super::mcp::call_with_globals(&env, &args, false, &["--offline".into()])?;
        for (surface, value) in [("CLI", cli), ("MCP", mcp)] {
            super::super::schema::Registry::embedded()?.validate(&value, &args, true)?;
            let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
            let bytes = super::super::source_terms::size(&value, "")?;
            writeln!(std::io::stdout(), "GPML_MEASURE {}", json!({"id":id,"section":section,"surface":surface,"full_bytes":full,"comparable_bytes":bytes,"tokens":bytes.div_ceil(4)})).map_err(|e| e.to_string())?;
            outputs.push(json!({"id":id,"section":section,"surface":surface,"output":value}));
        }
        }
    Ok(())
}
