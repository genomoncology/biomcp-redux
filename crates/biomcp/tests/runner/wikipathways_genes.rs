//! Feature pathway-wikipathways-genes: native data-node membership without identifier guesses.
use crate::runner::transcript::trial as transcript_trial;
use super::{launch::Environment, network_stub::Stub};
use crate::runner::pathway_budgets as budgets;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
type Check = Result<(), String>;
mod joins;
mod native;
use super::configuration::invoke;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("wikipathways-genes/retained-output-ceilings", || {
            native::saved().map_err(Into::into)
        })
        .with_ignored_flag(
            !super::launch::fixtures()
                .join("wikipathways-genes-native/outputs.json")
                .is_file(),
        ),
        transcript_trial(
            "wikipathways-genes/exact-xrefs-and-distinct-gene-pivots",
            || joins::run().map_err(Into::into),
        ),
        transcript_trial(
            "wikipathways-genes/single-attempt-provider-failures",
            || single_attempt().map_err(Into::into),
        ),
        transcript_trial(
            "wikipathways-genes/native-kinds-and-retained-paging",
            || paging().map_err(Into::into),
        ),
        transcript_trial(
            "wikipathways-genes/failed-bodies-never-enter-retained-membership",
            || failures().map_err(Into::into),
        ),
        transcript_trial("wikipathways-genes/retained-native-measurements", || {
            native::run().map_err(Into::into)
        })
        .with_ignored_flag(
            !super::launch::fixtures()
                .join("wikipathways-genes-native")
                .is_dir(),
        ),
    ]
}
fn catalog(search: bool) -> String {
    let mut row = json!({"id":"WP382","url":"https://www.wikipathways.org/instance/WP382",
        "name":"Synthetic pathway","species":"Homo sapiens","revision":"2025-08-29",
        "authors":"Synthetic author","description":"Native description","citedIn":""});
    if search && let Some(fields) = row.as_object_mut() {
        fields.insert("datanodes".into(), json!(""));
        fields.insert("annotations".into(), json!(""));
    }
    json!({"pathwayInfo":[row]}).to_string()
}
fn gpml() -> String {
    let mut body = String::from(
        "<Pathway xmlns=\"http://pathvisio.org/GPML/2013a\" Name=\"Synthetic pathway\" \
            Data-Source=\"WikiPathways\" Version=\"WP382_r140476\" Organism=\"Homo sapiens\">",
    );
    for n in 0..12 {
        let namespace = if n == 10 { "" } else { " Database=\"\"" };
        let xref = if n < 10 {
            format!("Database=\"Synthetic\" ID=\"{}\"", 7189 + n)
        } else {
            format!("{namespace} ID=\"7189\"")
        };
        body.push_str(&format!(
            "<DataNode TextLabel=\"Node {n}\" GraphId=\"n{n}\" Type=\"GeneProduct\"><Xref {xref}/></DataNode>"
        ));
    }
    body.push_str(
        "<DataNode TextLabel=\"Not a gene\" GraphId=\"p1\" Type=\"Pathway\"><Xref Database=\"\" \
        ID=\"\"/></DataNode><DataNode TextLabel=\"Metabolite\" GraphId=\"m1\" Type=\"Metabolite\"><Xref \
        Database=\"ChEBI\" ID=\"CHEBI:1\"/></DataNode></Pathway>",
    );
    body
}
fn seed() -> Result<(Environment, std::path::PathBuf, Value), String> {
    let stub = Stub::for_sources(
        &["www.wikipathways.org"],
        |request| {
            if request.contains(".gpml") {
                "application/octet-stream"
            } else {
                "application/json"
            }
        },
        |request| {
            if request.contains("findPathwaysByText.json") {
                Ok(catalog(true))
            } else if request.contains("getPathwayInfo.json") {
                Ok(catalog(false))
            } else if request.contains("/WP382/WP382.gpml") {
                Ok(gpml())
            } else {
                Err(format!("Unexpected source request: {request}"))
            }
        },
    )?;
    let env = super::variant::source_environment(&stub)?;
    let replay = env.root().join("replay");
    let record = replay.to_str().ok_or("record path")?;
    budgets::invoke(
        "runner/wikipathways_genes/seed/1",
        &env,
        ["sync", "wikipathways", "--record", record]
            .map(str::to_owned)
            .to_vec(),
        0,
    )?;
    let genes_record = env.root().join("genes-record");
    let genes_path = genes_record.to_str().ok_or("genes recording path")?;
    let first = budgets::invoke(
        "runner/wikipathways_genes/seed/2",
        &env,
        ["get", "pathway", "WP382", "genes", "--record", genes_path]
            .map(str::to_owned)
            .to_vec(),
        0,
    )?;
    for entry in fs::read_dir(genes_record.join("wikipathways")).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        fs::copy(
            entry.path(),
            replay.join("wikipathways").join(entry.file_name()),
        )
        .map_err(|e| e.to_string())?;
    }
    let requests = stub.finish()?;
    if requests.len() != 3 {
        return Err(format!(
            "Expected two catalog requests and one GPML request: {}",
            requests.len()
        ));
    }
    Ok((env, replay, first))
}
fn paging() -> Check {
    let (env, _replay, first) = seed()?;
    if first.pointer("/result/genes/data/page/total") != Some(&json!(12)) {
        return Err(format!("GeneProduct data-node count changed: {first}"));
    }
    let mut next = first;
    let mut seen = Vec::new();
    loop {
        let rows = next
            .pointer("/result/genes/data/items")
            .or_else(|| next.pointer("/result/items"))
            .and_then(Value::as_array)
            .ok_or("members")?;
        for row in rows {
            if row.get("gene").is_some() {
                return Err("Display label resolved an unsupported namespace".into());
            }
            let id = row
                .get("node_id")
                .and_then(Value::as_str)
                .ok_or("node identity")?;
            if seen.contains(&id.to_owned()) {
                return Err("Duplicate retained data node".into());
            }
            seen.push(id.to_owned());
            if id == "n10" && row.pointer("/reference/namespace").is_some() {
                return Err("Missing namespace became present".into());
            }
            if id == "n11" && row.pointer("/reference/namespace") != Some(&json!("")) {
                return Err("Empty namespace was lost".into());
            }
        }
        let step = next
            .get("next")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .find(|s| s.get("kind") == Some(&json!("page")));
        let Some(step) = step else {
            break;
        };
        let mut args: Vec<String> =
            serde_json::from_value(step["args"].clone()).map_err(|e| e.to_string())?;
        args.push("--offline".into());
        next = budgets::invoke("runner/wikipathways_genes/paging/1", &env, args, 0)?;
    }
    if seen != (0..12).map(|n| format!("n{n}")).collect::<Vec<_>>() {
        return Err(format!("Retained walk changed: {seen:?}"));
    }
    Ok(())
}

