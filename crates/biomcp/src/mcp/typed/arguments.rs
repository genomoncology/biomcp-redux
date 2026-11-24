//! Compact public schemas and checked argument-to-CLI spelling.
use serde_json::{Value, json};

pub(super) fn resolve<'a>(node: &'a Value, root: &'a Value) -> &'a Value {
    node.get("$ref")
        .and_then(Value::as_str)
        .and_then(|reference| root.pointer(reference.trim_start_matches('#')))
        .unwrap_or(node)
}
fn compact(node: &Value, root: &Value, optional: bool, enum_limit: usize) -> Value {
    let node = resolve(node, root);
    let Some(fields) = node.as_object() else {
        return node.clone();
    };
    let mut result = serde_json::Map::new();
    for (key, value) in fields {
        match key.as_str() {
            "type" if optional && value.is_array() => {
                let kinds: Vec<_> = value
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter(|kind| *kind != "null")
                    .cloned()
                    .collect();
                result.insert(
                    key.clone(),
                    if kinds.len() == 1 {
                        kinds.first().cloned().unwrap_or(Value::Null)
                    } else {
                        json!(kinds)
                    },
                );
            }
            "enum" if value.as_array().is_some_and(|v| v.len() > enum_limit) => {
                result.insert("type".into(), json!("string"));
            }
            "type"
            | "enum"
            | "const"
            | "minimum"
            | "maximum"
            | "minItems"
            | "maxItems"
            | "required"
            | "additionalProperties" => {
                result.insert(key.clone(), value.clone());
            }
            "items" => {
                result.insert(key.clone(), compact(value, root, false, enum_limit));
            }
            "anyOf" | "oneOf" => {
                let branches: Vec<_> = value
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter(|v| !optional || resolve(v, root).get("type") != Some(&json!("null")))
                    .map(|v| compact(v, root, optional, enum_limit))
                    .collect();
                if branches.len() == 1 {
                    if let Some(fields) = branches.first().and_then(Value::as_object) {
                        result.extend(fields.clone());
                    }
                } else {
                    result.insert(key.clone(), json!(branches));
                }
            }
            _ => {}
        }
    }
    Value::Object(result)
}
pub(super) fn input(schema: &Value) -> Value {
    input_with_limit(schema, crate::config::DEFAULT_TYPED_ENUM_LIMIT)
}
pub(super) fn input_with_limit(schema: &Value, enum_limit: usize) -> Value {
    let properties = schema
        .get("properties")
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
        .flat_map(|(key, node)| {
            let description = node
                .get("x-section-rule")
                .or_else(|| node.get("description"))
                .and_then(Value::as_str)
                .map(|text| {
                    let mut lines = text.lines();
                    let first = lines.next().unwrap_or_default();
                    // Generated schemas prepend a compact help pointer to the original prose.
                    let text = if first.contains("; help ") {
                        lines
                            .next()
                            .unwrap_or(first)
                            .split("; help ")
                            .next()
                            .unwrap_or(first)
                            .to_owned()
                    } else {
                        first.to_owned()
                    };
                    text.split(". ")
                        .next()
                        .unwrap_or_default()
                        .trim_end_matches('.')
                        .to_owned()
                        + "."
                });
            if let Some(flags) = node.get("x-cli-flags").and_then(Value::as_object) {
                flags
                    .keys()
                    .map(|flag| {
                        let mut property = json!({"type":"boolean"});
                        if let Some(text) = &description
                            && let Some(fields) = property.as_object_mut()
                        {
                            fields.insert("description".into(), json!(text));
                        }
                        (flag.clone(), property)
                    })
                    .collect()
            } else {
                let optional = !schema
                    .get("required")
                    .and_then(Value::as_array)
                    .is_some_and(|v| v.contains(&json!(key)));
                let mut property = compact(node, schema, optional, enum_limit);
                // Comma-separated text also takes a list, which the call joins with commas.
                if node.get("x-comma-separated") == Some(&json!(true))
                    && property.get("type") == Some(&json!("string"))
                    && let Some(fields) = property.as_object_mut()
                {
                    fields.insert("type".into(), json!(["string", "array"]));
                    fields.insert("items".into(), json!({"type": "string"}));
                }
                if let Some(text) = description
                    && let Some(fields) = property.as_object_mut()
                {
                    fields.insert("description".into(), json!(text));
                }
                vec![(key.clone(), property)]
            }
        })
        .collect::<serde_json::Map<_, _>>();
    let mut result = json!({"$schema":super::super::protocol::SCHEMA_DIALECT,
        "type":"object", "properties":properties,"additionalProperties":false});
    if let Some(required) = schema.get("required")
        && let Some(fields) = result.as_object_mut()
    {
        fields.insert("required".into(), required.clone());
    }
    result
}

pub(super) use crate::grammar::command_arguments as cli;

/// MCP clients may fill an unused argument with either placeholder.
pub(super) fn unset(value: &Value) -> bool {
    value.is_null() || value.as_str() == Some("")
}

pub(super) fn advertise_action(
    schema: &mut Value,
    (verb, entity, _): (&str, &str, &str),
    enum_limit: usize,
) {
    let required = schema.get("required").cloned().unwrap_or_default();
    if let Some(properties) = schema.get_mut("properties").and_then(Value::as_object_mut) {
        for (name, node) in properties {
            // These long vocabularies stay in command help and validation. Repeating them
            // here would exceed the trial search and variant get action budgets.
            if matches!(
                (verb, entity, name.as_str()),
                ("search", "trial", "status") | ("get", "variant", "evidence_significance")
            ) && let Some(fields) = node.as_object_mut()
            {
                fields.remove("enum");
            }
            if !required
                .as_array()
                .is_some_and(|keys| keys.contains(&json!(name)))
            {
                advertise_unset(node, enum_limit);
            }
        }
    }
    advertise_unset(schema, enum_limit);
}

/// Give an optional property an unset value. Free text already admits the empty
/// string, which saves repeating a null type on every text property.
pub(super) fn advertise_unset(node: &mut Value, enum_limit: usize) {
    let Some(fields) = node.as_object_mut() else {
        return;
    };
    if let Some(Value::Array(choices)) = fields.get_mut("enum") {
        if choices.len() < enum_limit {
            choices.push(Value::Null);
        } else {
            fields.remove("enum");
        }
    }
    let constrained = fields.contains_key("enum");
    if let Some(kind) = fields.get_mut("type") {
        match kind {
            Value::String(text) if text == "string" && !constrained => {}
            Value::Array(kinds) => {
                if !kinds.contains(&json!("string")) && !kinds.contains(&json!("null")) {
                    kinds.push(json!("null"));
                }
            }
            _ => *kind = json!([kind, "null"]),
        }
    } else if let Some(Value::Array(branches)) = fields.get_mut("anyOf") {
        branches.push(json!({"type":"null"}));
    }
}
