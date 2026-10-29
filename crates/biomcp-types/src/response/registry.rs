//! Typed reply discovery shared by schema publication and JSON readers.
use crate::*;
use serde::de::DeserializeOwned;

/// A schema identity and the concrete response type which reads it.
#[derive(Debug)]
pub struct ReplyRegistration {
    /// Artifact file stem.
    pub name: &'static str,
    /// Identity supplied by the response body.
    pub schema: &'static str,
    /// Read the concrete envelope and serialize it without changing its JSON shape.
    pub read_write: fn(&[u8]) -> Result<Vec<u8>, serde_json::Error>,
    /// Generate the same envelope schema used by publication.
    #[cfg(feature = "schema")]
    pub generate: fn() -> serde_json::Value,
}
fn read_write<T>(bytes: &[u8]) -> Result<Vec<u8>, serde_json::Error>
where
    T: ResponseBody + DeserializeOwned,
    T::Query: DeserializeOwned,
{
    let reply: Envelope<T> = serde_json::from_slice(bytes)?;
    serde_json::to_vec(&reply)
}
#[cfg(feature = "schema")]
fn generate<T>() -> serde_json::Value
where
    T: ResponseBody + schemars::JsonSchema,
    T::Query: schemars::JsonSchema,
{
    schemars::generate::SchemaSettings::draft2020_12()
        .into_generator()
        .into_root_schema_for::<Envelope<T>>()
        .to_value()
}
macro_rules! replies {
    ($($body:ty => $name:literal),+ $(,)?) => {
        &[$(ReplyRegistration {
            name: $name,
            schema: <$body as ResponseBody>::SCHEMA,
            read_write: read_write::<$body>,
            #[cfg(feature = "schema")]
            generate: generate::<$body>,
        }),+]
    };
}
/// Every enveloped reply, with identities taken from the registered body types.
pub fn reply_registry() -> &'static [ReplyRegistration] {
    replies! {
        ArticleEvidenceResult => "article-evidence-result",
        ArticleRecommendations => "article-recommendations",
        ArticlePdfResult => "article-pdf-result",
        ArticleSupplementResult => "article-supplement-result",
        ArticleTextResult => "article-text-result",
        ArticleEdgePage => "article-edge-page",
        ArticleCards => "article-cards",
        ArticlePage => "article-page",
        OrganizationPage => "organization-page",
        TrialDocumentResult => "trial-document-result",
        Health => "health",
        SchemaCatalog => "schema-catalog",
        Help => "help",
        KeyHelp => "key",
        RegisteredKeys => "registered-keys",
        GeneCards => "gene-cards",
        GenePage => "gene-page",
        StoreClean => "store-clean",
        SyncResult => "sync",
        GwasPage => "gwas-page",
        GwasAssociationCard => "gwas-association",
        GwasLoci => "gwas-loci",
        GwasStudy => "gwas-study",
        GwasAncestries => "gwas-ancestries",
        StudyPage => "study-page",
        StudyCard => "study-card",
        StudyProfiles => "study-profiles",
        StudySampleLists => "study-sample-lists",
        StudyFiles => "study-files",
        PgxPage => "pgx-page",
        PgxCard => "pgx-card",
        PgxGuidelines => "pgx-guidelines",
        PgxRecommendations => "pgx-recommendations",
        PgxDiplotype => "pgx-diplotype",
        PgxAnnotations => "pgx-annotations",
        PgxAnnotation => "pgx-annotation",
        VaersSummary => "vaers-summary",
        AdverseEventPage => "adverse-event-page",
        AdverseEventFaersCard => "adverse-event-faers-card",
        AdverseEventMaudeCard => "adverse-event-maude-card",
        AdverseEventDrugs => "adverse-event-drugs",
        AdverseEventReactions => "adverse-event-reactions",
        AdverseEventIdentifiers => "adverse-event-identifiers",
        AdverseEventDevices => "adverse-event-devices",
        AdverseEventNarrative => "adverse-event-narrative",
        VariantAssertions => "variant-assertions-result",
        VariantPopulationResult => "variant-population-result",
        VariantRecurrence => "variant-recurrence",
        VariantNormalization => "variant-normalization",
        VariantPage => "variant-page",
        VariantCard => "variant-card",
        DrugWhoPage => "drug-who-page",
        DrugWhoCard => "drug-who-card",
        DrugPage => "drug-page",
        DrugFdaPage => "drug-fda-page",
        DrugXrefs => "drug-xrefs",
        DrugCard => "drug-card",
        DrugLabel => "drug-label",
        DrugTargets => "drug-targets",
        DrugIndications => "drug-indications",
        DrugCivic => "drug-civic",
        DrugApprovals => "drug-approvals",
        DrugRegulatory => "drug-regulatory",
        DrugInteractions => "drug-interactions",
        DiagnosticPage => "diagnostic-page",
        DiagnosticCard => "diagnostic-card",
        DiagnosticGenes => "diagnostic-genes",
        DiagnosticConditions => "diagnostic-conditions",
        DiagnosticMethods => "diagnostic-methods",
        DiagnosticRegulatory => "diagnostic-regulatory",
        DiagnosticSupplements => "diagnostic-supplements",
        DiagnosticCompanion => "diagnostic-companion",
        NciTrialCards => "nci-trial-cards",
        NciTrialPage => "nci-trial-page",
        TrialCards => "trial-cards",
        TrialPage => "trial-page",
        WikiPathwaysSync => "wikipathways-sync",
        PathwayPage => "pathway-page",
        PathwayCard => "pathway-card",
        PathwayGenes => "pathway-genes",
        PathwayEvents => "pathway-events",
        ProteinPage => "protein-page",
        ProteinCard => "protein-card",
        ProteinFeatures => "protein-features",
        ProteinDomains => "protein-domains",
        ProteinIsoforms => "protein-isoforms",
        ProteinComplexes => "protein-complexes",
        ProteinComplexMembers => "protein-complex-members",
        ProteinStructures => "protein-structures",
        ProteinSequenceResult => "protein-sequence",
        ProteinFunctionResult => "protein-function",
        DiseasePage => "disease-page",
        DiseaseCard => "disease-card",
        DiseaseSynonyms => "disease-synonyms",
        DiseaseXrefs => "disease-xrefs",
        DiseaseHierarchy => "disease-hierarchy",
        DiseaseGenes => "disease-genes",
        DiseasePhenotypes => "disease-phenotypes",
        DiseaseSites => "disease-sites",
        DiseaseIncidence => "disease-incidence",
        DiseaseSurvival => "disease-survival",
        CellLinePage => "cell-line-page",
        CellLineCard => "cell-line-card",
        CellLineNames => "cell-line-names",
        CellLineIdentityPage => "cell-line-identity",
        CellLineXrefs => "cell-line-xrefs",
        CellLineVariants => "cell-line-variants",
        CellLineChembl => "cell-line-chembl",
        CellLineAssayCount => "cell-line-assay-count",
        PhenotypePage => "phenotype-search",
        PhenotypeTermCard => "phenotype-card",
        PhenotypeSynonyms => "phenotype-synonyms",
        PhenotypeXrefs => "phenotype-xrefs",
        PhenotypeHierarchy => "phenotype-hierarchy",
        PhenotypeDiseases => "phenotype-diseases",
        PhenotypeGenes => "phenotype-genes",
        AuthorPage => "author-page",
        AuthorCards => "author-card",
        AuthorPapers => "author-papers",
        AuthorPaperDetails => "author-paper-details",
        OrcidWorks => "orcid-works",
    }
}
