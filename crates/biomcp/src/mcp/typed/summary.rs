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
        && let Some(call) = correction(root, value, args)
    {
        calls.push(call.to_string());
    }
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
// The parser names the rejected option; omit it and let the parser check the correction.
fn correction(
    root: &crate::grammar::Root,
    value: &Value,
    args: &[std::ffi::OsString],
) -> Option<Value> {
    let option = value.pointer("/error/option")?.as_str()?;
    let mut skip = false;
    let values = args
        .iter()
        .filter_map(|arg| {
            let arg = arg.to_str()?;
            if skip {
                skip = false;
                return None;
            }
            if option.starts_with("--") && arg == option {
                skip = true;
                return None;
            }
            if option.starts_with("--") && arg.starts_with(&format!("{option}=")) {
                return None;
            }
            Some(Value::from(arg))
        })
        .collect::<Vec<_>>();
    translate(root, &values, true)
}
fn collect(root: &crate::grammar::Root, value: &Value, calls: &mut Vec<String>) {
    if let Some(args) = value.get("args").and_then(Value::as_array)
        && let Some(call) = translate(root, args, true)
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
fn translate(root: &crate::grammar::Root, args: &[Value], checked: bool) -> Option<Value> {
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
    if checked {
        return checked_call(root, verb, entity, &schema, &arguments);
    }
    // Decode an invalid original getter so its name can be routed to a valid search.
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
    let original = translate(root, &values, false)?;
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
        && let Some(call) = search_call(
            root,
            &entity,
            &schema,
            serde_json::Map::from_iter([("gene".into(), json!(gene)), (key.into(), json!(change))]),
        )
    {
        return Some(call);
    }
    let key = fields
        .iter()
        .find(|(_, node)| node.get("x-positional").is_some())
        .map(|(key, _)| key.as_str())
        .or_else(|| {
            ["query", "text", "name", "condition", "gene"]
                .into_iter()
                .find(|key| fields.contains_key(*key))
        })?;
    let value = if non_null_type(fields.get(key)?, &schema) == Some("array") {
        json!([input])
    } else {
        json!(input)
    };
    search_call(
        root,
        &entity,
        &schema,
        serde_json::Map::from_iter([(key.into(), value)]),
    )
}

// The command parser owns syntax and cross-field semantics, including source languages.
fn search_call(
    root: &crate::grammar::Root,
    entity: &str,
    schema: &Value,
    mut arguments: serde_json::Map<String, Value>,
) -> Option<Value> {
    if let Some(call) = checked_call(
        root,
        "search",
        entity,
        schema,
        &Value::Object(arguments.clone()),
    ) {
        return Some(call);
    }
    let input = super::arguments::input(schema);
    let source = input.pointer("/properties/source")?;
    for choice in choices(source) {
        let value = if non_null_type(source, &input) == Some("array") {
            json!([choice])
        } else {
            choice.clone()
        };
        arguments.insert("source".into(), value);
        if let Some(call) = checked_call(
            root,
            "search",
            entity,
            schema,
            &Value::Object(arguments.clone()),
        ) {
            return Some(call);
        }
    }
    None
}
fn checked_call(
    root: &crate::grammar::Root,
    verb: &str,
    entity: &str,
    schema: &Value,
    arguments: &Value,
) -> Option<Value> {
    if !super::validation::valid(&super::arguments::input(schema), arguments) {
        return None;
    }
    let args = super::arguments::cli(root, verb, entity, schema, arguments)?;
    let parsed = crate::grammar::parse(root, &args).ok()?;
    matches!(parsed.action, Some(biomcp_query::Action::Run { .. }))
        .then(|| json!({"name":super::name(verb,entity),"arguments":arguments}))
}
fn choices(node: &Value) -> Vec<&Value> {
    if let Some(values) = node.get("enum").and_then(Value::as_array) {
        return values.iter().collect();
    }
    if let Some(items) = node.get("items") {
        return choices(items);
    }
    ["anyOf", "oneOf"]
        .into_iter()
        .flat_map(|key| {
            node.get(key)
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .flat_map(choices)
        })
        .collect()
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
