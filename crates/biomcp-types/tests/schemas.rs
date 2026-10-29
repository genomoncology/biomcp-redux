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

#[path = "schemas/disease.rs"]
mod disease;

#[path = "schemas/registry.rs"]
mod registry;
use registry::{common_registry, research_registry};

fn article_registry() -> Vec<Contract> {
    contracts! {
        ArticleEvidence => ("article-evidence", "biomcp.article-evidence/1", query),
        ArticleRecommend => ("article-recommend", "biomcp.article-recommend/1", query),
        Envelope<ArticleEvidenceResult> => ("article-evidence-result", "biomcp.article-evidence/3"),
        Envelope<ArticleRecommendations> => ("article-recommendations", "biomcp.article-recommendations/5"),
        ArticlePdf => ("article-pdf", "biomcp.article-pdf/1", query),
        Envelope<ArticlePdfResult> => ("article-pdf-result", "biomcp.article-pdf/3"),
        ArticleSupplement => ("article-supplement", "biomcp.article-supplement/1", query),
        Envelope<ArticleSupplementResult> => ("article-supplement-result", "biomcp.article-supplement/4"),
        ArticleText => ("article-text", "biomcp.article-text/2", query),
        Envelope<ArticleTextResult> => ("article-text-result", "biomcp.article-text/4"),
        ArticleEdges => ("article-edges", "biomcp.article-edges/2", query),
        Envelope<ArticleEdgePage> => ("article-edge-page", "biomcp.article-edges/6"),
        ArticleGet => ("article-get", "biomcp.article-get/2", query),
        Article => ("article", "biomcp.article/3"),
        Envelope<ArticleCards> => ("article-cards", "biomcp.article-cards/5"),
        ArticleSearch => ("article-search", "biomcp.article-search/2", query),
        ArticleHeader => ("article-header", "biomcp.article-header/4"),
        Envelope<ArticlePage> => ("article-page", "biomcp.article-page/5"),
        EuropePmcId => ("europepmc-id", "biodata.europepmc-id/1"),
        Pmid => ("pmid", "biodata.pmid/1"),
        Pmcid => ("pmcid", "biodata.pmcid/1"),
        Doi => ("doi", "biodata.doi/1"),
    }
}

#[path = "schemas/cell_line.rs"]
mod cell_line;
#[path = "schemas/phenotype.rs"]
mod phenotype;

fn nci_discovery_contracts() -> Vec<Contract> {
    contracts! {
        BiomarkerSearch => ("biomarker-search", "biomcp.biomarker-search/1", query),
        BiomarkerGet => ("biomarker-get", "biomcp.biomarker-get/1", query),
        Envelope<BiomarkerPage> => ("biomarker-page", "biomcp.biomarker-page/2"),
        Envelope<BiomarkerResult> => ("biomarker-result", "biomcp.biomarker-result/2"),
        OrganizationSearch => ("organization-search", "biomcp.organization-search/1", query),
        Envelope<OrganizationPage> => ("organization-page", "biomcp.organization-page/2"),
    }
}

