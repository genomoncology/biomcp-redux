//! Feature study-catalog: restricted study flags apply to an existing catalog and continuations.
use super::{ACADEMIC, call};
use crate::runner::{configuration::invoke, http::Server, launch::Environment, study_catalog};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
const FLAG: &str = "msk_chord_2024";
const RESTRICTED: &str = "x-biomcp-use: restricted\r\n";
const COMMERCIAL: &str = "x-biomcp-use: commercial\r\n";

pub(super) fn check() -> Result<(), String> {
    let assets = Environment::new().map_err(|e| e.to_string())?;
    let mut rows = study_catalog::rows();
    rows.as_array_mut().ok_or("catalog")?.truncate(3);
    rows[0]["studyId"] = json!(FLAG);
    rows[2]["studyId"] = json!("brca_tcga");
    let replay = study_catalog::seed(&assets, rows, 200)?;
    recurrence_recordings(&replay)?;
    let mut server = Server::configured_globals(
        "127.0.0.1:0",
        None,
        Some(replay.clone()),
        None,
        None,
        &["--hosted".into()],
    )?;
    // Sync once through the ordinary CLI into the running server's isolated store.
    let env = server.environment();
    let mut sync = env.globals(Some(&replay), false);
    sync.extend(["sync".into(), "cbioportal-studies".into()]);
    invoke(env, sync, 0)?;
    fs::remove_file(replay.join("cbioportal").join(catalog_key())).map_err(|e| e.to_string())?;
    let first = data(call(
        &server,
        &["search", "study", "--limit", "1"],
        ACADEMIC,
        false,
    )?)?;
    if first.pointer("/page/total") != Some(&json!(3))
        || first.pointer("/result/items/0/studyId") != Some(&json!(FLAG))
    {
        return Err(format!("academic catalog: {first}"));
    }
    let pool = first
        .pointer("/page/pool/id")
        .and_then(Value::as_str)
        .ok_or("pool")?;
    let args = ["search", "study", "--limit", "1", "--pool", pool];
    let restricted = data(call(&server, &args, RESTRICTED, false)?)?;
    if restricted.pointer("/page/total") != Some(&json!(2))
        || restricted.pointer("/result/items/0/studyId") != Some(&json!("synthetic_1"))
    {
        return Err(format!(
            "restricted retained catalog leaked a flagged study: {restricted}"
        ));
    }
    notice(&restricted)?;
    let next = restricted["next"]
        .as_array()
        .and_then(|rows| rows.iter().find(|v| v["kind"] == "page"))
        .and_then(|v| v["args"].as_array())
        .ok_or("filtered continuation")?;
    let next = next
        .iter()
        .map(|v| v.as_str().ok_or("next arg"))
        .collect::<Result<Vec<_>, _>>()?;
    let second = data(call(&server, &next, RESTRICTED, false)?)?;
    if second.pointer("/result/items/0/studyId") != Some(&json!("brca_tcga"))
        || second.pointer("/page/total") != Some(&json!(2))
    {
        return Err(format!(
            "filtered continuation changed order or total: {second}"
        ));
    }
    for header in [ACADEMIC, COMMERCIAL] {
        let restored = data(call(&server, &args, header, false)?)?;
        if restored.pointer("/page/total") != Some(&json!(3))
            || restored.pointer("/result/items/0/studyId") != Some(&json!(FLAG))
        {
            return Err(format!(
                "restricted read changed the shared snapshot: {restored}"
            ));
        }
        let card = data(call(&server, &["get", "study", FLAG], header, false)?)?;
        if card.pointer("/result/study/header/studyId").is_none()
            && !card.to_string().contains("Synthetic Atlas 0")
        {
            return Err(format!("permitted study missing: {card}"));
        }
    }
    for section in [
        None,
        Some("profiles"),
        Some("sample-lists"),
        Some("files"),
        Some("samples"),
        Some("all"),
    ] {
        let mut args = vec!["get", "study", FLAG];
        args.extend(section);
        let reply = data(call(&server, &args, RESTRICTED, false)?)?;
        blocked(&reply)?;
    }
    let words = [
        "variant",
        "recurrence",
        "NC_000007.13:g.140494253G>A",
        "--source",
        "cbioportal",
        "--gene",
        "BRAF",
        "--study",
        FLAG,
        "--profile",
        "msk_chord_2024_mutations",
        "--sample-set",
        "msk_chord_2024_all",
        "--limit",
        "3",
    ];
    for header in [ACADEMIC, COMMERCIAL] {
        let reply = data(call(&server, &words, header, false)?)?;
        if reply.pointer("/result/observed_sample_count") != Some(&json!(1)) {
            return Err(format!("permitted recurrence missing: {reply}"));
        }
    }
    for page in ["0", "1"] {
        let mut args = words.to_vec();
        args.extend(["--page", page]);
        let reply = data(call(&server, &args, RESTRICTED, false)?)?;
        blocked(&reply)?;
        if reply.pointer("/result/observed_sample_count").is_some()
            || reply.pointer("/result/selected_sample_count").is_some()
        {
            return Err(format!("restricted recurrence leaked counts: {reply}"));
        }
    }
    server.stop()
}
fn data(reply: Value) -> Result<Value, String> {
    reply
        .pointer("/result/structuredContent")
        .cloned()
        .ok_or_else(|| format!("missing tool reply: {reply}"))
}
fn notice(reply: &Value) -> Result<(), String> {
    let text = reply["sources"].to_string();
    if !text.contains(FLAG) || !text.contains("left out") || !text.contains("noncommercial") {
        return Err(format!("study exclusion has no reason: {reply}"));
    }
    if reply.pointer("/sources/0/use") != Some(&json!("attribution")) {
        return Err(format!("cBioPortal lost attribution: {reply}"));
    }
    Ok(())
}
fn blocked(reply: &Value) -> Result<(), String> {
    notice(reply)?;
    if reply.get("result").is_some()
        || reply.pointer("/sources/0/reason") != Some(&json!("disabled"))
        || reply["next"]
            .as_array()
            .is_some_and(|rows| !rows.is_empty())
    {
        return Err(format!("flagged study was not withheld: {reply}"));
    }
    Ok(())
}
fn catalog_key() -> String {
    key(
        "GET",
        "https://www.cbioportal.org/api/studies?projection=DETAILED",
    )
}
fn key(method: &str, url: &str) -> String {
    format!("{:x}.json", Sha256::digest(format!("{method}\0{url}")))
}
fn recurrence_recordings(target: &Path) -> Result<(), String> {
    let original = crate::runner::launch::fixtures().join("replies/variant/recurrence");
    for source in fs::read_dir(original).map_err(|e| e.to_string())? {
        let source = source.map_err(|e| e.to_string())?;
        let destination = target.join(source.file_name());
        fs::create_dir_all(&destination).map_err(|e| e.to_string())?;
        for file in fs::read_dir(source.path()).map_err(|e| e.to_string())? {
            let file = file.map_err(|e| e.to_string())?;
            let text = fs::read_to_string(file.path()).map_err(|e| e.to_string())?;
            let text = text.replace("msk_impact_2017", FLAG);
            let record: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
            let filename = key(
                record["method"].as_str().ok_or("method")?,
                record["url"].as_str().ok_or("url")?,
            );
            fs::write(destination.join(filename), text).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
