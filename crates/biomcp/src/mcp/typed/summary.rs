//! Keep the CLI reply unchanged while spelling executable next calls in the text summary.
use serde_json::{Value, json};

pub(super) fn render(
    root: &crate::grammar::Root,
    value: &Value,
    args: &[std::ffi::OsString],
) -> String {
    let status = value
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("complete");
    let mut calls = Vec::new();
    collect(root, value, &mut calls);
    if calls.is_empty()
        && status == "usage"
        && value
            .pointer("/error/option")
            .and_then(Value::as_str)
            .is_none_or(|option| !option.starts_with('-'))
        && let Some(call) = replacement(root, args)
    {
        calls.push(call.to_string());
    }
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
    for (key, property) in input.get("properties")?.as_object()? {
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
            let kind = non_null_type(property, &input);
            if kind == Some("array") {
                json!(values)
            } else {
                let word = values.first()?;
                if kind == Some("integer") {
                    json!(word.parse::<u64>().ok()?)
                } else {
                    json!(word)
                }
            }
        };
        arguments.insert(key.clone(), value);
    }
    let arguments = Value::Object(arguments);
    super::validation::valid(&input, &arguments)
        .then(|| json!({"name":super::name(verb, entity),"arguments":arguments}))
}

// A failed exact-ID getter directs a plain name to its registry's search command.
fn replacement(root: &crate::grammar::Root, args: &[std::ffi::OsString]) -> Option<Value> {
    let values = args
        .iter()
        .filter_map(|word| word.to_str())
        .map(Value::from)
        .collect::<Vec<_>>();
    let original = translate(root, &values)?;
    let called = original.get("name")?.as_str()?.strip_prefix("get_")?;
    let entity = called.replace('_', "-");
    let (_, _, id) = crate::grammar::command_paths()
        .into_iter()
        .find(|(verb, child, _)| *verb == "search" && *child == entity)?;
    let schema = super::schema(id).ok()?;
    let fields = schema.get("properties")?.as_object()?;
    let original = original.get("arguments")?.as_object()?;
    let input = original.get("input").or_else(|| original.get("id"))?;
    let input = input
        .as_str()
        .or_else(|| input.as_array()?.first()?.as_str())?;
    let mut arguments = serde_json::Map::new();
    let words = input.split_whitespace().collect::<Vec<_>>();
    let shorthand = words.get(1).map(|change| {
        if change.starts_with("c.") {
            "coding"
        } else {
            "protein"
        }
    });
    if let [gene, change] = words.as_slice()
        && fields.contains_key("gene")
        && let Some(key) = shorthand.filter(|key| fields.contains_key(*key))
    {
        arguments.insert("gene".into(), json!(gene));
        arguments.insert(key.into(), json!(change));
    } else {
        let key = fields
            .iter()
            .find(|(_, node)| node.get("x-positional").is_some())
            .map(|(key, _)| key.as_str())
            .or_else(|| {
                ["query", "text", "name", "condition", "gene"]
                    .into_iter()
                    .find(|key| fields.contains_key(*key))
            })?;
        let node = super::arguments::resolve(fields.get(key)?, &schema);
        arguments.insert(
            key.into(),
            if node["type"] == "array" {
                json!([input])
            } else {
                json!(input)
            },
        );
    }
    let value = Value::Object(arguments);
    super::validation::valid(&super::arguments::input(&schema), &value)
        .then(|| json!({"name":super::name("search", &entity),"arguments":value}))
}

fn non_null_type<'a>(node: &'a Value, root: &'a Value) -> Option<&'a str> {
    let node = super::arguments::resolve(node, root);
    node["type"]
        .as_str()
        .or_else(|| {
            node["type"]
                .as_array()?
                .iter()
                .filter_map(Value::as_str)
                .find(|kind| *kind != "null")
        })
        .or_else(|| {
            node["anyOf"]
                .as_array()?
                .iter()
                .filter_map(|node| non_null_type(node, root))
                .find(|kind| *kind != "null")
        })
}