fn failures() -> Check {
    let (owner, replay, _) = seed()?;
    let _keep_owner = owner;
    let path = fs::read_dir(replay.join("wikipathways"))
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .find(|path| {
            fs::read(path)
                .ok()
                .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
                .and_then(|v| {
                    v.get("url")
                        .and_then(Value::as_str)
                        .map(|s| s.ends_with(".gpml"))
                })
                .unwrap_or(false)
        })
        .ok_or("GPML recording")?;
    let original: Value = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    for mode in [
        "wrong-wp",
        "wrong-species",
        "padded-species",
        "bad-revision",
        "broken",
        "entity",
        "namespace",
        "duplicate",
        "padded-id-collision",
        "node-bound",
        "oversized",
        "429",
        "500",
        "timeout",
    ] {
        let mut recording = original.clone();
        let exchange = recording.pointer_mut("/exchanges/0").ok_or("exchange")?;
        let body = failed_body(mode);
        *exchange.pointer_mut("/body/value").ok_or("recorded body")? = json!(body);
        if mode == "429" || mode == "500" {
            *exchange.get_mut("status").ok_or("recorded status")? =
                json!(mode.parse::<u16>().map_err(|e| e.to_string())?);
        }
        if mode == "timeout" {
            *exchange.get_mut("delay_ms").ok_or("recorded delay")? = json!(10_000);
        }
        fs::write(&path, recording.to_string()).map_err(|e| e.to_string())?;
        let env = Environment::new().map_err(|e| e.to_string())?;
        let mut sync = vec!["sync".into(), "wikipathways".into()];
        sync.extend(env.globals(Some(&replay), false));
        budgets::invoke("runner/wikipathways_genes/failures/1", &env, sync, 0)?;
        for offline in [false, true] {
            let mut args = vec!["pathway".into(), "genes".into(), "WP382".into()];
            args.extend(env.globals(Some(&replay), offline));
            let value = budgets::invoke("runner/wikipathways_genes/failures/2", &env, args, 4)
                .map_err(|e| format!("{mode} offline={offline}: {e}"))?;
            if value.get("status") != Some(&json!("failed"))
                || value.get("result").is_some()
                || value.pointer("/sources/0/source") != Some(&json!("wikipathways"))
                || value.pointer("/sources/0/terms") != Some(&json!("CC0 1.0 waiver"))
            {
                return Err(format!(
                    "{mode} was admitted or lost its source terms: {value}"
                ));
            }
        }
    }
    Ok(())
}

