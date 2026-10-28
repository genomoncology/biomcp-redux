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
        ($($ty:ty => ($name:literal, $id:expr)),+ $(,)?) => {
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
        HgvsText => ("hgvs-text", "biodata.hgvs-text/3"),
        Expression => ("expression", "biodata.expression/2"),
        ExpressionSyntax => ("expression-syntax", "biodata.expression-syntax/1"),
        Representation => ("representation", "biodata.representation/3"),
        VariantId => ("variant-id", "biodata.variant-id/2"),
        Variant => ("variant", "biodata.variant/5"),
        RsId => ("rs-id", "biodata.rs-id/1"),
        CaId => ("ca-id", "biodata.ca-id/1"),
        PaId => ("pa-id", "biodata.pa-id/1"),
        VcvId => ("vcv-id", "biodata.vcv-id/1"),
        RcvId => ("rcv-id", "biodata.rcv-id/1"),
        ScvId => ("scv-id", "biodata.scv-id/1"),
        ClinVarVariationId => ("clinvar-variation-id", "biodata.clinvar-variation-id/1"),
        ClinVarAlleleId => ("clinvar-allele-id", "biodata.clinvar-allele-id/1"),
        CosmicId => ("cosmic-id", "biodata.cosmic-id/1"),
        MondoId => ("mondo-id", "biodata.mondo-id/1"),
        NcitId => ("ncit-id", "biodata.ncit-id/1"),
        VrsId => ("vrs-id", "biodata.vrs-id/1"),
        Contig => ("contig", "biodata.contig/1"),

    }
}

fn research_registry() -> Vec<Contract> {
    contracts! {
        ResearchSearch<ResearchEnvelope> => ("research-search", ResearchSearch::<ResearchEnvelope>::SCHEMA),
        ResearchFetch<ResearchEnvelope> => ("research-fetch", ResearchFetch::<ResearchEnvelope>::SCHEMA),
    }
}

