//! Reuse existing recorded caller cases without maintaining a second entity inventory.
use super::super::super::{
    case_data,
    launch::Environment,
    mcp::{exchange_with_globals, request},
    response_cases,
};
use serde_json::{Value, json};

pub(super) fn check(tool: &Value) -> Result<(), String> {
    let name = tool["name"].as_str().ok_or("tool name")?;
    let (verb, entity) = name.split_once('_').ok_or("tool path")?;
    let entity = entity.replace('_', "-");
    let mut paths = Vec::new();
    case_data::collect(
        &super::super::super::launch::fixtures().join("cases"),
        &mut paths,
    )
    .map_err(|e| e.to_string())?;
    paths.sort();
    for path in paths {
        let case = case_data::read_case(&path)?;
        if case.replay.is_none() {
            continue;
        }
        for step in case
            .steps
            .iter()
            .filter(|step| step.exit == 0 && !step.offline)
        {
            let Some(args) = &step.run else {
                continue;
            };
            if args.first().map(String::as_str) != Some(verb) || args.get(1) != Some(&entity) {
                continue;
            }
            let env = Environment::new().map_err(|e| e.to_string())?;
            let mut config = case.config.clone().unwrap_or_else(|| json!({}));
            env.config(&config).map_err(|e| e.to_string())?;
            let replay = response_cases::replay(&env, case.replay.as_deref(), case.cache_acquired)?;
            let globals = env.globals(replay.as_deref(), false);
            let output = env
                .command(
                    env!("CARGO_BIN_EXE_biomcp"),
                    &super::super::super::launch::arguments(args, &globals),
                )
                .output()
                .map_err(|e| e.to_string())?;
            if !output.status.success() {
                continue;
            }
            let expected: Value =
                serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
            if expected["status"] == "usage" || expected["status"] == "failed" {
                continue;
            }
            let (_, _, id) = biomcp_types::command_paths()
                .into_iter()
                .find(|(v, e, _)| *v == verb && *e == entity)
                .ok_or("command")?;
            let output = env
                .command(env!("CARGO_BIN_EXE_biomcp"), &["schema".into(), id.into()])
                .output()
                .map_err(|e| e.to_string())?;
            let query: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
            let Some(input) = input(args, &query) else {
                continue;
            };
            if !jsonschema::validator_for(&tool["inputSchema"])
                .map_err(|e| e.to_string())?
                .is_valid(&input)
            {
                continue;
            }
            config["mcp_tool_mode"] = json!("typed");
            env.config(&config).map_err(|e| e.to_string())?;
            let reply = exchange_with_globals(
                &env,
                &[request(
                    json!(1),
                    "tools/call",
                    json!({"name":name,"arguments":input}),
                )],
                &globals,
            )?;
            let actual = reply[0]
                .pointer("/result/structuredContent")
                .ok_or_else(|| format!("{name}: {}", reply[0]))?;
            if actual.to_string() != expected.to_string() {
                return Err(format!(
                    "{name} recorded CLI mismatch ({path:?}): {actual} versus {expected}"
                ));
            }
            if !jsonschema::validator_for(&tool["outputSchema"])
                .map_err(|e| e.to_string())?
                .is_valid(actual)
            {
                return Err(format!("{name} output schema mismatch"));
            }
            return Ok(());
        }
    }
    Err(format!(
        "no successful recorded search/get witness for {name}"
    ))
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
fn input(args: &[String], query: &Value) -> Option<Value> {
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
        if let Some((key, value)) = properties
            .iter()
            .find_map(|(key, p)| p.get("x-cli-flags")?.get(flag).map(|v| (key, v)))
        {
            result.insert(key.clone(), value.clone());
            continue;
        }
        let property = properties.get(&key)?;
        let kind = resolved(property, query);
        let value = if kind["type"] == "boolean" {
            json!(inline.unwrap_or("true").parse::<bool>().ok()?)
        } else {
            let word = inline
                .map(str::to_owned)
                .or_else(|| words.next().cloned())?;
            if kind["type"] == "integer" {
                json!(word.parse::<u64>().ok()?)
            } else if kind["type"] == "array" {
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
    let mut fields = properties
        .iter()
        .filter_map(|(key, p)| p["x-positional"].as_u64().map(|i| (i, key, p)))
        .collect::<Vec<_>>();
    fields.sort_by_key(|(index, _, _)| *index);
    let mut position = 0;
    for (_, key, property) in fields {
        let kind = resolved(property, query);
        if kind["type"] == "array" {
            result.insert(key.clone(), json!(&positional[position..]));
            position = positional.len();
        } else if let Some(value) = positional.get(position) {
            result.insert(key.clone(), json!(value));
            position += 1;
        }
    }
    (position == positional.len()).then_some(Value::Object(result))
}
