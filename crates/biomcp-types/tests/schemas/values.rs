use super::{generate, registry};
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
    assert!(!id.is_valid(&overflow));
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
    let base = json!({"title": "A checked article", "authors": []});
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

#[test]
fn every_generated_schema_describes_its_root() {
    for contract in registry() {
        let schema = (contract.generate)();
        assert!(
            schema["description"]
                .as_str()
                .is_some_and(|text| !text.trim().is_empty()),
            "{} needs a root description",
            contract.name
        );
    }
}

#[test]
fn generated_metadata_preserves_partial_dates_and_describes_error_codes() {
    let date = generate::<PartialDate>();
    assert_eq!(date["pattern"], PartialDate::pattern());
    assert!(date.get("format").is_none());
    let full_dates = generate::<biomcp_types::AdverseEventSearch>();
    for field in ["received_from", "received_to"] {
        assert_eq!(full_dates["properties"][field]["format"], "date");
    }
    let timestamp = generate::<biodata::StudyImportTime>();
    assert!(
        timestamp.get("format").is_none(),
        "a timezone-free native timestamp is not RFC 3339"
    );
    let error = generate::<ErrorEnvelope>();
    assert_eq!(
        error["$defs"]["ErrorCode"]["description"],
        "Stable error codes for command usage, local failures and expired results."
    );
}

const LINKS: &[(&str, &[(&str, &str)])] = &[
    ("VariantComponent", &[("license", "uri")]),
    ("StudyFiles", &[("license_notice", "uri")]),
    (
        "AdverseEventFaersCard",
        &[("source_url", "uri"), ("report_url", "uri")],
    ),
    (
        "AdverseEventMaudeCard",
        &[("source_url", "uri"), ("report_url", "uri")],
    ),
    ("ArticleSaved", &[("url", "uri")]),
    ("Attribution", &[("url", "uri")]),
    ("AuthorCard", &[("url", "uri")]),
    ("AuthorPaperAccess", &[("url", "uri")]),
    ("CellLineCard", &[("url", "uri")]),
    ("CellLineXref", &[("url", "uri")]),
    ("CivicGene", &[("url", "uri")]),
    ("ClinGenValidity", &[("report_url", "uri")]),
    ("ComponentTerms", &[("url", "uri")]),
    ("DiagnosticCellLink", &[("href", "uri-reference")]),
    ("DiagnosticDrugApplication", &[("url", "uri")]),
    ("DiagnosticPmaHeader", &[("url", "uri")]),
    ("DiseaseAssociationReference", &[("url", "uri")]),
    ("DrugFdaDocument", &[("url", "uri")]),
    ("DrugHeader", &[("url", "uri")]),
    ("DrugIdentity", &[("url", "uri")]),
    ("DrugLabelReference", &[("url", "uri")]),
    ("DrugNativeReference", &[("ref_url", "uri")]),
    ("DrugNativeTarget", &[("url", "uri")]),
    ("ErrorDetail", &[("url", "uri")]),
    ("GtrHeader", &[("url", "uri")]),
    ("GwasAncestry", &[("source_url", "uri")]),
    ("GwasLocus", &[("source_url", "uri")]),
    (
        "GwasStudy",
        &[("summary_statistics_url", "uri"), ("license_url", "uri")],
    ),
    ("OncokbCitation", &[("link", "uri")]),
    ("OrcidExternalId", &[("url", "uri")]),
    ("OrcidWorkAssertion", &[("url", "uri")]),
    ("PathwayLiteratureRef", &[("url", "uri")]),
    ("PathwayMember", &[("source_url", "uri")]),
    ("PdfLink", &[("url", "uri")]),
    ("PgxCrossReference", &[("_url", "uri")]),
    ("PgxGuidelineGene", &[("url", "uri")]),
    ("PgxGuidelineReference", &[("guideline_url", "uri")]),
    ("PgxPair", &[("guidelineurl", "uri")]),
    ("PgxRecommendation", &[("guidelineurl", "uri")]),
    ("ProteinComplexHeader", &[("url", "uri")]),
    ("ProteinHeader", &[("url", "uri")]),
    ("ProteinStructureRef", &[("url", "uri")]),
    ("RecurrenceTerms", &[("url", "uri")]),
    ("ResearchFetch", &[("url", "uri")]),
    ("ResearchHit", &[("url", "uri")]),
    ("SourceHealth", &[("terms_url", "uri")]),
    ("SourceOutcome", &[("terms_url", "uri")]),
    ("SourceRecord", &[("url", "uri")]),
    ("StudyCard", &[("source_url", "uri")]),
    ("SupplementContent", &[("url", "uri")]),
    ("TextContent", &[("url", "uri")]),
    ("TrialCard", &[("url", "uri")]),
    ("TrialDocument", &[("url", "uri")]),
    ("TrialHeader", &[("url", "uri")]),
    ("StudyFile", &[("htmlUrl", "uri"), ("downloadUrl", "uri")]),
    ("OncokbVariantRecord", &[("link", "uri")]),
    ("PathwayEventRef", &[("source_url", "uri")]),
];

