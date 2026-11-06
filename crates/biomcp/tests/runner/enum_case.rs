//! Every published query choice keeps its meaning across letter case and adapters.
use crate::runner::{
    launch::Environment,
    mcp::{exchange_with_globals, request},
};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![crate::runner::transcript::trial(
        "query-validation/enumerated-letter-case",
        || check().map_err(Into::into),
    )]
}

fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"mcp_tool_mode":"typed", "mcp_typed_output_schema":true}))
        .map_err(|e| e.to_string())?;
    local_choices(&env)?;
    let mut failures = Vec::new();
    let mut count = 0;
    for &(verb, entity, id) in biomcp_types::command_paths() {
        let schema = cli(&env, &["schema".into(), id.into()])?;
        let properties = schema["properties"].as_object().ok_or("properties")?;
        for (key, property) in properties {
            if property.get("x-cli-flags").is_some() {
                continue; // These are separate boolean flags on both public adapters.
            }
            let Some(choice) = choice_node(property, &schema)
                .get("enum")
                .and_then(Value::as_array)
                .and_then(|v| v.iter().find_map(Value::as_str))
            else {
                continue;
            };
            count += 1;
            if let Err(error) = parameter(&env, verb, entity, &schema, key, choice) {
                failures.push(format!("{verb} {entity}.{key}: {error}"));
            }
        }
    }
    println!("ENUM_CASE parameters={count} failures={}", failures.len());
    if failures.is_empty() && count > 0 {
        Ok(())
    } else {
        Err(failures.join("\n"))
    }
}

fn parameter(
    env: &Environment,
    verb: &str,
    entity: &str,
    schema: &Value,
    key: &str,
    canonical: &str,
) -> Result<(), String> {
    let mut values = seed(verb, entity, schema)?;
    let property = &schema["properties"][key];
    let array = resolved(property, schema).get("type") == Some(&json!("array"));
    let value = |text: &str| if array { json!([text]) } else { json!(text) };
    values[key] = value(canonical);
    let expected = cli(env, &argv(verb, entity, schema, &values)?)?;
    if expected.pointer("/error/code") == Some(&json!("usage.arguments")) {
        return Err(format!("canonical baseline refused: {expected}"));
    }
    for spelling in [canonical.to_ascii_lowercase(), canonical.to_ascii_uppercase()] {
        values[key] = value(&spelling);
        let actual = cli(env, &argv(verb, entity, schema, &values)?)?;
        if actual != expected {
            return Err(format!("CLI {spelling}: expected {expected}; got {actual}"));
        }
        let frame = request(json!(1), "tools/call", json!({
            "name":format!("{verb}_{entity}").replace('-', "_"), "arguments":values
        }));
        let replies = exchange_with_globals(env, &[frame], &["--offline".into()])?;
        if replies.first().and_then(|r| r.pointer("/result/structuredContent")) != Some(&expected) {
            return Err(format!("typed {spelling}: expected {expected}; got {replies:?}"));
        }
    }
    Ok(())
}

fn choice_node<'a>(node: &'a Value, root: &'a Value) -> &'a Value {
    let node = resolved(node, root);
    node.get("items").map_or(node, |items| choice_node(items, root))
}

fn resolved<'a>(node: &'a Value, root: &'a Value) -> &'a Value {
    if let Some(reference) = node.get("$ref").and_then(Value::as_str) {
        return resolved(root.pointer(reference.trim_start_matches('#')).unwrap_or(node), root);
    }
    if let Some(branch) = node.get("anyOf").and_then(Value::as_array)
        .and_then(|v| v.iter().find(|n| n.get("type") != Some(&json!("null"))))
    {
        return resolved(branch, root);
    }
    node
}

