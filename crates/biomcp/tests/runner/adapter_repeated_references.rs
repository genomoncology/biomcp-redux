//! Feature pathway-reactome: every reference occurrence discloses rejected optional fields.
use super::{Stub, invoke};
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    for (native, field) in [
        ("displayName", "members.name"),
        ("url", "members.source_url"),
        ("schemaClass", "members.schema_class"),
    ] {
        for invalid in [json!(""), json!({"bad": true})] {
        let stub = reference_stub(native, invalid)?;
        let env = super::super::super::variant::source_environment(&stub)?;
        let value = invoke(
            &env,
            ["pathway", "genes", "R-HSA-5673001", "details"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
            0,
        )?;
        let problems = value
            .get("problems")
            .and_then(Value::as_array)
            .ok_or("missing problems")?;
        let rejected: Vec<_> = problems
            .iter()
            .filter(|p| {
                p.get("field")
                    .and_then(Value::as_str)
                    .is_some_and(|f| f.starts_with("members."))
            })
            .collect();
        if rejected.len() != 1
            || rejected.first().and_then(|p| p.get("field")) != Some(&json!(field))
        {
            return Err(format!(
                "repeated {native} lost its single disclosure: {value}"
            ));
        }
        for (path, expected) in [
            ("/result/items/0/name", json!("AKT1")),
            (
                "/result/items/0/source_url",
                json!("https://www.uniprot.org/uniprotkb/P31749"),
            ),
            (
                "/result/items/0/schema_class",
                json!("ReferenceGeneProduct"),
            ),
            ("/result/items/0/physical_context_count", json!(2)),
            ("/result/reference_occurrences", json!(2)),
            ("/result/physical_entities", json!(2)),
        ] {
            if value.pointer(path) != Some(&expected) {
                return Err(format!("repeated {native} changed {path}: {value}"));
            }
        }
        if value
            .pointer("/result/items")
            .and_then(Value::as_array)
            .map(Vec::len)
            != Some(1)
            || value
                .pointer("/result/items/0/participants")
                .and_then(Value::as_array)
                .map(Vec::len)
                != Some(2)
        {
            return Err(format!("repeated reference changed grouping: {value}"));
        }
        stub.finish()?;
        }
    }
    Ok(())
}

fn reference_stub(native: &'static str, invalid: Value) -> Result<Stub, String> {
    Stub::for_sources(
        &["reactome.org"],
        |_| "application/json",
        move |request| {
            let mut reference = json!({
                "stId":"UniProt:P31749", "identifier":"P31749",
                "displayName":"AKT1", "schemaClass":"ReferenceGeneProduct",
                "url":"https://www.uniprot.org/uniprotkb/P31749"
            });
            let first = reference.clone();
            *reference.get_mut(native).ok_or("missing reference field")? = invalid.clone();
            Ok(if request.contains("/database/version") {
                json!(97)
            } else if request.contains("/participants/") {
                json!([
                    {"peDbId":1,"displayName":"First context","schemaClass":"EntityWithAccessionedSequence",
                     "refEntities":[first]},
                    {"peDbId":2,"displayName":"Second context","schemaClass":"EntityWithAccessionedSequence",
                     "refEntities":[reference]}
                ])
            } else {
                json!({"stId":"R-HSA-5673001", "displayName":"Synthetic pathway",
                       "speciesName":"Homo sapiens", "schemaClass":"Pathway",
                       "stIdVersion":"R-HSA-5673001.1"})
            }
            .to_string())
        },
    )
}
