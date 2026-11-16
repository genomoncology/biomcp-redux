//! Project only protocol-owned Steps; upstream text and query values remain data.
use serde_json::Value;

pub(super) fn project(
    root: &crate::grammar::Root,
    value: &mut Value,
    tools: &super::grouping::Catalog,
) -> std::io::Result<Vec<Value>> {
    let Some(id) = value.get("schema").and_then(Value::as_str) else {
        return Ok(Vec::new());
    };
    let text =
        crate::schema::find(id).ok_or_else(|| std::io::Error::other("missing reply schema"))?;
    let schema: Value = serde_json::from_str(text).map_err(std::io::Error::other)?;
    let mut calls = Vec::new();
    for path in crate::schema::steps::paths(&schema, &schema, value)
        .into_iter()
        .rev()
    {
        let call = value.pointer(&path).and_then(|step| {
            let words = step.get("args")?.as_array()?;
            let args = without_globals(root, words)?;
            let mut call = super::summary::translate(root, &args, tools)?;
            if let Some(chosen) = chosen_fields(words) {
                let arguments = call.get_mut("arguments")?.as_object_mut()?;
                arguments.insert("fields".into(), chosen);
            }
            if let Some(kind) = step.get("kind") {
                call.as_object_mut()?.insert("kind".into(), kind.clone());
            }
            if let Some(options) = step.get("options").and_then(Value::as_object)
                && let Some(options) = options_for(&call, &args, options, tools)
            {
                call.as_object_mut()?.insert("options".into(), options);
            }
            Some(call)
        });
        if let Some(call) = call {
            if let Some(step) = value.pointer_mut(&path) {
                *step = call.clone();
            }
            calls.push(call);
        } else {
            crate::schema::steps::remove(value, &path);
        }
    }
    calls.reverse();
    Ok(calls)
}

/// The typed call for command words, with any carried `--fields=a,b` as its `fields` list.
pub(super) fn call(
    root: &crate::grammar::Root,
    words: &[Value],
    tools: &super::grouping::Catalog,
) -> Option<Value> {
    let args = without_globals(root, words)?;
    let mut call = super::summary::translate(root, &args, tools)?;
    if let Some(chosen) = chosen_fields(words) {
        let arguments = call.get_mut("arguments")?.as_object_mut()?;
        arguments.insert("fields".into(), chosen);
    }
    Some(call)
}

/// A step's command-line options under the argument names the called tool advertises; an
/// option the tool does not advertise is left out.
fn options_for(
    call: &Value,
    args: &[Value],
    options: &serde_json::Map<String, Value>,
    tools: &super::grouping::Catalog,
) -> Option<Value> {
    let tool = tools
        .tools
        .iter()
        .find(|tool| tool.get("name") == call.get("name"))?;
    let mut properties = tool.pointer("/inputSchema/properties")?;
    if let Some(action) = call.pointer("/arguments/action").and_then(Value::as_str) {
        properties = properties.get(action)?.get("properties")?;
    }
    let command = super::name(args.first()?.as_str()?, args.get(1)?.as_str()?);
    let mut typed = serde_json::Map::new();
    for (option, placeholder) in options {
        let field = option.trim_start_matches('-').replace('-', "_");
        let named = super::grouping::advertised(&command, &serde_json::json!({ field: null }));
        let Some(name) = named.as_object().and_then(|fields| fields.keys().next()) else {
            continue;
        };
        if properties.get(name).is_some() {
            typed.insert(name.clone(), placeholder.clone());
        }
    }
    (!typed.is_empty()).then_some(Value::Object(typed))
}

/// A carried `--fields=a,b` global becomes the typed call's `fields` list.
fn chosen_fields(words: &[Value]) -> Option<Value> {
    let chosen = words
        .iter()
        .filter_map(Value::as_str)
        .find_map(|word| word.strip_prefix("--fields="))?;
    Some(Value::Array(
        chosen
            .split(',')
            .map(|name| Value::String(name.into()))
            .collect(),
    ))
}

fn without_globals(root: &crate::grammar::Root, args: &[Value]) -> Option<Vec<Value>> {
    let mut result = Vec::new();
    let mut words = args.iter();
    while let Some(word) = words.next() {
        let text = word.as_str()?;
        if text == "--" {
            result.push(word.clone());
            result.extend(words.cloned());
            break;
        }
        let flag = text
            .strip_prefix("--")
            .unwrap_or_default()
            .split('=')
            .next()?;
        // Typed tools do not advertise the force-refresh global. Omitting it would
        // change a refresh into an ordinary cached call, so omit that suggestion.
        if flag == "fresh" {
            return None;
        }
        if let Some(global) = root
            .get_arguments()
            .find(|arg| arg.get_long() == Some(flag))
        {
            if global.get_action().takes_values() && !text.contains('=') {
                words.next()?;
            }
        } else {
            result.push(word.clone());
        }
    }
    Some(result)
}

pub(super) fn allowed(call: &Value, tools: &[Value]) -> bool {
    tools
        .iter()
        .find(|tool| tool.get("name") == call.get("name"))
        .is_some_and(|tool| choices(&tool["inputSchema"], &call["arguments"]))
}

// The canonical parser already checks types, ranges and required fields. The
// advertised schema can additionally remove fields and narrow enum choices.
fn choices(schema: &Value, value: &Value) -> bool {
    if schema
        .get("enum")
        .and_then(Value::as_array)
        .is_some_and(|values| !values.contains(value))
    {
        return false;
    }
    if let Some(branches) = schema
        .get("anyOf")
        .or_else(|| schema.get("oneOf"))
        .and_then(Value::as_array)
    {
        return branches.iter().any(|branch| choices(branch, value));
    }
    if schema.get("type") == Some(&Value::String("null".into())) {
        return value.is_null();
    }
    match value {
        Value::Object(fields) => fields.iter().all(|(key, value)| {
            schema
                .get("properties")
                .and_then(|fields| fields.get(key))
                .map_or_else(
                    // A schema that names its fields offers only those, whether or not it
                    // also forbids others: a suggested call never uses an unadvertised one.
                    || {
                        schema.get("properties").is_none()
                            && schema.get("additionalProperties") != Some(&Value::Bool(false))
                    },
                    |node| choices(node, value),
                )
        }),
        Value::Array(values) => schema
            .get("items")
            .is_none_or(|node| values.iter().all(|value| choices(node, value))),
        _ => true,
    }
}
