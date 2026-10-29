//! Public schema contract tests; file I/O stays outside the pure library.
#![cfg(feature = "schema")]
#[path = "schemas/annotations.rs"]
mod annotations;
#[path = "schemas/compatibility.rs"]
mod compatibility;
#[path = "schemas/compatibility_cases.rs"]
mod compatibility_cases;
#[path = "schemas/drug.rs"]
mod drug;
#[path = "schemas/generation.rs"]
mod generation;
#[path = "schemas/ownership.rs"]
mod ownership;
#[path = "schemas/policy.rs"]
mod policy;
#[path = "schemas/release.rs"]
mod release;
#[path = "schemas/values.rs"]
mod values;

use biodata::*;
use biomcp_types::*;
use schemars::{JsonSchema, generate::SchemaSettings};
use serde_json::Value;
use std::{fs, path::Path};

struct Contract {
    kind: crate::policy::ContractKind,
    name: &'static str,
    id: &'static str,
    rust_type: &'static str,
    generate: fn() -> Value,
}

#[expect(
    clippy::expect_used,
    reason = "Schema test setup must serialize a generated schema"
)]
fn generate<T: JsonSchema>() -> Value {
    serde_json::to_value(
        SchemaSettings::draft2020_12()
            .into_generator()
            .into_root_schema_for::<T>(),
    )
    .expect("schema serializes")
}

macro_rules! contracts {
        ($($ty:ty => ($name:literal, $id:expr $(, $kind:ident)?)),+ $(,)?) => {
            vec![$(Contract {
                kind: contracts!(@kind $($kind)?),
                name: $name,
                id: $id,
                rust_type: std::any::type_name::<$ty>(),
                generate: contracts!(@generate $ty $(, $kind)?),
            }),+]
        };
        (@generate $ty:ty, query) => { crate::generation::query::<$ty> };
        (@generate $ty:ty) => { crate::generation::reply::<$ty> };
        (@kind query) => { crate::policy::ContractKind::Query };
        (@kind) => { crate::policy::ContractKind::Reply };
    }

macro_rules! reply_contracts {
    ($($body:ty => $name:literal),+ $(,)?) => {
        vec![$(Contract {
            kind: policy::ContractKind::Reply,
            name: $name,
            id: <$body as ResponseBody>::SCHEMA,
            rust_type: std::any::type_name::<Envelope<$body>>(),
            generate: generation::reply::<Envelope<$body>>,
        }),+]
    };
}

#[path = "schemas/disease.rs"]
mod disease;

#[path = "schemas/registry.rs"]
mod registry;
use registry::{common_registry, research_registry};

fn article_registry() -> Vec<Contract> {
    contracts! {
        ArticleEvidence => ("article-evidence", "biomcp.article-evidence/3", query),
        ArticleRecommend => ("article-recommend", "biomcp.article-recommend/3", query),
        ArticlePdf => ("article-pdf", "biomcp.article-pdf/3", query),
        ArticleSupplement => ("article-supplement", "biomcp.article-supplement/3", query),
        ArticleText => ("article-text", "biomcp.article-text/4", query),
        ArticleEdges => ("article-edges", "biomcp.article-edges/4", query),
        ArticleGet => ("article-get", "biomcp.article-get/3", query),
        Article => ("article", "biomcp.article/4"),
        ArticleSearch => ("article-search", "biomcp.article-search/3", query),
        ArticleHeader => ("article-header", "biomcp.article-header/5"),
        EuropePmcId => ("europepmc-id", "biodata.europepmc-id/2"),
        Pmid => ("pmid", "biodata.pmid/2"),
        Pmcid => ("pmcid", "biodata.pmcid/2"),
        Doi => ("doi", "biodata.doi/2"),
    }
}

#[path = "schemas/cell_line.rs"]
mod cell_line;
#[path = "schemas/phenotype.rs"]
mod phenotype;

fn nci_discovery_contracts() -> Vec<Contract> {
    contracts! {
        BiomarkerSearch => ("biomarker-search", "biomcp.biomarker-search/2", query),
        BiomarkerGet => ("biomarker-get", "biomcp.biomarker-get/1", query),
        OrganizationSearch => ("organization-search", "biomcp.organization-search/2", query),
    }
}