fn seed(verb: &str, entity: &str, schema: &Value) -> Result<Value, String> {
    let mut values = json!({});
    for key in schema.get("required").and_then(Value::as_array).into_iter().flatten() {
        let key = key.as_str().ok_or("required name")?;
        let node = resolved(&schema["properties"][key], schema);
        let text = node.get("enum").and_then(Value::as_array)
            .and_then(|v| v.first()).and_then(Value::as_str)
            .unwrap_or_else(|| operand(if matches!(verb, "get" | "search") { entity } else { verb }, key));
        values[key] = if node.get("type") == Some(&json!("array")) {
            json!([text])
        } else { json!(text) };
    }
    if entity == "pgx" {
        values["gene"] = json!("CYP2D6");
    }
    if verb == "search" && schema["properties"].get("query").is_some() {
        values["query"] = json!("KRAS");
    }
    Ok(values)
}

fn operand<'a>(entity: &str, key: &'a str) -> &'a str {
    match key {
        "gene" => "KRAS",
        "study" => "study",
        "profile" => "profile",
        "sample_set" => "all",
        "text" | "query" => "melanoma",
        "pmid" => "12345678",
        _ => match entity {
            "variant" => "rs7412",
            "trial" => "NCT00000001",
            "article" => "12345678",
            "protein" => "P01116",
            "disease" => "MONDO:0007254",
            "phenotype" => "HP:0001250",
            "pathway" => "R-HSA-109581",
            "drug" => "CHEMBL941",
            "author" => "orcid:0000-0002-1825-0097",
            "adverse-event" => "10000001",
            "diagnostic" => "GTR000000001.1",
            "biomarker" => "C96022",
            "cell-line" => "CVCL_0030",
            "gwas" => "GCST000001",
            _ => "KRAS",
        },
    }
}

fn argv(verb: &str, entity: &str, schema: &Value, values: &Value) -> Result<Vec<String>, String> {
    let mut args = vec!["--offline".into(), verb.into(), entity.into()];
    let mut positional = Vec::new();
    for (key, value) in values.as_object().ok_or("values")? {
        let property = &schema["properties"][key];
        let words = if let Some(values) = value.as_array() { values.clone() } else { vec![value.clone()] };
        let words = words.iter().map(|v| v.as_str().ok_or("string operand").map(str::to_owned))
            .collect::<Result<Vec<_>, _>>()?;
        if let Some(index) = property.get("x-positional").and_then(Value::as_u64) {
            positional.push((index, words));
        } else {
            for word in words { args.push(format!("--{}={word}", key.replace('_', "-"))); }
        }
    }
    positional.sort_by_key(|(index, _)| *index);
    args.extend(positional.into_iter().flat_map(|(_, words)| words));
    Ok(args)
}

fn cli(env: &Environment, args: &[String]) -> Result<Value, String> {
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), args)
        .output().map_err(|e| e.to_string())?;
    serde_json::from_slice(&output.stdout).map_err(|e| format!("CLI envelope: {e}"))
}

fn local_choices(env: &Environment) -> Result<(), String> {
    for (before, choice, after) in [
        (vec!["--format"], "json", vec!["version"]),
        (vec!["--offline", "sync"], "hgnc", vec![]),
        (vec!["key", "set"], "oncokb", vec![]),
        (vec!["key", "remove"], "oncokb", vec![]),
        (vec!["help", "key"], "oncokb", vec![]),
        (vec!["help", "network"], "proxy", vec![]),
        (vec!["serve", "--tool-mode"], "typed", vec![]),
        (vec!["mcp-config", "--client"], "mcp-servers", vec![]),
    ] {
        let run = |spelling: &str| {
            let args = before.iter().map(|v| (*v).to_owned()).chain([spelling.into()])
                .chain(after.iter().map(|v| (*v).to_owned())).collect::<Vec<_>>();
            env.command(env!("CARGO_BIN_EXE_biomcp"), &args).output().map_err(|e| e.to_string())
        };
        let expected = run(choice)?;
        for spelling in [choice.to_ascii_lowercase(), choice.to_ascii_uppercase()] {
            let actual = run(&spelling)?;
            if actual.status != expected.status || actual.stdout != expected.stdout {
                return Err(format!("local enum {before:?} {spelling}: {} / {}",
                    String::from_utf8_lossy(&expected.stdout), String::from_utf8_lossy(&actual.stdout)));
            }
        }
    }
    Ok(())
}
