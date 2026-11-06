//! Feature study-catalog: restricted study flags apply to an existing catalog and continuations.
use super::{ACADEMIC, call};
use crate::runner::{configuration::invoke, http::Server, launch::Environment, study_catalog};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
const FLAG: &str = "msk_chord_2024";
const FLAGS: &[(&str, &str)] = &[
    (FLAG, "noncommercial"),
    ("asclc_msk_2024", "noncommercial"),
    ("hcc_msk_2024", "noncommercial"),
    ("msk_ctdna_vte_2024", "noncommercial"),
    ("msk_met_2021", "noncommercial"),
    ("pancreas_msk_2024", "noncommercial"),
    ("panet_msk_2018", "noncommercial"),
    ("prostate_msk_2024", "noncommercial"),
    ("msk_impact_50k_2026", "noncommercial"),
    ("all_phase2_target_2018_pub", "pediatric"),
    ("aml_target_2018_pub", "pediatric"),
    ("nbl_target_2018_pub", "pediatric"),
    ("wt_target_2018_pub", "pediatric"),
    ("rt_target_2018_pub", "pediatric"),
    ("angs_painter_2020", "clinical decision-making"),
    ("angs_painter_2025", "clinical decision-making"),
    ("brca_mbcproject_wagle_2017", "clinical decision-making"),
    ("brca_mbcproject_2022", "clinical decision-making"),
    ("mpcproject_broad_2021", "clinical decision-making"),
];
const RESTRICTED: &str = "x-biomcp-use: restricted\r\n";
const COMMERCIAL: &str = "x-biomcp-use: commercial\r\n";

pub(super) fn check() -> Result<(), String> {
    let assets = Environment::new().map_err(|e| e.to_string())?;
    let mut rows = study_catalog::rows();
    rows.as_array_mut().ok_or("catalog")?.truncate(3);
    rows[0]["studyId"] = json!(FLAG);
    rows[2]["studyId"] = json!("brca_tcga");
    let template = rows[0].clone();
    for (id, _) in &FLAGS[1..] {
        let mut row = template.clone();
        row["studyId"] = json!(id);
        rows.as_array_mut().ok_or("catalog")?.push(row);
    }
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
    if first.pointer("/page/total") != Some(&json!(21))
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
        || restricted.pointer("/page/pool/size") != Some(&json!(2))
        || restricted.pointer("/result/items/0/studyId") != Some(&json!("synthetic_1"))
    {
        return Err(format!(
            "restricted retained catalog leaked a flagged study: {restricted}"
        ));
    }
    for (id, reason) in FLAGS {
        notice(&restricted, id, reason)?;
        let direct = data(call(&server, &["get", "study", id], RESTRICTED, false)?)?;
        blocked(&direct, id, reason)?;
    }
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
        if restored.pointer("/page/total") != Some(&json!(21))
            || restored.pointer("/page/pool/size") != Some(&json!(21))
            || restored.pointer("/result/items/0/studyId") != Some(&json!(FLAG))
        {
            return Err(format!(
                "restricted read changed the shared snapshot: {restored}"
            ));
        }
        for id in [FLAG, "all_phase2_target_2018_pub", "angs_painter_2020"] {
            let card = data(call(&server, &["get", "study", id], header, false)?)?;
            if !card.to_string().contains("Synthetic Atlas 0") {
                return Err(format!("permitted study missing: {card}"));
            }
        }
    }
    for section in [
        None,
        Some("profiles"),
        Some("sample-lists"),
        Some("files"),
        Some("samples"),
        Some("mutations"),
        Some("all"),
    ] {
        let mut args = vec!["get", "study", FLAG];
        args.extend(section);
        if section == Some("mutations") {
            args.extend(["--gene", "BRAF"]);
        }
        let reply = data(call(&server, &args, RESTRICTED, false)?)?;
        blocked(&reply, FLAG, "noncommercial")?;
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
        for (id, reason) in [
            (FLAG, "noncommercial"),
            ("all_phase2_target_2018_pub", "pediatric"),
            ("angs_painter_2020", "clinical decision-making"),
        ] {
            args[8] = id;
            let reply = data(call(&server, &args, RESTRICTED, false)?)?;
            blocked(&reply, id, reason)?;
            if reply.pointer("/result/observed_sample_count").is_some()
                || reply.pointer("/result/selected_sample_count").is_some()
            {
                return Err(format!("restricted recurrence leaked counts: {reply}"));
            }
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
fn notice(reply: &Value, id: &str, reason: &str) -> Result<(), String> {
    let text = reply["sources"].to_string();
    if !text.contains(id) || !text.contains("left out") || !text.contains(reason) {
        return Err(format!("study exclusion has no reason: {reply}"));
    }
    if reply.pointer("/sources/0/use") != Some(&json!("attribution")) {
        return Err(format!("cBioPortal lost attribution: {reply}"));
    }
    Ok(())
}
fn blocked(reply: &Value, id: &str, reason: &str) -> Result<(), String> {
    notice(reply, id, reason)?;
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
    let digest = Sha256::digest(format!("{method}\0{url}"));
    let hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
    format!("{hex}.json")
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
