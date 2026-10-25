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
        if !path.extension().is_some_and(|v| v == "json") {
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
    for (id, native) in [
        ("GTR000192572.11", "-1"),
        ("GTR000192578.9", "-1"),
        ("GTR000567741.1", "-1"),
        ("GTR000900001.1", "0"),
        ("GTR000900002.1", " opaque "),
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
        budget(&page, 4096)?;
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
    budget(&positive, 4096)?;
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
    budget(&empty, 4096)
}
fn budget(value: &Value, cap: usize) -> Result<(), String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    println!("native value page {} bytes", bytes);
    if bytes > cap {
        return Err(format!("native value page {bytes} exceeds {cap}"));
    }
    Ok(())
}
