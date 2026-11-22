//! Describe the first two levels of a typed result using its canonical schema.
use super::output_schema::{canonical, shape};
use serde_json::{Map, Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    io,
};

pub(super) fn for_commands(paths: &[(&str, &str, &str)]) -> io::Result<Value> {
    for_replies(command_replies(paths.iter().copied()), BTreeSet::new())
}

fn command_replies<'a>(
    paths: impl Iterator<Item = (&'a str, &'a str, &'a str)>,
) -> BTreeSet<String> {
    paths
        .flat_map(|(verb, entity, id)| crate::schema::replies(id, verb, entity))
        .collect()
}

pub(super) fn for_saved() -> io::Result<Value> {
    // save_search forwards a failed search as-is. Its search tool already describes those rows.
    let forwarded = command_replies(
        crate::grammar::command_paths()
            .into_iter()
            .filter(|(verb, ..)| *verb == "search"),
    );
    let saved = <biomcp_types::Saved as biomcp_types::ResponseBody>::SCHEMA;
    for_replies(BTreeSet::from([saved.into()]), forwarded)
}

pub(super) fn for_help() -> io::Result<Value> {
    for_replies(
        [vec!["help".into()], vec!["help".into(), "key".into()]]
            .iter()
            .flat_map(|path| crate::schema::local_replies(path))
            .collect(),
        BTreeSet::new(),
    )
}

fn for_replies(ids: BTreeSet<String>, forwarded: BTreeSet<String>) -> io::Result<Value> {
    let variants = ids
        .iter()
        .map(|id| result_variant(id))
        .collect::<io::Result<Vec<_>>>()?;
    let failure = json!({"properties":{
        "schema":{"const":"biomcp.error/4"},"status":{"type":"string"},
        "error":{"type":"object"},"next":{"type":"array"},"local_error":{"type":["string","null"]}},
        "required":["schema","status","error"]});
    let mut branches = vec![envelope(variants)?, failure];
    if !forwarded.is_empty() {
        branches.push(envelope(
            forwarded
                .iter()
                .map(|id| json!({"properties":{"schema":{"const":id}}}))
                .collect(),
        )?);
    }
    Ok(json!({"$schema":super::protocol::SCHEMA_DIALECT,"type":"object","anyOf":branches}))
}

fn envelope(variants: Vec<Value>) -> io::Result<Value> {
    let health = canonical(<biomcp_types::Health as biomcp_types::ResponseBody>::SCHEMA)?;
    let mut reply = shape(&health)?;
    let record = reply
        .as_object_mut()
        .ok_or_else(|| io::Error::other("missing reply schema object"))?;
    let fields = record
        .get_mut("properties")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| io::Error::other("missing reply properties"))?;
    fields.shift_remove("schema");
    fields.shift_remove("result");
    fields.insert("status".into(), json!({"type":"string"}));
    fields.insert("query".into(), json!({"type":["object","array","null"]}));
    record.insert("anyOf".into(), json!(variants));
    Ok(reply)
}

fn result_variant(id: &str) -> io::Result<Value> {
    let root = canonical(id)?;
    let result = root
        .pointer("/properties/result")
        .ok_or_else(|| io::Error::other("missing result schema"))?;
    let result = named(&root, &[result], true);
    Ok(json!({"properties":{"schema":{"const":id},"result":result}}))
}

// Alternatives contribute possible fields. A section or a projected row can omit fields.
fn named(root: &Value, nodes: &[&Value], rows: bool) -> Value {
    let mut schema = kinds(root, nodes);
    let mut fields = BTreeMap::<&str, Vec<&Value>>::new();
    for node in nodes.iter().flat_map(|node| variants(root, node)) {
        if let Some(properties) = node.get("properties").and_then(Value::as_object) {
            for (name, field) in properties {
                fields.entry(name).or_default().push(field);
            }
        }
    }
    if fields.is_empty() {
        return schema;
    }
    let row = rows.then(|| primary_row(root, &fields)).flatten();
    let mut fields = fields
        .into_iter()
        .map(|(name, nodes)| (name.into(), kinds(root, &nodes)))
        .collect::<Map<_, _>>();
    if let Some((name, row)) = row
        && let Some(field) = fields.get_mut(name).and_then(Value::as_object_mut)
    {
        field.insert("items".into(), row);
    }
    if let Some(shape) = schema.as_object_mut() {
        shape.insert("properties".into(), Value::Object(fields));
    }
    schema
}

fn primary_row<'a>(
    root: &Value,
    fields: &BTreeMap<&'a str, Vec<&Value>>,
) -> Option<(&'a str, Value)> {
    ["items", "rows", "results"]
        .into_iter()
        .filter_map(|name| fields.get_key_value(name))
        .chain(fields)
        .find_map(|(&name, nodes)| {
            let items = nodes
                .iter()
                .flat_map(|node| variants(root, node))
                .filter_map(|node| node.get("items"))
                .collect::<Vec<_>>();
            let row = named(root, &items, false);
            row.get("properties").is_some().then_some((name, row))
        })
}

fn variants<'a>(root: &'a Value, node: &'a Value) -> Vec<&'a Value> {
    if let Some(pointer) = node
        .get("$ref")
        .and_then(Value::as_str)
        .and_then(|value| value.strip_prefix('#'))
        && let Some(target) = root.pointer(pointer)
    {
        return variants(root, target);
    }
    let mut found = vec![node];
    for key in ["anyOf", "oneOf", "allOf"] {
        if let Some(choices) = node.get(key).and_then(Value::as_array) {
            found.extend(choices.iter().flat_map(|choice| variants(root, choice)));
        }
    }
    found
}

fn kinds(root: &Value, nodes: &[&Value]) -> Value {
    let mut types = BTreeSet::new();
    for node in nodes.iter().flat_map(|node| variants(root, node)) {
        match node.get("type") {
            Some(Value::String(kind)) => {
                types.insert(kind.as_str());
            }
            Some(Value::Array(kinds)) => types.extend(kinds.iter().filter_map(Value::as_str)),
            _ => {
                if let Some(literal) = node.get("const") {
                    types.insert(match literal {
                        Value::Null => "null",
                        Value::Bool(_) => "boolean",
                        Value::Number(_) => "number",
                        Value::String(_) => "string",
                        Value::Array(_) => "array",
                        Value::Object(_) => "object",
                    });
                }
            }
        }
    }
    if types.contains("number") {
        types.remove("integer");
    }
    match types.len() {
        0 => json!({}),
        1 => json!({"type":types.first()}),
        _ => json!({"type":types}),
    }
}