fn registry() -> Vec<Contract> {
    let mut contracts = contracts! {
        TrialDocumentGet => ("trial-document", "biomcp.trial-document/2", query),
        HttpHealth => ("http-health", "biomcp.http-health/2"),
        Markdown => ("markdown", "biodata.markdown/2"),
        ErrorEnvelope => ("error", "biomcp.error/3"),
        GeneGet => ("gene-get", "biomcp.gene-get/2", query),
        GeneSearch => ("gene-search", "biomcp.gene-search/2", query),
        Request => ("request", "biomcp.request/30", query),
        HgncId => ("hgnc-id", "biodata.hgnc-id/2"),
        PartialDate => ("partial-date", "biodata.partial-date/2"),
        Position1 => ("position1", "biodata.position1/1"),
        Span0 => ("span0", "biodata.span0/1"),
        Text => ("text", "biodata.text/2"),
        Source => ("source", "biomcp.source/1"),
        Reason => ("reason", "biomcp.reason/2"),
        ProblemCode => ("problem-code", "biomcp.problem-code/1"),
        Problem => ("problem", "biomcp.problem/2"),
        GeneInput => ("gene-input", "biomcp.gene-input/2", query),
        Gene => ("gene", "biodata.gene/6"),
        GeneHeader => ("gene-header", "biomcp.gene-header/5"),
        GeneResolution => ("gene-resolution", "biomcp.gene-resolution/6"),
        NcbiGeneId => ("ncbi-gene-id", "biodata.ncbi-gene-id/2"),
        EnsemblGeneId => ("ensembl-gene-id", "biodata.ensembl-gene-id/2"),
        GeneSymbol => ("gene-symbol", "biodata.gene-symbol/2"),
        UniProtAcc => ("uniprot-acc", "biodata.uniprot-acc/1"),
        OmimId => ("omim-id", "biodata.omim-id/1"),
        AccessionBase => ("accession-base", "biodata.accession-base/2"),
        GeneRef => ("gene-ref", "biodata.gene-ref/3"),
        Cytoband => ("cytoband", "biodata.cytoband/1"),
        GeneLocation => ("gene-location", "biodata.gene-location/1"),
        GenomicInterval => ("genomic-interval", "biodata.genomic-interval/2"),
        GeneStatus => ("gene-status", "biodata.gene-status/1"),
        LocusGroup => ("locus-group", "biodata.locus-group/1"),
        LocusType => ("locus-type", "biodata.locus-type/1"),
        MatchKind => ("match-kind", "biodata.match-kind/1"),
        Match => ("gene-match", "biomcp.gene-match/2"),
        Mane => ("mane", "biodata.mane/1"),
        Summary => ("gene-summary", "biodata.gene-summary/3"),
        SummarySource => ("summary-source", "biodata.summary-source/1"),
        HgncStatus => ("hgnc-status", "biodata.hgnc-status/2"),
        Build => ("build", "biodata.build/2"),
        Strand => ("strand", "biodata.strand/1"),
    };
    contracts.extend(contracts! {
        GwasGetCommand => ("gwas-get", "biomcp.gwas-get/2", query),
        GwasSearchCommand => ("gwas-search", "biomcp.gwas-search/4", query),
    });
    contracts.extend(nci_discovery_contracts());
    contracts.extend(diagnostic_contracts());
    contracts.extend(pathway_contracts());
    contracts.extend(protein_contracts());
    contracts.extend(common_registry());
    contracts.extend(article_registry());
    contracts.extend(research_registry());
    contracts.extend(disease::registry());
    contracts.extend(phenotype::registry());
    contracts.extend(cell_line::registry());
    contracts.extend(author::registry());
    contracts.extend(study_registry());
    contracts.extend(adverse_event_contracts());
    contracts.extend(variant_contracts());
    contracts.extend(trial_contracts());
    contracts.extend(drug_contracts());
    contracts.extend(biomcp_types::reply_types!(reply_contracts));
    contracts
}
fn study_registry() -> Vec<Contract> {
    contracts! {
        StudySearch => ("study-search", "biomcp.study-search/2", query),
        StudyGet => ("study-get", "biomcp.study-get/1", query),
    }
}

fn adverse_event_contracts() -> Vec<Contract> {
    contracts! {
        PgxSearch => ("pgx-search", "biomcp.pgx-search/3", query),
        PgxGet => ("pgx-get", "biomcp.pgx-get/3", query),
        PgxGetCommand => ("pgx-get-command", "biomcp.pgx-get-command/3", query),
        PgxAnnotationGet => ("pgx-annotation-get", "biomcp.pgx-annotation-get/2", query),
        AdverseEventSearch => ("adverse-event-search", "biomcp.adverse-event-search/4", query),
        AdverseEventGet => ("adverse-event-get", "biomcp.adverse-event-get/2", query),
    }
}

