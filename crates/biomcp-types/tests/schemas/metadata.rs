//! Generated schema metadata stays attached to the owning public types.
use super::{generate, registry};
use biodata::PartialDate;
use biomcp_types::ErrorEnvelope;

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
    assert_eq!(date["pattern"], r"^[0-9]{4}(-[0-9]{2}(-[0-9]{2})?)?$");
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
