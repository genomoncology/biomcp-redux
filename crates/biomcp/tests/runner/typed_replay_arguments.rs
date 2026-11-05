//! Translate recorded command arguments through their existing declarations.
#[path = "../../src/grammar/process_arguments.rs"]
mod process_arguments;
use serde_json::{Value, json};

pub(super) fn query_arguments(args: &[String]) -> (Vec<String>, Vec<String>, Result<(), String>) {
    let parser = process_arguments::command();
    let mut query = Vec::new();
    let mut globals = Vec::new();
    let mut words = args.iter().peekable();
    while let Some(word) = words.next() {
        if word == "--" {
            query.push(word.clone());
            query.extend(words.cloned());
            break;
        }
        let (flag, inline) = word
            .split_once('=')
            .map_or((word.as_str(), false), |(flag, _)| (flag, true));
        let Some(argument) = parser.get_arguments().find(|arg| {
            arg.get_long()
                .is_some_and(|name| flag == format!("--{name}"))
        }) else {
            query.push(word.clone());
            continue;
        };
        let following = words
            .peek()
            .map(|word| std::ffi::OsString::from(word.as_str()));
        if !inline && process_arguments::boolean_value(argument, following.as_ref()) {
            globals.push(format!("{word}={}", words.next().unwrap_or(word)));
        } else {
            globals.push(word.clone());
            if !inline
                && argument.get_action().takes_values()
                && let Some(value) = words.next()
            {
                globals.push(value.clone());
            }
        }
    }
    let validation = parser
        .subcommand_required(false)
        .arg_required_else_help(false)
        .try_get_matches_from(std::iter::once("biomcp").chain(globals.iter().map(String::as_str)))
        .map(|_| ())
        .map_err(|error| error.to_string());
    (query, globals, validation)
}

pub(super) fn json_globals(globals: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    let mut words = globals.iter();
    while let Some(word) = words.next() {
        if word == "--format" {
            result.push(word.clone());
            if let Some(value) = words.next() {
                result.push(if value == "md" {
                    "json".into()
                } else {
                    value.clone()
                });
            }
        } else if word == "--format=md" {
            result.push("--format=json".into());
        } else {
            result.push(word.clone());
        }
    }
    result
}

fn resolved<'a>(node: &'a Value, root: &'a Value) -> &'a Value {
    if let Some(reference) = node.get("$ref").and_then(Value::as_str) {
        return root
            .pointer(reference.trim_start_matches('#'))
            .unwrap_or(node);
    }
    if let Some(branch) = node
        .get("anyOf")
        .and_then(Value::as_array)
        .and_then(|v| v.iter().find(|n| n["type"] != "null"))
    {
        return resolved(branch, root);
    }
    node
}
pub(super) fn input(args: &[String], query: &Value) -> Option<Value> {
    let properties = query["properties"].as_object()?;
    let mut result = serde_json::Map::new();
    let mut positional = Vec::new();
    let mut words = args.iter().skip(2).peekable();
    while let Some(word) = words.next() {
        if word == "--" {
            positional.extend(words.cloned());
            break;
        }
        let Some(flag) = word.strip_prefix("--") else {
            positional.push(word.clone());
            continue;
        };
        let (flag, inline) = flag
            .split_once('=')
            .map_or((flag, None), |(a, b)| (a, Some(b)));
        let key = flag.replace('-', "_");
        if properties.values().any(|property| {
            property
                .get("x-cli-flags")
                .and_then(|flags| flags.get(flag))
                .is_some()
        }) {
            result.insert(flag.into(), json!(true));
            continue;
        }
        let property = properties.get(&key)?;
        let kind = type_name(resolved(property, query));
        let value = if kind == Some("boolean") {
            let word = inline
                .map(str::to_owned)
                .or_else(|| {
                    words
                        .peek()
                        .filter(|word| matches!(word.as_str(), "true" | "false"))?;
                    words.next().cloned()
                })
                .unwrap_or_else(|| "true".into());
            word.parse::<bool>()
                .map_or_else(|_| json!(word), |value| json!(value))
        } else {
            let word = inline
                .map(str::to_owned)
                .or_else(|| words.next().cloned())?;
            if kind == Some("integer") {
                word.parse::<u64>()
                    .map_or_else(|_| json!(word), |value| json!(value))
            } else if kind == Some("number") {
                word.parse::<f64>()
                    .ok()
                    .filter(|value| value.is_finite())
                    .map_or_else(|| json!(word), |value| json!(value))
            } else if kind == Some("array") {
                let mut values = if property["x-comma-separated"] == true {
                    word.split(',').map(str::to_owned).collect::<Vec<_>>()
                } else {
                    vec![word]
                };
                if property["x-comma-separated"] != true && property["x-trailing-section"] != true {
                    while words.peek().is_some_and(|v| !v.starts_with('-')) {
                        values.push(words.next()?.clone());
                    }
                }
                let entry = result.entry(key.clone()).or_insert(json!([]));
                entry
                    .as_array_mut()?
                    .extend(values.into_iter().map(Value::String));
                continue;
            } else {
                json!(word)
            }
        };
        result.insert(key, value);
    }
    positional_fields(properties, query, result, &positional)
}
fn positional_fields(
    properties: &serde_json::Map<String, Value>,
    query: &Value,
    mut result: serde_json::Map<String, Value>,
    positional: &[String],
) -> Option<Value> {
    let mut fields = properties
        .iter()
        .filter_map(|(key, p)| p["x-positional"].as_u64().map(|i| (i, key, p)))
        .collect::<Vec<_>>();
    fields.sort_by_key(|(index, _, _)| *index);
    let mut position = 0;
    for (_, key, property) in fields {
        let kind = type_name(resolved(property, query));
        if kind == Some("array") {
            result.insert(key.clone(), json!(positional.get(position..)?));
            position = positional.len();
        } else if let Some(value) = positional.get(position) {
            result.insert(key.clone(), json!(value));
            position += 1;
        }
    }
    (position == positional.len()).then_some(Value::Object(result))
}

fn type_name(node: &Value) -> Option<&str> {
    node["type"].as_str().or_else(|| {
        node["type"]
            .as_array()?
            .iter()
            .filter_map(Value::as_str)
            .find(|kind| *kind != "null")
    })
}
