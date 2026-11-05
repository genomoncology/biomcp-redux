//! Reuse existing recorded caller cases without maintaining a second entity inventory.
use super::super::super::{
    case_data,
    launch::Environment,
    mcp::{exchange_with_globals, request},
    response_cases,
};
use serde_json::{Value, json};

pub(super) fn check(tool: &Value, case_index: &mut usize) -> Result<(), String> {
    let name = tool
        .get("name")
        .and_then(Value::as_str)
        .ok_or("tool name")?;
    let (verb, entity, _) = biomcp_types::command_paths()
        .into_iter()
        .find(|(verb, entity, _)| format!("{verb}_{entity}").replace('-', "_") == name)
        .ok_or_else(|| format!("{name}: absent from query registry"))?;
    let mut paths = Vec::new();
    let cases = super::super::super::launch::fixtures().join("cases");
    case_data::collect(&cases, &mut paths).map_err(|e| e.to_string())?;
    paths.sort();
    let mut calls = 0;
    let mut excluded = 0;
    let mut failures = Vec::new();
    for path in paths {
        let case = case_data::read_case(&path)?;
        for (index, step) in case.steps.iter().enumerate() {
            let Some(args) = &step.run else { continue };
            if args.first().map(String::as_str) != Some(verb)
                || args.get(1).map(String::as_str) != Some(entity)
            {
                continue;
            }
            let label = format!("{name} {} step {}", path.display(), index + 1);
            super::super::super::transcript::case_step(*case_index);
            use std::io::Write;
            writeln!(
                std::io::stdout(),
                "TYPED_REPLAY_CASE index={case_index} case={label}"
            )
            .map_err(|e| e.to_string())?;
            *case_index += 1;
            match candidate(tool, args, &case, index, &label, &mut calls) {
                Ok(true) => {}
                Ok(false) => excluded += 1,
                Err(error) => failures.push(format!("{label}: {error}")),
            }
        }
    }
    if calls == 0 {
        failures.push(format!("{name}: no recorded typed call; no exemption"));
    }
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "TYPED_REPLAY tool={name} calls={calls} excluded={excluded} failures={}",
        failures.len()
    )
    .map_err(|e| e.to_string())?;
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("\n"))
    }
}

fn prepare(
    case: &case_data::Case,
    index: usize,
) -> Result<(Environment, Vec<String>, Value), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let (globals, config) = prepare_in(
        &env,
        case,
        index,
        std::path::Path::new(env!("CARGO_BIN_EXE_biomcp")),
    )?;
    Ok((env, globals, config))
}

fn prepare_in(
    env: &Environment,
    case: &case_data::Case,
    index: usize,
    binary: &std::path::Path,
) -> Result<(Vec<String>, Value), String> {
    let config = case.config.clone().unwrap_or_else(|| json!({}));
    env.config(&config).map_err(|e| e.to_string())?;
    let replay = response_cases::replay(env, case.replay.as_deref(), case.cache_acquired)?;
    let environment = env.using_executable(binary);
    let registry = super::super::super::schema::Registry::embedded()?;
    let mut previous = None;
    let mut previous_step = None;
    for step in case.steps.get(..index).ok_or("case step index")? {
        previous = Some(super::super::super::run_sequence(
            step,
            &environment,
            &Vec::new(),
            registry.as_ref(),
            replay.as_deref(),
            previous.as_ref(),
            previous_step,
        )?);
        previous_step = Some(step);
    }
    let step = case.steps.get(index).ok_or("case step index")?;
    let globals = env.globals(replay.as_deref(), step.offline);
    Ok((globals, config))
}

