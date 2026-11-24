//! Verify that strict function clients can fill every advertised entity property.
use crate::runner::{
    launch::Environment,
    mcp::{exchange_with_globals, request},
};
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|error| error.to_string())?;
    env.config(&json!({"mcp_tool_mode":"typed"}))
        .map_err(|error| error.to_string())?;
    let replies = exchange_with_globals(
        &env,
        &[request(json!(1), "tools/list", json!({}))],
        &["--offline".into()],
    )?;
    let tools = replies
        .first()
        .and_then(|reply| reply.pointer("/result/tools"))
        .and_then(Value::as_array)
        .ok_or("typed tool definitions missing")?;
    for tool in tools {
        // Saved work embeds an open-ended MCP call, with its own argument contract.
        if tool["name"] == "saved" {
            continue;
        }
        let mut strict = tool["inputSchema"].clone();
        let name = tool["name"].as_str().ok_or("tool name missing")?;
        let example = normalize(&mut strict, name)?;
        let validator = jsonschema::validator_for(&strict).map_err(|error| error.to_string())?;
        if let Some(example) = example {
            validator
                .validate(&example)
                .map_err(|error| format!("{name}: all-property input rejected: {error}"))?;
        }
    }
    Ok(())
}

/// Mirror function clients that close objects and require all their properties. Check
/// omission before normalizing so the client cannot mask a missing unset choice.
fn normalize(schema: &mut Value, path: &str) -> Result<Option<Value>, String> {
    if schema.get("oneOf").is_some() {
        return Err(format!("{path}: strict function schemas use anyOf"));
    }
    let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
        return Ok(None);
    };
    let originally_required = schema["required"].as_array().cloned().unwrap_or_default();
    let names = properties.keys().cloned().collect::<Vec<_>>();
    let mut sample = serde_json::Map::new();
    for name in &names {
        let property = &mut schema["properties"][name];
        let field = format!("{path}.{name}");
        let nested = normalize(property, &field)?;
        let validator = jsonschema::validator_for(property).map_err(|error| error.to_string())?;
        let unset = [Value::Null, json!("")]
            .into_iter()
            .find(|value| validator.is_valid(value));
        if !originally_required.contains(&json!(name)) {
            sample.insert(
                name.clone(),
                unset.ok_or_else(|| format!("{field}: no unset value"))?,
            );
        } else if let Some(value) = nested.or(unset).or_else(|| {
            ["get", "example"]
                .into_iter()
                .map(|value| json!(value))
                .chain([json!(["help"]), json!(1), json!(false)])
                .find(|value| validator.is_valid(value))
        }) {
            sample.insert(name.clone(), value);
        } else {
            return Err(format!("{field}: no sample value for required property"));
        }
    }
    schema["required"] = json!(names);
    schema["additionalProperties"] = json!(false);
    let sample = Value::Object(sample);
    jsonschema::validator_for(schema)
        .map_err(|error| error.to_string())?
        .validate(&sample)
        .map_err(|error| format!("{path}: strict input rejected: {error}"))?;
    Ok(Some(sample))
}
