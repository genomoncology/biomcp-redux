use super::generate;
use biodata::*;
use biomcp_types::*;
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
fn enum_values<T: JsonSchema + Serialize + DeserializeOwned + std::fmt::Display>(values: &[T]) {
    let validator = validator::<T>();
    for value in values {
        assert!(validator.is_valid(&serde_json::to_value(value).unwrap()));
    }
    for value in [
        json!("distinctive-unrecognized-value"),
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
        assert_eq!(value.to_string(), spelling.as_str().unwrap());
        assert_eq!(&serde_json::to_value(value).unwrap(), spelling);
    }
}

#[test]
fn enum_schemas_validate_canonical_public_values() {
    for spelling in generate::<Source>()["enum"].as_array().unwrap() {
        let source: Source = serde_json::from_value(spelling.clone()).unwrap();
        assert_eq!(serde_json::to_value(source.text()).unwrap(), *spelling);
    }
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
    enum_values(&[GeneStatus::Approved]);
    enum_values(&[LocusGroup::ProteinCoding]);
    enum_values(&[LocusType::Readthrough]);
    enum_values(&[MatchKind::Symbol]);
    enum_values(&[SummarySource::RefSeq]);
    enum_values(&[HgncStatus::EntryWithdrawn]);
    enum_values(&[Build::GRCh37, Build::GRCh38]);
    enum_values(&[Strand::Plus]);
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
    for text in ["HGNC:1", "HGNC:4294967295"] {
        assert!(id.is_valid(&json!(text)));
    }
    for value in [
        json!("HGNC:0"),
        json!("hgnc:1"),
        json!("HGNC:01"),
        json!("HGNC:1x"),
        json!(1),
    ] {
        assert!(!id.is_valid(&value));
    }
    let date = validator::<PartialDate>();
    for text in ["0001", "2024-02", "2024-02-29"] {
        let value: PartialDate = text.parse().unwrap();
        assert!(date.is_valid(&serde_json::to_value(value).unwrap()));
    }
    for value in [
        json!("2024-2"),
        json!("2024-02-1"),
        json!("2024-01-01x"),
        json!(2024),
    ] {
        assert!(!date.is_valid(&value));
    }
    let position = validator::<Position1>();
    for number in [1, u32::MAX] {
        assert!(position.is_valid(&serde_json::to_value(Position1::new(number).unwrap()).unwrap()));
    }
    for value in [
        json!(0),
        json!(-1),
        json!(4294967296_u64),
        json!(1.5),
        json!("1"),
    ] {
        assert!(!position.is_valid(&value));
    }
    let span = validator::<Span0>();
    assert!(span.is_valid(&serde_json::to_value(Span0::new(0, u32::MAX).unwrap()).unwrap()));
    for value in [
        json!({"start":0}),
        json!({"end":1}),
        json!({"start":0,"end":1,"extra":true}),
        json!({"start":-1,"end":1}),
        json!({"start":0,"end":4294967296_u64}),
    ] {
        assert!(!span.is_valid(&value));
    }
    // These constraints require runtime checks beyond JSON Schema's vocabulary.
    let overflow = json!("HGNC:4294967296");
    assert!(id.is_valid(&overflow));
    assert!(serde_json::from_value::<HgncId>(overflow).is_err());
    assert!(date.is_valid(&json!("2023-02-29")));
    assert!(serde_json::from_value::<PartialDate>(json!("2023-02-29")).is_err());
    assert!(span.is_valid(&json!({"start":2,"end":1})));
    assert!(serde_json::from_value::<Span0>(json!({"start":2,"end":1})).is_err());
}

#[test]
fn usage_error_round_trips_and_rejects_changed_contract_values() {
    let envelope = ErrorEnvelope::schema_usage();
    let value = serde_json::to_value(&envelope).unwrap();
    assert_eq!(
        value,
        json!({
            "schema": "biomcp.error/1",
            "status": "usage",
            "error": {
                "code": "usage.schema",
                "message": "use biomcp schema to list known artifact IDs"
            }
        })
    );
    assert_eq!(
        serde_json::from_value::<ErrorEnvelope>(value.clone()).unwrap(),
        envelope
    );
    let schema = validator::<ErrorEnvelope>();
    assert!(schema.is_valid(&value));
    for pointer in ["/schema", "/status", "/error/code", "/error/message"] {
        for replacement in [
            json!("unsupported"),
            Value::Null,
            json!(42),
            json!({value.pointer(pointer).unwrap().as_str().unwrap(): null}),
        ] {
            let mut changed = value.clone();
            *changed.pointer_mut(pointer).unwrap() = replacement;
            assert!(!schema.is_valid(&changed), "{pointer}: {changed}");
            assert!(serde_json::from_value::<ErrorEnvelope>(changed).is_err());
        }
    }
    for (pointer, fields) in [
        ("", vec!["schema", "status", "error"]),
        ("/error", vec!["code", "message"]),
    ] {
        let mut unknown = value.clone();
        unknown
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("unexpected".into(), json!(true));
        assert!(!schema.is_valid(&unknown));
        assert!(serde_json::from_value::<ErrorEnvelope>(unknown).is_err());
        for field in fields {
            let mut missing = value.clone();
            missing
                .pointer_mut(pointer)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove(field);
            assert!(!schema.is_valid(&missing));
            assert!(serde_json::from_value::<ErrorEnvelope>(missing).is_err());
        }
    }
}