fn variant_contracts() -> Vec<Contract> {
    contracts! {
        VariantAssertionsGet => ("variant-assertions", "biomcp.variant-assertions/4", query),
        PopulationFrequency => ("population-frequency", "biodata.population-frequency/7"),
        VariantPopulationQuery => ("variant-population", "biomcp.variant-population/4", query),
        Statement => ("statement", "biodata.statement/6"),
        VariantRecurrenceQuery => ("variant-recurrence-query", "biomcp.variant-recurrence-query/5", query),
        VariantNormalize => ("variant-normalize", "biomcp.variant-normalize/5", query),
        VariantSearch => ("variant-search", "biomcp.variant-search/4", query),
        VariantGet => ("variant-get", "biomcp.variant-get/5", query),
    }
}

fn drug_contracts() -> Vec<Contract> {
    contracts! {
        DrugId => ("drug-id", "biodata.drug-id/2"),
        DrugIdentityLevel => ("drug-identity-level", "biodata.drug-identity-level/1"),
        DrugIdentity => ("drug-identity", "biodata.drug-identity/3"),
        DrugIdentityRelation => ("drug-identity-relation", "biodata.drug-identity-relation/4"),
        DrugProduct => ("drug-product", "biodata.drug-product/4"),
        DrugLabelReference => ("drug-label-reference", "biodata.drug-label-reference/4"),
        DrugLabelSection => ("drug-label-section", "biodata.drug-label-section/4"),
        DrugNativeTarget => ("drug-native-target", "biodata.drug-native-target/2"),
        DrugDiseaseReference => ("drug-disease-reference", "biodata.drug-disease-reference/2"),
        DrugStage => ("drug-stage", "biodata.drug-stage/1"),
        DrugNativeReference => ("drug-native-reference", "biodata.drug-native-reference/2"),
        DrugTargetRelation => ("drug-target-relation", "biodata.drug-target-relation/5"),
        DrugIndicationClaim => ("drug-indication-claim", "biodata.drug-indication-claim/5"),
        DrugInteractionClaim => ("drug-interaction-claim", "biodata.drug-interaction-claim/5"),
        AuthorizationDate => ("authorization-date", "biodata.authorization-date/3"),
        AuthorizationEvent => ("authorization-event", "biodata.authorization-event/4"),
        DrugNameSelector => ("drug-name-selector", "biomcp.drug-name-selector/1", query),
        DrugSearch => ("drug-search", "biomcp.drug-search/3", query),
        DrugGet => ("drug-get", "biomcp.drug-get/3", query),
        DrugWhoGet => ("drug-who-get", "biomcp.drug-who-get/2", query),
        DrugGetCommand => ("drug-get-command", "biomcp.drug-get-command/2", query),
        DrugWhoRow => ("drug-who-row", "biomcp.drug-who-row/2"),
        DrugSource => ("drug-source", "biomcp.drug-source/1"),
        DrugSection => ("drug-section", "biomcp.drug-section/1"),
        DrugHeader => ("drug-header", "biomcp.drug-header/3"),
        DrugRecallHeader => ("drug-recall-header", "biomcp.drug-recall-header/2"),
    }
}

fn diagnostic_contracts() -> Vec<Contract> {
    contracts! {
        DiagnosticSearch => ("diagnostic-search", "biomcp.diagnostic-search/3", query),
        DiagnosticGet => ("diagnostic-get", "biomcp.diagnostic-get/2", query),
    }
}

fn trial_contracts() -> Vec<Contract> {
    contracts! {
        TrialGet => ("trial-get", "biomcp.trial-get/2", query),
        TrialConditionResolution => ("trial-condition-resolution", "biomcp.trial-condition-resolution/4"),
        TrialSearch => ("trial-search", "biomcp.trial-search/5", query),
        NctId => ("nct-id", "biodata.nct-id/1"),
        TrialHeader => ("trial-header", "biomcp.trial-header/4"),
    }
}

#[test]
fn checked_in_schema_definition_names_have_one_body() -> Result<(), Box<dyn std::error::Error>> {
    release::checked_in_schema_definition_names_have_one_body()
}

