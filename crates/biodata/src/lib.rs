#![doc = include_str!("../README.md")]

mod response;
mod source;
mod text;

pub use response::{ErrorCode, ErrorEnvelope, ErrorStatus, Problem, ProblemCode};
pub use source::{Reason, Source};
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

pub use response::{SyncQuery, SyncResult};

pub use response::{Constraint, FiniteNumber, FiniteNumberError, GeneSections, Protein};
