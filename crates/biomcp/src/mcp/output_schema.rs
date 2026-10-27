//! Self-contained output unions selected from the command reply catalog.
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    io,
};

pub(super) fn for_tool(name: &str) -> io::Result<Value> {
    let mut document: Value =
        serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/mcp-schemas.json")))
            .map_err(io::Error::other)?;
    let mut ids = crate::help::dispatch(&[])?
        .commands
        .into_iter()
        .flat_map(|command| command.reply_schemas)
        .collect::<BTreeSet<_>>();
    ids.extend(["biomcp.error/1".into(), "schema-document".into()]);
    if name != "biomcp" {
        ids.insert(format!("biomcp.research-{name}/1"));
    }
    let roots = document
        .get_mut("roots")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| io::Error::other("missing schema roots"))?;
    let branches = ids
        .iter()
        .map(|id| {
            roots
                .get(id)
                .cloned()
                .ok_or_else(|| io::Error::other(format!("missing reply schema {id}")))
        })
        .collect::<io::Result<Vec<_>>>()?;
    let definitions = document.get("$defs").and_then(Value::as_object);
    let mut selected = BTreeMap::new();
    let mut pending = BTreeSet::new();
    references(&json!(branches), &mut pending);
    while let Some(name) = pending.pop_first() {
        if selected.contains_key(&name) {
            continue;
        }
        if let Some(value) = definitions.and_then(|defs| defs.get(&name)) {
            references(value, &mut pending);
            selected.insert(name, value.clone());
        }
    }
    Ok(
        json!({"$schema":"https://json-schema.org/draft/2020-12/schema", "type":"object", "anyOf":branches, "$defs":selected}),
    )
}
fn references(value: &Value, names: &mut BTreeSet<String>) {
    match value {
        Value::Object(object) => {
            if let Some(reference) = object
                .get("$ref")
                .and_then(Value::as_str)
                .and_then(|r| r.strip_prefix("#/$defs/"))
            {
                names.insert(reference.replace("~1", "/").replace("~0", "~"));
            }
            for value in object.values() {
                references(value, names);
            }
        }
        Value::Array(values) => {
            for value in values {
                references(value, names);
            }
        }
        _ => {}
    }
}
