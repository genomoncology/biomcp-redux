//! Feature mcp: advertised schemas validate actual reply forms and reject unrelated objects.
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
