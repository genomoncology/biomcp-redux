mod civic;
mod clingen;
mod gencc;
pub use gencc::{GenccGene, GenccSubmission};
mod constraint;
mod expression;
mod oncokb;
mod pathways;
mod protein;
use super::{ItemStatus, ResponseBody, Section};
use crate::GeneSection;
use crate::{GeneGet, GeneHeader, GeneInput, GeneResolution, GeneSearch};
pub use civic::CivicGene;
pub use clingen::{ClinGenGene, ClinGenValidity};
pub use constraint::Constraint;
pub use expression::{CancerPtpm, GeneExpression, RnaAbundance, TissueNtpm};
pub use oncokb::{Oncokb, OncokbCard, OncokbGeneType, OncokbTranscript};
pub use pathways::{Capped, PathwayRef, ReactomeId};
pub use protein::Protein;
use serde::{Deserialize, Serialize};
/// Answer to one requested input.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GeneItem {
    /// Verified keyed source card annotation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oncokb: Option<OncokbCard>,
    /// Original checked input.
    pub input: GeneInput,
    /// This input's outcome.
    pub status: ItemStatus,
    /// Absent only when identity lookup failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<GeneResolution>,
    /// Requested sections, present only for a resolved identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<GeneSections>,
    /// Applicable sections left out of this request.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub more: Box<[GeneSection]>,
}
/// Gene answers in input order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GeneCards {
    /// Requested inputs and their answers.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[GeneItem]>,
}
/// One page of gene headers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GenePage {
    /// Headers after ranking and paging.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[GeneHeader]>,
}
impl ResponseBody for GeneCards {
    const SCHEMA: &'static str = "biomcp.gene-cards/8";
    type Query = GeneGet;
}
impl ResponseBody for GenePage {
    const SCHEMA: &'static str = "biomcp.gene-page/5";
    type Query = GeneSearch;
}
/// Typed dispatch result before the interface constructs an envelope.
#[derive(Clone, Debug)]
pub enum QueryBody {
    /// Native source author candidates.
    AuthorPage(Option<crate::AuthorPage>),
    /// Exact provider author cards and per-input outcomes.
    AuthorCards(crate::AuthorCards),
    /// Compact indexed papers.
    AuthorPapers(Option<crate::AuthorPapers>),
    /// Detailed indexed papers.
    AuthorPaperDetails(Option<crate::AuthorPaperDetails>),
    /// Public ORCID groups.
    OrcidWorks(Option<crate::OrcidWorks>),
    /// Exact accession-linked native metadata.
    CellLineChembl(crate::CellLineChembl),
    /// Native assay total without assay listings.
    CellLineAssayCount(crate::CellLineAssayCount),
    /// Native source-ordered cell line headers.
    CellLinePage(crate::CellLinePage),
    /// Exact cell identity or unselected candidates.
    CellLineCard(crate::CellLineCard),
    /// Native GWAS association page, absent after failure.
    GwasPage(Option<crate::GwasPage>),
    /// Exact source association card.
    GwasAssociation(Option<crate::GwasAssociationCard>),
    /// Exact source study card.
    GwasStudy(Option<crate::GwasStudy>),
    /// Source association locus context.
    GwasLoci(Option<crate::GwasLoci>),
    /// Local retained study ancestry rows.
    GwasAncestries(Option<crate::GwasAncestries>),
    /// Native InterPro match page.
    ProteinDomains(Option<crate::ProteinDomains>),
    /// Exact participant-admitted complex page.
    ProteinComplexes(Option<crate::ProteinComplexes>),
    /// Native members of one exact containing complex.
    ProteinComplexMembers(Option<crate::ProteinComplexMembers>),
    /// Exact filtered native features.
    ProteinFeatures(Option<crate::ProteinFeatures>),
    /// Native alternative products.
    ProteinIsoforms(Option<crate::ProteinIsoforms>),
    /// Source PDB cross-reference properties.
    ProteinStructures(Option<crate::ProteinStructures>),
    /// Whole exact source sequence.
    ProteinSequence(Option<crate::ProteinSequenceResult>),
    /// Native reviewed-human protein headers.
    ProteinPage(crate::ProteinPage),
    /// Caller-ordered exact protein cards and unselected candidates.
    ProteinCard(crate::ProteinCard),
    /// Native ordered HP term candidates.
    PhenotypePage(crate::PhenotypePage),
    /// Complete independent publisher term card.
    PhenotypeCard(Option<crate::PhenotypeTermCard>),
    /// Native source synonym page.
    PhenotypeSynonyms(Option<crate::PhenotypeSynonyms>),
    /// Native source cross-reference page.
    PhenotypeXrefs(Option<crate::PhenotypeXrefs>),
    /// Native source direct parent and child links.
    PhenotypeHierarchy(Option<crate::PhenotypeHierarchy>),
    /// Exact direct native disease association rows.
    PhenotypeDiseases(crate::PhenotypeDiseases),
    /// Exact direct native human gene association rows.
    PhenotypeGenes(crate::PhenotypeGenes),
    /// Native ordered human pathway headers.
    PathwayPage(crate::PathwayPage),
    /// Native NCI biomarker headers.
    BiomarkerPage(crate::BiomarkerPage),
    /// Exact native biomarker card or candidates.
    BiomarkerResult(crate::BiomarkerResult),
    /// Biomarker observation exceeded its bound.
    BiomarkerTooBroad,
    /// Native NCI organization observation.
    OrganizationPage(crate::OrganizationPage),
    /// Unsized organization observation exceeded its byte bound.
    OrganizationTooBroad,
    /// Checked exact pathway card or summary.
    PathwayCard(Option<crate::PathwayCard>),
    /// Retained native membership references.
    PathwayGenes(Option<crate::PathwayGenes>),
    /// Retained native contained events.
    PathwayEvents(Option<crate::PathwayEvents>),
    /// Source-ordered study catalog metadata.
    StudyPage(Option<crate::StudyPage>),
    /// Exact stored study metadata.
    StudyCard(Option<crate::StudyCard>),
    /// Independently paged native molecular-profile metadata.
    StudyProfiles(Option<crate::StudyProfiles>),
    /// Independently paged sample-list metadata without sample members.
    StudySampleLists(Option<crate::StudySampleLists>),
    /// Source-owned study file metadata and explicit download pointers.
    StudyFiles(Option<crate::StudyFiles>),
    /// Whole physical FDA companion association rows.
    DiagnosticCompanion(crate::DiagnosticCompanion),
    /// Source-native CPIC pair page.
    PgxPage(crate::PgxPage),
    /// Exact source gene identity, absent after failure or source-local zero.
    PgxCard(Option<crate::PgxCard>),
    /// Independent native recommendation page.
    PgxRecommendations(crate::PgxRecommendations),
    /// Native guideline references.
    PgxGuidelines(crate::PgxGuidelines),
    /// Complete source diplotype lookup.
    PgxDiplotype(crate::PgxDiplotype),
    /// One independently paged ClinPGx min collection.
    PgxAnnotations(crate::PgxAnnotations),
    /// Exact clinical base annotation.
    PgxAnnotation(Option<crate::PgxAnnotation>),
    /// Source-ordered diagnostic candidates.
    DiagnosticPage(crate::DiagnosticPage),
    /// Exact selected diagnostic card.
    DiagnosticCard(Option<crate::DiagnosticCard>),
    /// Native gene membership rows.
    DiagnosticGenes(crate::DiagnosticGenes),
    /// Native condition membership rows.
    DiagnosticConditions(crate::DiagnosticConditions),
    /// Independent native method entries.
    DiagnosticMethods(crate::DiagnosticMethods),
    /// Exact selected submission statement fragments.
    DiagnosticRegulatory(crate::DiagnosticRegulatory),
    /// Native supplement headers for one exact application.
    DiagnosticSupplements(crate::DiagnosticSupplements),
    /// Report get belongs to its next runtime slice.
    UnsupportedAdverseEvent,
    /// Exact checked FAERS report.
    AdverseEventFaersCard(Option<crate::AdverseEventFaersCard>),
    /// Independent native drug rows.
    AdverseEventDrugs(Option<crate::AdverseEventDrugs>),
    /// Independent native reaction rows.
    AdverseEventReactions(Option<crate::AdverseEventReactions>),
    /// One original harmonized identifier array.
    AdverseEventIdentifiers(Option<crate::AdverseEventIdentifiers>),
    /// Source-native adverse event report page.
    AdverseEventPage(crate::AdverseEventPage),
    /// Independent native vaccine aggregate table.
    VaersSummary(Option<crate::VaersSummary>),
    /// Exact native device report card.
    AdverseEventMaudeCard(Option<crate::AdverseEventMaudeCard>),
    /// Complete native device rows.
    AdverseEventDevices(Option<crate::AdverseEventDevices>),
    /// Whole native narrative rows.
    AdverseEventNarrative(Option<crate::AdverseEventNarrative>),
    /// Native disease candidate page.
    DiseasePage(crate::DiseasePage),
    /// Exact source concept when present.
    DiseaseCard(Option<crate::DiseaseCard>),
    /// Native scoped synonym page.
    DiseaseSynonyms(crate::DiseaseSynonyms),
    /// Native cross-reference page.
    DiseaseXrefs(crate::DiseaseXrefs),
    /// Native parent relations.
    DiseaseHierarchy(crate::DiseaseHierarchy),
    /// Native source gene association rows.
    DiseaseGenes(crate::DiseaseGenes),
    /// Native source phenotype association rows.
    DiseasePhenotypes(crate::DiseasePhenotypes),
    /// Native site catalog.
    DiseaseSites(crate::DiseaseSites),
    /// Native incidence rows.
    DiseaseIncidence(crate::DiseaseIncidence),
    /// Native survival rows.
    DiseaseSurvival(crate::DiseaseSurvival),
    /// Native candidate page.
    DrugPage(crate::DrugPage),
    /// WHO prequalification rows, independent of authorization facts.
    DrugWhoPage(crate::DrugWhoPage),
    /// One exact WHO reference and its unchanged native rows.
    DrugWhoCard(Option<crate::DrugWhoCard>),
    /// Exact source identity, absent on failed retrieval.
    DrugCard(Option<crate::DrugCard>),
    /// Original native label text with mechanical fragment continuation.
    DrugLabel(Option<crate::DrugLabel>),
    /// Exact EMA record and native regulator fields from one local snapshot.
    DrugRegulatory(Option<crate::DrugRegulatory>),
    /// FDA application submissions and independent native products.
    DrugApprovals(Option<crate::DrugApprovals>),
    /// Source-native FDA application and product candidate page.
    DrugFdaPage(crate::DrugFdaPage),
    /// FDA native drug recall records.
    DrugRecallPage(crate::DrugRecallPage),
    /// Exact native drug recall.
    DrugRecallCard(Option<crate::DrugRecallCard>),
    /// Source-local native drug interaction partners.
    DrugInteractions(Option<crate::DrugInteractions>),

