//! Publish input constraints separately from extensible output descriptions.
use super::{generate, policy::ContractKind};
use schemars::JsonSchema;
use serde_json::Value;

pub(super) fn reply<T: JsonSchema>() -> Value {
    let mut schema = generate::<T>();
    prepare(&mut schema, ContractKind::Reply);
    schema
}

pub(super) fn query<T: JsonSchema>() -> Value {
    let mut schema = generate::<T>();
    prepare(&mut schema, ContractKind::Query);
    schema
}

fn prepare(node: &mut Value, kind: ContractKind) {
    match node {
        Value::Object(object) => {
            let growable = object.remove("x-enum-open") == Some(Value::Bool(true));
            if matches!(kind, ContractKind::Reply) {
                if object.get("additionalProperties") == Some(&Value::Bool(false)) {
                    object.remove("additionalProperties");
                }
                if growable && let Some(known) = object.remove("enum") {
                    object.insert("x-extensible-enum".into(), known);
                }
            }
            for child in object.values_mut() {
                prepare(child, kind);
            }
        }
        Value::Array(children) => {
            for child in children {
                prepare(child, kind);
            }
        }
        _ => {}
    }
}

#[test]
fn published_reply_contracts_are_open_and_query_choices_stay_closed() {
    for contract in super::registry() {
        let mut schema = (contract.generate)();
        schema["$id"] = contract.id.into();
        super::compatibility::check_reviewed(
            &schema, &schema, contract.kind, &super::policy::Reviews::default(),
        ).unwrap_or_else(|error| panic!("{}: {error}", contract.name));
        match contract.kind {
            ContractKind::Reply => assert_open(&schema, contract.name),
            ContractKind::Query => {
                assert!(
                    schema.get("x-extensible-enum").is_none(),
                    "{}",
                    contract.name
                );
            }
        }
    }
    for (schema, expected) in [
        (reply::<biomcp_types::Source>(), "hgnc"),
        (reply::<biomcp_types::Reason>(), "not_found"),
        (reply::<biomcp_types::DrugSource>(), "chembl"),
        (reply::<biodata::Proposition>(), "pathogenicity"),
        (reply::<biodata::Predicate>(), "is_causal_for"),
    ] {
        assert!(schema.get("enum").is_none(), "reply enum remains closed");
        assert!(
            schema["x-extensible-enum"]
                .as_array()
                .unwrap()
                .contains(&expected.into())
        );
        let validator = jsonschema::draft202012::new(&schema).unwrap();
        assert!(validator.is_valid(&serde_json::json!("future_value")));
        assert!(!validator.is_valid(&serde_json::json!(42)));
    }
    let drug = query::<biomcp_types::DrugGet>();
    let validator = jsonschema::draft202012::new(&drug).unwrap();
    assert!(validator.is_valid(&serde_json::json!({"input":"CHEMBL941","source":"chembl"})));
    assert!(
        !validator.is_valid(&serde_json::json!({"input":"CHEMBL941","source":"future_source"}))
    );
    let schema = query::<biomcp_types::GeneGet>();
    assert_eq!(schema["additionalProperties"], false);
    let validator = jsonschema::draft202012::new(&schema).unwrap();
    assert!(validator.is_valid(&serde_json::json!({"input":["KRAS"]})));
    assert!(!validator.is_valid(&serde_json::json!({"input":["KRAS"],"future_field":true})));
}

fn assert_open(value: &Value, name: &str) {
    match value {
        Value::Object(object) => {
            assert_ne!(
                object.get("additionalProperties"),
                Some(&Value::Bool(false)),
                "{name}"
            );
            assert!(
                !object.contains_key("x-enum-open"),
                "unpublished marker in {name}"
            );
            for child in object.values() {
                assert_open(child, name);
            }
        }
        Value::Array(items) => {
            for child in items {
                assert_open(child, name);
            }
        }
        _ => {}
    }
}
