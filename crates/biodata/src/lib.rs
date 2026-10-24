#![doc = include_str!("../README.md")]

mod article;
pub use article::{
    AbstractPart, Article, ArticleInput, ArticleMatchKind, Correction, CorrectionKind,
    CorrectionRelation,
};
pub use article::{ArticleHeader, PublicationType};
pub use id::{Doi, EuropePmcId, Pmcid, Pmid, SemanticScholarId};
pub use query::{ArticleFilter, ArticleSearch, ArticleSort, ArticleSource, ArticleType};
pub use query::{ArticleGet, ArticleSection, ArticleSectionWord};
pub use query::{ArticleSupplement, ArticleText, SheetRows};
pub use response::{ArticleCardItem, ArticleCards, ArticleMatch, ArticleResolution};
pub use response::{ArticleItem, ArticleKey, ArticlePage, LitVarCandidate, PoolInfo, SourceNotice};
pub use response::{
    ArticleSupplementResult, CellFragment, FileKind, SheetInfo, SheetRow, SupplementContent,
    SupplementFile, Supplements,
};
pub use response::{
    ArticleTextResult, FigureCaption, FullText, License, TextContent, TextHit, TextPart,
    TextPartKind,
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
    Help, HelpArgument, HelpCommand, HelpWord, KeyHelp, Oncokb, OncokbCard, OncokbGeneType,
    OncokbTranscript, SyncQuery, SyncResult,
};

pub use response::{Constraint, FiniteNumber, FiniteNumberError, GeneSections, Protein};

pub use response::{
    CancerPtpm, Capped, CivicGene, ClinGenGene, ClinGenValidity, Expression, PathwayRef,
    ReactomeId, ReactomeIdInvalid, RnaAbundance, TissueNtpm,
};

pub use response::{AnnotatedEntity, AnnotationKind, Annotations, ArticleSections};

mod markdown;
pub use markdown::Markdown;
pub use query::ArticleEdges;
pub use response::{ArticleEdgePage, Citations};

pub use response::{Health, SourceHealth};
