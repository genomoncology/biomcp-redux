//! Unchanged retained reverse association requests prove source rows in both interfaces.
use super::{Stub, native, raw_invoke, setup};
use crate::runner::tools_api::budget;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::io::Write;
use std::{collections::BTreeMap, fs, path::Path};
pub(super) fn run(directory: &Path) -> Result<(), String> {
    let plan: Vec<Value> =
        serde_json::from_slice(&fs::read(directory.join("plan.json")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let mut replies = BTreeMap::new();
    for row in &plan {
        let bytes = fs::read(directory.join(text(row, "body")?)).map_err(|e| e.to_string())?;
        let hash = Sha256::digest(&bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        if hash != text(row, "sha256")? {
            return Err("retained body hash changed".into());
        }
        replies.insert(
            text(row, "target")?.to_owned(),
            String::from_utf8(bytes).map_err(|e| e.to_string())?,
        );
    }
    let stub = Stub::for_sources(
        &["api-v3.monarchinitiative.org"],
        |_| "application/json",
        move |request| {
            let target = request.split_whitespace().nth(1).ok_or("target")?;
            replies
                .get(target)
                .cloned()
                .ok_or_else(|| format!("undeclared association request: {target}"))
        },
    )?;
    let env = setup(&stub)?;
    let output = directory.join("product-proof");
    fs::create_dir_all(&output).map_err(|e| e.to_string())?;
    let mut completed = 0;
    for (index, row) in plan.iter().enumerate() {
        if !output.join(format!("{index}-cli-mcp.json")).is_file() {
            prove(directory, &output, &env, index, row)?;
            completed += 1;
        }
        retained(directory, &output, index, row)?;
    }
    if stub.finish()?.len() != completed {
        return Err("retained proof request count changed".into());
    }
    Ok(())
}
fn prove(
    directory: &Path,
    output: &Path,
    env: &super::super::launch::Environment,
    index: usize,
    row: &Value,
) -> Result<(), String> {
    let section = text(row, "section")?;
    let limit = row
        .get("limit")
        .and_then(Value::as_u64)
        .ok_or("limit")?
        .to_string();
    let offset = row
        .get("offset")
        .and_then(Value::as_u64)
        .ok_or("offset")?
        .to_string();
    let mut words = vec![
        "get",
        "phenotype",
        "HP:0001250",
        section,
        "--source",
        "monarch",
    ];
    if limit != "1" {
        words.extend(["--section-limit", &limit]);
    }
    if offset != "0" {
        words.extend(["--section-offset", &offset]);
    }
    let recorded = output.join(format!("record-{index}"));
    let mut recording = words.clone();
    recording.extend(["--record", recorded.to_str().ok_or("record path")?]);
    raw_invoke(env, &recording, 0)?;
    native::restore_date(
        &recorded,
        row.get("fetched")
            .and_then(Value::as_u64)
            .ok_or("fetched")?,
    )?;
    let mut replay = words.clone();
    replay.extend(["--replay", recorded.to_str().ok_or("recordpath")?]);
    let value = raw_invoke(env, &replay, 0)?;
    let expected: Value = serde_json::from_slice(
        &fs::read(directory.join(text(row, "body")?)).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    verify(&expected, &value)?;
    let comparable = super::super::source_terms::size(&value, "")?;
    let ceiling = ceiling(row)?;
    writeln!(
        std::io::stderr(),
        "phenotype-associations {section} limit{limit} offset{offset}: comparable={comparable} tokens={} ceiling={}",
        comparable.div_ceil(4),
        (comparable * 115).div_ceil(100)
    )
    .map_err(|e| e.to_string())?;
    if comparable > ceiling {
        return Err("association section exceeds design ceiling".into());
    }
    native::mcp(&recorded, &words, &value, 0, Some(ceiling))?;
    fs::write(
        output.join(format!("{index}-cli-mcp.json")),
        serde_json::to_vec_pretty(&value).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
fn verify(expected: &Value, value: &Value) -> Result<(), String> {
    if value.pointer("/page/total") != expected.get("total")
        || value.pointer("/page/offset") != expected.get("offset")
        || value.pointer("/result/direct") != Some(&json!(true))
    {
        return Err("native scope or totals changed".into());
    }
    let rows = expected
        .get("items")
        .and_then(Value::as_array)
        .ok_or("nativeitems")?;
    let mapped = value
        .pointer("/result/items")
        .and_then(Value::as_array)
        .ok_or("mappeditems")?;
    if rows.len() != mapped.len() {
        return Err("source rows dropped or deduplicated".into());
    }
    for (row, mapped) in rows.iter().zip(mapped) {
        let mapped = mapped.get("association").ok_or("mapped association")?;
        for (from, to) in [
            ("subject", "subject/id"),
            ("object", "object/id"),
            ("subject_taxon", "subject_taxon"),
            ("original_subject", "original_subject"),
            ("original_object", "original_object"),
            ("negated", "negated"),
            ("subject_closure", "subject_closure"),
            ("object_closure", "object_closure"),
        ] {
            if let Some(source) = row.get(from).filter(|v| !v.is_null()) {
                if mapped.pointer(&format!("/{to}")) != Some(source) {
                    return Err(format!("native {from} changed"));
                }
            } else if !matches!(from, "subject_closure" | "object_closure")
                && mapped.pointer(&format!("/{to}")).is_some()
            {
                return Err(format!("absent {from} invented"));
            }
        }
        qualifiers(row, mapped)?;
        if mapped.get("requested_id") != Some(&json!("HP:0001250")) {
            return Err("requested HP identity changed".into());
        }
        for field in ["has_evidence", "publications"] {
            let target = if field == "has_evidence" {
                "evidence"
            } else {
                field
            };
            let source = row
                .get(field)
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            let ids = mapped
                .get(target)
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .map(|r| r.get("id").cloned().ok_or("reference ID"))
                .collect::<Result<Vec<_>, _>>()?;
            if ids != source {
                return Err(format!("native {field} moved or changed"));
            }
        }
    }
    Ok(())
}
fn text<'a>(row: &'a Value, key: &str) -> Result<&'a str, String> {
    row.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing {key}"))
}

fn qualifiers(raw: &Value, mapped: &Value) -> Result<(), String> {
    let expected = [
        "frequency_qualifier",
        "onset_qualifier",
        "sex_qualifier",
        "disease_context_qualifier",
        "species_context_qualifier",
        "stage_qualifier",
    ];
    let rows = mapped
        .get("qualifiers")
        .and_then(Value::as_array)
        .ok_or("mapped qualifiers")?;
    for field in expected {
        let source = raw.get(field).filter(|v| !v.is_null());
        let selected = rows
            .iter()
            .find(|r| r.get("field").and_then(Value::as_str) == Some(field));
        match (source, selected) {
            (Some(id), Some(row)) if row.get("id") == Some(id) => {
                let label = raw.get(format!("{field}_label")).filter(|v| !v.is_null());
                if row.get("label") != label {
                    return Err(format!("native {field} label moved"));
                }
            }
            (None, None) => (),
            _ => return Err(format!("native {field} moved or changed")),
        }
    }
    Ok(())
}

fn ceiling(row: &Value) -> Result<usize, String> {
    let section = text(row, "section")?;
    let limit = row.get("limit").and_then(Value::as_u64).ok_or("limit")?;
    let offset = row.get("offset").and_then(Value::as_u64).ok_or("offset")?;
    budget(&format!("phenotype-association/{section}/{limit}/{offset}"))
}
fn retained(directory: &Path, output: &Path, index: usize, row: &Value) -> Result<(), String> {
    let read = |p: &Path| -> Result<Value, String> {
        serde_json::from_slice(&fs::read(p).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
    };
    let value = read(&output.join(format!("{index}-cli-mcp.json")))?;
    verify(&read(&directory.join(text(row, "body")?))?, &value)?;
    let offset = row.get("offset").and_then(Value::as_u64).ok_or("offset")?;
    let limit = row.get("limit").and_then(Value::as_u64).ok_or("limit")?;
    let section = text(row, "section")?;
    let mut expected = vec![
        "get".to_owned(),
        "phenotype".into(),
        "HP:0001250".into(),
        section.into(),
    ];
    if limit != 1 {
        expected.extend(["--section-limit".into(), limit.to_string()]);
    }
    expected.extend(["--section-offset".into(), (offset + limit).to_string()]);
    let page = value
        .get("next")
        .and_then(Value::as_array)
        .ok_or("next page")?
        .iter()
        .find(|s| s.get("kind") == Some(&json!("page")))
        .ok_or("native page step")?;
    if page.get("args") != Some(&json!(expected)) {
        return Err(format!("native next arguments changed: {page}"));
    }
    let total = if section == "diseases" { 2561 } else { 1409 };
    let id = match (section, offset) {
        ("diseases", 0) => "MONDO:0014379",
        ("diseases", 1) => "MONDO:0012596",
        ("genes", 0) => "HGNC:9910",
        ("genes", 1) => "HGNC:29529",
        _ => return Err("unmeasured identity".into()),
    };
    if value.pointer("/page/total") != Some(&json!(total))
        || value.pointer("/result/items/0/association/subject/id") != Some(&json!(id))
    {
        return Err("retained association identity or total changed".into());
    }
    let bytes = super::super::source_terms::size(&value, "")?;
    let cap = ceiling(row)?;
    if bytes > cap || bytes.div_ceil(4) > cap.div_ceil(4) {
        return Err("measured association cap exceeded".into());
    }
    let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
    writeln!(
        std::io::stderr(),
        "retained {section} limit{limit} offset{offset}: full={full} comparable={bytes} tokens={} cap={cap} \
        token_cap={}",
        bytes.div_ceil(4),
        cap.div_ceil(4)
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
