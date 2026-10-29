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
        ArticleEvidence => ("article-evidence", "biomcp.article-evidence/2", query),
        ArticleRecommend => ("article-recommend", "biomcp.article-recommend/2", query),
        Envelope<ArticleEvidenceResult> => ("article-evidence-result", "biomcp.article-evidence/4"),
        Envelope<ArticleRecommendations> => ("article-recommendations", "biomcp.article-recommendations/6"),
        ArticlePdf => ("article-pdf", "biomcp.article-pdf/2", query),
        Envelope<ArticlePdfResult> => ("article-pdf-result", "biomcp.article-pdf/4"),
        ArticleSupplement => ("article-supplement", "biomcp.article-supplement/2", query),
        Envelope<ArticleSupplementResult> => ("article-supplement-result", "biomcp.article-supplement/5"),
        ArticleText => ("article-text", "biomcp.article-text/3", query),
        Envelope<ArticleTextResult> => ("article-text-result", "biomcp.article-text/5"),
        ArticleEdges => ("article-edges", "biomcp.article-edges/3", query),
        Envelope<ArticleEdgePage> => ("article-edge-page", "biomcp.article-edges/7"),
        ArticleGet => ("article-get", "biomcp.article-get/3", query),
        Article => ("article", "biomcp.article/4"),
        Envelope<ArticleCards> => ("article-cards", "biomcp.article-cards/6"),
        ArticleSearch => ("article-search", "biomcp.article-search/3", query),
        ArticleHeader => ("article-header", "biomcp.article-header/5"),
        Envelope<ArticlePage> => ("article-page", "biomcp.article-page/6"),
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
        Envelope<TrialDocumentResult> => ("trial-document-result", "biomcp.trial-document/5"),
        HttpHealth => ("http-health", "biomcp.http-health/1"),
        Envelope<Health> => ("health", "biomcp.health/5"),
        Markdown => ("markdown", "biodata.markdown/1"),
        Envelope<SchemaCatalog> => ("schema-catalog", "biomcp.schema-catalog/3"),
        Envelope<Help> => ("help", "biomcp.help/4"),
        Envelope<KeyHelp> => ("key", "biomcp.key/4"),
        Envelope<RegisteredKeys> => ("registered-keys", "biomcp.registered-keys/4"),
        ErrorEnvelope => ("error", "biomcp.error/2"),
        GeneGet => ("gene-get", "biomcp.gene-get/2", query),
        GeneSearch => ("gene-search", "biomcp.gene-search/1", query),
        Request => ("request", "biomcp.request/30", query),
        Envelope<GeneCards> => ("gene-cards", "biomcp.gene-cards/9"),
        Envelope<GenePage> => ("gene-page", "biomcp.gene-page/6"),
        Envelope<StoreClean> => ("store-clean", "biomcp.store-clean/3"),
        Envelope<SyncResult> => ("sync", "biomcp.sync/6"),
        HgncId => ("hgnc-id", "biodata.hgnc-id/2"),
        PartialDate => ("partial-date", "biodata.partial-date/2"),
        Position1 => ("position1", "biodata.position1/1"),
        Span0 => ("span0", "biodata.span0/1"),
        Text => ("text", "biodata.text/1"),
        Source => ("source", "biomcp.source/1"),
        Reason => ("reason", "biomcp.reason/2"),
        ProblemCode => ("problem-code", "biomcp.problem-code/1"),
        Problem => ("problem", "biomcp.problem/1"),
        GeneInput => ("gene-input", "biomcp.gene-input/2", query),
        Gene => ("gene", "biodata.gene/5"),
        GeneHeader => ("gene-header", "biomcp.gene-header/5"),
        GeneResolution => ("gene-resolution", "biomcp.gene-resolution/6"),
        NcbiGeneId => ("ncbi-gene-id", "biodata.ncbi-gene-id/2"),
        EnsemblGeneId => ("ensembl-gene-id", "biodata.ensembl-gene-id/2"),
        GeneSymbol => ("gene-symbol", "biodata.gene-symbol/2"),
        UniProtAcc => ("uniprot-acc", "biodata.uniprot-acc/1"),
        OmimId => ("omim-id", "biodata.omim-id/1"),
        AccessionBase => ("accession-base", "biodata.accession-base/2"),
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
        Summary => ("gene-summary", "biodata.gene-summary/2"),
        SummarySource => ("summary-source", "biodata.summary-source/1"),
        HgncStatus => ("hgnc-status", "biodata.hgnc-status/2"),
        Build => ("build", "biodata.build/2"),
        Strand => ("strand", "biodata.strand/1"),
    };
    contracts.extend(contracts! {
        GwasGetCommand => ("gwas-get", "biomcp.gwas-get/1", query),
        GwasSearchCommand => ("gwas-search", "biomcp.gwas-search/3", query),
        Envelope<GwasPage> => ("gwas-page", "biomcp.gwas-page/5"),
        Envelope<GwasAssociationCard> => ("gwas-association", "biomcp.gwas-association/3"),
        Envelope<GwasLoci> => ("gwas-loci", "biomcp.gwas-loci/3"),
        Envelope<GwasStudy> => ("gwas-study", "biomcp.gwas-study/3"),
        Envelope<GwasAncestries> => ("gwas-ancestries", "biomcp.gwas-ancestries/3"),
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
        Envelope<StudyPage> => ("study-page", "biomcp.study-page/3"),
        Envelope<StudyCard> => ("study-card", "biomcp.study-card/4"),
        Envelope<StudyProfiles> => ("study-profiles", "biomcp.study-profiles/2"),
        Envelope<StudySampleLists> => ("study-sample-lists", "biomcp.study-sample-lists/2"),
        Envelope<StudyFiles> => ("study-files", "biomcp.study-files/2"),
    }
}