fn failed_body(mode: &str) -> String {
    match mode {
        "wrong-wp" => gpml().replace("WP382_r140476", "WP100_r140476"),
        "wrong-species" => gpml().replace("Homo sapiens", "Mus musculus"),
        "padded-species" => gpml().replace("Homo sapiens", " Homo sapiens "),
        "bad-revision" => gpml().replace("WP382_r140476", "WP382_r0"),
        "broken" => "<Pathway>".into(),
        "entity" => format!(
            "<!DOCTYPE Pathway [<!ENTITY x SYSTEM \"file:///etc/passwd\">]>{}",
            gpml()
        ),
        "namespace" => gpml().replace("<Xref", "<Xref xmlns=\"urn:foreign\""),
        "duplicate" => gpml().replace("GraphId=\"n1\"", "GraphId=\"n0\""),
        "padded-id-collision" => gpml().replace("GraphId=\"n1\"", "GraphId=\" n0 \""),
        "node-bound" => {
            gpml().replace("</Pathway>", &("<Unknown/>".repeat(100_001) + "</Pathway>"))
        }
        "oversized" => gpml().replace("</Pathway>", &(" ".repeat(8 * 1024 * 1024) + "</Pathway>")),
        _ => gpml(),
    }
}

fn single_attempt() -> Check {
    for status in [429, 500] {
        let stub = Stub::for_source_status(
            "www.wikipathways.org",
            "application/json",
            |request| {
                if request.contains("findPathwaysByText.json") {
                    Ok(catalog(true))
                } else if request.contains("getPathwayInfo.json") {
                    Ok(catalog(false))
                } else {
                    Ok("provider rejected GPML".into())
                }
            },
            move |request| {
                if request.contains(".gpml") {
                    status
                } else {
                    200
                }
            },
        )?;
        let env = super::variant::source_environment(&stub)?;
        budgets::invoke(
            "runner/wikipathways_genes/single_attempt/1",
            &env,
            vec!["sync".into(), "wikipathways".into()],
            0,
        )?;
        budgets::invoke(
            "runner/wikipathways_genes/single_attempt/2",
            &env,
            vec!["pathway".into(), "genes".into(), "WP382".into()],
            4,
        )?;
        let count = stub.finish()?.len();
        if count != 3 {
            return Err(format!(
                "GPML status {status} retried: {count} total requests"
            ));
        }
    }
    Ok(())
}