#[test]
fn generated_links_expose_uri_metadata() {
    // These are source-owned record links, not fields guessed from their spelling.
    let schemas: Vec<_> = registry()
        .into_iter()
        .map(|contract| (contract.generate)())
        .collect();
    for (owner, fields) in LINKS {
        for (field, format) in *fields {
            let mut seen = false;
            for schema in &schemas {
                let node = if schema["title"] == *owner {
                    Some(schema)
                } else {
                    schema
                        .get("$defs")
                        .and_then(|definitions| definitions.get(*owner))
                };
                if let Some(node) = node {
                    for branch in std::iter::once(node).chain(
                        node.get("oneOf")
                            .and_then(serde_json::Value::as_array)
                            .into_iter()
                            .flatten(),
                    ) {
                        if let Some(property) = branch
                            .get("properties")
                            .and_then(|properties| properties.get(*field))
                        {
                            seen = true;
                            assert_eq!(property["format"], *format, "{owner}.{field}");
                        }
                    }
                }
            }
            assert!(seen, "missing generated link {owner}.{field}");
        }
    }
}

// Compare admission at the public parser and schema boundaries, including near misses.
fn text_admissions<T: JsonSchema + std::str::FromStr>(examples: &[&str]) {
    let schema = validator::<T>();
    for text in examples {
        assert_eq!(
            schema.is_valid(&json!(text)),
            text.parse::<T>().is_ok(),
            "{}: {text:?}",
            std::any::type_name::<T>()
        );
    }
}

