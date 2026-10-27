//! Feature mcp-parity: advertised schemas validate actual reply forms and reject unrelated objects.
use super::super::launch::Environment;
use serde_json::{Value, json};
use std::io::Write;

pub(super) fn check(tools: &[Value], reply: &Value) -> Result<(), String> {
    for advertised in tools {
        let schema = advertised
            .get("outputSchema")
            .ok_or("output schema missing")?;
        if schema == &json!({"type":"object"}) {
            return Err("tool advertises a bare object output schema".into());
        }
        local_references(schema, schema)?;
        let validator = jsonschema::validator_for(schema).map_err(|error| error.to_string())?;
        for invalid in [
            json!({"unrelated":true}),
            json!({"$id":"biomcp.health/2", "$schema":"unsupported"}),
        ] {
            if validator.is_valid(&invalid) {
                return Err(
                    "output schema accepts an unrelated object or unsupported schema dialect"
                        .into(),
                );
            }
        }
    }
    for args in [
        json!(["health"]),
        json!(["help"]),
        json!(["schema"]),
        json!(["schema", "biomcp.health/2"]),
        json!(["invalid-command"]),
    ] {
        super::super::research::call(
            &Environment::new().map_err(|e| e.to_string())?,
            "biomcp",
            json!({"args":args}),
            &[],
        )?;
    }
    writeln!(
        std::io::stdout(),
        "MCP_TOOLS_BYTES={}",
        serde_json::to_vec(reply).map_err(|e| e.to_string())?.len()
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn local_references(root: &Value, node: &Value) -> Result<(), String> {
    match node {
        Value::Object(fields) => {
            if let Some(reference) = fields.get("$ref").and_then(Value::as_str)
                && reference
                    .strip_prefix('#')
                    .and_then(|pointer| root.pointer(pointer))
                    .is_none()
            {
                return Err(format!(
                    "output schema reference is not locally resolvable: {reference}"
                ));
            }
            for value in fields.values() {
                local_references(root, value)?;
            }
        }
        Value::Array(values) => {
            for value in values {
                local_references(root, value)?;
            }
        }
        _ => {}
    }
    Ok(())
}