fn article_registry() -> Vec<Contract> {
    contracts! {
        ArticleEvidence => ("article-evidence", "biodata.article-evidence/1"),
        ArticleRecommend => ("article-recommend", "biodata.article-recommend/1"),
        Envelope<ArticleEvidenceResult> => ("article-evidence-result", "biomcp.article-evidence/2"),
        Envelope<ArticleRecommendations> => ("article-recommendations", "biomcp.article-recommendations/4"),
        ArticlePdf => ("article-pdf", "biodata.article-pdf/1"),
        Envelope<ArticlePdfResult> => ("article-pdf-result", "biomcp.article-pdf/2"),
        ArticleSupplement => ("article-supplement", "biodata.article-supplement/1"),
        Envelope<ArticleSupplementResult> => ("article-supplement-result", "biomcp.article-supplement/3"),
        ArticleText => ("article-text", "biodata.article-text/2"),
        Envelope<ArticleTextResult> => ("article-text-result", "biomcp.article-text/3"),
        ArticleEdges => ("article-edges", "biodata.article-edges/2"),
        Envelope<ArticleEdgePage> => ("article-edge-page", "biomcp.article-edges/5"),
        ArticleGet => ("article-get", "biodata.article-get/2"),
        Article => ("article", "biodata.article/2"),
        Envelope<ArticleCards> => ("article-cards", "biomcp.article-cards/4"),
        ArticleSearch => ("article-search", "biodata.article-search/1"),
        ArticleHeader => ("article-header", "biodata.article_header/3"),
        Envelope<ArticlePage> => ("article-page", "biomcp.article-page/4"),
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
        BiomarkerSearch => ("biomarker-search", "biomcp.biomarker-search/1"),
        BiomarkerGet => ("biomarker-get", "biomcp.biomarker-get/1"),
        Envelope<BiomarkerPage> => ("biomarker-page", "biomcp.biomarker-page/1"),
        Envelope<BiomarkerResult> => ("biomarker-result", "biomcp.biomarker-result/1"),
        OrganizationSearch => ("organization-search", "biomcp.organization-search/1"),
        Envelope<OrganizationPage> => ("organization-page", "biomcp.organization-page/1"),
        TrialDocumentGet => ("trial-document", "biodata.trial-document/1"),
        Envelope<TrialDocumentResult> => ("trial-document-result", "biomcp.trial-document/3"),
        HttpHealth => ("http-health", "biomcp.http-health/1"),
        Envelope<Health> => ("health", "biomcp.health/3"),
        Markdown => ("markdown", "biodata.markdown/1"),
        Envelope<SchemaCatalog> => ("schema-catalog", "biomcp.schema-catalog/1"),
        Envelope<Help> => ("help", "biomcp.help/3"),
        Envelope<KeyHelp> => ("key", "biomcp.key/2"),
        Envelope<RegisteredKeys> => ("registered-keys", "biomcp.registered-keys/2"),
        ErrorEnvelope => ("error", "biomcp.error/1"),
        GeneGet => ("gene-get", "biodata.gene-get/1"),
        GeneSearch => ("gene-search", "biodata.gene-search/1"),
        Request => ("request", "biodata.request/28"),
        Envelope<GeneCards> => ("gene-cards", "biomcp.gene-cards/6"),
        Envelope<GenePage> => ("gene-page", "biomcp.gene-page/3"),
        Envelope<StoreClean> => ("store-clean", "biomcp.store-clean/1"),
        Envelope<SyncResult> => ("sync", "biomcp.sync/5"),
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
        GeneHeader => ("gene-header", "biomcp.gene-header/2"),
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
        Envelope<GwasPage> => ("gwas-page", "biomcp.gwas-page/3"),
        Envelope<GwasAssociationCard> => ("gwas-association", "biomcp.gwas-association/1"),
        Envelope<GwasLoci> => ("gwas-loci", "biomcp.gwas-loci/1"),
        Envelope<GwasStudy> => ("gwas-study", "biomcp.gwas-study/1"),
        Envelope<GwasAncestries> => ("gwas-ancestries", "biomcp.gwas-ancestries/1"),
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
        Envelope<StudyPage> => ("study-page", "biomcp.study-page/1"),
        Envelope<StudyCard> => ("study-card", "biomcp.study-card/2"),
        Envelope<StudyProfiles> => ("study-profiles", "biomcp.study-profiles/1"),
        Envelope<StudySampleLists> => ("study-sample-lists", "biomcp.study-sample-lists/1"),
        Envelope<StudyFiles> => ("study-files", "biomcp.study-files/1"),
    }
}

fn adverse_event_contracts() -> Vec<Contract> {
    contracts! {
        PgxSearch => ("pgx-search", "biomcp.pgx-search/2"),
        PgxGet => ("pgx-get", "biomcp.pgx-get/1"),
        PgxGetCommand => ("pgx-get-command", "biomcp.pgx-get-command/1"),
        PgxAnnotationGet => ("pgx-annotation-get", "biomcp.pgx-annotation-get/1"),
        Envelope<PgxPage> => ("pgx-page", "biomcp.pgx-page/2"),
        Envelope<PgxCard> => ("pgx-card", "biomcp.pgx-card/1"),
        Envelope<PgxGuidelines> => ("pgx-guidelines", "biomcp.pgx-guidelines/1"),
        Envelope<PgxRecommendations> => ("pgx-recommendations", "biomcp.pgx-recommendations/1"),
        Envelope<PgxDiplotype> => ("pgx-diplotype", "biomcp.pgx-diplotype/1"),
        Envelope<PgxAnnotations> => ("pgx-annotations", "biomcp.pgx-annotations/1"),
        Envelope<PgxAnnotation> => ("pgx-annotation", "biomcp.pgx-annotation/1"),
        AdverseEventSearch => ("adverse-event-search", "biomcp.adverse-event-search/2"),
        AdverseEventGet => ("adverse-event-get", "biomcp.adverse-event-get/1"),
        Envelope<VaersSummary> => ("vaers-summary", "biomcp.vaers-summary/1"),
        Envelope<AdverseEventPage> => ("adverse-event-page", "biomcp.adverse-event-page/2"),
        Envelope<AdverseEventFaersCard> => ("adverse-event-faers-card", "biomcp.adverse-event-faers-card/1"),
        Envelope<AdverseEventMaudeCard> => ("adverse-event-maude-card", "biomcp.adverse-event-maude-card/1"),
        Envelope<AdverseEventDrugs> => ("adverse-event-drugs", "biomcp.adverse-event-drugs/1"),
        Envelope<AdverseEventReactions> => ("adverse-event-reactions", "biomcp.adverse-event-reactions/1"),
        Envelope<AdverseEventIdentifiers> => ("adverse-event-identifiers", "biomcp.adverse-event-identifiers/1"),
        Envelope<AdverseEventDevices> => ("adverse-event-devices", "biomcp.adverse-event-devices/1"),
        Envelope<AdverseEventNarrative> => ("adverse-event-narrative", "biomcp.adverse-event-narrative/1"),
    }
}

