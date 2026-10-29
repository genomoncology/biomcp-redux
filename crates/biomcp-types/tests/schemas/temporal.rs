//! Temporal field meanings stay consistent across every registered schema.
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

type Types = BTreeMap<String, BTreeSet<String>>;

fn collect(node: &Value, types: &mut Types) {
    if let Some(properties) = node.get("properties").and_then(Value::as_object) {
        for name in ["source_updated", "updated", "fetched", "retrieved"] {
            if let Some(property) = properties.get(name) {
                property_types(property, types.entry(name.into()).or_default());
            }
        }
    }
    match node {
        Value::Object(values) => values.values().for_each(|value| collect(value, types)),
        Value::Array(values) => values.iter().for_each(|value| collect(value, types)),
        _ => {}
    }
}

fn property_types(node: &Value, types: &mut BTreeSet<String>) {
    if let Some(reference) = node.get("$ref").and_then(Value::as_str) {
        types.insert(reference.trim_start_matches("#/$defs/Reply_").to_owned());
    } else if let Some(kind) = node.get("type").and_then(Value::as_str)
        && kind != "null"
    {
        types.insert(format!(
            "{kind}:{}",
            node.get("format").unwrap_or(&Value::Null)
        ));
    }
    for keyword in ["anyOf", "oneOf", "allOf"] {
        for branch in node
            .get(keyword)
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            property_types(branch, types);
        }
    }
}

fn check(types: &Types) -> Result<(), String> {
    for (name, definitions) in types {
        if definitions.len() != 1 {
            return Err(format!(
                "temporal property {name} has conflicting types: {definitions:?}"
            ));
        }
    }
    Ok(())
}

#[test]
fn temporal_properties_have_one_type_across_registered_schemas() -> Result<(), String> {
    let mut types = Types::new();
    for contract in super::registry() {
        collect(&(contract.generate)(), &mut types);
    }
    check(&types)?;
    for name in ["updated", "retrieved"] {
        assert_eq!(
            types.get(name),
            Some(&BTreeSet::from(["PartialDate".into()]))
        );
    }
    assert_eq!(
        types.get("source_updated"),
        Some(&BTreeSet::from(["SourceUpdated".into()]))
    );
    assert_eq!(
        types.get("fetched"),
        Some(&BTreeSet::from(["integer:\"uint64\"".into()]))
    );
    Ok(())
}

#[test]
fn temporal_property_check_rejects_conflicting_definition_types() {
    let mut types = Types::new();
    collect(
        &json!({"properties":{"source_updated":{"$ref":"#/$defs/Reply_SourceUpdated"}},
        "$defs":{"Other":{"properties":{"source_updated":{"$ref":"#/$defs/Reply_Text"}}}}}),
        &mut types,
    );
    assert!(check(&types).unwrap_err().contains("source_updated"));
}
