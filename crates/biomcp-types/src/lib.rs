#![doc = include_str!("../README.md")]
// The fixed migration-message schema expands one enum of wire literals.
#![recursion_limit = "256"]
use biodata::*;

mod research;
pub use research::{
    ResearchEnvelope, ResearchFetch, ResearchHit, ResearchSearch, ResearchSearchMetadata,
};
mod predictor;
mod query;
// Export new public response types inside `response` and again at the crate root.
// `unreachable_pub` reports a missing crate-root re-export.
mod response;
pub use response::{SchemaCatalog, SchemaEntry};
pub use response::{SourceTimestamp, SourceTimestampTimezone};
mod source;

pub use predictor::{
    ComponentLabel, ComponentTerms, ComponentUpload, CosmicCoordinates, CosmicIdentity,
    CosmicRecord, PredictorContext, SubjectQualification, VariantComponent, VariantConservation,
    VariantCosmic, VariantPredictors,
};
pub use query::PhaseFilter;
pub use query::SponsorType;
pub use query::TrialDocumentGet;
pub use query::VariantPopulationQuery;
pub use query::{ArticleEdges, EdgeSource};
pub use query::{ArticleEvidence, ArticlePdf, ArticleRecommend};
pub use query::{
    ArticleFilter, ArticleSearch, ArticleSort, ArticleSource, ArticleType, RetractionFilter,
};
pub use query::{ArticleGet, ArticleSection, ArticleSectionWord};
pub use query::{ArticleSupplement, ArticleText, SheetRows};
pub use query::{CivicClaimKind, VariantSection};
pub use query::{GeneGet, GeneSearch, GeneSection, GeneSectionWord, Request};
pub use query::{
    MyVariantQuery, VariantRegion, VariantSearch, VariantSearchField, VariantSearchSource,
    protein_query_forms,
};
pub use query::{NormalizationSource, VariantGet, VariantNormalize, VariantResolver};
pub use query::{RecurrenceSource, VariantRecurrenceQuery};
pub use query::{TrialFilter, TrialSearch, TrialSource};
pub use query::{TrialGet, TrialSection};
pub use query::{VariantInput, VariantNamespace};
pub use response::ComponentHealth;
pub use response::TrialParameter;
pub use response::VariantPage;
pub use response::{AnnotatedEntity, AnnotationKind, Annotations, ArticleSections, Tldr};
pub use response::{ArticleCardItem, ArticleCards, ArticleMatch, ArticleResolution};
pub use response::{ArticleEdgeItem, ArticleEdgePage, CitationIntent, Citations};
pub use response::{
    ArticleEvidenceResult, ArticleRecommendations, CitationContexts, CitationParagraphs,
    EvidenceContext, EvidenceNotice, EvidenceParagraph,
};
pub use response::{
    ArticleItem, ArticleKey, ArticlePage, ArticleSentence, CacheProblem, LitVarCandidate, PoolInfo,
    SourceNotice,
};
pub use response::{ArticlePdfResult, OpenAccessStatus, PdfLink, PdfSaved};
pub use response::{
    ArticleSaved, ArticleTextResult, FigureCaption, FullText, TextContent, TextHit, TextPart,
    TextPartKind,
};
pub use response::{
    ArticleSupplementResult, CellFragment, FileKind, FirstSheetRow, SheetInfo, SheetRow,
    SupplementContent, SupplementFile, Supplements,
};
pub use response::{
    CancerPtpm, Capped, CivicGene, ClinGenGene, ClinGenValidity, GenccGene, GenccSubmission,
    GeneExpression, PathwayRef, ReactomeId, ReactomeIdInvalid, RnaAbundance, TissueNtpm,
};
pub use response::{
    CivicClaimPage, CivicMember, CivicMembership, CivicNativeClaim, CivicProfile, CivicStatement,
    HotspotAlternate, HotspotMembership, HotspotRecord, OncokbCitation, OncokbVariantRecord,
    VariantCivic, VariantHotspots, VariantOncokb, VariantSections,
};
pub use response::{
    ConfigurationHealth, ConfigurationNotice, ConfigurationNoticeCode, ConfigurationVariable,
    DisableGroup, DisabledComponent, Health, HttpHealth, LocalBundleState, SourceHealth,
};
pub use response::{Constraint, GeneSections, Protein};
pub use response::{
    Envelope, GeneCards, GeneItem, GenePage, ItemStatus, PageInfo, QueryBody, ResponseBody,
    Section, SourceOutcome, SourceRecord, SourceState, Status, Step, StepKind,
};
pub use response::{ErrorCode, ErrorEnvelope, ErrorMessage, ErrorStatus, Problem, ProblemCode};
pub use response::{
    GnomadCounts, GwasAssociation, GwasEffectAllele, GwasTrait, VariantGwas, VariantPopulation,
    VariantPopulationResult,
};
pub use response::{
    Help, HelpArgument, HelpCommand, HelpQuery, HelpWord, KeyHelp, KeyState, Oncokb, OncokbCard,
    OncokbGeneType, OncokbTranscript, RegisteredKey, RegisteredKeys, StoreClean, SyncQuery,
    SyncResult,
};
pub use response::{
    NciArm, NciArmRecord, NciCriterion, NciEligibility, NciEligibilityPage,
    NciEligibilityStructured, NciIntervention, NciOutcome, NciTextPage, NciTrialSections,
};
pub use response::{
    NciDisease, NciDiseaseRef, NciTrial, NciTrialCards, NciTrialItem, NciTrialPage,
};
pub use response::{
    NormalizationField, NormalizationMessage, NormalizationResult, VariantNormalization,
};
pub use response::{RecurrenceGranularity, RecurrenceStudy, RecurrenceTerms, VariantRecurrence};
pub use response::{
    TrialArmRecord, TrialCardItem, TrialCards, TrialContactRecord, TrialEligibilityPage,
    TrialRecords, TrialSections,
};
pub use response::{TrialConditionResolution, TrialPage};
pub use response::{TrialDocumentResult, TrialDocumentSaved};
pub use response::{VariantCard, VariantHeader, VariantResolution, VariantTranscript};
pub use source::{Reason, Source, Use};