fn variant_contracts() -> Vec<Contract> {
    contracts! {
        VariantAssertionsGet => ("variant-assertions", "biodata.variant-assertions/2"),
        Envelope<VariantAssertions> => ("variant-assertions-result", "biomcp.variant-assertions/4"),
        PopulationFrequency => ("population-frequency", "biodata.population-frequency/4"),
        VariantPopulationQuery => ("variant-population", "biodata.variant-population/3"),
        Envelope<VariantPopulationResult> => ("variant-population-result", "biomcp.variant-population/7"),
        Statement => ("statement", "biodata.statement/4"),
        VariantRecurrenceQuery => ("variant-recurrence-query", "biodata.variant-recurrence-query/3"),
        Envelope<VariantRecurrence> => ("variant-recurrence", "biomcp.variant-recurrence/6"),
        VariantNormalize => ("variant-normalize", "biodata.variant-normalize/4"),
        Envelope<VariantNormalization> => ("variant-normalization", "biomcp.variant-normalization/5"),
        VariantSearch => ("variant-search", "biodata.variant-search/2"),
        Envelope<VariantPage> => ("variant-page", "biomcp.variant-page/10"),
        VariantGet => ("variant-get", "biodata.variant-get/3"),
        Envelope<VariantCard> => ("variant-card", "biomcp.variant-card/9"),
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
        DrugNameSelector => ("drug-name-selector", "biomcp.drug-name-selector/1"),
        DrugSearch => ("drug-search", "biomcp.drug-search/2"),
        DrugGet => ("drug-get", "biomcp.drug-get/2"),
        DrugWhoGet => ("drug-who-get", "biomcp.drug-who-get/1"),
        DrugGetCommand => ("drug-get-command", "biomcp.drug-get-command/1"),
        DrugWhoRow => ("drug-who-row", "biomcp.drug-who-row/1"),
        Envelope<DrugWhoPage> => ("drug-who-page", "biomcp.drug-who-page/1"),
        Envelope<DrugWhoCard> => ("drug-who-card", "biomcp.drug-who-card/1"),
        DrugSource => ("drug-source", "biomcp.drug-source/1"),
        DrugSection => ("drug-section", "biomcp.drug-section/1"),
        DrugHeader => ("drug-header", "biomcp.drug-header/2"),
        Envelope<DrugPage> => ("drug-page", "biomcp.drug-page/3"),
        Envelope<DrugFdaPage> => ("drug-fda-page", "biomcp.drug-fda-page/2"),
        Envelope<DrugXrefs> => ("drug-xrefs", "biomcp.drug-xrefs/1"),
        Envelope<DrugCard> => ("drug-card", "biomcp.drug-card/2"),
        Envelope<DrugLabel> => ("drug-label", "biomcp.drug-label/3"),
        Envelope<DrugTargets> => ("drug-targets", "biomcp.drug-targets/3"),
        Envelope<DrugIndications> => ("drug-indications", "biomcp.drug-indications/3"),
        Envelope<DrugCivic> => ("drug-civic", "biomcp.drug-civic/6"),
        Envelope<DrugApprovals> => ("drug-approvals", "biomcp.drug-approvals/3"),
        Envelope<DrugRegulatory> => ("drug-regulatory", "biomcp.drug-regulatory/2"),
        Envelope<DrugInteractions> => ("drug-interactions", "biomcp.drug-interactions/3"),
    }
}

