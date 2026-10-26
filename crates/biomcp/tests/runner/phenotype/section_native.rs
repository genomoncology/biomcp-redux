//! Native term replies stay outside Git and are admitted only by exact hash.
use super::{invoke, native, setup, Stub};
use serde_json::{Value, json};
use std::{fs, io::Write, path::Path};
pub(super) fn run(root: &Path) -> Result<(), String> {
    let plan: Vec<Value> = serde_json::from_slice(&fs::read(root.join("plan.json")).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let outputs = root.join("outputs");
    fs::create_dir(&outputs).map_err(|e| e.to_string())?;
    for (index, row) in plan.iter().enumerate() {
        prove(root, &outputs, index, row)?;
    }
    Ok(())
}
fn text<'a>(row: &'a Value, key: &str) -> Result<&'a str, String> {
    row.get(key).and_then(Value::as_str).ok_or_else(|| format!("missing {key}"))
}
fn prove(root: &Path, outputs: &Path, index: usize, row: &Value) -> Result<(), String> {
    let (raw, body) = native::archived_body(root, row)?;
    let source = text(row, "source")?;
    let hpo = source == "hpo";
    let host = if hpo { "ontology.jax.org" } else { "api-v3.monarchinitiative.org" };
    let target = text(row, "target")?.to_string();
    let stub = Stub::for_source(host, "application/json", move |request| {
        if request.split_whitespace().nth(1) != Some(&target) { return Err(format!("uncaptured entity request: {request}")); }
        Ok(body.clone())
    })?;
    let env = setup(&stub)?;
    let words = row.get("words").and_then(Value::as_array).ok_or("words")?.iter().map(|v| v.as_str().ok_or("word")).collect::<Result<Vec<_>,_>>()?;
    let directory = outputs.join(format!("record-{index:02}"));
    let mut record = words.clone();
    record.extend(["--record", directory.to_str().ok_or("record path")?]);
    let expected = expected(&raw, hpo, text(row, "section")?)?;
    let exit = if expected.as_ref().is_none_or(Vec::is_empty) { 1 } else { 0 };
    let value = invoke(&env, &record, exit)?;
    verify(&value, expected.as_deref(), row)?;
    native::restore_date(&directory, row.get("fetched").and_then(Value::as_u64).ok_or("fetched")?)?;
    let mut replay = words.clone(); replay.extend(["--replay",directory.to_str().ok_or("record path")?]);
    let result = invoke(&env,&replay,exit)?;
    verify(&result,expected.as_deref(),row)?;
    native::mcp(&directory,&words,&result,exit,None)?;
    let full = serde_json::to_vec(&result).map_err(|e|e.to_string())?.len();
    let comparable = super::super::source_terms::size(&result,"")?;
    writeln!(std::io::stdout(),"phenotype section {index:02}: CLI={} MCP={full} comparable={comparable}",full+1).map_err(|e|e.to_string())?;
    fs::write(outputs.join(format!("{index:02}.json")),serde_json::to_vec(&result).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    if stub.finish()?.len()!=1 { return Err("section fetched another request".into()); }
    Ok(())
}
fn expected(raw: &Value, hpo: bool, section: &str) -> Result<Option<Vec<Value>>, String> {
    let fields: Vec<(&str, Option<&str>)> = match section {
        "synonyms" if hpo => vec![("synonyms",None)],
        "synonyms" => vec![("synonym",None),("exact_synonym",Some("exact")),("broad_synonym",Some("broad")),("narrow_synonym",Some("narrow")),("related_synonym",Some("related"))],
        "xrefs" => vec![(if hpo {"xrefs"} else {"xref"},None)],
        "hierarchy" => vec![("/node_hierarchy/super_classes",Some("super_classes")),("/node_hierarchy/sub_classes",Some("sub_classes"))],
        _ => return Err("section".into()),
    };
    let mut available = false; let mut result = Vec::new();
    for (field, relation) in fields {
        let value = if field.starts_with('/') { raw.pointer(field) } else { raw.get(field) };
        if let Some(rows) = value.filter(|v| !v.is_null()).and_then(Value::as_array) {
            available = true;
            for row in rows {
                let item = match section {
                    "synonyms" => { let mut item=json!({"value":row}); if let Some(relation)=relation { item.as_object_mut().ok_or("item")?.insert("relationship".into(),json!(relation)); } item },
                    "xrefs" => json!({"id":row}),
                    _ => json!({"id":row.get("id").ok_or("link ID")?,"label":row.get("name").ok_or("link label")?,"relation":relation.ok_or("relation")?}),
                };
                result.push(item);
            }
        }
    }
    Ok(available.then_some(result))
}
fn verify(value: &Value, expected: Option<&[Value]>, row: &Value) -> Result<(), String> {
    let Some(expected)=expected else {
        if value.get("result").is_some() || value.get("page").is_some() { return Err("native null section became empty".into()); }
        return Ok(());
    };
    let offset=row.get("offset").and_then(Value::as_u64).unwrap_or(0) as usize;
    let limit=row.get("limit").and_then(Value::as_u64).unwrap_or(5) as usize;
    let items=value.pointer("/result/items").and_then(Value::as_array).ok_or("items")?;
    let selected=expected.iter().skip(offset).take(limit).collect::<Vec<_>>();
    if items.len()!=selected.len() || value.pointer("/page/total")!=Some(&json!(expected.len())) || value.pointer("/page/offset")!=Some(&json!(offset)) { return Err("native collection page changed".into()); }
    for (item,expected) in items.iter().zip(selected) {
        let mut item=item.clone(); item.as_object_mut().ok_or("item")?.remove("attribution");
        if &item!=expected { return Err(format!("native row changed: {item} != {expected}")); }
    }
    Ok(())
}