    /// An exact selected document's bounded native link page.
    DrugXrefs(Option<crate::DrugXrefs>),
    /// Source-native drug target and mechanism page.
    DrugTargets(Option<crate::DrugTargets>),
    /// Source-native drug indication page.
    DrugIndications(Option<crate::DrugIndications>),
    /// Source-local CIViC therapy evidence and assertion streams.
    DrugCivic(Option<crate::DrugCivic>),
    /// Source-native variant assertions.
    VariantAssertions(crate::VariantAssertions),
    /// Independent provider normalization results.
    VariantNormalization(crate::VariantNormalization),
    /// Native NCI discovery page.
    NciTrialPage(crate::NciTrialPage),
    /// Native NCI exact cards.
    NciTrialCards(crate::NciTrialCards),
    /// A selected trial provider cannot apply a checked capability.
    UnsupportedTrialFilter {
        /// Explicit source that cannot apply the option.
        source: crate::Source,
        /// Checked trial selector, without caller values.
        option: crate::TrialFilter,
    },
    /// Fixed NCI disease identity guidance.
    NciDiseaseError,
    /// Opaque source syntax cannot enforce a selected component terms restriction.
    VariantQueryPolicyError,

    /// Source-qualified variant identity.
    VariantCard(crate::VariantCard),
    /// Source population observations.
    VariantPopulation(crate::VariantPopulationResult),
    /// Bounded cohort sample observations.
    VariantRecurrence(crate::VariantRecurrence),
    /// Native MyVariant candidate page.
    VariantPage(crate::VariantPage),
    /// Source trial document receipt.
    TrialDocument(crate::TrialDocumentResult),
    /// Exact trial cards and source protocol sections.
    TrialCards(crate::TrialCards),
    /// Native registry search page.
    TrialPage(crate::TrialPage),
    /// Explicit PDF save result.
    ArticlePdf(crate::ArticlePdfResult),
    /// Invalid destination or unavailable save surface, retaining the source link.
    PdfSaveRefused(crate::PdfLink),
    /// An explicit article save was refused, retaining its source URL.
    ArticleSaveRefused(crate::Text),
    /// Citation context evidence.
    ArticleEvidence(crate::ArticleEvidenceResult),
    /// Source-native recommendations.
    ArticleRecommendations(crate::ArticleRecommendations),
    /// Licensed supplement reader.
    ArticleSupplement(crate::ArticleSupplementResult),
    /// A selected source cannot apply the checked filters.
    UnsupportedFilter {
        /// Selected source that cannot apply the filter.
        source: crate::Source,
        /// Exact unsupported selector, without caller text.
        option: crate::ArticleFilter,
    },
    /// Bounded licensed body reader.
    ArticleText(crate::ArticleTextResult),
    /// Invalid body reader position.
    TextPositionError,
    /// Citation graph page.
    ArticleEdges(crate::ArticleEdgePage),
    /// Citation ordering exceeds the supported PMID window.
    TooManyPmids {
        /// Unique PMID count before enrichment.
        count: u64,
    },
    /// PubMed article cards.
    ArticleCards(crate::ArticleCards),
    /// Invalid snapshot selector; the interface spells its typed restart.
    PoolError {
        /// Whether selectors differ from an existing snapshot.
        mismatch: bool,
    },
    /// PubMed search page.
    ArticlePage(crate::ArticlePage),
    /// Gene get answers.
    GeneCards(GeneCards),
    /// Gene search page.
    GenePage(GenePage),
}