fn diagnostic_contracts() -> Vec<Contract> {
    contracts! {
        DiagnosticSearch => ("diagnostic-search", "biomcp.diagnostic-search/1"),
        DiagnosticGet => ("diagnostic-get", "biomcp.diagnostic-get/1"),
        Envelope<DiagnosticPage> => ("diagnostic-page", "biomcp.diagnostic-page/2"),
        Envelope<DiagnosticCard> => ("diagnostic-card", "biomcp.diagnostic-card/1"),
        Envelope<DiagnosticGenes> => ("diagnostic-genes", "biomcp.diagnostic-genes/2"),
        Envelope<DiagnosticConditions> => ("diagnostic-conditions", "biomcp.diagnostic-conditions/2"),
        Envelope<DiagnosticMethods> => ("diagnostic-methods", "biomcp.diagnostic-methods/2"),
        Envelope<DiagnosticRegulatory> => ("diagnostic-regulatory", "biomcp.diagnostic-regulatory/1"),
        Envelope<DiagnosticSupplements> => ("diagnostic-supplements", "biomcp.diagnostic-supplements/1"),
        Envelope<DiagnosticCompanion> => ("diagnostic-companion", "biomcp.diagnostic-companion/1"),
    }
}

fn trial_contracts() -> Vec<Contract> {
    contracts! {
        TrialGet => ("trial-get", "biodata.trial-get/1"),
        Envelope<NciTrialCards> => ("nci-trial-cards", "biomcp.nci-trial-cards/3"),
        Envelope<NciTrialPage> => ("nci-trial-page", "biomcp.nci-trial-page/5"),
        Envelope<TrialCards> => ("trial-cards", "biomcp.trial-cards/5"),
        TrialConditionResolution => ("trial-condition-resolution", "biomcp.trial-condition-resolution/2"),
        TrialSearch => ("trial-search", "biodata.trial-search/3"),
        Envelope<TrialPage> => ("trial-page", "biomcp.trial-page/9"),
        NctId => ("nct-id", "biodata.nct-id/1"),
        TrialHeader => ("trial-header", "biodata.trial_header/2"),
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
        Envelope<WikiPathwaysSync> => ("wikipathways-sync", "biomcp.wikipathways-sync/2"),
        Envelope<PathwayPage> => ("pathway-page", "biomcp.pathway-page/3"),
        Envelope<PathwayCard> => ("pathway-card", "biomcp.pathway-card/5"),
        Envelope<PathwayGenes> => ("pathway-genes", "biomcp.pathway-genes/4"),
        Envelope<PathwayEvents> => ("pathway-events", "biomcp.pathway-events/2"),
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
        Envelope<ProteinPage> => ("protein-page", "biomcp.protein-page/1"),
        Envelope<ProteinCard> => ("protein-card", "biomcp.protein-card/4"),
        Envelope<ProteinFeatures> => ("protein-features", "biomcp.protein-features/1"),
        Envelope<ProteinDomains> => ("protein-domains", "biomcp.protein-domains/3"),
        Envelope<ProteinIsoforms> => ("protein-isoforms", "biomcp.protein-isoforms/1"),
        Envelope<ProteinComplexes> => ("protein-complexes", "biomcp.protein-complexes/3"),
        Envelope<ProteinComplexMembers> => ("protein-complex-members", "biomcp.protein-complex-members/1"),
        Envelope<ProteinStructures> => ("protein-structures", "biomcp.protein-structures/1"),
        Envelope<ProteinSequenceResult> => ("protein-sequence", "biomcp.protein-sequence/1"),
        Envelope<ProteinFunctionResult> => ("protein-function", "biomcp.protein-function/2")
    }
}

#[path = "schemas/author.rs"]
mod author;

#[test]
fn generated_schemas_preserve_release_contracts() -> Result<(), Box<dyn std::error::Error>> {
    release::generated_schemas_preserve_release_contracts()
}