fn adverse_event_contracts() -> Vec<Contract> {
    contracts! {
        PgxSearch => ("pgx-search", "biomcp.pgx-search/2", query),
        PgxGet => ("pgx-get", "biomcp.pgx-get/2", query),
        PgxGetCommand => ("pgx-get-command", "biomcp.pgx-get-command/2", query),
        PgxAnnotationGet => ("pgx-annotation-get", "biomcp.pgx-annotation-get/2", query),
        Envelope<PgxPage> => ("pgx-page", "biomcp.pgx-page/4"),
        Envelope<PgxCard> => ("pgx-card", "biomcp.pgx-card/3"),
        Envelope<PgxGuidelines> => ("pgx-guidelines", "biomcp.pgx-guidelines/3"),
        Envelope<PgxRecommendations> => ("pgx-recommendations", "biomcp.pgx-recommendations/3"),
        Envelope<PgxDiplotype> => ("pgx-diplotype", "biomcp.pgx-diplotype/3"),
        Envelope<PgxAnnotations> => ("pgx-annotations", "biomcp.pgx-annotations/3"),
        Envelope<PgxAnnotation> => ("pgx-annotation", "biomcp.pgx-annotation/3"),
        AdverseEventSearch => ("adverse-event-search", "biomcp.adverse-event-search/3", query),
        AdverseEventGet => ("adverse-event-get", "biomcp.adverse-event-get/1", query),
        Envelope<VaersSummary> => ("vaers-summary", "biomcp.vaers-summary/4"),
        Envelope<AdverseEventPage> => ("adverse-event-page", "biomcp.adverse-event-page/5"),
        Envelope<AdverseEventFaersCard> => ("adverse-event-faers-card", "biomcp.adverse-event-faers-card/2"),
        Envelope<AdverseEventMaudeCard> => ("adverse-event-maude-card", "biomcp.adverse-event-maude-card/2"),
        Envelope<AdverseEventDrugs> => ("adverse-event-drugs", "biomcp.adverse-event-drugs/2"),
        Envelope<AdverseEventReactions> => ("adverse-event-reactions", "biomcp.adverse-event-reactions/3"),
        Envelope<AdverseEventIdentifiers> => ("adverse-event-identifiers", "biomcp.adverse-event-identifiers/2"),
        Envelope<AdverseEventDevices> => ("adverse-event-devices", "biomcp.adverse-event-devices/2"),
        Envelope<AdverseEventNarrative> => ("adverse-event-narrative", "biomcp.adverse-event-narrative/2"),
    }
}

