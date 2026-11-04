//! Keep the CLI reply unchanged while spelling executable next calls in the text summary.
use serde_json::{Value, json};

pub(super) fn render(root: &crate::grammar::Root, value: &Value) -> String {
    let status = value
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("complete");
    let mut calls = Vec::new();
    collect(root, value, &mut calls);
    if calls.is_empty() {
        format!("{status}: JSON reply is in structuredContent.")
    } else {
        format!("{status}: next {}", calls.join("; "))
    }
}
fn collect(root: &crate::grammar::Root, value: &Value, calls: &mut Vec<String>) {
    if let Some(args) = value.get("args").and_then(Value::as_array)
        && let Some(call) = translate(root, args)
    {
        let text = call.to_string();
        if !calls.contains(&text) && calls.len() < 3 {
            calls.push(text);
        }
    }
    match value {
        Value::Object(fields) => {
            for (key, value) in fields {
                // Only protocol-owned follow-ups are interpreted; source text remains data.
                if matches!(key.as_str(), "next" | "error" | "problems") {
                    collect(root, value, calls);
                }
            }
        }
        Value::Array(values) => {
            for value in values {
                collect(root, value, calls);
            }
        }
        _ => {}
    }
}
fn translate(root: &crate::grammar::Root, args: &[Value]) -> Option<Value> {
    let args = args.iter().map(Value::as_str).collect::<Option<Vec<_>>>()?;
    let mut argv = vec!["biomcp"];
    argv.extend(args);
    let matches = clap::Command::clone(root).try_get_matches_from(argv).ok()?;
    let (verb, matches) = matches.subcommand()?;
    let (entity, matches) = matches.subcommand()?;
    let (_, _, id) = crate::grammar::command_paths()
        .into_iter()
        .find(|(v, e, _)| *v == verb && *e == entity)?;
    let schema = super::schema(id).ok()?;
    let input = super::arguments::input(&schema);
    let command = root.find_subcommand(verb)?.find_subcommand(entity)?;
    let mut arguments = serde_json::Map::new();
    for (key, property) in input["properties"].as_object()? {
        if matches.value_source(key) != Some(clap::parser::ValueSource::CommandLine) {
            continue;
        }
        let arg = command
            .get_arguments()
            .find(|arg| arg.get_id().as_str() == key)?;
        let value = if matches!(arg.get_action(), clap::ArgAction::SetTrue) {
            Value::Bool(matches.get_flag(key))
        } else {
            let values = matches
                .get_raw(key)?
                .map(|v| v.to_str())
                .collect::<Option<Vec<_>>>()?;
            let kind = super::arguments::resolve(property, &input);
            if kind["type"] == "array" {
                json!(values)
            } else {
                let word = values.first()?;
                if kind["type"] == "integer" {
                    json!(word.parse::<u64>().ok()?)
                } else {
                    json!(word)
                }
            }
        };
        arguments.insert(key.clone(), value);
    }
    Some(json!({"name":super::name(verb, entity),"arguments":arguments}))
}