mod article;

pub use article::{
    AbstractPart, Article, ArticleInput, ArticleMatchKind, Correction, CorrectionKind,
    CorrectionRelation,
};
pub use article::{ArticleHeader, Indexing, MeshHeading, MeshQualifier, PublicationType};

pub use query::GeneInput;
pub use response::{GeneHeader, GeneResolution, Match};

pub use response::TrialHeader;

pub use query::{AssertionSource, VariantAssertionsGet};
pub use response::{
    AssertionAssessment, AssertionCondition, AssertionConditionRelationship, AssertionDetail,
    AssertionDocument, AssertionDomain, AssertionKind, UnmappedAssertion, VariantAssertion,
    VariantAssertions,
};

pub use query::{DiseaseGet, DiseaseOntology, DiseaseSearch, DiseaseSection, DiseaseSource};

pub use response::{
    DiseaseAssociationItem, DiseaseCard, DiseaseGenes, DiseaseHeader, DiseaseHierarchy,
    DiseaseIncidence, DiseasePage, DiseasePhenotypes, DiseaseSite, DiseaseSites, DiseaseSurvival,
    DiseaseSynonyms, DiseaseXrefs,
};
mod drug;
pub use drug::{
    DrugApprovals, DrugCard, DrugCivic, DrugCivicClaim, DrugCivicMember, DrugCivicPage,
    DrugCivicProfile, DrugCivicStreams, DrugFdaCandidate, DrugFdaDocument, DrugFdaIngredient,
    DrugFdaPage, DrugFdaProduct, DrugFdaProperty, DrugFdaSubmission, DrugGet, DrugHeader,
    DrugIndications, DrugInteractions, DrugLabel, DrugLabelFragment, DrugLabelText, DrugNameMatch,
    DrugNameSelector, DrugOncokbLookup, DrugOncokbMissing, DrugPage, DrugRecords, DrugRegulatory,
    DrugSearch, DrugSection, DrugSource, DrugTargets, DrugWhoCard, DrugWhoGet, DrugWhoPage,
    DrugWhoRow, DrugXrefs,
};

pub use query::{CheckedQuery, QueryInvalid};

mod adverse_event;
pub use adverse_event::{
    AdverseEventDevice, AdverseEventDevices, AdverseEventDrug, AdverseEventDrugs,
    AdverseEventFaersCard, AdverseEventGet, AdverseEventHeader, AdverseEventIdentifierRecords,
    AdverseEventIdentifierSection, AdverseEventIdentifiers, AdverseEventIdentityCounts,
    AdverseEventMaudeCard, AdverseEventNarrative, AdverseEventNarrativeItem, AdverseEventPage,
    AdverseEventReaction, AdverseEventReactionMatch, AdverseEventReactions, AdverseEventRecords,
    AdverseEventSearch, AdverseEventSection, AdverseEventSerious, AdverseEventSource, ReportUnit,
    VAERS_INFLUENZA, VaersCell, VaersNotice, VaersRow, VaersScope, VaersSection, VaersSummary,
    vaers_vaccine_code,
};

mod pgx;
pub use pgx::*;
pub use query::{
    DiagnosticGet, DiagnosticGetSource, DiagnosticSearch, DiagnosticSearchSource, DiagnosticSection,
};
pub use response::{
    DiagnosticCandidate, DiagnosticCard, DiagnosticCompanion, DiagnosticConditions,
    DiagnosticGenes, DiagnosticLaboratory, DiagnosticMethods, DiagnosticPage, DiagnosticRegulatory,
    DiagnosticRegulatoryField, DiagnosticRegulatoryFragment, DiagnosticSupplements, GtrCard,
    GtrHeader, GtrNativeStatus,
};

pub use response::{
    DiagnosticClearance, DiagnosticPmaCard, DiagnosticPmaHeader, DiagnosticPmaRegulatory,
};

/// Source-specific diagnostic page counting vocabulary.
pub use response::DiagnosticCountUnit;

#[cfg(feature = "schema")]
pub use drug::DrugGetCommand;

mod gwas;
pub use gwas::*;
mod phenotype;
pub use phenotype::*;
pub use query::{
    PathwayEventsQuery, PathwayGenesQuery, PathwayGet, PathwaySearch, PathwaySection, PathwaySource,
};

pub use response::{
    PathwayCard, PathwayEvents, PathwayGenes, PathwayHeader, PathwayPage, PathwayPageInfo,
    PathwayTotalKind, WikiPathwaysCatalogReceipt, WikiPathwaysSync,
};

mod protein;
pub use protein::*;
mod study;
pub use study::*;

mod cell_line;
pub use cell_line::*;

mod author;
pub use author::*;

pub use response::{
    AlphaGenomeBiosample, AlphaGenomeDataType, AlphaGenomeGene, AlphaGenomeInterval,
    AlphaGenomeOntology, AlphaGenomeScorer, AlphaGenomeScorerPage, AlphaGenomeTrack,
    AlphaGenomeValue, AlphaGenomeVariant, VariantAlphaGenome,
};
pub use response::{
    AlphaGenomeBiosampleType, AlphaGenomeEndedness, AlphaGenomeOntologyType, AlphaGenomeStrand,
};
pub use response::{AlphaGenomeNonfiniteKind, AlphaGenomeScalar};
