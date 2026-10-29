//! Independent GTR values retain exact test and relation identities.
use super::{super::launch::Environment, mechanics::{call, seed}};
use serde_json::{Value, json};
use std::fs;

pub(super) fn card_fields() -> Result<(), String> {
    for (field, pointer) in [
        ("lab_id", "/result/header/laboratory"),
        ("last_touch_date", "/result/last_touch_date"),
        ("test_create_date", "/result/test_create_date"),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = seed(&env)?;
        call(&env, &replay, &["sync", "gtr"], 0)?;
        let cache = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
            .map_err(|e| e.to_string())?;
        let changed = cache.execute(
            "UPDATE bundle_row SET row=json_set(row,?1,' ') WHERE bundle='gtr' AND json_extract(row,'$.value.test_accession_ver')='GTR000006692.3'",
            [format!("$.value.{field}")],
        ).map_err(|e| e.to_string())?;
        if changed != 1 { return Err(format!("edited {changed} native version rows")); }
        let value = call(&env, &replay, &["get", "diagnostic", "GTR000006692.3", "--source", "gtr"], 0)?;
        if value.pointer("/result/header/id") != Some(&json!("GTR000006692.3"))
            || value.pointer(pointer).is_some()
            || value.pointer("/problems/0/field") != Some(&json!(field))
            || value.get("problems").and_then(Value::as_array).map(Vec::len) != Some(1)
        { return Err(format!("invalid {field} discarded source version: {value}")); }
    }
    Ok(())
}

pub(super) fn relations() -> Result<(), String> {
    for (object, column, field, section, pointer, count) in [
        ("gene", 3, "GTR_identifier", "genes", "/result/items/0/native_medgen", 2),
        ("gene", 7, "gene_symbol", "genes", "/result/items/0/gene/label", 2),
        ("condition", 5, "object_name", "conditions", "/result/items/0/condition/label", 1),
        ("condition", 3, "GTR_identifier", "conditions", "/result/items/0", 0),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = seed(&env)?;
        mutate(&replay, object, column)?;
        call(&env, &replay, &["sync", "gtr"], 0)?;
        let value = call(&env, &replay, &["get", "diagnostic", "GTR000006692.3", "--source", "gtr", section], 0)?;
        if value.pointer(pointer).is_some()
            || value.pointer("/page/total") != Some(&json!(count))
            || value.pointer("/problems/0/field") != Some(&json!(field))
            || value.get("problems").and_then(Value::as_array).map(Vec::len) != Some(1)
        { return Err(format!("invalid {field} rejected sibling relations: {value}")); }
    }
    Ok(())
}

fn mutate(replay: &std::path::Path, object: &str, column: usize) -> Result<(), String> {
    for entry in fs::read_dir(replay.join("gtr")).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().is_none_or(|v| v != "json") { continue; }
        let mut value: Value = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
        if !value.get("url").and_then(Value::as_str).is_some_and(|u| u.ends_with("test_condition_gene.txt")) { continue; }
        let body = value.pointer_mut("/exchanges/0/body/value").ok_or("relation body")?;
        let mut changed = false;
        let lines: Vec<_> = body.as_str().ok_or("native text")?.lines().map(|line| {
            let mut cells: Vec<_> = line.split('\t').map(str::to_owned).collect();
            if !changed && cells.first().is_some_and(|id| id == "GTR000006692.3")
                && cells.get(2).is_some_and(|kind| kind == object)
                && let Some(cell) = cells.get_mut(column)
            { *cell = " ".into(); changed = true; }
            cells.join("\t")
        }).collect();
        if !changed { return Err("native relation target not found".into()); }
        *body = json!(format!("{}\n", lines.join("\n")));
        fs::write(path, value.to_string()).map_err(|e| e.to_string())?;
    }
    Ok(())
}
