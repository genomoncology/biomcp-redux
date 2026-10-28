//! Synthetic failures preserve source outcomes and never admit rejected values.
use crate::runner::transcript::trial as transcript_trial;
use super::budgets;
use super::{Environment, Stub, invoke, setup};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    let mut rows = Vec::new();
    for surface in [
        "search",
        "hpo-card",
        "monarch-card",
        "hpo-synonyms",
        "monarch-synonyms",
        "diseases",
        "genes",
    ] {
        for fault in ["429", "500", "timeout", "broken", "error-body"] {
            rows.push(transcript_trial(
                format!("phenotype-identity/synthetic-{surface}-{fault}"),
                move || failure(surface, fault).map_err(Into::into),
            ));
        }
    }
    for fault in ["hpo-id", "monarch-id", "MP", "UPHENO", "taxon"] {
        rows.push(transcript_trial(
            format!("phenotype-identity/synthetic-reject-{fault}-without-cache"),
            move || identity(fault).map_err(Into::into),
        ));
    }
    rows
}
fn args(surface: &str) -> Vec<&str> {
    match surface {
        "search" => vec!["search", "phenotype", "synthetic"],
        "hpo-card" => vec!["get", "phenotype", "HP:0000001", "--source", "hpo"],
        "monarch-card" => vec!["get", "phenotype", "HP:0000001", "--source", "monarch"],
        "hpo-synonyms" => vec![
            "get",
            "phenotype",
            "HP:0000001",
            "synonyms",
            "--source",
            "hpo",
        ],
        "monarch-synonyms" => vec![
            "get",
            "phenotype",
            "HP:0000001",
            "synonyms",
            "--source",
            "monarch",
        ],
        section => vec![
            "get",
            "phenotype",
            "HP:0000001",
            section,
            "--source",
            "monarch",
        ],
    }
}
fn source(surface: &str) -> &'static str {
    if surface.starts_with("hpo") {
        "hpo"
    } else {
        "monarch"
    }
}
fn failure(surface: &str, fault: &str) -> Result<(), String> {
    let body = match fault {
        "broken" => "{".into(),
        _ => json!({"error":"synthetic source failure"}).to_string(),
    };
    let (env, directory) = seed(
        surface,
        fault,
        body,
        fault.parse().unwrap_or(200),
        if fault == "timeout" { 20000 } else { 0 },
    )?;
    let words = args(surface);
    let mut replay = words.clone();
    replay.extend(["--replay", directory.to_str().ok_or("replay path")?]);
    let result = invoke(
        &format!("faults/failure/invoke1@surface={surface},fault={fault}"),
        &env,
        &replay,
        4,
    )?;
    failed(&result, source(surface))?;
    budgets::mcp(
        &format!("faults/failure/mcp1@surface={surface},fault={fault}"),
        &directory,
        &words,
        &result,
        4,
    )
}
fn failed(value: &Value, source: &str) -> Result<(), String> {
    if value.get("status") != Some(&json!("failed"))
        || value.get("result").is_some()
        || value.pointer("/sources/0/source") != Some(&json!(source))
        || value.pointer("/sources/0/state") != Some(&json!("failed"))
        || value
            .pointer("/sources/0/terms")
            .and_then(Value::as_str)
            .is_none_or(str::is_empty)
        || value
            .pointer("/sources/0/terms_url")
            .and_then(Value::as_str)
            .is_none_or(str::is_empty)
    {
        return Err(format!(
            "source failure became empty or lost terms: {value}"
        ));
    }
    Ok(())
}
fn seed(
    surface: &str,
    fault: &str,
    body: String,
    status: u16,
    delay: u64,
) -> Result<(Environment, std::path::PathBuf), String> {
    let hpo = source(surface) == "hpo";
    let stub = Stub::for_source(
        if hpo {
            "ontology.jax.org"
        } else {
            "api-v3.monarchinitiative.org"
        },
        "application/json",
        move |_| Ok("{}".into()),
    )?;
    let env = setup(&stub)?;
    let directory = env.root().join("seed");
    // A rejected synthetic seed still records the checked request without provider I/O.
    let mut words = args(surface);
    words.extend(["--record", directory.to_str().ok_or("record path")?]);
    invoke(
        &format!("faults/seed/invoke1@surface={surface},fault={fault}"),
        &env,
        &words,
        4,
    )?;
    if stub.finish()?.len() != 1 {
        return Err("seed request count".into());
    }
    for entry in fs::read_dir(directory.join(source(surface))).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let mut value: Value =
            serde_json::from_slice(&fs::read(entry.path()).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        let exchange = json!({"status":status,"content_type":"application/json","delay_ms":delay,"retry_after_ms":null,"body":{"kind":"text","value":body}});
        *value.get_mut("exchanges").ok_or("exchanges")? =
            json!([exchange.clone(), exchange.clone(), exchange]);
        fs::write(entry.path(), value.to_string()).map_err(|e| e.to_string())?;
    }
    Ok((env, directory))
}
fn identity(fault: &str) -> Result<(), String> {
    let surface = match fault {
        "hpo-id" => "hpo-card",
        "monarch-id" => "monarch-card",
        "taxon" => "genes",
        _ => "search",
    };
    let mut term = super::node();
    *term.get_mut("id").ok_or("synthetic node id")? =
        json!(if fault == "hpo-id" || fault == "monarch-id" {
            "HP:0000002"
        } else if fault == "MP" {
            "MP:0000001"
        } else {
            "UPHENO:0000001"
        });
    let body = if surface == "search" {
        json!({"limit":5,"offset":0,"total":1,"items":[term]})
    } else if surface == "genes" {
        let mut row = super::associations::row();
        *row.get_mut("object")
            .ok_or("synthetic association object")? = json!("HP:0000001");
        *row.get_mut("subject_taxon")
            .ok_or("synthetic association taxon")? = json!("NCBITaxon:10090");
        json!({"limit":1,"offset":0,"total":1,"items":[row]})
    } else {
        term
    };
    let stub = Stub::for_source_status(
        if source(surface) == "hpo" {
            "ontology.jax.org"
        } else {
            "api-v3.monarchinitiative.org"
        },
        "application/json",
        move |_| Ok(body.to_string()),
        |_| 200,
    )?;
    let env = setup(&stub)?;
    let words = args(surface);
    let mut result = Value::Null;
    for n in 0..2 {
        let directory = env.root().join(format!("identity-{n}"));
        result = super::record(
            &format!("faults/identity/record1@fault={fault},attempt={n}"),
            &env,
            &words,
            &directory,
            4,
        )?;
        failed(&result, source(surface))?;
        if n == 0 {
            budgets::mcp(
                &format!("faults/identity/mcp1@fault={fault},attempt={n}"),
                &directory,
                &words,
                &result,
                4,
            )?;
        }
    }
    if result
        .get("problems")
        .and_then(Value::as_array)
        .is_none_or(Vec::is_empty)
    {
        return Err(format!("identity rejection lost its problem: {result}"));
    }
    if stub.finish()?.len() != 2 {
        return Err("rejected identity entered cache".into());
    }
    Ok(())
}
