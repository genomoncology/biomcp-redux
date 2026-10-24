#![doc = include_str!("../README.md")]

mod article;
pub use article::{
    AbstractPart, Article, ArticleInput, ArticleMatchKind, Correction, CorrectionKind,
    CorrectionRelation,
};
pub use article::{ArticleHeader, Indexing, MeshHeading, MeshQualifier, PublicationType};
pub use id::{Doi, EuropePmcId, Pmcid, Pmid, SemanticScholarId};
pub use query::{
    ArticleFilter, ArticleSearch, ArticleSort, ArticleSource, ArticleType, RetractionFilter,
};
pub use query::{ArticleGet, ArticleSection, ArticleSectionWord};
pub use query::{ArticleSupplement, ArticleText, SheetRows};
pub use response::HotspotMembership;
pub use response::{ArticleCardItem, ArticleCards, ArticleMatch, ArticleResolution};
pub use response::{
    ArticleItem, ArticleKey, ArticlePage, ArticleSentence, CivicMembership, CivicProfile,
    CivicStatement, HotspotAlternate, HotspotRecord, LitVarCandidate, PoolInfo, SourceNotice,
    VariantCivic, VariantHotspots,
};
pub use response::{
    ArticleSaved, ArticleTextResult, FigureCaption, FullText, License, TextContent, TextHit,
    TextPart, TextPartKind,
};
pub use response::{
    ArticleSupplementResult, CellFragment, FileKind, SheetInfo, SheetRow, SupplementContent,
    SupplementFile, Supplements,
};
pub use response::{CivicClaimPage, CivicMember};
mod response;
mod source;
mod text;

pub use response::{ErrorCode, ErrorEnvelope, ErrorMessage, ErrorStatus, Problem, ProblemCode};
pub use source::{Reason, Source, Use};
pub use text::Text;

mod coord;
mod date;
mod error;
mod id;
mod macros;

pub use coord::{Position1, Span0};
pub use date::PartialDate;
pub use error::Error;
pub use id::{
    Cytoband, EnsemblGeneId, EnsemblTranscript, EntrezGeneId, GeneSymbol, HgncId, OmimId,
    RefSeqTranscript, UniProtAcc,
};

mod gene;
pub use gene::{
    Gene, GeneHeader, GeneInput, GeneResolution, GeneStatus, HgncStatus, LocusGroup, LocusType,
    ManeSelect, Match, MatchKind, Summary, SummarySource,
};

pub use coord::{Build, Contig, GenomicInterval, Strand};

mod query;
pub use query::{GeneGet, GeneSearch, GeneSection, GeneSectionWord, Request};
pub use response::{
    Envelope, GeneCards, GeneItem, GenePage, ItemStatus, PageInfo, QueryBody, ResponseBody,
    Section, SourceOutcome, SourceRecord, SourceState, Status, Step, StepKind,
};

pub use response::{
    Help, HelpArgument, HelpCommand, HelpWord, KeyHelp, KeyState, Oncokb, OncokbCard,
    OncokbGeneType, OncokbTranscript, RegisteredKey, RegisteredKeys, SyncQuery, SyncResult,
};

pub use response::{Constraint, FiniteNumber, FiniteNumberError, GeneSections, Protein};

pub use response::{
    CancerPtpm, Capped, CivicGene, ClinGenGene, ClinGenValidity, GeneExpression, PathwayRef,
    ReactomeId, ReactomeIdInvalid, RnaAbundance, TissueNtpm,
};

pub use response::{AnnotatedEntity, AnnotationKind, Annotations, ArticleSections, Tldr};

pub use query::{ArticleEdges, EdgeSource};
pub use response::{ArticleEdgeItem, ArticleEdgePage, CitationIntent, Citations};
mod markdown;
pub use markdown::Markdown;
pub use response::{Health, HttpHealth, SourceHealth};