/// Explicit source synchronization selector.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SyncQuery {
    /// Source whose complete identity data is synchronized.
    pub source: crate::Source,
}
/// Published bundle metadata after synchronization.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SyncResult {
    /// Catalog content revision and acquisition time, separate from source headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<crate::StudySnapshot>,
    /// Native publisher ETag when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<Box<str>>,
    /// Native publisher Last-Modified when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<Box<str>>,
    /// Imported rows.
    pub rows: u64,
    /// Imported source-file bytes before native file decoding.
    pub bytes: u64,
    /// Source body bytes received in this sync; excludes previous partials, 304 reuse and replay.
    pub downloaded_bytes: u64,
    /// Distinct repeated identities across the whole source version report.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duplicate_identities: Option<u64>,
}
impl ResponseBody for SyncResult {
    const SCHEMA: &'static str = "biomcp.sync/5";
    type Query = SyncQuery;
}

/// Landed gene sections, each retaining its source and explicit state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GeneSections {
    /// Reviewed human proteins listed by HGNC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protein: Option<Section<Box<[Protein]>>>,
    /// CIViC gene-level curation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub civic: Option<Section<CivicGene>>,
    /// ClinGen validity and dosage assessments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clingen: Option<Section<ClinGenGene>>,
    /// GenCC assessments, retained separately for each submitter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gencc: Option<Section<GenccGene>>,
    /// Population constraint reported by gnomAD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint: Option<Section<Constraint>>,
    /// Human Protein Atlas expression annotations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<Section<GeneExpression>>,
    /// Lowest-level human Reactome mappings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pathways: Option<Section<Capped<PathwayRef>>>,
    /// Keyed source gene details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oncokb: Option<Section<Oncokb>>,
}
