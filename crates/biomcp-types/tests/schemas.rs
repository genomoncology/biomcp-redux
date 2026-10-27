//! Public schema contract tests; file I/O stays outside the pure library.
#![cfg(feature = "schema")]
#[path = "schemas/compatibility.rs"]
mod compatibility;
#[path = "schemas/compatibility_cases.rs"]
mod compatibility_cases;
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
    name: &'static str,
    id: &'static str,
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
        ($($ty:ty => ($name:literal, $id:literal)),+ $(,)?) => {
            vec![$(Contract { name: $name, id: $id, generate: generate::<$ty> }),+]
        };
    }

#[path = "schemas/disease.rs"]
mod disease;

fn common_registry() -> Vec<Contract> {
    contracts! {
        Accession => ("accession", "biodata.accession/2"),
        Chromosome => ("chromosome", "biodata.chromosome/1"),
        Base => ("base", "biodata.base/1"),
        Sequence => ("sequence", "biodata.sequence/1"),
        SequenceLocation => ("sequence-location", "biodata.sequence-location/2"),
        Allele => ("allele", "biodata.allele/2"),
        HgvsText => ("hgvs-text", "biodata.hgvs-text/2"),
        Expression => ("expression", "biodata.expression/2"),
        ExpressionSyntax => ("expression-syntax", "biodata.expression-syntax/1"),
        Representation => ("representation", "biodata.representation/3"),
        VariantId => ("variant-id", "biodata.variant-id/2"),
        Variant => ("variant", "biodata.variant/3"),
        RsId => ("rs-id", "biodata.rs-id/1"),
        CaId => ("ca-id", "biodata.ca-id/1"),
        PaId => ("pa-id", "biodata.pa-id/1"),
        VcvId => ("vcv-id", "biodata.vcv-id/1"),
        RcvId => ("rcv-id", "biodata.rcv-id/1"),
        ScvId => ("scv-id", "biodata.scv-id/1"),
        ClinVarVariationId => ("clinvar-variation-id", "biodata.clinvar-variation-id/1"),
        ClinVarAlleleId => ("clinvar-allele-id", "biodata.clinvar-allele-id/1"),
        CosmicId => ("cosmic-id", "biodata.cosmic-id/1"),
        VrsId => ("vrs-id", "biodata.vrs-id/1"),
        Contig => ("contig", "biodata.contig/1"),

    }
}

fn research_registry() -> Vec<Contract> {
    contracts! {
        ResearchSearch<ResearchEnvelope> => ("research-search", "biomcp.research-search/1"),
        ResearchFetch<ResearchEnvelope> => ("research-fetch", "biomcp.research-fetch/1"),
    }
}

