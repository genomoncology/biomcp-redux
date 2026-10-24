use super::generate;
use biodata::{Problem, ProblemCode, Reason, Source, Text};
use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};

fn validator<T: JsonSchema>() -> jsonschema::Validator {
    jsonschema::draft202012::new(&generate::<T>()).expect("valid local schema")
}

fn enum_values<T: JsonSchema + Serialize + DeserializeOwned>(values: &[T]) {
    let validator = validator::<T>();
    for value in values {
        assert!(validator.is_valid(&serde_json::to_value(value).unwrap()));
    }
    for value in [json!("unknown"), json!("HGNC"), json!(""), json!(123), Value::Null] {
        assert!(!validator.is_valid(&value));
    }
    let schema = generate::<T>();
    for spelling in schema["enum"].as_array().expect("string enum") {
        let value: T = serde_json::from_value(spelling.clone()).expect("canonical spelling");
        assert_eq!(&serde_json::to_value(value).unwrap(), spelling);
    }
}

#[test]
fn enum_schemas_validate_canonical_public_values() {
    enum_values(&[Source::Hgnc, Source::MyGene, Source::PubMed, Source::EuropePmc,
        Source::PubTator, Source::ClinicalTrialsGov, Source::Civic]);
    enum_values(&[Reason::NotFound, Reason::RateLimited, Reason::Unavailable, Reason::Rejected,
        Reason::BadReply, Reason::DeadlineExceeded, Reason::Offline, Reason::NoKey,
        Reason::Disabled, Reason::ReplayMiss, Reason::Syncing]);
    enum_values(&[ProblemCode::ValueUnknown]);
}

#[test]
fn record_schema_matches_serialization_and_rejects_unknown_fields() {
    let validator = validator::<Problem>();
    for record in [None, Some(Text::new("record-123").unwrap())] {
        let problem = Problem { source: Source::PubMed, record,
            field: "publication_type".into(), code: ProblemCode::ValueUnknown };
        let value = serde_json::to_value(problem).unwrap();
        assert!(validator.is_valid(&value));
        let mut unknown = value.clone();
        unknown["unexpected"] = json!(true);
        assert!(!validator.is_valid(&unknown));
        for field in ["source", "field", "code"] {
            let mut missing = value.clone();
            missing.as_object_mut().unwrap().remove(field);
            assert!(!validator.is_valid(&missing), "missing {field}");
        }
        let mut invalid = value;
        invalid["record"] = json!(3);
        assert!(!validator.is_valid(&invalid));
    }
    let text = validator::<Text>();
    assert!(text.is_valid(&serde_json::to_value(Text::new("summary").unwrap()).unwrap()));
    assert!(!text.is_valid(&json!("")));
    assert!(!text.is_valid(&json!(42)));
}
