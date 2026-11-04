//! Compact public schemas and checked argument-to-CLI spelling.
use serde_json::{Value, json};
use std::ffi::OsString;

pub(super) fn resolve<'a>(node: &'a Value, root: &'a Value) -> &'a Value {
    node.get("$ref").and_then(Value::as_str)
        .and_then(|reference| root.pointer(reference.trim_start_matches('#'))).unwrap_or(node)
}
fn compact(node: &Value, root: &Value) -> Value {
    let node = resolve(node, root);
    let Some(fields) = node.as_object() else { return node.clone(); };
    let mut result = serde_json::Map::new();
    for (key, value) in fields {
        match key.as_str() {
            "type" | "enum" | "const" | "minimum" | "maximum" | "minItems" | "maxItems" |
            "required" | "additionalProperties" => { result.insert(key.clone(), value.clone()); }
            "description" => { result.insert(key.clone(), Value::String(value.as_str()
                .unwrap_or_default().lines().next().unwrap_or_default().chars().take(80).collect())); }
            "items" => { result.insert(key.clone(), compact(value, root)); }
            "anyOf" | "oneOf" => { result.insert(key.clone(), Value::Array(value.as_array()
                .into_iter().flatten().map(|v| compact(v, root)).collect())); }
            _ => {}
        }
    }
    Value::Object(result)
}
pub(super) fn input(schema: &Value) -> Value {
    let properties = schema["properties"].as_object().into_iter().flatten()
        .flat_map(|(key, node)| {
            if let Some(flags) = node.get("x-cli-flags").and_then(Value::as_object) {
                flags.keys().map(|flag| (flag.clone(), json!({"type":"boolean"}))).collect()
            } else { vec![(key.clone(), compact(node, schema))] }
        }).collect::<serde_json::Map<_, _>>();
    let mut result = json!({"$schema":super::super::protocol::SCHEMA_DIALECT,
        "type":"object", "properties":properties,"additionalProperties":false});
    if let Some(required) = schema.get("required") { result["required"] = required.clone(); }
    result
}
fn words(value: &Value) -> Option<Vec<String>> {
    match value {
        Value::String(value) => Some(vec![value.clone()]),
        Value::Number(value) => Some(vec![value.to_string()]),
        Value::Array(values) => values.iter().map(words).collect::<Option<Vec<_>>>()
            .map(|values| values.into_iter().flatten().collect()),
        _ => None,
    }
}
pub(super) fn cli(root: &crate::grammar::Root, verb: &str, entity: &str, schema: &Value,
    values: &Value) -> Option<Vec<OsString>> {
    let command = root.find_subcommand(verb)?.find_subcommand(entity)?;
    let mut args = vec![verb.into(), entity.into()];
    // Parser indices own positional order; flags precede them to keep literal values safe.
    let mut positional = Vec::new();
    for (key, value) in values.as_object()? {
        if value.is_null() { continue; }
        let argument = command.get_arguments().find(|arg| arg.get_id().as_str() == key)?;
        if let Some(index) = argument.get_index() {
            positional.push((index, words(value)?));
            continue;
        }
        let flag = argument.get_long()?;
        if let Some(boolean) = value.as_bool() {
            if argument.is_require_equals_set() { args.push(format!("--{flag}={boolean}").into()); }
            else if boolean { args.push(format!("--{flag}").into()); }
            continue;
        }
        let values = words(value)?;
        if schema["properties"][key].get("x-comma-separated") == Some(&Value::Bool(true)) {
            args.push(format!("--{flag}={}", values.join(",")).into());
        } else {
            for value in values { args.push(format!("--{flag}={value}").into()); }
        }
    }
    positional.sort_by_key(|(index, _)| *index);
    args.push("--".into());
    args.extend(positional.into_iter().flat_map(|(_, values)| values).map(OsString::from));
    Some(args)
}