#[test]
fn mane_schema_enforces_its_curated_versioned_transcript_boundary() {
    let schema = validator::<Mane>();
    for (refseq, ensembl, valid) in [
        ("NM_004333.6", "ENST00000646891.2", true),
        ("NR_123456.1", "ENST00000646891.2", true),
        ("NM_004333", "ENST00000646891.2", false),
        ("NM_004333.6", "ENST00000646891", false),
        ("XM_004333.6", "ENST00000646891.2", false),
        ("XR_004333.6", "ENST00000646891.2", false),
    ] {
        let value = json!({"status":"select", "refseq":refseq,"ensembl":ensembl});
        assert_eq!(schema.is_valid(&value), valid, "{value}");
        assert_eq!(serde_json::from_value::<Mane>(value).is_ok(), valid);
    }
}

#[test]
fn search_schema_and_serde_require_a_present_nonnull_selector() {
    let schema = validator::<GeneSearch>();
    for (value, valid) in [
        (json!({}), false),
        (json!({"limit":10,"offset":0}), false),
        (json!({"query":null}), false),
        (json!({"band":null}), false),
        (json!({"query":"BRAF"}), true),
        (json!({"band":"9p21.3"}), true),
        (json!({"query":"BRAF","band":"9p21.3"}), true),
        (json!({"query":"BRAF","band":null}), false),
    ] {
        assert_eq!(schema.is_valid(&value), valid, "schema: {value}");
        assert_eq!(
            serde_json::from_value::<GeneSearch>(value.clone()).is_ok(),
            valid,
            "serde: {value}"
        );
    }
}

#[test]
fn article_accepts_each_identity_and_rejects_an_unidentified_record() {
    let base = json!({
        "title": "A checked article", "types": [], "corrections": [],
        "authors": [], "abstract_parts": []
    });
    let schema = validator::<Article>();
    for (field, id) in [
        ("pmid", "33123537"),
        ("pmcid", "PMC7573235"),
        ("europepmc_id", "PPR1064174"),
        ("doi", "10.3389/fcell.2020.573599"),
    ] {
        let mut value = base.clone();
        value[field] = json!(id);
        assert!(schema.is_valid(&value), "schema rejected {field}");
        let article: Article = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(article).unwrap(), value);
    }
    for value in [base.clone(), {
        let mut value = base;
        for field in ["pmid", "pmcid", "doi", "europepmc_id"] {
            value[field] = Value::Null;
        }
        value
    }] {
        assert!(!schema.is_valid(&value));
        assert!(serde_json::from_value::<Article>(value).is_err());
    }
}

#[test]
fn article_input_schema_preserves_preprint_local_identifiers() {
    let schema = validator::<ArticleInput>();
    for (id, valid) in [
        ("PPR1295425", true),
        ("PPR0001", true),
        ("PPR0000", false),
        ("PPR", false),
        ("PPR1/other", false),
    ] {
        let value = json!(id);
        assert_eq!(schema.is_valid(&value), valid, "{id}");
        assert_eq!(
            serde_json::from_value::<ArticleInput>(value).is_ok(),
            valid,
            "{id}"
        );
    }
}

#[test]
fn article_access_filter_schema_and_serde_share_the_native_policy() {
    let schema = validator::<ArticleSearch>();
    for policy in ["include", "exclude"] {
        let value = json!({"text":"AKT1 E17K", "open_access":true, "no_preprints":true, "retracted":policy});
        assert!(schema.is_valid(&value));
        let query: ArticleSearch = serde_json::from_value(value).expect("checked access policy");
        assert!(query.open_access() && query.no_preprints());
        assert_eq!(query.exclude_retracted(), policy == "exclude");
    }
    for value in [
        json!({"text":"AKT1", "retracted":"INCLUDE"}),
        json!({"text":"AKT1", "retracted":null}),
        json!({"text":"AKT1", "open_access":"true"}),
    ] {
        assert!(!schema.is_valid(&value));
        assert!(serde_json::from_value::<ArticleSearch>(value).is_err());
    }
}

#[test]
fn curie_runtime_and_published_schema_accept_the_same_checked_ids()
-> Result<(), Box<dyn std::error::Error>> {
    let schema: Value = serde_json::from_str(include_str!("../../schemas/statement.json"))?;
    let curie = schema
        .pointer("/$defs/Curie")
        .ok_or("published Curie definition missing")?;
    let validator = jsonschema::draft202012::new(curie)?;
    for (value, accepted) in [
        ("DepMap:ACH-000362", true),
        ("HGNC:3765", true),
        ("HGNC:HGNC:7132", true),
        ("prefix_1.-:α/β", true),
        ("DepMap:ACH\u{007f}1", false),
        ("DepMap:ACH\u{0080}1", false),
        ("DepMap:ACH\u{009f}1", false),
        ("DepMap:ACH\t1", false),
        ("DepMap:ACH\u{00a0}1", false),
        ("DepMap:ACH\n1", false),
        ("DepMap:ACH1\n", false),
        ("DepMap:ACH\u{0000}1", false),
        ("DepMap:", false),
        (":ACH1", false),
        ("DepMap ACH1", false),
        ("préfix:ACH1", false),
    ] {
        assert_eq!(
            value.parse::<Curie>().is_ok(),
            accepted,
            "runtime: {value:?}"
        );
        assert_eq!(
            validator.is_valid(&json!(value)),
            accepted,
            "schema: {value:?}"
        );
    }
    Ok(())
}
