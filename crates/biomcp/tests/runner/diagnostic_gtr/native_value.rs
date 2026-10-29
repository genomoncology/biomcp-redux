//! Feature diagnostic-gtr: synthetic labels retain captured native identity shapes.
use super::{
    super::launch::Environment,
    mechanics::{call, seed},
};
use serde_json::{Value, json};
use std::fs;

pub(super) fn run() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    seed_native(&replay)?;
    let receipt = call(&env, &replay, &["sync", "gtr"], 0)?;
    if receipt.pointer("/result/rows") != Some(&json!(25)) {
        return Err(format!("unusual rows rejected bundle: {receipt}"));
    }
    check_pages(&env, &replay)
}
fn seed_native(replay: &std::path::Path) -> Result<(), String> {
    for entry in fs::read_dir(replay.join("gtr")).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().is_none_or(|v| v != "json") {
            continue;
        }
        let mut value: Value = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        if !value
            .get("url")
            .and_then(Value::as_str)
            .is_some_and(|v| v.ends_with("test_condition_gene.txt"))
        {
            continue;
        }
        let body = value
            .pointer_mut("/exchanges/0/body/value")
            .ok_or("missing body")?;
        let mut text = body.as_str().ok_or("body is not text")?.to_owned();
        // Three retained -1 identities and a nearby positive identity; all labels are synthetic.
        for (id, kind, medgen, native, symbol) in [
            ("GTR000192572.11", "Research", "CN166714", "-1", "-"),
            ("GTR000192578.9", "Research", "CN166714", "-1", "-"),
            ("GTR000567741.1", "Clinical", "CN258947", "-1", "-"),
            (
                "GTR000174333.8",
                "Clinical",
                "C1414504",
                "2147",
                "SYNTHETIC",
            ),
            ("GTR000900001.1", "Clinical", "C0000001", "0", "SYNTHETIC"),
            (
                "GTR000900002.1",
                "Clinical",
                "C0000001",
                " opaque ",
                "SYNTHETIC",
            ),
        ] {
            text.push_str(&format!("{id}\t{kind}\tgene\t{medgen}\t\tSynthetic native value fixture\t{native}\t{symbol}\n"));
        }
        *body = json!(text);
        fs::write(&path, value.to_string()).map_err(|e| e.to_string())?;
    }
    Ok(())
}
fn check_pages(env: &Environment, replay: &std::path::Path) -> Result<(), String> {
    for (id, native, cap) in [
        ("GTR000192572.11", "-1", 763),
        ("GTR000192578.9", "-1", 761),
        ("GTR000567741.1", "-1", 761),
        ("GTR000900001.1", "0", 769),
        ("GTR000900002.1", " opaque ", 777),
    ] {
        let page = call(
            env,
            replay,
            &["get", "diagnostic", id, "--source", "gtr", "genes"],
            0,
        )?;
        if page.pointer("/result/items/0/native_gene_or_snomed_ct_id") != Some(&json!(native))
            || page.pointer("/result/items/0/gene/ncbi_gene_id").is_some()
            || page
                .get("next")
                .is_some_and(|v| v.as_array().is_none_or(|v| !v.is_empty()))
            || page.pointer("/result/distinct_id_total") != Some(&json!(0))
        {
            return Err(format!("native value became a gene pivot: {page}"));
        }
        budget(&page, cap)?;
    }
    check_known(env, replay)
}
fn check_known(env: &Environment, replay: &std::path::Path) -> Result<(), String> {
    let positive = call(
        env,
        replay,
        &[
            "get",
            "diagnostic",
            "GTR000174333.8",
            "--source",
            "gtr",
            "genes",
        ],
        0,
    )?;
    if positive.pointer("/result/items/0/gene/ncbi_gene_id") != Some(&json!("2147"))
        || positive.pointer("/next/0/args") != Some(&json!(["get", "gene", "2147"]))
        || positive
            .pointer("/result/items/0/native_gene_or_snomed_ct_id")
            .is_some()
    {
        return Err(format!("positive native gene lost its pivot: {positive}"));
    }
    budget(&positive, 826)?;
    let empty = call(
        env,
        replay,
        &[
            "get",
            "diagnostic",
            "GTR000511179.13",
            "--source",
            "gtr",
            "genes",
        ],
        0,
    )?;
    if empty
        .pointer("/result/items/1/native_gene_or_snomed_ct_id")
        .is_some()
        || empty.pointer("/result/items/1/gene/ncbi_gene_id").is_some()
    {
        return Err(format!("empty native value invented an identity: {empty}"));
    }
    budget(&empty, 998)
}
fn budget(value: &Value, cap: usize) -> Result<(), String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    if bytes > cap {
        return Err(format!("native value page {bytes} exceeds {cap}"));
    }
    Ok(())
}

pub(super) fn text_admission() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    call(&env, &replay, &["sync", "gtr"], 0)?;
    let args = ["get", "diagnostic", "GTR000006692.3", "--source", "gtr"];
    let empty = call(&env, &replay, &args, 0)?;
    if empty
        .pointer("/result/header/laboratory/institution")
        .is_some()
        || empty
            .get("problems")
            .and_then(Value::as_array)
            .is_some_and(|p| !p.is_empty())
    {
        return Err(format!("native empty cell produced a problem: {empty}"));
    }
    // Synthetic stored-row edit isolates view admission from whole-cell bundle retention.
    let cache = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    let changed = cache
        .execute(
            "UPDATE bundle_row SET row=json_set(row,'$.value.name_of_institution',?1) \
         WHERE bundle='gtr' AND json_extract(row,'$.value.test_accession_ver')='GTR000006692.3'",
            ["x".repeat(biodata::Text::MAX_CHARS + 1)],
        )
        .map_err(|e| e.to_string())?;
    if changed != 1 {
        return Err(format!("edited {changed} native rows"));
    }
    let invalid = call(&env, &replay, &args, 0)?;
    if invalid.pointer("/result/header/id") != Some(&json!("GTR000006692.3"))
        || invalid
            .pointer("/result/header/laboratory/institution")
            .is_some()
        || !invalid
            .get("problems")
            .and_then(Value::as_array)
            .is_some_and(|p| {
                p.iter().any(|p| {
                    p.get("source") == Some(&json!("gtr"))
                        && p.get("field") == Some(&json!("name_of_institution"))
                })
            })
    {
        return Err(format!(
            "invalid prose lost its identity or disclosure: {invalid}"
        ));
    }
    Ok(())
}