fn registry() -> Vec<Contract> {
    let mut contracts = contracts! {
        TrialDocumentGet => ("trial-document", "biomcp.trial-document/1", query),
        Envelope<TrialDocumentResult> => ("trial-document-result", "biomcp.trial-document/4"),
        HttpHealth => ("http-health", "biomcp.http-health/1"),
        Envelope<Health> => ("health", "biomcp.health/4"),
        Markdown => ("markdown", "biodata.markdown/1"),
        Envelope<SchemaCatalog> => ("schema-catalog", "biomcp.schema-catalog/2"),
        Envelope<Help> => ("help", "biomcp.help/4"),
        Envelope<KeyHelp> => ("key", "biomcp.key/3"),
        Envelope<RegisteredKeys> => ("registered-keys", "biomcp.registered-keys/3"),
        ErrorEnvelope => ("error", "biomcp.error/2"),
        GeneGet => ("gene-get", "biomcp.gene-get/1", query),
        GeneSearch => ("gene-search", "biomcp.gene-search/1", query),
        Request => ("request", "biomcp.request/29", query),
        Envelope<GeneCards> => ("gene-cards", "biomcp.gene-cards/8"),
        Envelope<GenePage> => ("gene-page", "biomcp.gene-page/5"),
        Envelope<StoreClean> => ("store-clean", "biomcp.store-clean/2"),
        Envelope<SyncResult> => ("sync", "biomcp.sync/6"),
        HgncId => ("hgnc_id", "biodata.hgnc-id/1"),
        PartialDate => ("partial-date", "biodata.partial-date/1"),
        Position1 => ("position1", "biodata.position1/1"),
        Span0 => ("span0", "biodata.span0/1"),
        Text => ("text", "biodata.text/1"),
        Source => ("source", "biomcp.source/1"),
        Reason => ("reason", "biomcp.reason/2"),
        ProblemCode => ("problem-code", "biomcp.problem-code/1"),
        Problem => ("problem", "biomcp.problem/1"),
        GeneInput => ("gene-input", "biomcp.gene-input/1", query),
        Gene => ("gene", "biodata.gene/5"),
        GeneHeader => ("gene-header", "biomcp.gene-header/4"),
        GeneResolution => ("gene-resolution", "biomcp.gene-resolution/5"),
        NcbiGeneId => ("ncbi-gene-id", "biodata.ncbi-gene-id/1"),
        EnsemblGeneId => ("ensembl-gene-id", "biodata.ensembl-gene-id/1"),
        GeneSymbol => ("gene-symbol", "biodata.gene-symbol/2"),
        UniProtAcc => ("uniprot-acc", "biodata.uniprot-acc/1"),
        OmimId => ("omim-id", "biodata.omim-id/1"),
        AccessionBase => ("accession-base", "biodata.accession-base/1"),
        GeneRef => ("gene-ref", "biodata.gene-ref/2"),
        Cytoband => ("cytoband", "biodata.cytoband/1"),
        GeneLocation => ("gene-location", "biodata.gene-location/1"),
        GenomicInterval => ("genomic-interval", "biodata.genomic-interval/2"),
        GeneStatus => ("gene-status", "biodata.gene-status/1"),
        LocusGroup => ("locus-group", "biodata.locus-group/1"),
        LocusType => ("locus-type", "biodata.locus-type/1"),
        MatchKind => ("match-kind", "biodata.match-kind/2"),
        Match => ("gene-match", "biomcp.gene-match/2"),
        Mane => ("mane", "biodata.mane/1"),
        Summary => ("gene-summary", "biodata.gene-summary/1"),
        SummarySource => ("summary-source", "biodata.summary-source/1"),
        HgncStatus => ("hgnc-status", "biodata.hgnc-status/2"),
        Build => ("build", "biodata.build/2"),
        Strand => ("strand", "biodata.strand/1"),
    };
    contracts.extend(contracts! {
        GwasGetCommand => ("gwas-get", "biomcp.gwas-get/1", query),
        GwasSearchCommand => ("gwas-search", "biomcp.gwas-search/2", query),
        Envelope<GwasPage> => ("gwas-page", "biomcp.gwas-page/4"),
        Envelope<GwasAssociationCard> => ("gwas-association", "biomcp.gwas-association/2"),
        Envelope<GwasLoci> => ("gwas-loci", "biomcp.gwas-loci/2"),
        Envelope<GwasStudy> => ("gwas-study", "biomcp.gwas-study/2"),
        Envelope<GwasAncestries> => ("gwas-ancestries", "biomcp.gwas-ancestries/2"),
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
    contracts
}
fn study_registry() -> Vec<Contract> {
    contracts! {
        StudySearch => ("study-search", "biomcp.study-search/1", query),
        StudyGet => ("study-get", "biomcp.study-get/1", query),
        Envelope<StudyPage> => ("study-page", "biomcp.study-page/2"),
        Envelope<StudyCard> => ("study-card", "biomcp.study-card/3"),
        Envelope<StudyProfiles> => ("study-profiles", "biomcp.study-profiles/2"),
        Envelope<StudySampleLists> => ("study-sample-lists", "biomcp.study-sample-lists/2"),
        Envelope<StudyFiles> => ("study-files", "biomcp.study-files/2"),
    }
}

fn adverse_event_contracts() -> Vec<Contract> {
    contracts! {
        PgxSearch => ("pgx-search", "biomcp.pgx-search/2", query),
        PgxGet => ("pgx-get", "biomcp.pgx-get/1", query),
        PgxGetCommand => ("pgx-get-command", "biomcp.pgx-get-command/1", query),
        PgxAnnotationGet => ("pgx-annotation-get", "biomcp.pgx-annotation-get/1", query),
        Envelope<PgxPage> => ("pgx-page", "biomcp.pgx-page/3"),
        Envelope<PgxCard> => ("pgx-card", "biomcp.pgx-card/2"),
        Envelope<PgxGuidelines> => ("pgx-guidelines", "biomcp.pgx-guidelines/2"),
        Envelope<PgxRecommendations> => ("pgx-recommendations", "biomcp.pgx-recommendations/2"),
        Envelope<PgxDiplotype> => ("pgx-diplotype", "biomcp.pgx-diplotype/2"),
        Envelope<PgxAnnotations> => ("pgx-annotations", "biomcp.pgx-annotations/2"),
        Envelope<PgxAnnotation> => ("pgx-annotation", "biomcp.pgx-annotation/2"),
        AdverseEventSearch => ("adverse-event-search", "biomcp.adverse-event-search/2", query),
        AdverseEventGet => ("adverse-event-get", "biomcp.adverse-event-get/1", query),
        Envelope<VaersSummary> => ("vaers-summary", "biomcp.vaers-summary/3"),
        Envelope<AdverseEventPage> => ("adverse-event-page", "biomcp.adverse-event-page/4"),
        Envelope<AdverseEventFaersCard> => ("adverse-event-faers-card", "biomcp.adverse-event-faers-card/2"),
        Envelope<AdverseEventMaudeCard> => ("adverse-event-maude-card", "biomcp.adverse-event-maude-card/2"),
        Envelope<AdverseEventDrugs> => ("adverse-event-drugs", "biomcp.adverse-event-drugs/2"),
        Envelope<AdverseEventReactions> => ("adverse-event-reactions", "biomcp.adverse-event-reactions/2"),
        Envelope<AdverseEventIdentifiers> => ("adverse-event-identifiers", "biomcp.adverse-event-identifiers/2"),
        Envelope<AdverseEventDevices> => ("adverse-event-devices", "biomcp.adverse-event-devices/2"),
        Envelope<AdverseEventNarrative> => ("adverse-event-narrative", "biomcp.adverse-event-narrative/2"),
    }
}

fn variant_contracts() -> Vec<Contract> {
    contracts! {
        VariantAssertionsGet => ("variant-assertions", "biomcp.variant-assertions/2", query),
        Envelope<VariantAssertions> => ("variant-assertions-result", "biomcp.variant-assertions/5"),
        PopulationFrequency => ("population-frequency", "biodata.population-frequency/6"),
        VariantPopulationQuery => ("variant-population", "biomcp.variant-population/3", query),
        Envelope<VariantPopulationResult> => ("variant-population-result", "biomcp.variant-population/9"),
        Statement => ("statement", "biodata.statement/5"),
        VariantRecurrenceQuery => ("variant-recurrence-query", "biomcp.variant-recurrence-query/3", query),
        Envelope<VariantRecurrence> => ("variant-recurrence", "biomcp.variant-recurrence/7"),
        VariantNormalize => ("variant-normalize", "biomcp.variant-normalize/4", query),
        Envelope<VariantNormalization> => ("variant-normalization", "biomcp.variant-normalization/6"),
        VariantSearch => ("variant-search", "biomcp.variant-search/2", query),
        Envelope<VariantPage> => ("variant-page", "biomcp.variant-page/12"),
        VariantGet => ("variant-get", "biomcp.variant-get/3", query),
        Envelope<VariantCard> => ("variant-card", "biomcp.variant-card/11"),
    }
}

fn drug_contracts() -> Vec<Contract> {
    contracts! {
        DrugId => ("drug-id", "biodata.drug-id/2"),
        DrugIdentityLevel => ("drug-identity-level", "biodata.drug-identity-level/1"),
        DrugIdentity => ("drug-identity", "biodata.drug-identity/2"),
        DrugIdentityRelation => ("drug-identity-relation", "biodata.drug-identity-relation/2"),
        DrugProduct => ("drug-product", "biodata.drug-product/2"),
        DrugLabelReference => ("drug-label-reference", "biodata.drug-label-reference/2"),
        DrugLabelSection => ("drug-label-section", "biodata.drug-label-section/2"),
        DrugNativeTarget => ("drug-native-target", "biodata.drug-native-target/1"),
        DrugDiseaseReference => ("drug-disease-reference", "biodata.drug-disease-reference/1"),
        DrugStage => ("drug-stage", "biodata.drug-stage/1"),
        DrugNativeReference => ("drug-native-reference", "biodata.drug-native-reference/1"),
        DrugTargetRelation => ("drug-target-relation", "biodata.drug-target-relation/4"),
        DrugIndicationClaim => ("drug-indication-claim", "biodata.drug-indication-claim/3"),
        DrugInteractionClaim => ("drug-interaction-claim", "biodata.drug-interaction-claim/3"),
        AuthorizationDate => ("authorization-date", "biodata.authorization-date/1"),
        AuthorizationEvent => ("authorization-event", "biodata.authorization-event/2"),
        DrugNameSelector => ("drug-name-selector", "biomcp.drug-name-selector/1", query),
        DrugSearch => ("drug-search", "biomcp.drug-search/2", query),
        DrugGet => ("drug-get", "biomcp.drug-get/2", query),
        DrugWhoGet => ("drug-who-get", "biomcp.drug-who-get/1", query),
        DrugGetCommand => ("drug-get-command", "biomcp.drug-get-command/1", query),
        DrugWhoRow => ("drug-who-row", "biomcp.drug-who-row/1"),
        Envelope<DrugWhoPage> => ("drug-who-page", "biomcp.drug-who-page/2"),
        Envelope<DrugWhoCard> => ("drug-who-card", "biomcp.drug-who-card/2"),
        DrugSource => ("drug-source", "biomcp.drug-source/1"),
        DrugSection => ("drug-section", "biomcp.drug-section/1"),
        DrugHeader => ("drug-header", "biomcp.drug-header/2"),
        Envelope<DrugPage> => ("drug-page", "biomcp.drug-page/4"),
        Envelope<DrugFdaPage> => ("drug-fda-page", "biomcp.drug-fda-page/3"),
        Envelope<DrugXrefs> => ("drug-xrefs", "biomcp.drug-xrefs/2"),
        Envelope<DrugCard> => ("drug-card", "biomcp.drug-card/3"),
        Envelope<DrugLabel> => ("drug-label", "biomcp.drug-label/4"),
        Envelope<DrugTargets> => ("drug-targets", "biomcp.drug-targets/4"),
        Envelope<DrugIndications> => ("drug-indications", "biomcp.drug-indications/4"),
        Envelope<DrugCivic> => ("drug-civic", "biomcp.drug-civic/7"),
        Envelope<DrugApprovals> => ("drug-approvals", "biomcp.drug-approvals/4"),
        Envelope<DrugRegulatory> => ("drug-regulatory", "biomcp.drug-regulatory/3"),
        Envelope<DrugInteractions> => ("drug-interactions", "biomcp.drug-interactions/4"),
    }
}

fn diagnostic_contracts() -> Vec<Contract> {
    contracts! {
        DiagnosticSearch => ("diagnostic-search", "biomcp.diagnostic-search/1", query),
        DiagnosticGet => ("diagnostic-get", "biomcp.diagnostic-get/1", query),
        Envelope<DiagnosticPage> => ("diagnostic-page", "biomcp.diagnostic-page/3"),
        Envelope<DiagnosticCard> => ("diagnostic-card", "biomcp.diagnostic-card/2"),
        Envelope<DiagnosticGenes> => ("diagnostic-genes", "biomcp.diagnostic-genes/3"),
        Envelope<DiagnosticConditions> => ("diagnostic-conditions", "biomcp.diagnostic-conditions/3"),
        Envelope<DiagnosticMethods> => ("diagnostic-methods", "biomcp.diagnostic-methods/3"),
        Envelope<DiagnosticRegulatory> => ("diagnostic-regulatory", "biomcp.diagnostic-regulatory/2"),
        Envelope<DiagnosticSupplements> => ("diagnostic-supplements", "biomcp.diagnostic-supplements/2"),
        Envelope<DiagnosticCompanion> => ("diagnostic-companion", "biomcp.diagnostic-companion/2"),
    }
}

fn trial_contracts() -> Vec<Contract> {
    contracts! {
        TrialGet => ("trial-get", "biomcp.trial-get/1", query),
        Envelope<NciTrialCards> => ("nci-trial-cards", "biomcp.nci-trial-cards/4"),
        Envelope<NciTrialPage> => ("nci-trial-page", "biomcp.nci-trial-page/6"),
        Envelope<TrialCards> => ("trial-cards", "biomcp.trial-cards/6"),
        TrialConditionResolution => ("trial-condition-resolution", "biomcp.trial-condition-resolution/2"),
        TrialSearch => ("trial-search", "biomcp.trial-search/3", query),
        Envelope<TrialPage> => ("trial-page", "biomcp.trial-page/11"),
        NctId => ("nct-id", "biodata.nct-id/1"),
        TrialHeader => ("trial-header", "biomcp.trial-header/2"),
    }
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
        PathwaySearch => ("pathway-search", "biomcp.pathway-search/2", query),
        PathwayGet => ("pathway-get", "biomcp.pathway-get/2", query),
        PathwayGenesQuery => ("pathway-genes-query", "biomcp.pathway-genes-query/2", query),
        PathwayEventsQuery => ("pathway-events-query", "biomcp.pathway-events-query/1", query),
        Envelope<WikiPathwaysSync> => ("wikipathways-sync", "biomcp.wikipathways-sync/3"),
        Envelope<PathwayPage> => ("pathway-page", "biomcp.pathway-page/4"),
        Envelope<PathwayCard> => ("pathway-card", "biomcp.pathway-card/6"),
        Envelope<PathwayGenes> => ("pathway-genes", "biomcp.pathway-genes/5"),
        Envelope<PathwayEvents> => ("pathway-events", "biomcp.pathway-events/3"),
    }
}

fn protein_contracts() -> Vec<Contract> {
    contracts! {
        ProteinFeaturesQuery => ("protein-features-query", "biomcp.protein-features-query/1", query),
        ProteinRelationQuery => ("protein-relation-query", "biomcp.protein-relation-query/1", query),
        ProteinDomainsQuery => ("protein-domains-query", "biomcp.protein-domains-query/1", query),
        ProteinComplexMembersQuery => ("protein-complex-members-query", "biomcp.protein-complex-members-query/1", query),
        ProteinSequenceQuery => ("protein-sequence-query", "biomcp.protein-sequence-query/1", query),
        ProteinSearch => ("protein-search", "biomcp.protein-search/1", query),
        ProteinGet => ("protein-get", "biomcp.protein-get/1", query),
        Envelope<ProteinPage> => ("protein-page", "biomcp.protein-page/2"),
        Envelope<ProteinCard> => ("protein-card", "biomcp.protein-card/5"),
        Envelope<ProteinFeatures> => ("protein-features", "biomcp.protein-features/2"),
        Envelope<ProteinDomains> => ("protein-domains", "biomcp.protein-domains/4"),
        Envelope<ProteinIsoforms> => ("protein-isoforms", "biomcp.protein-isoforms/2"),
        Envelope<ProteinComplexes> => ("protein-complexes", "biomcp.protein-complexes/4"),
        Envelope<ProteinComplexMembers> => ("protein-complex-members", "biomcp.protein-complex-members/2"),
        Envelope<ProteinStructures> => ("protein-structures", "biomcp.protein-structures/2"),
        Envelope<ProteinSequenceResult> => ("protein-sequence", "biomcp.protein-sequence/2"),
        Envelope<ProteinFunctionResult> => ("protein-function", "biomcp.protein-function/3")
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