pub use query::{ArticleEvidence, ArticlePdf, ArticleRecommend};
pub use response::{
    ArticleEvidenceResult, ArticleRecommendations, CitationContexts, CitationParagraphs,
    EvidenceContext, EvidenceNotice, EvidenceParagraph,
};

mod variant;
pub use variant::{Representation, Variant};

pub use query::{CivicClaimKind, VariantGet, VariantResolver, VariantSection};
pub use response::{ArticlePdfResult, OpenAccessStatus, PdfLink, PdfSaved};
pub use response::{VariantCard, VariantHeader, VariantResolution, VariantSections};

pub use coord::Chromosome;

pub use variant::{
    Accession, Allele, Base, EnsemblFeature, LrgPart, RefSeqPrefix, Sequence, SequenceLocation,
};

pub use variant::{Expression, ExpressionSyntax, HgvsText, VariantId};

pub use query::{VariantInput, VariantNamespace};
mod trial;
pub use query::PhaseFilter;
pub use query::SponsorType;
pub use query::{TrialFilter, TrialSearch, TrialSource};
pub use response::TrialPage;
pub use response::TrialParameter;
pub use trial::{NctId, StudyType, TrialHeader, TrialPhase, TrialSex, TrialStatus};

mod trial_detail;
pub use trial_detail::{
    TrialAgeBound, TrialArm, TrialCard, TrialEligibility, TrialIntervention, TrialOutcome,
    TrialOutcomeKind,
};

pub use query::{TrialGet, TrialSection};
pub use response::{
    TrialArmRecord, TrialCardItem, TrialCards, TrialContactRecord, TrialEligibilityPage,
    TrialRecords, TrialSections,
};

pub use trial_detail::{TrialDate, TrialDateKind};

pub use trial_detail::{TrialArmKind, TrialInterventionKind};

pub use response::{
    NciDisease, NciDiseaseRef, NciTrial, NciTrialCards, NciTrialItem, NciTrialPage,
};

pub use response::{
    NciArm, NciArmRecord, NciCriterion, NciEligibility, NciEligibilityPage,
    NciEligibilityStructured, NciIntervention, NciOutcome, NciTextPage, NciTrialSections,
};
mod trial_sites;
pub use trial_sites::{
    TrialContact, TrialContactRole, TrialCoordinates, TrialLocation, TrialOfficial,
    TrialOfficialRole, TrialSiteStatus,
};

mod trial_publications;
pub use query::TrialDocumentGet;
pub use response::{TrialDocumentResult, TrialDocumentSaved};
pub use trial_publications::{TrialDocument, TrialReference};

pub use variant::{
    Attribution, Coding, Concept, Curie, Direction, Predicate, Proposition, Statement, Term,
    VariantRef,
};

pub use variant::{
    AcmgAmp2015Code, AmpAscoCap2017LevelCode, AmpAscoCap2017TierCode, ClingenCgcVicc2022Code,
    ClingenLowPenetrance2024Code,
};

pub use response::{
    GwasAssociation, GwasEffectAllele, PopulationGroup, PopulationSubset, VariantGwas,
    VariantPopulation, VariantPopulationDetail,
};

pub use variant::CivicEvidenceLevelCode;

pub use query::VariantPopulationQuery;
pub use query::{AssertionSource, VariantAssertionsGet};
pub use response::{
    AssertionCondition, AssertionDetail, AssertionDomain, AssertionKind, UnmappedAssertion,
    VariantAssertion, VariantAssertions,
};
pub use variant::{
    ComponentTerms, CosmicCoordinates, CosmicIdentity, CosmicRecord, PredictorScore,
    VariantComponent, VariantConservation, VariantCosmic, VariantPredictors,
};

pub use response::{OncokbVariantRecord, VariantOncokb};
pub use variant::PopulationFrequency;

pub use response::VariantPopulationResult;

pub use query::{MyVariantQuery, ProteinChange, VariantRegion, VariantSearch, VariantSearchSource};
pub use response::GwasTrait;
pub use response::VariantPage;
