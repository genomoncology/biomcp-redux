use super::generate;
use biodata::{HgncId, PartialDate, Position1, Span0, Problem, ProblemCode, Reason, Source, Text};
use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};

#[expect(
    clippy::expect_used,
    reason = "Schema test setup must build a validator"
)]
fn validator<T: JsonSchema>() -> jsonschema::Validator {
    jsonschema::draft202012::new(&generate::<T>()).expect("valid local schema")
}

#[expect(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "Public enum schema table fails on invalid test fixtures"
)]
fn enum_values<T: JsonSchema + Serialize + DeserializeOwned>(values: &[T]) {
    let validator = validator::<T>();
    for value in values {
        assert!(validator.is_valid(&serde_json::to_value(value).unwrap()));
    }
    for value in [
        json!("unknown"),
        json!("HGNC"),
        json!(""),
        json!(123),
        Value::Null,
    ] {
        assert!(!validator.is_valid(&value));
    }
    let schema = generate::<T>();
    for spelling in schema
        .get("enum")
        .and_then(Value::as_array)
        .expect("string enum")
    {
        let value: T = serde_json::from_value(spelling.clone()).expect("canonical spelling");
        assert_eq!(&serde_json::to_value(value).unwrap(), spelling);
    }
}

#[test]
fn enum_schemas_validate_canonical_public_values() {
    enum_values(&[
        Source::Hgnc,
        Source::MyGene,
        Source::PubMed,
        Source::EuropePmc,
        Source::PubTator,
        Source::ClinicalTrialsGov,
        Source::Civic,
    ]);
    enum_values(&[
        Reason::NotFound,
        Reason::RateLimited,
        Reason::Unavailable,
        Reason::Rejected,
        Reason::BadReply,
        Reason::DeadlineExceeded,
        Reason::Offline,
        Reason::NoKey,
        Reason::Disabled,
        Reason::ReplayMiss,
        Reason::Syncing,
    ]);
    enum_values(&[ProblemCode::ValueUnknown]);
}

#[test]
fn record_schema_matches_serialization_and_rejects_unknown_fields() {
    let problem_validator = validator::<Problem>();
    for record in [None, Some(Text::new("record-123").unwrap())] {
        let problem = Problem {
            source: Source::PubMed,
            record,
            field: "publication_type".into(),
            code: ProblemCode::ValueUnknown,
        };
        let value = serde_json::to_value(problem).unwrap();
        assert!(problem_validator.is_valid(&value));
        let mut unknown = value.clone();
        unknown["unexpected"] = json!(true);
        assert!(!problem_validator.is_valid(&unknown));
        for field in ["source", "field", "code"] {
            let mut missing = value.clone();
            missing.as_object_mut().unwrap().remove(field);
            assert!(!problem_validator.is_valid(&missing), "missing {field}");
        }
        let mut invalid = value;
        invalid["record"] = json!(3);
        assert!(!problem_validator.is_valid(&invalid));
    }
    let text = validator::<Text>();
    assert!(text.is_valid(&serde_json::to_value(Text::new("summary").unwrap()).unwrap()));
    assert!(!text.is_valid(&json!("")));
    assert!(!text.is_valid(&json!(42)));
}

#[test]
fn value_schemas_describe_canonical_text_and_coordinate_bounds() {
    let id = validator::<HgncId>();
    for text in ["HGNC:1", "HGNC:4294967295"] { assert!(id.is_valid(&json!(text))); }
    for value in [json!("HGNC:0"), json!("hgnc:1"), json!("HGNC:01"), json!("HGNC:1x"), json!(1)] {
        assert!(!id.is_valid(&value));
    }
    let date = validator::<PartialDate>();
    for text in ["0001", "2024-02", "2024-02-29"] {
        let value: PartialDate = text.parse().unwrap();
        assert!(date.is_valid(&serde_json::to_value(value).unwrap()));
    }
    for value in [json!("2024-2"), json!("2024-02-1"),json!("2024-01-01x"),json!(2024)] {
        assert!(!date.is_valid(&value));
    }
    let position = validator::<Position1>();
    for number in [1,u32::MAX] {
        assert!(position.is_valid(&serde_json::to_value(Position1::new(number).unwrap()).unwrap()));
    }
    for value in [json!(0),json!(-1),json!(4294967296_u64),json!(1.5),json!("1")] {
        assert!(!position.is_valid(&value));
    }
    let span = validator::<Span0>();
    assert!(span.is_valid(&serde_json::to_value(Span0::new(0,u32::MAX).unwrap()).unwrap()));
    for value in [json!({"start":0}),json!({"end":1}),json!({"start":0,"end":1,"extra":true}),json!({"start":-1,"end":1}),json!({"start":0,"end":4294967296_u64})] {
        assert!(!span.is_valid(&value));
    }
    // These constraints require runtime checks beyond JSON Schema's vocabulary.
    for value in [json!("HGNC:4294967296")] {
        assert!(id.is_valid(&value));
        assert!(serde_json::from_value::<HgncId>(value).is_err());
    }
    assert!(date.is_valid(&json!("2023-02-29")));
    assert!(serde_json::from_value::<PartialDate>(json!("2023-02-29")).is_err());
    assert!(span.is_valid(&json!({"start":2,"end":1})));
    assert!(serde_json::from_value::<Span0>(json!({"start":2,"end":1})).is_err());
}