#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "Schema contract tests own file and environment I/O outside the pure library"
)]
fn checked_in_schemas_match_types() {
    let update = std::env::var_os("UPDATE_SCHEMAS").is_some_and(|value| value == "1");
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas");
    let reviews = policy::read_reviews().expect("read reviewed schema transitions");
    // Collect and check the whole registry before writing any contract.
    let pending: Vec<_> = registry()
        .into_iter()
        .map(|contract| {
            let mut new = (contract.generate)();
            new["$id"] = contract.id.into();
            compatibility::check_reviewed(&new, &new, contract.kind, &reviews)
                .unwrap_or_else(|error| panic!("{}: {error}", contract.name));
            let path = root.join(format!("{}.json", contract.name));
            if path.exists() {
                let old: Value =
                    serde_json::from_str(&fs::read_to_string(&path).expect("read contract"))
                        .expect("parse contract");
                compatibility::check_reviewed(&old, &new, contract.kind, &reviews)
                    .unwrap_or_else(|error| panic!("{}: {error}", contract.name));
            } else {
                assert!(update, "missing {}", path.display());
            }
            let bytes = format!(
                "{}\n",
                serde_json::to_string_pretty(&new).expect("serialize schema")
            );
            (path, bytes)
        })
        .collect();
    if update {
        fs::create_dir_all(&root).expect("create schema directory");
    }
    for (path, bytes) in pending {
        if update {
            fs::write(&path, bytes).expect("write contract");
        } else {
            assert_eq!(
                fs::read_to_string(&path).expect("read contract"),
                bytes,
                "schema drift at {}; review compatibility before UPDATE_SCHEMAS=1",
                path.display()
            );
        }
    }
}

#[test]
fn consumed_queries_stay_inside_the_flat_command_grammar() {
    for schema in [
        generate::<BiomarkerSearch>(),
        generate::<BiomarkerGet>(),
        generate::<StudySearch>(),
        generate::<StudyGet>(),
        generate::<GeneGet>(),
        generate::<GeneSearch>(),
        generate::<ArticleSearch>(),
    ] {
        assert_eq!(schema["type"], "object");
        assert!(schema.get("oneOf").is_none());
        if schema.get("anyOf").is_some() {
            assert!(
                compatibility::selectors(&schema)
                    .expect("selector union")
                    .is_some()
            );
        }
        for property in schema["properties"]
            .as_object()
            .expect("query properties")
            .values()
        {
            flat_value(&schema, property);
        }
    }
}
#[expect(
    clippy::expect_used,
    reason = "The grammar contract must fail on unresolved generated schema references"
)]
fn flat_value(root: &Value, node: &Value) {
    if let Some(reference) = node.get("$ref").and_then(Value::as_str) {
        flat_value(
            root,
            root.pointer(reference.strip_prefix('#').expect("local reference"))
                .expect("resolved reference"),
        );
    } else if let Some(branches) = node.get("anyOf").and_then(Value::as_array) {
        assert_eq!(
            branches.len(),
            2,
            "Query schemas must stay inside the supported flat command grammar"
        );
        assert_eq!(
            branches
                .iter()
                .filter(|branch| branch["type"] == "null")
                .count(),
            1,
            "Query schemas must stay inside the supported flat command grammar"
        );
        for branch in branches.iter().filter(|branch| branch["type"] != "null") {
            flat_value(root, branch);
        }
    } else if node["type"] == "array" {
        let items = node.get("items").expect("array items");
        assert_ne!(
            items["type"], "array",
            "Query schemas must stay inside the supported flat command grammar"
        );
        flat_value(root, items);
    } else {
        assert!(
            ["string", "integer", "boolean"].contains(&node["type"].as_str().expect("scalar type")),
            "Query schemas must stay inside the supported flat command grammar"
        );
    }
}

fn pathway_contracts() -> Vec<Contract> {
    contracts! {
        PathwaySearch => ("pathway-search", "biomcp.pathway-search/4", query),
        PathwayGet => ("pathway-get", "biomcp.pathway-get/3", query),
        PathwayGenesQuery => ("pathway-genes-query", "biomcp.pathway-genes-query/3", query),
        PathwayEventsQuery => ("pathway-events-query", "biomcp.pathway-events-query/2", query),
    }
}