fn candidate(
    tool: &Value,
    args: &[String],
    case: &case_data::Case,
    index: usize,
    label: &str,
    calls: &mut usize,
) -> Result<bool, String> {
    let (env, mut globals, mut config) = prepare(case, index)?;
    let (args, extra) = query_arguments(args);
    globals.extend(extra);
    let step = case.steps.get(index).ok_or("case step index")?;
    let expected = cli_case(&env, &args, &globals, step)?;
    let verb = args.first().ok_or("command verb")?;
    let entity = args.get(1).ok_or("command entity")?;
    let (_, _, id) = biomcp_types::command_paths()
        .into_iter()
        .find(|(v, e, _)| *v == verb.as_str() && *e == entity.as_str())
        .ok_or("command")?;
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &["schema".into(), id.into()])
        .output()
        .map_err(|e| e.to_string())?;
    let query: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    let Some(input) = input(&args, &query) else {
        if expected.get("status") != Some(&json!("usage")) {
            return Err(format!("valid CLI cannot translate: {args:?}"));
        }
        use std::io::Write;
        writeln!(std::io::stdout(), "TYPED_REPLAY_CLI_SYNTAX case={label} step={} args={args:?} reason={} CLI_witness={expected} typed_coverage=typed_tools::rejections(unknown-key/non-object/malformed-value)",
            index + 1, expected.get("error").unwrap_or(&Value::Null)).map_err(|e| e.to_string())?;
        return Ok(false);
    };
    let mut schema_failure = None;
    if let Err(error) = validate_input(tool, &input) {
        if expected.get("status") == Some(&json!("usage")) {
            use std::io::Write;
            writeln!(std::io::stdout(), "TYPED_REPLAY_EXPECTED_INVALID case={label} step={} {error}; CLI_witness={expected}", index + 1)
                .map_err(|e| e.to_string())?;
        } else {
            schema_failure = Some(error);
        }
    }
    let frozen_parity = single_parity(&env, case, index, &args, &globals);
    // Keep this comparison independent even when a frozen setup or call failed.
    reset_prepare(
        &env,
        case,
        index,
        std::path::Path::new(env!("CARGO_BIN_EXE_biomcp")),
    )?;
    let expected = cli_case(&env, &args, &globals, step)?;
    reset_prepare(
        &env,
        case,
        index,
        std::path::Path::new(env!("CARGO_BIN_EXE_biomcp")),
    )?;
    config
        .as_object_mut()
        .ok_or("configuration object")?
        .insert("mcp_tool_mode".into(), json!("typed"));
    env.config(&config).map_err(|e| e.to_string())?;
    let typed_parity = compare(tool, &input, &expected, &args, &env, &globals, calls);
    let mut failures = Vec::new();
    if let Some(error) = schema_failure {
        failures.push(error);
    }
    if let Err(error) = frozen_parity {
        failures.push(error);
    }
    if let Err(error) = typed_parity {
        failures.push(error);
    }
    if failures.is_empty() {
        Ok(true)
    } else {
        Err(failures.join("\n"))
    }
}

