//! Compose reply contracts through one generator so definitions retain type identity.
use schemars::{JsonSchema, generate::SchemaSettings};
use serde_json::{Value, json};
use std::{collections::BTreeMap, error::Error, fs, path::Path};

#[path = "../../biomcp-types/schema_registry/mod.rs"]
mod registry;

#[derive(JsonSchema)]
struct SchemaDocument {
    #[serde(rename = "$id")]
    _id: String,
    #[serde(rename = "$schema")]
    _dialect: String,
}

pub(super) fn write(
    schemas: &BTreeMap<String, String>,
    output: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut generator = SchemaSettings::draft2020_12().into_generator();
    let mut roots = serde_json::Map::new();
    for contract in registry::registry() {
        let artifact: Value = serde_json::from_str(
            schemas
                .get(contract.id)
                .ok_or_else(|| format!("missing {}", contract.name))?,
        )?;
        if artifact.pointer("/properties/schema/const").is_some()
            || contract.id == "biomcp.error/1"
            || contract.id.starts_with("biomcp.research-")
        {
            roots.insert(
                contract.id.into(),
                Value::from((contract.generate)(&mut generator, false)),
            );
        }
    }
    let mut raw = Value::from(SchemaDocument::json_schema(&mut generator));
    raw["properties"]["$schema"] = json!({"const":"https://json-schema.org/draft/2020-12/schema"});
    raw["properties"]["$id"] = json!({"enum":schemas.keys().collect::<Vec<_>>()});
    roots.insert("schema-document".into(), raw);
    fs::write(
        output.join("mcp-schemas.json"),
        serde_json::to_vec(&json!({
            "roots":roots,"$defs":generator.take_definitions(true)
        }))?,
    )?;
    Ok(())
}
