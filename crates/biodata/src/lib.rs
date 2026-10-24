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
pub use id::{HgncId, Cytoband, EnsemblGeneId, EnsemblTranscript, EntrezGeneId, GeneSymbol, OmimId, RefSeqTranscript, UniProtAcc};

mod gene;
pub use gene::{GeneInput, GeneStatus, LocusGroup, MatchKind, SummarySource, HgncStatus, ManeSelect, Summary, LocusType, Gene, GeneHeader, GeneResolution, Match};

pub use coord::{Build, Contig, Strand, GenomicInterval};
