//! Native term replies stay outside Git and are admitted only by exact hash.
use super::{Stub, native, raw_invoke, setup};
use serde_json::{Value, json};
use std::{fs, io::Write, path::Path};
pub(super) fn run(root: &Path) -> Result<(), String> {
    let plan: Vec<Value> =
        serde_json::from_slice(&fs::read(root.join("plan.json")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let outputs = root.join("outputs");
    fs::create_dir(&outputs).map_err(|e| e.to_string())?;
    for (index, row) in plan.iter().enumerate() {
        prove(root, &outputs, index, row)?;
    }
    Ok(())
}
fn text<'a>(row: &'a Value, key: &str) -> Result<&'a str, String> {
    row.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing {key}"))
}
fn prove(root: &Path, outputs: &Path, index: usize, row: &Value) -> Result<(), String> {
    let (raw, body) = native::archived_body(root, row)?;
    let hpo = text(row, "source")? == "hpo";
    let (env, stub) = environment(row, body)?;
    let words = row
        .get("words")
        .and_then(Value::as_array)
        .ok_or("words")?
        .iter()
        .map(|v| v.as_str().ok_or("word"))
        .collect::<Result<Vec<_>, _>>()?;
    let directory = outputs.join(format!("record-{index:02}"));
    let mut record = words.clone();
    record.extend(["--record", directory.to_str().ok_or("record path")?]);
    let expected = expected(&raw, hpo, text(row, "section")?)?;
    let exit = if expected.as_ref().is_none_or(Vec::is_empty) {
        1
    } else {
        0
    };
    let value = raw_invoke(&env, &record, exit)?;
    verify(&value, expected.as_deref(), row)?;
    if text(row, "section")? == "synonyms" {
        measure_card(&env, &directory, outputs, row)?;
    }
    if raw.get("id") == Some(&json!("HP:0001250"))
        && text(row, "section")? == "hierarchy"
        && row.get("offset") == Some(&json!(0))
        && row.get("limit") == Some(&json!(5))
    {
        walk(
            &env,
            &value,
            expected.as_deref().ok_or("native hierarchy")?,
            row,
        )?;
    }
    if let Some(fetched) = row.get("fetched").and_then(Value::as_u64) {
        native::restore_date(&directory, fetched)?;
    }
    let mut replay = words.clone();
    replay.extend(["--replay", directory.to_str().ok_or("record path")?]);
    let result = raw_invoke(&env, &replay, exit)?;
    verify(&result, expected.as_deref(), row)?;
    native::mcp(
        &directory,
        &words,
        &result,
        exit,
        Some(budget(&format!("section-{index:02}"), &result)?),
    )?;
    let full = serde_json::to_vec(&result)
        .map_err(|e| e.to_string())?
        .len();
    let comparable = super::super::source_terms::size(&result, "")?;
    writeln!(
        std::io::stdout(),
        "phenotype section {index:02}: CLI={} MCP={full} comparable={comparable}",
        full + 1
    )
    .map_err(|e| e.to_string())?;
    fs::write(
        outputs.join(format!("{index:02}.json")),
        serde_json::to_vec(&result).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    if stub.finish()?.len() != 1 {
        return Err("section fetched another request".into());
    }
    Ok(())
}
fn expected(raw: &Value, hpo: bool, section: &str) -> Result<Option<Vec<Value>>, String> {
    let fields: Vec<(&str, Option<&str>)> = match section {
        "synonyms" if hpo => vec![("synonyms", None)],
        "synonyms" => vec![
            ("synonym", None),
            ("exact_synonym", Some("exact")),
            ("broad_synonym", Some("broad")),
            ("narrow_synonym", Some("narrow")),
            ("related_synonym", Some("related")),
        ],
        "xrefs" => vec![(if hpo { "xrefs" } else { "xref" }, None)],
        "hierarchy" => vec![
            ("/node_hierarchy/super_classes", Some("super_classes")),
            ("/node_hierarchy/sub_classes", Some("sub_classes")),
        ],
        _ => return Err("section".into()),
    };
    let mut available = false;
    let mut result = Vec::new();
    for (field, relation) in fields {
        let value = if field.starts_with('/') {
            raw.pointer(field)
        } else {
            raw.get(field)
        };
        if let Some(rows) = value.filter(|v| !v.is_null()).and_then(Value::as_array) {
            available = true;
            for row in rows {
                let item = match section {
                    "synonyms" => {
                        let mut item = json!({"value":row});
                        if let Some(relation) = relation {
                            item.as_object_mut()
                                .ok_or("item")?
                                .insert("relationship".into(), json!(relation));
                        }
                        item
                    }
                    "xrefs" => json!({"id":row}),
                    _ => {
                        json!({"id":row.get("id").ok_or("link ID")?,"label":row.get("name").ok_or("link label")?,"relation":relation.ok_or("relation")?})
                    }
                };
                result.push(item);
            }
        }
    }
    Ok(available.then_some(result))
}
fn verify(value: &Value, expected: Option<&[Value]>, row: &Value) -> Result<(), String> {
    let Some(expected) = expected else {
        if value.get("result").is_some() || value.get("page").is_some() {
            return Err("native null section became empty".into());
        }
        return Ok(());
    };
    let offset = row.get("offset").and_then(Value::as_u64).unwrap_or(0) as usize;
    let limit = row.get("limit").and_then(Value::as_u64).unwrap_or(5) as usize;
    let items = value
        .pointer("/result/items")
        .and_then(Value::as_array)
        .ok_or("items")?;
    let selected = expected.iter().skip(offset).take(limit).collect::<Vec<_>>();
    if items.len() != selected.len()
        || value.pointer("/page/total") != Some(&json!(expected.len()))
        || value.pointer("/page/offset") != Some(&json!(offset))
    {
        return Err("native collection page changed".into());
    }
    for (item, expected) in items.iter().zip(selected) {
        let mut item = item.clone();
        item.as_object_mut().ok_or("item")?.remove("attribution");
        if &item != expected {
            return Err(format!("native row changed: {item} != {expected}"));
        }
    }
    Ok(())
}

fn walk(
    env: &super::super::launch::Environment,
    first: &Value,
    expected: &[Value],
    row: &Value,
) -> Result<(), String> {
    let mut value = first.clone();
    let mut offset = 0_u64;
    let mut token: Option<String> = None;
    while let Some(next) = value
        .get("next")
        .and_then(Value::as_array)
        .and_then(|next| next.first())
    {
        if next.get("kind") != Some(&json!("page")) {
            return Err("hierarchy offered an unrelated step".into());
        }
        let words: Vec<String> =
            serde_json::from_value(next.get("args").ok_or("next args")?.clone())
                .map_err(|e| e.to_string())?;
        let selected = words
            .windows(2)
            .find(|pair| pair.first().is_some_and(|v| v == "--snapshot"))
            .and_then(|pair| pair.get(1))
            .ok_or("retained hierarchy selector")?;
        if token.as_ref().is_some_and(|token| token != selected) {
            return Err("hierarchy walk changed retained reply".into());
        }
        token = Some(selected.clone());
        offset += 5;
        let mut page = row.clone();
        page.as_object_mut()
            .ok_or("row")?
            .insert("offset".into(), json!(offset));
        value = raw_invoke(
            env,
            &words.iter().map(String::as_str).collect::<Vec<_>>(),
            0,
        )?;
        verify(&value, Some(expected), &page)?;
        if value.pointer("/page/returned")
            != Some(&json!(
                expected.len().saturating_sub(offset as usize).min(5)
            ))
            || value.pointer("/query/section") != Some(&json!("hierarchy"))
            || value.pointer("/query/source") != Some(&json!("monarch"))
        {
            return Err("hierarchy next selectors changed".into());
        }
    }
    if offset != 10 {
        return Err("native hierarchy walk did not reach its terminal local page".into());
    }
    Ok(())
}

fn measure_card(
    env: &super::super::launch::Environment,
    directory: &Path,
    outputs: &Path,
    row: &Value,
) -> Result<(), String> {
    let words = row.get("words").and_then(Value::as_array).ok_or("words")?;
    let id = words.get(2).and_then(Value::as_str).ok_or("identity")?;
    let source = text(row, "source")?;
    let args = ["get", "phenotype", id, "--source", source];
    let card = raw_invoke(env, &args, 0)?;
    let key = format!("card-{}-{source}", id.replace(':', "_"));
    native::mcp(directory, &args, &card, 0, Some(budget(&key, &card)?))?;
    let full = serde_json::to_vec(&card).map_err(|e| e.to_string())?.len();
    let comparable = super::super::source_terms::size(&card, "")?;
    writeln!(
        std::io::stdout(),
        "phenotype card {id} {source}: CLI={} MCP={full} comparable={comparable}",
        full + 1
    )
    .map_err(|e| e.to_string())?;
    fs::write(
        outputs.join(format!("card-{}-{source}.json", id.replace(':', "_"))),
        serde_json::to_vec(&card).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn environment(
    row: &Value,
    body: String,
) -> Result<(super::super::launch::Environment, Stub), String> {
    let source = text(row, "source")?;
    let hpo = source == "hpo";
    let host = if hpo {
        "ontology.jax.org"
    } else {
        "api-v3.monarchinitiative.org"
    };
    let target = text(row, "target")?.to_string();
    let stub = Stub::for_source_status(
        host,
        "application/json",
        move |request| {
            if request.split_whitespace().nth(1) != Some(&target) {
                return Err(format!("uncaptured entity request: {request}"));
            }
            Ok(body.clone())
        },
        |_| 200,
    )?;
    let env = setup(&stub)?;
    Ok((env, stub))
}

fn measurements(key: &str) -> Result<[usize; 2], String> {
    let values: std::collections::BTreeMap<String, [usize; 2]> = serde_saphyr::from_str(
        &crate::runner::launch::fixture_text("phenotype-section-budgets.yaml")?,
    )
    .map_err(|e| e.to_string())?;
    values
        .get(key)
        .copied()
        .ok_or_else(|| format!("unmeasured phenotype case {key}"))
}
pub(super) fn ceiling(key: &str) -> Result<usize, String> {
    let [_, comparable] = measurements(key)?;
    Ok((comparable * 115).div_ceil(100))
}
fn budget(key: &str, value: &Value) -> Result<usize, String> {
    let [full, comparable] = measurements(key)?;
    let actual = serde_json::to_vec(value).map_err(|e| e.to_string())?.len() + 1;
    let compact = super::super::source_terms::size(value, "")?;
    if actual > (full * 115).div_ceil(100) || compact > (comparable * 115).div_ceil(100) {
        return Err(format!(
            "{key}: full or comparable response exceeded its measured ceiling"
        ));
    }
    ceiling(key)
}