fn protein_contracts() -> Vec<Contract> {
    contracts! {
        ProteinFeaturesQuery => ("protein-features-query", "biomcp.protein-features-query/2", query),
        ProteinRelationQuery => ("protein-relation-query", "biomcp.protein-relation-query/2", query),
        ProteinDomainsQuery => ("protein-domains-query", "biomcp.protein-domains-query/2", query),
        ProteinComplexMembersQuery => ("protein-complex-members-query", "biomcp.protein-complex-members-query/2", query),
        ProteinSequenceQuery => ("protein-sequence-query", "biomcp.protein-sequence-query/1", query),
        ProteinSearch => ("protein-search", "biomcp.protein-search/2", query),
        ProteinGet => ("protein-get", "biomcp.protein-get/2", query),
    }
}

#[path = "schemas/author.rs"]
mod author;

#[test]
fn generated_schemas_preserve_release_contracts() -> Result<(), Box<dyn std::error::Error>> {
    release::generated_schemas_preserve_release_contracts()
}

#[test]
fn published_reply_contracts_are_open_and_query_choices_stay_closed() {
    for contract in registry() {
        let mut schema = (contract.generate)();
        schema["$id"] = contract.id.into();
        compatibility::check_reviewed(&schema, &schema, contract.kind, &policy::Reviews::default())
            .unwrap_or_else(|error| panic!("{}: {error}", contract.name));
        match contract.kind {
            policy::ContractKind::Reply => assert_open(&schema, contract.name),
            policy::ContractKind::Query => {
                assert!(
                    schema.get("x-extensible-enum").is_none(),
                    "{}",
                    contract.name
                );
            }
        }
    }
    for (schema, expected) in [
        (generation::reply::<biomcp_types::Source>(), "hgnc"),
        (generation::reply::<biomcp_types::Reason>(), "not-found"),
        (generation::reply::<biomcp_types::DrugSource>(), "chembl"),
        (generation::reply::<biodata::Proposition>(), "pathogenicity"),
        (generation::reply::<biodata::Predicate>(), "is_causal_for"),
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
    let drug = generation::query::<biomcp_types::DrugGet>();
    let validator = jsonschema::draft202012::new(&drug).unwrap();
    assert!(validator.is_valid(&serde_json::json!({"input":"CHEMBL941","source":"chembl"})));
    assert!(
        !validator.is_valid(&serde_json::json!({"input":"CHEMBL941","source":"future_source"}))
    );
    let schema = generation::query::<biomcp_types::GeneGet>();
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

#[test]
fn population_frequencies_share_one_bounded_fraction() {
    let schema = generate::<biodata::PopulationFrequency>();
    for field in ["allele_frequency", "faf95"] {
        assert_eq!(
            schema["properties"][field]["anyOf"][0]["$ref"], "#/$defs/Fraction",
            "{field} uses Fraction"
        );
    }
    assert_eq!(schema["$defs"]["Fraction"]["minimum"], 0.0);
    assert_eq!(schema["$defs"]["Fraction"]["maximum"], 1.0);
}

#[test]
fn source_forms_and_profile_members_match_admitted_subject_levels()
-> Result<(), Box<dyn std::error::Error>> {
    let validator = jsonschema::draft202012::new(&generate::<VariantRef>())?;
    let allele = serde_json::json!({"level":"allele", "id":"VCV000013961.3"});
    let protein = serde_json::json!({"level":"protein", "gene":{"label":"EGFR"}, "label":"source label"});
    let source = |forms| {
        serde_json::json!({
            "level":"source", "record":"civic:1", "feature":"native feature", "forms":forms
        })
    };
    let profile = |members| {
        serde_json::json!({
            "level":"profile", "record":"1", "label":"native profile", "members":members
        })
    };
    for (value, valid) in [
        (source(vec![allele.clone()]), true),
        (source(vec![protein.clone()]), true),
        (profile(vec![allele.clone(), protein, source(vec![])]), true),
        (source(vec![source(vec![])]), false),
        (source(vec![profile(vec![])]), false),
        (profile(vec![profile(vec![])]), false),
        (profile(vec![source(vec![source(vec![allele])])]), false),
    ] {
        assert_eq!(validator.is_valid(&value), valid, "schema: {value}");
        assert_eq!(
            serde_json::from_value::<VariantRef>(value.clone()).is_ok(),
            valid,
            "admission: {value}"
        );
    }
    Ok(())
}