fn variant_contracts() -> Vec<Contract> {
    contracts! {
        VariantAssertionsGet => ("variant-assertions", "biomcp.variant-assertions/3", query),
        Envelope<VariantAssertions> => ("variant-assertions-result", "biomcp.variant-assertions/6"),
        PopulationFrequency => ("population-frequency", "biodata.population-frequency/6"),
        VariantPopulationQuery => ("variant-population", "biomcp.variant-population/4", query),
        Envelope<VariantPopulationResult> => ("variant-population-result", "biomcp.variant-population/10"),
        Statement => ("statement", "biodata.statement/5"),
        VariantRecurrenceQuery => ("variant-recurrence-query", "biomcp.variant-recurrence-query/4", query),
        Envelope<VariantRecurrence> => ("variant-recurrence", "biomcp.variant-recurrence/8"),
        VariantNormalize => ("variant-normalize", "biomcp.variant-normalize/5", query),
        Envelope<VariantNormalization> => ("variant-normalization", "biomcp.variant-normalization/7"),
        VariantSearch => ("variant-search", "biomcp.variant-search/3", query),
        Envelope<VariantPage> => ("variant-page", "biomcp.variant-page/13"),
        VariantGet => ("variant-get", "biomcp.variant-get/4", query),
        Envelope<VariantCard> => ("variant-card", "biomcp.variant-card/12"),
    }
}

fn drug_contracts() -> Vec<Contract> {
    contracts! {
        DrugId => ("drug-id", "biodata.drug-id/2"),
        DrugIdentityLevel => ("drug-identity-level", "biodata.drug-identity-level/1"),
        DrugIdentity => ("drug-identity", "biodata.drug-identity/2"),
        DrugIdentityRelation => ("drug-identity-relation", "biodata.drug-identity-relation/3"),
        DrugProduct => ("drug-product", "biodata.drug-product/3"),
        DrugLabelReference => ("drug-label-reference", "biodata.drug-label-reference/3"),
        DrugLabelSection => ("drug-label-section", "biodata.drug-label-section/3"),
        DrugNativeTarget => ("drug-native-target", "biodata.drug-native-target/1"),
        DrugDiseaseReference => ("drug-disease-reference", "biodata.drug-disease-reference/1"),
        DrugStage => ("drug-stage", "biodata.drug-stage/1"),
        DrugNativeReference => ("drug-native-reference", "biodata.drug-native-reference/1"),
        DrugTargetRelation => ("drug-target-relation", "biodata.drug-target-relation/4"),
        DrugIndicationClaim => ("drug-indication-claim", "biodata.drug-indication-claim/4"),
        DrugInteractionClaim => ("drug-interaction-claim", "biodata.drug-interaction-claim/4"),
        AuthorizationDate => ("authorization-date", "biodata.authorization-date/2"),
        AuthorizationEvent => ("authorization-event", "biodata.authorization-event/3"),
        DrugNameSelector => ("drug-name-selector", "biomcp.drug-name-selector/1", query),
        DrugSearch => ("drug-search", "biomcp.drug-search/2", query),
        DrugGet => ("drug-get", "biomcp.drug-get/2", query),
        DrugWhoGet => ("drug-who-get", "biomcp.drug-who-get/1", query),
        DrugGetCommand => ("drug-get-command", "biomcp.drug-get-command/1", query),
        DrugWhoRow => ("drug-who-row", "biomcp.drug-who-row/1"),
        Envelope<DrugWhoPage> => ("drug-who-page", "biomcp.drug-who-page/3"),
        Envelope<DrugWhoCard> => ("drug-who-card", "biomcp.drug-who-card/2"),
        DrugSource => ("drug-source", "biomcp.drug-source/1"),
        DrugSection => ("drug-section", "biomcp.drug-section/1"),
        DrugHeader => ("drug-header", "biomcp.drug-header/2"),
        Envelope<DrugPage> => ("drug-page", "biomcp.drug-page/5"),
        Envelope<DrugRecallPage> => ("drug-recall-page", "biomcp.drug-recall-page/1"),
        Envelope<DrugRecallCard> => ("drug-recall-card", "biomcp.drug-recall-card/1"),
        DrugRecallHeader => ("drug-recall-header", "biomcp.drug-recall-header/1"),
        Envelope<DrugFdaPage> => ("drug-fda-page", "biomcp.drug-fda-page/4"),
        Envelope<DrugXrefs> => ("drug-xrefs", "biomcp.drug-xrefs/3"),
        Envelope<DrugCard> => ("drug-card", "biomcp.drug-card/4"),
        Envelope<DrugLabel> => ("drug-label", "biomcp.drug-label/5"),
        Envelope<DrugTargets> => ("drug-targets", "biomcp.drug-targets/5"),
        Envelope<DrugIndications> => ("drug-indications", "biomcp.drug-indications/5"),
        Envelope<DrugCivic> => ("drug-civic", "biomcp.drug-civic/8"),
        Envelope<DrugApprovals> => ("drug-approvals", "biomcp.drug-approvals/5"),
        Envelope<DrugRegulatory> => ("drug-regulatory", "biomcp.drug-regulatory/4"),
        Envelope<DrugInteractions> => ("drug-interactions", "biomcp.drug-interactions/5"),
    }
}