fn cli_case(
    env: &Environment,
    args: &[String],
    globals: &[String],
    step: &case_data::Step,
) -> Result<Value, String> {
    let mut command = env.command(
        env!("CARGO_BIN_EXE_biomcp"),
        &super::super::super::launch::arguments(args, globals),
    );
    let output = super::super::super::execution::output(&mut command, step.stdin.as_deref())?;
    if output.status.code() != Some(step.exit) {
        return Err(format!(
            "CLI exit changed: expected {}, actual {:?}: {args:?}",
            step.exit,
            output.status.code()
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}

fn reset_prepare(
    env: &Environment,
    case: &case_data::Case,
    index: usize,
    binary: &std::path::Path,
) -> Result<(), String> {
    for entry in std::fs::read_dir(env.root()).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            std::fs::remove_dir_all(path)
        } else {
            std::fs::remove_file(path)
        }
        .map_err(|e| e.to_string())?;
    }
    for directory in ["home", "config", "cache"] {
        std::fs::create_dir(env.root().join(directory)).map_err(|e| e.to_string())?;
    }
    prepare_in(env, case, index, binary).map(|_| ())
}

fn single_parity(
    env: &Environment,
    case: &case_data::Case,
    index: usize,
    args: &[String],
    globals: &[String],
) -> Result<(), String> {
    let frozen = super::super::super::launch::typed_baseline().ok_or("missing frozen baseline")?;
    let current = std::path::Path::new(env!("CARGO_BIN_EXE_biomcp"));
    let frames = [request(
        json!(1),
        "tools/call",
        json!({"name":"biomcp","arguments":{"args":args}}),
    )];
    let mut server_args = vec!["serve".into()];
    server_args.extend_from_slice(globals);
    reset_prepare(env, case, index, &frozen)?;
    let old = super::baseline::exchange(env, &frozen, &server_args, &frames)?;
    reset_prepare(env, case, index, current)?;
    let new = super::baseline::exchange(env, current, &server_args, &frames)?;
    let mut failure = None;
    if old != new {
        reset_prepare(env, case, index, current)?;
        let control = super::baseline::exchange(env, current, &server_args, &frames)?;
        failure = Some(if new == control {
            "frozen/current deterministic single RAW mismatch; exact frozen/current bytes are in this case indexed transcript".to_owned()
        } else {
            "CURRENT/CURRENT nondeterministic single RAW mismatch in identical reset root; exact frozen/current/control bytes are in this case indexed transcript; no fields exempted".to_owned()
        });
    }
    match failure {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

fn compare(
    tool: &Value,
    input: &Value,
    expected: &Value,
    args: &[String],
    env: &Environment,
    globals: &[String],
    calls: &mut usize,
) -> Result<(), String> {
    let name = tool
        .get("name")
        .and_then(Value::as_str)
        .ok_or("tool name")?;
    let reply = exchange_with_globals(
        env,
        &[request(
            json!(1),
            "tools/call",
            json!({"name":name,"arguments":input}),
        )],
        globals,
    )?;
    let reply = reply.first().ok_or("missing typed reply")?;
    *calls += 1;
    let actual = reply
        .pointer("/result/structuredContent")
        .ok_or_else(|| format!("{name}: {reply}"))?;
    if serde_json::to_vec(actual).map_err(|e| e.to_string())?
        != serde_json::to_vec(expected).map_err(|e| e.to_string())?
    {
        return Err(format!(
            "{name} decoded CLI JSON mismatch: {actual} versus {expected}"
        ));
    }
    super::super::super::schema::Registry::embedded()?.validate(
        actual,
        args,
        expected.get("status") != Some(&json!("usage")),
    )?;
    if let Some(schema) = tool.get("outputSchema")
        && !jsonschema::validator_for(schema)
            .map_err(|e| e.to_string())?
            .is_valid(actual)
    {
        return Err(format!("{name} output schema mismatch"));
    }
    Ok(())
}

fn validate_input(tool: &Value, input: &Value) -> Result<(), String> {
    let schema = tool.get("inputSchema").ok_or("missing input schema")?;
    let validator = jsonschema::validator_for(schema).map_err(|e| e.to_string())?;
    validator
        .validate(input)
        .map_err(|e| format!("typed schema rejected recorded case: {e}; input={input}"))
}

fn query_arguments(args: &[String]) -> (Vec<String>, Vec<String>) {
    let mut query = Vec::new();
    let mut globals = Vec::new();
    let mut words = args.iter();
    while let Some(word) = words.next() {
        match word.as_str() {
            "--format" => {
                words.next();
            }
            "--disable-sources" => {
                globals.push(word.clone());
                if let Some(value) = words.next() {
                    globals.push(value.clone());
                }
            }
            "--offline" | "--fresh" => globals.push(word.clone()),
            _ => query.push(word.clone()),
        }
    }
    (query, globals)
}

pub(super) fn rejects_overstrict_schema(tool: &Value) -> Result<(), String> {
    let mut tool = tool.clone();
    tool.as_object_mut()
        .ok_or("tool object")?
        .insert("inputSchema".into(), json!(false));
    let error = match check(&tool, &mut 0) {
        Ok(()) => return Err("over-strict schema silently skipped recorded cases".into()),
        Err(error) => error,
    };
    if !error.contains("typed schema rejected recorded case") {
        return Err(format!(
            "over-strict replay failed for the wrong reason: {error}"
        ));
    }
    Ok(())
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
        if properties.values().any(|property| {
            property
                .get("x-cli-flags")
                .and_then(|flags| flags.get(flag))
                .is_some()
        }) {
            result.insert(key, json!(true));
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
