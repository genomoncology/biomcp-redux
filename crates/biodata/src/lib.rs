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
pub use response::{ArticleCardItem, ArticleCards, ArticleMatch, ArticleResolution};
pub use response::{
    ArticleItem, ArticleKey, ArticlePage, ArticleSentence, LitVarCandidate, PoolInfo, SourceNotice,
};
pub use response::{
    ArticleSaved, ArticleTextResult, FigureCaption, FullText, License, TextContent, TextHit,
    TextPart, TextPartKind,
};
pub use response::{
    ArticleSupplementResult, CellFragment, FileKind, SheetInfo, SheetRow, SupplementContent,
    SupplementFile, Supplements,
};
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
pub use variant::{Variant, Representation};

pub use query::{VariantGet, VariantResolver};
pub use response::{VariantCard, VariantHeader, VariantResolution};
pub use response::{ArticlePdfResult, OpenAccessStatus, PdfLink, PdfSaved};

pub use coord::Chromosome;

pub use variant::{Accession, EnsemblFeature, LrgPart, RefSeqPrefix, Base, Sequence, Allele, SequenceLocation};

pub use variant::{VariantId, HgvsText, Expression, ExpressionSyntax};

pub use query::{VariantInput, VariantNamespace};