fn diagnostic_contracts() -> Vec<Contract> {
    contracts! {
        DiagnosticSearch => ("diagnostic-search", "biomcp.diagnostic-search/2", query),
        DiagnosticGet => ("diagnostic-get", "biomcp.diagnostic-get/1", query),
        Envelope<DiagnosticPage> => ("diagnostic-page", "biomcp.diagnostic-page/4"),
        Envelope<DiagnosticCard> => ("diagnostic-card", "biomcp.diagnostic-card/3"),
        Envelope<DiagnosticGenes> => ("diagnostic-genes", "biomcp.diagnostic-genes/4"),
        Envelope<DiagnosticConditions> => ("diagnostic-conditions", "biomcp.diagnostic-conditions/4"),
        Envelope<DiagnosticMethods> => ("diagnostic-methods", "biomcp.diagnostic-methods/3"),
        Envelope<DiagnosticRegulatory> => ("diagnostic-regulatory", "biomcp.diagnostic-regulatory/2"),
        Envelope<DiagnosticSupplements> => ("diagnostic-supplements", "biomcp.diagnostic-supplements/2"),
        Envelope<DiagnosticCompanion> => ("diagnostic-companion", "biomcp.diagnostic-companion/3"),
    }
}

fn trial_contracts() -> Vec<Contract> {
    contracts! {
        TrialGet => ("trial-get", "biomcp.trial-get/1", query),
        Envelope<NciTrialCards> => ("nci-trial-cards", "biomcp.nci-trial-cards/4"),
        Envelope<NciTrialPage> => ("nci-trial-page", "biomcp.nci-trial-page/7"),
        Envelope<TrialCards> => ("trial-cards", "biomcp.trial-cards/7"),
        TrialConditionResolution => ("trial-condition-resolution", "biomcp.trial-condition-resolution/3"),
        TrialSearch => ("trial-search", "biomcp.trial-search/4", query),
        Envelope<TrialPage> => ("trial-page", "biomcp.trial-page/12"),
        NctId => ("nct-id", "biodata.nct-id/1"),
        TrialHeader => ("trial-header", "biomcp.trial-header/3"),
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
        PathwaySearch => ("pathway-search", "biomcp.pathway-search/3", query),
        PathwayGet => ("pathway-get", "biomcp.pathway-get/2", query),
        PathwayGenesQuery => ("pathway-genes-query", "biomcp.pathway-genes-query/2", query),
        PathwayEventsQuery => ("pathway-events-query", "biomcp.pathway-events-query/1", query),
        Envelope<WikiPathwaysSync> => ("wikipathways-sync", "biomcp.wikipathways-sync/3"),
        Envelope<PathwayPage> => ("pathway-page", "biomcp.pathway-page/5"),
        Envelope<PathwayCard> => ("pathway-card", "biomcp.pathway-card/7"),
        Envelope<PathwayGenes> => ("pathway-genes", "biomcp.pathway-genes/6"),
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
        Envelope<ProteinPage> => ("protein-page", "biomcp.protein-page/3"),
        Envelope<ProteinCard> => ("protein-card", "biomcp.protein-card/6"),
        Envelope<ProteinFeatures> => ("protein-features", "biomcp.protein-features/3"),
        Envelope<ProteinDomains> => ("protein-domains", "biomcp.protein-domains/5"),
        Envelope<ProteinIsoforms> => ("protein-isoforms", "biomcp.protein-isoforms/2"),
        Envelope<ProteinComplexes> => ("protein-complexes", "biomcp.protein-complexes/4"),
        Envelope<ProteinComplexMembers> => ("protein-complex-members", "biomcp.protein-complex-members/3"),
        Envelope<ProteinStructures> => ("protein-structures", "biomcp.protein-structures/2"),
        Envelope<ProteinSequenceResult> => ("protein-sequence", "biomcp.protein-sequence/3"),
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