fn article_registry() -> Vec<Contract> {
    contracts! {
        ArticleEvidence => ("article-evidence", "biodata.article-evidence/1"),
        ArticleRecommend => ("article-recommend", "biodata.article-recommend/1"),
        Envelope<ArticleEvidenceResult> => ("article-evidence-result", "biomcp.article_evidence/2"),
        Envelope<ArticleRecommendations> => ("article-recommendations", "biomcp.article_recommendations/3"),
        ArticlePdf => ("article-pdf", "biodata.article-pdf/1"),
        Envelope<ArticlePdfResult> => ("article-pdf-result", "biomcp.article_pdf/2"),
        ArticleSupplement => ("article-supplement", "biodata.article-supplement/1"),
        Envelope<ArticleSupplementResult> => ("article-supplement-result", "biomcp.article_supplement/2"),
        ArticleText => ("article-text", "biodata.article-text/2"),
        Envelope<ArticleTextResult> => ("article-text-result", "biomcp.article_text/3"),
        ArticleEdges => ("article-edges", "biodata.article-edges/2"),
        Envelope<ArticleEdgePage> => ("article-edge-page", "biomcp.article_edges/4"),
        ArticleGet => ("article-get", "biodata.article-get/2"),
        Article => ("article", "biodata.article/1"),
        Envelope<ArticleCards> => ("article-cards", "biomcp.article_cards/3"),
        ArticleSearch => ("article-search", "biodata.article-search/1"),
        ArticleHeader => ("article-header", "biodata.article_header/2"),
        Envelope<ArticlePage> => ("article-page", "biomcp.article_page/3"),
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

fn registry() -> Vec<Contract> {
    let mut contracts = contracts! {
        TrialDocumentGet => ("trial-document", "biodata.trial-document/1"),
        Envelope<TrialDocumentResult> => ("trial-document-result", "biomcp.trial_document/2"),
        HttpHealth => ("http-health", "biomcp.http-health/1"),
        Envelope<Health> => ("health", "biomcp.health/2"),
        Markdown => ("markdown", "biodata.markdown/1"),
        Envelope<Help> => ("help", "biomcp.help/2"),
        Envelope<KeyHelp> => ("key", "biomcp.key/2"),
        Envelope<RegisteredKeys> => ("registered-keys", "biomcp.registered-keys/2"),
        ErrorEnvelope => ("error", "biomcp.error/1"),
        GeneGet => ("gene-get", "biodata.gene-get/1"),
        GeneSearch => ("gene-search", "biodata.gene-search/1"),
        Request => ("request", "biodata.request/26"),
        Envelope<GeneCards> => ("gene-cards", "biomcp.gene_cards/4"),
        Envelope<GenePage> => ("gene-page", "biomcp.gene_page/3"),
        Envelope<StoreClean> => ("store-clean", "biomcp.store-clean/1"),
        Envelope<SyncResult> => ("sync", "biomcp.sync/4"),
        HgncId => ("hgnc-id", "biodata.hgnc-id/1"),
        PartialDate => ("partial-date", "biodata.partial-date/1"),
        Position1 => ("position1", "biodata.position1/1"),
        Span0 => ("span0", "biodata.span0/1"),
        Text => ("text", "biodata.text/1"),
        Source => ("source", "biodata.source/1"),
        Reason => ("reason", "biodata.reason/1"),
        ProblemCode => ("problem-code", "biodata.problem-code/1"),
        Problem => ("problem", "biodata.problem/1"),
        GeneInput => ("gene-input", "biodata.gene-input/1"),
        Gene => ("gene", "biodata.gene/3"),
        GeneHeader => ("gene-header", "biomcp.gene_header/2"),
        GeneResolution => ("gene-resolution", "biodata.gene-resolution/3"),
        NcbiGeneId => ("ncbi-gene-id", "biodata.ncbi-gene-id/1"),
        EnsemblGeneId => ("ensembl-gene-id", "biodata.ensembl-gene-id/1"),
        GeneSymbol => ("gene-symbol", "biodata.gene-symbol/2"),
        UniProtAcc => ("uniprot-acc", "biodata.uniprot-acc/1"),
        OmimId => ("omim-id", "biodata.omim-id/1"),
        AccessionBase => ("accession-base", "biodata.accession-base/1"),
        GeneRef => ("gene-ref", "biodata.gene-ref/1"),
        Cytoband => ("cytoband", "biodata.cytoband/1"),
        GenomicInterval => ("genomic-interval", "biodata.genomic-interval/2"),
        GeneStatus => ("gene-status", "biodata.gene-status/1"),
        LocusGroup => ("locus-group", "biodata.locus-group/1"),
        LocusType => ("locus-type", "biodata.locus-type/1"),
        MatchKind => ("match-kind", "biodata.match-kind/1"),
        Match => ("gene-match", "biodata.gene-match/1"),
        Mane => ("mane", "biodata.mane/1"),
        Summary => ("gene-summary", "biodata.gene-summary/1"),
        SummarySource => ("summary-source", "biodata.summary-source/1"),
        HgncStatus => ("hgnc-status", "biodata.hgnc-status/2"),
        Build => ("build", "biodata.build/2"),
        Strand => ("strand", "biodata.strand/1"),
    };
    contracts.extend(contracts! {
        GwasGetCommand => ("gwas-get", "biomcp.gwas-get/1"),
        GwasSearchCommand => ("gwas-search", "biomcp.gwas-search/2"),
        Envelope<GwasPage> => ("gwas-page", "biomcp.gwas_page/3"),
        Envelope<GwasAssociationCard> => ("gwas-association", "biomcp.gwas_association/1"),
        Envelope<GwasLoci> => ("gwas-loci", "biomcp.gwas_loci/1"),
        Envelope<GwasStudy> => ("gwas-study", "biomcp.gwas_study/1"),
        Envelope<GwasAncestries> => ("gwas-ancestries", "biomcp.gwas_ancestries/1"),
    });
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
        StudySearch => ("study-search", "biomcp.study-search/1"),
        StudyGet => ("study-get", "biomcp.study-get/1"),
        Envelope<StudyPage> => ("study-page", "biomcp.study_page/1"),
        Envelope<StudyCard> => ("study-card", "biomcp.study_card/2"),
        Envelope<StudyProfiles> => ("study-profiles", "biomcp.study_profiles/1"),
        Envelope<StudySampleLists> => ("study-sample-lists", "biomcp.study_sample_lists/1"),
        Envelope<StudyFiles> => ("study-files", "biomcp.study_files/1"),
    }
}

fn adverse_event_contracts() -> Vec<Contract> {
    contracts! {
        PgxSearch => ("pgx-search", "biomcp.pgx-search/2"),
        PgxGet => ("pgx-get", "biomcp.pgx-get/1"),
        PgxGetCommand => ("pgx-get-command", "biomcp.pgx-get-command/1"),
        PgxAnnotationGet => ("pgx-annotation-get", "biomcp.pgx-annotation-get/1"),
        Envelope<PgxPage> => ("pgx-page", "biomcp.pgx_page/2"),
        Envelope<PgxCard> => ("pgx-card", "biomcp.pgx_card/1"),
        Envelope<PgxGuidelines> => ("pgx-guidelines", "biomcp.pgx_guidelines/1"),
        Envelope<PgxRecommendations> => ("pgx-recommendations", "biomcp.pgx_recommendations/1"),
        Envelope<PgxDiplotype> => ("pgx-diplotype", "biomcp.pgx_diplotype/1"),
        Envelope<PgxAnnotations> => ("pgx-annotations", "biomcp.pgx_annotations/1"),
        Envelope<PgxAnnotation> => ("pgx-annotation", "biomcp.pgx_annotation/1"),
        AdverseEventSearch => ("adverse-event-search", "biomcp.adverse-event-search/1"),
        AdverseEventGet => ("adverse-event-get", "biomcp.adverse-event-get/1"),
        Envelope<VaersSummary> => ("vaers-summary", "biomcp.vaers_summary/1"),
        Envelope<AdverseEventPage> => ("adverse-event-page", "biomcp.adverse_event_page/2"),
        Envelope<AdverseEventFaersCard> => ("adverse-event-faers-card", "biomcp.adverse_event_faers_card/1"),
        Envelope<AdverseEventMaudeCard> => ("adverse-event-maude-card", "biomcp.adverse_event_maude_card/1"),
        Envelope<AdverseEventDrugs> => ("adverse-event-drugs", "biomcp.adverse_event_drugs/1"),
        Envelope<AdverseEventReactions> => ("adverse-event-reactions", "biomcp.adverse_event_reactions/1"),
        Envelope<AdverseEventIdentifiers> => ("adverse-event-identifiers", "biomcp.adverse_event_identifiers/1"),
        Envelope<AdverseEventDevices> => ("adverse-event-devices", "biomcp.adverse_event_devices/1"),
        Envelope<AdverseEventNarrative> => ("adverse-event-narrative", "biomcp.adverse_event_narrative/1"),
    }
}

fn variant_contracts() -> Vec<Contract> {
    contracts! {
        VariantAssertionsGet => ("variant-assertions", "biodata.variant-assertions/1"),
        Envelope<VariantAssertions> => ("variant-assertions-result", "biomcp.variant_assertions/3"),
        PopulationFrequency => ("population-frequency", "biodata.population-frequency/3"),
        VariantPopulationQuery => ("variant-population", "biodata.variant-population/2"),
        Envelope<VariantPopulationResult> => ("variant-population-result", "biomcp.variant_population/4"),
        Statement => ("statement", "biodata.statement/3"),
        VariantRecurrenceQuery => ("variant-recurrence-query", "biodata.variant-recurrence-query/2"),
        Envelope<VariantRecurrence> => ("variant-recurrence", "biomcp.variant_recurrence/4"),
        VariantNormalize => ("variant-normalize", "biodata.variant-normalize/2"),
        Envelope<VariantNormalization> => ("variant-normalization", "biomcp.variant_normalization/3"),
        VariantSearch => ("variant-search", "biodata.variant-search/2"),
        Envelope<VariantPage> => ("variant-page", "biomcp.variant_page/6"),
        VariantGet => ("variant-get", "biodata.variant-get/2"),
        Envelope<VariantCard> => ("variant-card", "biomcp.variant_card/5"),
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
        DrugTargetRelation => ("drug-target-relation", "biodata.drug-target-relation/3"),
        DrugIndicationClaim => ("drug-indication-claim", "biodata.drug-indication-claim/3"),
        DrugInteractionClaim => ("drug-interaction-claim", "biodata.drug-interaction-claim/3"),
        AuthorizationDate => ("authorization-date", "biodata.authorization-date/1"),
        AuthorizationEvent => ("authorization-event", "biodata.authorization-event/2"),
        DrugNameSelector => ("drug-name-selector", "biomcp.drug_name_selector/1"),
        DrugSearch => ("drug-search", "biomcp.drug-search/2"),
        DrugGet => ("drug-get", "biomcp.drug-get/2"),
        DrugWhoGet => ("drug-who-get", "biomcp.drug-who-get/1"),
        DrugGetCommand => ("drug-get-command", "biomcp.drug-get-command/1"),
        DrugWhoRow => ("drug-who-row", "biomcp.drug-who-row/1"),
        Envelope<DrugWhoPage> => ("drug-who-page", "biomcp.drug_who_page/1"),
        Envelope<DrugWhoCard> => ("drug-who-card", "biomcp.drug_who_card/1"),
        DrugSource => ("drug-source", "biomcp.drug-source/1"),
        DrugSection => ("drug-section", "biomcp.drug-section/1"),
        DrugHeader => ("drug-header", "biomcp.drug-header/2"),
        Envelope<DrugPage> => ("drug-page", "biomcp.drug_page/3"),
        Envelope<DrugFdaPage> => ("drug-fda-page", "biomcp.drug_fda_page/1"),
        Envelope<DrugXrefs> => ("drug-xrefs", "biomcp.drug_xrefs/1"),
        Envelope<DrugCard> => ("drug-card", "biomcp.drug_card/2"),
        Envelope<DrugLabel> => ("drug-label", "biomcp.drug_label/3"),
        Envelope<DrugTargets> => ("drug-targets", "biomcp.drug_targets/3"),
        Envelope<DrugIndications> => ("drug-indications", "biomcp.drug_indications/3"),
        Envelope<DrugCivic> => ("drug-civic", "biomcp.drug_civic/4"),
        Envelope<DrugApprovals> => ("drug-approvals", "biomcp.drug_approvals/3"),
        Envelope<DrugRegulatory> => ("drug-regulatory", "biomcp.drug_regulatory/2"),
        Envelope<DrugInteractions> => ("drug-interactions", "biomcp.drug_interactions/3"),
    }
}

fn diagnostic_contracts() -> Vec<Contract> {
    contracts! {
        DiagnosticSearch => ("diagnostic-search", "biomcp.diagnostic-search/1"),
        DiagnosticGet => ("diagnostic-get", "biomcp.diagnostic-get/1"),
        Envelope<DiagnosticPage> => ("diagnostic-page", "biomcp.diagnostic_page/2"),
        Envelope<DiagnosticCard> => ("diagnostic-card", "biomcp.diagnostic_card/1"),
        Envelope<DiagnosticGenes> => ("diagnostic-genes", "biomcp.diagnostic_genes/2"),
        Envelope<DiagnosticConditions> => ("diagnostic-conditions", "biomcp.diagnostic_conditions/2"),
        Envelope<DiagnosticMethods> => ("diagnostic-methods", "biomcp.diagnostic_methods/2"),
        Envelope<DiagnosticRegulatory> => ("diagnostic-regulatory", "biomcp.diagnostic_regulatory/1"),
        Envelope<DiagnosticSupplements> => ("diagnostic-supplements", "biomcp.diagnostic_supplements/1"),
        Envelope<DiagnosticCompanion> => ("diagnostic-companion", "biomcp.diagnostic_companion/1"),
    }
}

fn trial_contracts() -> Vec<Contract> {
    contracts! {
        TrialGet => ("trial-get", "biodata.trial-get/1"),
        Envelope<NciTrialCards> => ("nci-trial-cards", "biomcp.nci_trial_cards/2"),
        Envelope<NciTrialPage> => ("nci-trial-page", "biomcp.nci_trial_page/3"),
        Envelope<TrialCards> => ("trial-cards", "biomcp.trial_cards/2"),
        TrialConditionResolution => ("trial-condition-resolution", "biomcp.trial_condition_resolution/1"),
        TrialSearch => ("trial-search", "biodata.trial-search/2"),
        Envelope<TrialPage> => ("trial-page", "biomcp.trial_page/5"),
        NctId => ("nct-id", "biodata.nct-id/1"),
        TrialHeader => ("trial-header", "biodata.trial_header/1"),
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
    // Collect and check the whole registry before writing any contract.
    let pending: Vec<_> = registry()
        .into_iter()
        .map(|contract| {
            let mut new = (contract.generate)();
            new["$id"] = contract.id.into();
            compatibility::check(&new, &new)
                .unwrap_or_else(|error| panic!("{}: {error}", contract.name));
            let path = root.join(format!("{}.json", contract.name));
            if path.exists() {
                let old: Value =
                    serde_json::from_str(&fs::read_to_string(&path).expect("read contract"))
                        .expect("parse contract");
                compatibility::check(&old, &new)
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
        assert_eq!(branches.len(), 2);
        assert_eq!(
            branches
                .iter()
                .filter(|branch| branch["type"] == "null")
                .count(),
            1
        );
        for branch in branches.iter().filter(|branch| branch["type"] != "null") {
            flat_value(root, branch);
        }
    } else if node["type"] == "array" {
        let items = node.get("items").expect("array items");
        assert_ne!(items["type"], "array");
        flat_value(root, items);
    } else {
        assert!(
            ["string", "integer", "boolean"].contains(&node["type"].as_str().expect("scalar type"))
        );
    }
}

fn pathway_contracts() -> Vec<Contract> {
    contracts! {
        PathwaySearch => ("pathway-search", "biomcp.pathway-search/2"),
        PathwayGet => ("pathway-get", "biomcp.pathway-get/2"),
        PathwayGenesQuery => ("pathway-genes-query", "biomcp.pathway-genes-query/2"),
        PathwayEventsQuery => ("pathway-events-query", "biomcp.pathway-events-query/1"),
        Envelope<WikiPathwaysSync> => ("wikipathways-sync", "biomcp.wikipathways_sync/1"),
        Envelope<PathwayPage> => ("pathway-page", "biomcp.pathway_page/3"),
        Envelope<PathwayCard> => ("pathway-card", "biomcp.pathway_card/5"),
        Envelope<PathwayGenes> => ("pathway-genes", "biomcp.pathway_genes/4"),
        Envelope<PathwayEvents> => ("pathway-events", "biomcp.pathway_events/2"),
    }
}

fn protein_contracts() -> Vec<Contract> {
    contracts! {
        ProteinFeaturesQuery => ("protein-features-query", "biomcp.protein-features-query/1"),
        ProteinRelationQuery => ("protein-relation-query", "biomcp.protein-relation-query/1"),
        ProteinDomainsQuery => ("protein-domains-query", "biomcp.protein-domains-query/1"),
        ProteinComplexMembersQuery => ("protein-complex-members-query", "biomcp.protein-complex-members-query/1"),
        ProteinSequenceQuery => ("protein-sequence-query", "biomcp.protein-sequence-query/1"),
        ProteinSearch => ("protein-search", "biomcp.protein-search/1"),
        ProteinGet => ("protein-get", "biomcp.protein-get/1"),
        Envelope<ProteinPage> => ("protein-page", "biomcp.protein_page/1"),
        Envelope<ProteinCard> => ("protein-card", "biomcp.protein_card/3"),
        Envelope<ProteinFeatures> => ("protein-features", "biomcp.protein_features/1"),
        Envelope<ProteinDomains> => ("protein-domains", "biomcp.protein_domains/2"),
        Envelope<ProteinIsoforms> => ("protein-isoforms", "biomcp.protein_isoforms/1"),
        Envelope<ProteinComplexes> => ("protein-complexes", "biomcp.protein_complexes/2"),
        Envelope<ProteinComplexMembers> => ("protein-complex-members", "biomcp.protein_complex_members/1"),
        Envelope<ProteinStructures> => ("protein-structures", "biomcp.protein_structures/1"),
        Envelope<ProteinSequenceResult> => ("protein-sequence", "biomcp.protein_sequence/1"),
        Envelope<ProteinFunctionResult> => ("protein-function", "biomcp.protein_function/2")
    }
}

#[path = "schemas/author.rs"]
mod author;

#[test]
fn generated_schemas_preserve_release_contracts() -> Result<(), Box<dyn std::error::Error>> {
    release::generated_schemas_preserve_release_contracts()
}
