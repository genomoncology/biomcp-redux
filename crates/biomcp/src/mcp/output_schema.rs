//! Compact tool reply shapes selected from the canonical command schema catalog.
use serde_json::{Value, json};
use std::{collections::BTreeSet, io};

pub(super) fn for_tool(name: &str) -> io::Result<Value> {
    let mut ids = crate::help::dispatch(&[])?
        .commands
        .into_iter()
        .filter(|command| match name {
            "search" => command.command.first().is_some_and(|word| word == "search"),
            "fetch" => command.command.first().is_some_and(|word| word == "get"),
            _ => true,
        })
        .flat_map(|command| command.reply_schemas)
        .collect::<BTreeSet<_>>();
    if name == "fetch" {
        ids.extend(crate::schema::local_replies(&["help".into()]));
    }
    let health = canonical("biomcp.health/2")?;
    let status = health
        .pointer("/properties/status")
        .and_then(|node| resolve(node, &health))
        .ok_or_else(|| io::Error::other("missing status schema"))?;
    let envelope = json!({"properties":{
        "schema":{"type":"string","enum":ids},
        "status":status,
        "result":{"type":"object"}},"required":["schema","status"]});
    let mut branches = vec![envelope, shape(&canonical("biomcp.error/1")?)?];
    if name == "biomcp" {
        branches.push(json!({"properties":{
            "$id":{"type":"string","pattern":"^(biomcp|biodata)\\.[a-z0-9_-]+/[1-9][0-9]*$"},
            "$schema":{"const":"https://json-schema.org/draft/2020-12/schema"}},
            "required":["$id","$schema"]}));
    } else {
        branches.push(shape(&canonical(&format!("biomcp.research-{name}/1"))?)?);
    }
    Ok(
        json!({"type":"object","description":"Compact reply shape. Use biomcp schema <id> for the complete contract.","anyOf":branches}),
    )
}
fn canonical(id: &str) -> io::Result<Value> {
    serde_json::from_str(
        crate::schema::find(id).ok_or_else(|| io::Error::other("missing reply schema"))?,
    )
    .map_err(io::Error::other)
}
fn resolve<'a>(node: &'a Value, root: &'a Value) -> Option<&'a Value> {
    if let Some(reference) = node.get("$ref").and_then(Value::as_str) {
        root.pointer(reference.strip_prefix('#')?)
    } else {
        Some(node)
    }
}
fn shape(root: &Value) -> io::Result<Value> {
    let properties = root
        .get("properties")
        .and_then(Value::as_object)
        .ok_or_else(|| io::Error::other("missing reply properties"))?;
    let properties = properties
        .iter()
        .map(|(key, node)| {
            let node =
                resolve(node, root).ok_or_else(|| io::Error::other("unresolved reply property"))?;
            let value = if let Some(kind) = node.get("type") {
                json!({"type":kind})
            } else if let Some(choices) = node.get("anyOf").and_then(Value::as_array) {
                let kinds = choices
                    .iter()
                    .filter_map(|choice| resolve(choice, root)?.get("type"))
                    .collect::<Vec<_>>();
                json!({"type":kinds})
            } else {
                return Err(io::Error::other("missing reply property type"));
            };
            Ok((key.clone(), value))
        })
        .collect::<io::Result<serde_json::Map<_, _>>>()?;
    Ok(json!({"properties":properties,"required":root.get("required")}))
}
