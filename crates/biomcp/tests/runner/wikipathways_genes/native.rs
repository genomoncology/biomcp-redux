//! Replay the two retained publisher bodies with the exact approved HGNC rows.
use super::{Check, Environment, Stub, Value, invoke, json};
use std::{fs, io::Write};

pub(super) fn run() -> Check {
    let input = super::super::launch::fixtures().join("wikipathways-genes-native");
    let expected: std::collections::BTreeMap<String, Vec<String>> =
        serde_json::from_slice(&fs::read(input.join("nodes.json")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let search = fs::read_to_string(input.join("search.json")).map_err(|e| e.to_string())?;
    let get = fs::read_to_string(input.join("get.json")).map_err(|e| e.to_string())?;
    let hgnc = fs::read_to_string(input.join("hgnc.tsv")).map_err(|e| e.to_string())?;
    let withdrawn = fs::read_to_string(input.join("withdrawn.tsv")).map_err(|e| e.to_string())?;
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
            } else if r.contains("withdrawn.txt") {
                Ok(withdrawn.clone())
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
        walk(
            &env,
            first,
            count,
            id,
            expected.get(id).ok_or("native source nodes")?,
        )?;
        measure(&env, id, &mut outputs)?;
    }
    if stub.finish()?.len() != 6 {
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
    let (revision, data_nodes) = match id {
        "WP382" => ("WP382_r140476", 263),
        "WP100" => ("WP100_r141191", 69),
        _ => return Err(format!("Unexpected retained pathway {id}")),
    };
    loop {
        if value.pointer("/result/page/total") != Some(&json!(count))
            || value.pointer("/result/page/total_kind") != Some(&json!("data-nodes"))
            || value.pointer("/result/header/revision") != Some(&json!(revision))
            || value.pointer("/result/physical_entities") != Some(&json!(data_nodes))
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
            .into_iter()
            .flatten()
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
        return Err(format!(
            "Native walk lost or reordered source nodes: {}",
            ids.len()
        ));
    }
    if id == "WP382" {
        for (namespace, native, hgnc) in [
            ("Entrez Gene", "7189", "HGNC:12036"),
            ("Ensembl", "ENSG00000158711", "HGNC:3326"),
            ("HGNC", "CACNG3", "HGNC:1407"),
        ] {
            if !joins.iter().any(|(reference, gene)| {
                reference["namespace"] == namespace
                    && reference["value"] == native
                    && gene == &json!(hgnc)
            }) {
                return Err(format!(
                    "Missing exact native join {namespace}:{native} to {hgnc}"
                ));
            }
        }
    }
    Ok(())
}

fn measure(env: &Environment, id: &str, outputs: &mut Vec<Value>) -> Check {
    for section in ["card", "genes", "all", "members"] {
        let args = match section {
            "card" => vec!["get", "pathway", id],
            "members" => vec!["pathway", "genes", id],
            _ => vec!["get", "pathway", id, section],
        }
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();
        let cli = invoke(env, args.clone(), 0)?;
        let mcp = super::super::mcp::call_with_globals(env, &args, false, &["--offline".into()])?;
        for (surface, value) in [("CLI", cli), ("MCP", mcp)] {
            ceiling(id, section, &value)?;
            super::super::schema::Registry::embedded()?.validate(&value, &args, true)?;
            let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
            let bytes = super::super::source_terms::size(&value, "")?;
            writeln!(
                std::io::stdout(),
                "GPML_MEASURE {}",
                json!(
                    {"id":id,"section":section,"surface":surface,"full_bytes":full,"comparable_bytes":bytes,"tokens":bytes.div_ceil(4)}
                )
            )
            .map_err(|e| e.to_string())?;
            outputs.push(json!({"id":id,"section":section,"surface":surface,"output":value}));
        }
    }
    Ok(())
}

fn ceiling(id: &str, section: &str, value: &Value) -> Check {
    let budgets: std::collections::BTreeMap<String, [usize; 3]> = serde_json::from_str(
        include_str!("../../../../../tests/wikipathways-gpml-budgets.json"),
    )
    .map_err(|e| e.to_string())?;
    let key = format!("{id}/{section}");
    let baseline = budgets.get(&key).ok_or("native command baseline")?;
    let full = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    let bytes = super::super::source_terms::size(value, "")?;
    if [full, bytes, bytes.div_ceil(4)]
        .iter()
        .zip(baseline)
        .any(|(actual, measured)| *actual > (measured * 115).div_ceil(100))
    {
        return Err(format!(
            "{key} exceeds its measured fifteen-percent ceiling"
        ));
    }
    Ok(())
}
pub(super) fn saved() -> Check {
    let path = super::super::launch::fixtures().join("wikipathways-genes-native/outputs.json");
    let outputs: Vec<Value> = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if outputs.len() != 16 {
        return Err("Retained output did not include every CLI/MCP pair".into());
    }
    for output in outputs {
        let id = output
            .get("id")
            .and_then(Value::as_str)
            .ok_or("retained identity")?;
        let section = output
            .get("section")
            .and_then(Value::as_str)
            .ok_or("retained section")?;
        ceiling(id, section, output.get("output").ok_or("retained result")?)?;
    }
    Ok(())
}