#[test]
fn text_value_patterns_match_parser_admissions() {
    macro_rules! cases {
        ($($ty:ty => [$($text:expr),* $(,)?]),* $(,)?) => {$(
            text_admissions::<$ty>(&[$($text),*]);
        )*};
    }
    cases! {
        HgncId => ["HGNC:1", "HGNC:0", "HGNC:4294967295", "HGNC:4294967296"],
        NcbiGeneId => ["1", "01", "4294967295", "4294967296"],
        Pmid => ["1", "0", "4294967295", "4294967296"],
        Pmcid => ["PMC1", "PMC0", "PMC4294967295", "PMC4294967296"],
        Doi => ["10.1/a", "10.1/a\u{0080}", "10.1/a\u{009f}", "10.1/α", "10.1/a\n", "10.1/a\u{feff}", "10.1/a\u{0085}"],
        EuropePmcId => ["PPR0001", "PPR0000", "ABCDEFGH1", "ABCDEFGHI1", &format!("PPR{}", "1".repeat(61)), &format!("PPR{}", "1".repeat(62))],
        SemanticScholarId => ["0123456789abcdef0123456789ABCDEF0123456789", "0123456789abcdef0123456789abcdef012345678g"],
        GeneSymbol => ["BRAF", "123", "A/B@c", &"A".repeat(255), &"A".repeat(256)],
        OmimId => ["000001", "00000", "0000000"],
        UniProtAcc => ["P15056", "A0A023GPI8", "p15056", "P15056-1"],
        Cytoband => ["7q31.2", "7q01.2", "23q31.2", "7q3"],
        EnsemblGeneId => ["ENSG00000000001", "ENSG00000000001.1", "ENSG00000000001.0", "ENSG00000000001.4294967295", "ENSG00000000001.4294967296"],
        CaId => ["CA000001", "CA1", "CA000000", "CA0000001", "CA1000000", "CA18446744073709551615", "CA18446744073709551616"],
        PaId => ["PA1", "PA0", "PA01", "PA18446744073709551615", "PA18446744073709551616"],
        RsId => ["rs1", "rs0", "rs01", "rs18446744073709551615", "rs18446744073709551616"],
        ClinVarVariationId => ["clinvar:1", "clinvar:0", "clinvar:18446744073709551615", "clinvar:18446744073709551616"],
        ClinVarAlleleId => ["1", "0", "01", "18446744073709551615", "18446744073709551616"],
        VcvId => ["VCV000000001", "VCV000000000", "VCV000000001.0", "VCV000000001.1"],
        RcvId => ["RCV000000001", "RCV000000000", "RCV000000001.01"],
        ScvId => ["SCV000000001", "SCV000000000", "SCV000000001.1"],
        CosmicId => ["COSV1", "COSM1", "COSN1", "COSV0", "COSV01"],
        VrsId => ["ga4gh:VA.012345678901234567890123456789_-", "ga4gh:VA.0123456789012345678901234567890+"],
        VariantId => ["CA000001", "CA000000", "VCV000000001", "VCV000000000", "rs18446744073709551616"],
        Accession => ["NM_004333.6", "NM_004333.0", "NM_004333.65535", "NM_004333.65536", "NM_000004333.1", "NM_001004333.1", "LRG_0001t01", "LRG_4294967295p65535", "LRG_4294967296", "LRG_1p65536"],
        AccessionBase => ["NM_004333", "NM_000004333", "NM_001004333", "LRG_4294967296", "ENST00000000001"],
        HgvsText => ["NM_004333.6:c.1A>G", "NM_004333.0:c.1A>G", "NM_004333.6(123):c.1A>G", "NM_004333.6(BRAF):c.1A>G", "NP_004324.1:c.1A>G", "NP_004324.1:p.?", "NC_000007.14(NP_004324.1):g.1A>G", "NC_000007.14(NP_004324.0):g.1A>G", "NM_004333.65536:c.1A>G"],
        Base => ["A", "N", "a", "U", "AA"],
        Sequence => ["", "ACGTRYSWKMBDHVN", "a", "U", "AC GT"],
        Chromosome => ["1", "22", "23", "X", "MT", "M", "chr1", "01"],
        Contig => ["chr1", "chr 1", &"x".repeat(255), &"x".repeat(256)],
        Curie => ["HGNC:1", "A:α", "A:", "A:a\u{0080}", "A:a\u{feff}"],
        HpId => ["HP:0000001", "HP:000001", "HP:00000001"],
        MondoId => ["MONDO:0000001", "MONDO:000001", "MONDO:00000001"],
        NcitId => ["C1", "C01", "C", "C1A"],
        NctId => ["NCT00000001", "NCT00000000", "NCT000000001"],
        ReportId => ["0", "0001", "", "FAERS1"],
        DrugId => ["CHEMBL1", "DB:1", "db:1", "DB:1:2", "DB:1/2", "DB:1//2"],
        GwasStudyId => ["GCST000001", "GCST000000", "GCST00001", "GCST0000001"],
        GwasAssociationId => ["association:1", "association:0", "association:18446744073709551615", "association:18446744073709551616"],
        GwasTraitId => ["EFO_0001", "HP:0000001", "HP:", "_0001"],
        ReactomeId => ["R-HSA-1", "R-HSA-0", "R-HSA-01", "R-MMU-1"],
        WikiPathwaysId => ["WP1", "WP0", "WP01", "WP1.1"],
        UniProtIsoform => ["P15056", "P15056-1", "P15056-0", "P15056-01"],
        AminoAcids => ["ACDEFGHIKLMNPQRSTVWYUOX", "", "B", "a"],
        InterProId => ["IPR000001", "IPR00001", "IPR0000001"],
        ComplexPortalId => ["CPX-1", "CPX-0", "CPX-01"],
        StudyId => ["study_1", "study/1", "_study"],
        StudyProfileId => ["study.1", "study/1", "_study"],
        StudySampleListId => ["study-1", "study/1", "_study"],
        CbioportalCancerTypeId => ["type_1", "type/1", "_type"],
        StudyResourceId => ["resource-1", "resource/1", "_resource"],
        StudyBlobSha => ["0123456789abcdef0123456789abcdef0123456789", "0123456789ABCDEF0123456789ABCDEF0123456789"],
        StudyImportTime => ["2024-02-29 23:59:59", "0000-01-01 00:00:00", "2024-13-01 00:00:00", "2024-01-32 00:00:00", "2024-01-01 24:00:00"],
        PartialDate => ["0001", "0000", "2024-02", "2024-13", "2024-01-32"],
        CellosaurusAccession => ["CVCL_0001", "CVCL_AA01", "CVCL_00001", "CVCL_aa01"],
        CellLineSourceId => ["DepMap:ACH-1", "DepMap:", "A:a\u{009f}", "A:a\u{feff}"],
        ChemblCellId => ["CHEMBL1", "CHEMBL01", "CHEMBL", "CHEMBL1A"],
        GtrTestId => ["GTR000000001.1", "GTR000000001.0", "GTR000000001.01"],
        WhoProductId => ["1234-123-12", "12345-12345-12", "123-123-12"],
        PmaNumber => ["P000001", "P00001", "P0000001"],
        PmaSupplement => ["S001", "S000", "S01", "S1000"],
        PmaSupplementCell => ["", "S001", "S000", "S01"],
        FdaDrugApplication => ["NDA000001", "BLA000001", "NDA00001", "NDA0000001"],
        DiagnosticDeviceId => ["P000001/S001", "P000001/S000", "K000001", "DEN000001", "H000001"],
        ClinPgxAccession => ["PA1", "PA0", "PA01"],
        PgxGeneKey => ["BRAF", "123", &"A".repeat(255), &"A".repeat(256)],
        PgxDiplotypeExpression => [" *1/*2 ", " ", "\u{0085}", "\u{feff}"],
        SemanticScholarAuthorId => ["1", "0", "01", "A"],
        OrcidId => ["0000-0002-1825-0097", "0000-0002-1825-009", "0000-0002-1825-00x7"],
        AuthorId => ["orcid:0000-0002-1825-0097", "semantic-scholar:1", "semanticscholar:1", "semantic-scholar:0"],
        OrcidClientId => ["0000-0002-1825-0097", "APP-0123456789ABCDEF", "APP-0123456789abcDEF"],
        ArticleInput => ["1", "PMC1", "PPR0001", "PPR0000", "4294967296", "PMC4294967296", "10.1/a\u{0080}", "https://doi.org/10.1/a"],
        GeneInput => ["BRAF", " HGNC:1 ", "HGNC:0", "hgnc:1", "ENSGbad", "0001", "4294967296", "Gene alias"],
        VariantInput => ["CA000001", "CA000000", "VCV000000000", "chr22:g.1A>G", "chr23:g.1A>G", "18446744073709551615", "18446744073709551616"],
        SheetRows => ["1-1", "01-1", "1-4294967296"],
        VariantRegion => ["chr1:1-2", "chr23:1-2", "chr1:01-2", "chr1:1-4294967296"],
        MyVariantQuery => [" query ", " ", "a\u{0080}", "\u{0085}", "\u{feff}", &"a".repeat(4102), &"a".repeat(4103)],
    }
}

// These semantic constraints intentionally remain in the parser.
#[test]
fn text_value_schema_supersets_name_their_runtime_rules() {
    fn runtime_rule<T: JsonSchema + std::str::FromStr>(text: &str, rule: &str) {
        let schema = generate::<T>();
        assert!(schema["description"].as_str().unwrap().contains(rule));
        assert!(validator::<T>().is_valid(&json!(text)), "schema: {text}");
        assert!(text.parse::<T>().is_err(), "parser: {text}");
    }
    runtime_rule::<OrcidId>("0000-0002-1825-0098", "checksum");
    runtime_rule::<AuthorId>("orcid:0000-0002-1825-0098", "checksum");
    runtime_rule::<OrcidClientId>("0000-0002-1825-0098", "checksum");
    runtime_rule::<PartialDate>("2023-02-29", "calendar");
    runtime_rule::<StudyImportTime>("2023-02-29 00:00:00", "calendar");
    runtime_rule::<SheetRows>("2-1", "end");
    runtime_rule::<SheetRows>("1-201", "200");
    runtime_rule::<VariantRegion>("chr1:2-1", "end");
    runtime_rule::<MyVariantQuery>(&format!("a{}", "α".repeat(2051)), "UTF-8 byte");
}
