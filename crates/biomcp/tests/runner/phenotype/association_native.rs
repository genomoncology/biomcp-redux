//! Unchanged retained reverse association requests prove source rows in both interfaces.
use super::{Stub, invoke, native, record, setup};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
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
        if output.join(format!("{index}-cli-mcp.json")).is_file() {
            continue;
        }
        prove(directory, &output, &env, index, row)?;
        completed += 1;
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
    let limit = row["limit"].as_u64().ok_or("limit")?.to_string();
    let offset = row["offset"].as_u64().ok_or("offset")?.to_string();
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
    record(env, &words, &recorded, 0)?;
    native::restore_date(&recorded, row["fetched"].as_u64().ok_or("fetched")?)?;
    let mut replay = words.clone();
    replay.extend(["--replay", recorded.to_str().ok_or("recordpath")?]);
    let value = invoke(env, &replay, 0)?;
    let expected: Value = serde_json::from_slice(
        &fs::read(directory.join(text(row, "body")?)).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    verify(&expected, &value)?;
    let comparable = super::super::source_terms::size(&value, "")?;
    let ceiling = if limit == "1" { 9888 } else { 15792 };
    eprintln!(
        "phenotype-associations {section} limit{limit} offset{offset}: comparable={comparable} tokens={} ceiling={}",
        comparable.div_ceil(4),
        (comparable * 115).div_ceil(100)
    );
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
    let rows = expected["items"].as_array().ok_or("nativeitems")?;
    let mapped = value
        .pointer("/result/items")
        .and_then(Value::as_array)
        .ok_or("mappeditems")?;
    if rows.len() != mapped.len() {
        return Err("source rows dropped or deduplicated".into());
    }
    for (row, mapped) in rows.iter().zip(mapped) {
        let mapped = &mapped["association"];
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
        if mapped["requested_id"] != "HP:0001250" {
            return Err("requested HP identity changed".into());
        }
        for field in ["has_evidence", "publications"] {
            let target = if field == "has_evidence" {
                "evidence"
            } else {
                field
            };
            let source = row[field].as_array().cloned().unwrap_or_default();
            let ids = mapped[target]
                .as_array()
                .ok_or("referencearray")?
                .iter()
                .map(|r| r["id"].clone())
                .collect::<Vec<_>>();
            if ids != source {
                return Err(format!("native {field} moved or changed"));
            }
        }
    }
    Ok(())
}
fn text<'a>(row: &'a Value, key: &str) -> Result<&'a str, String> {
    row[key].as_str().ok_or_else(|| format!("missing {key}"))
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
    let rows = mapped["qualifiers"].as_array().ok_or("mapped qualifiers")?;
    for field in expected {
        let source = raw.get(field).filter(|v| !v.is_null());
        let selected = rows.iter().find(|r| r["field"] == field);
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
