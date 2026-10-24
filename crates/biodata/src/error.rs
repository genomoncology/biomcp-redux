/// A rejected value, without retaining the input text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The checked EntrezGeneIdInvalid value was rejected.
    EntrezGeneIdInvalid,
    /// The checked GeneSymbolInvalid value was rejected.
    GeneSymbolInvalid,
    /// The checked OmimIdInvalid value was rejected.
    OmimIdInvalid,
    /// The checked UniProtAccInvalid value was rejected.
    UniProtAccInvalid,
    /// The checked CytobandInvalid value was rejected.
    CytobandInvalid,
    /// The checked EnsemblGeneIdInvalid value was rejected.
    EnsemblGeneIdInvalid,
    /// The checked EnsemblTranscriptInvalid value was rejected.
    EnsemblTranscriptInvalid,
    /// The checked RefSeqTranscriptInvalid value was rejected.
    RefSeqTranscriptInvalid,
    /// An HGNC identifier was not canonical positive u32 text.
    HgncIdInvalid,
    /// A date had invalid precision or calendar components.
    DateInvalid,
    /// A one-based position was zero.
    PositionZero,
    /// A span ended before its start.
    SpanOrder,
    /// Text contained no non-whitespace characters.
    TextEmpty,
    /// A source name was not recognized.
    SourceUnknown,
    /// A reason name was not recognized.
    ReasonUnknown,
    /// A problem code was not recognized.
    ProblemCodeUnknown,
}

impl Error {
    /// Return the stable machine-readable code.
    pub fn code(self) -> &'static str {
        match self {
            Self::EntrezGeneIdInvalid => "id.entrez.invalid",
            Self::GeneSymbolInvalid => "gene.symbol.invalid",
            Self::OmimIdInvalid => "id.omim.invalid",
            Self::UniProtAccInvalid => "id.uniprot.invalid",
            Self::CytobandInvalid => "cytoband.invalid",
            Self::EnsemblGeneIdInvalid => "id.ensembl_gene.invalid",
            Self::EnsemblTranscriptInvalid => "id.ensembl_transcript.invalid",
            Self::RefSeqTranscriptInvalid => "id.refseq_transcript.invalid",

            Self::HgncIdInvalid => "id.hgnc.invalid",
            Self::DateInvalid => "date.invalid",
            Self::PositionZero => "position.zero",
            Self::SpanOrder => "span.order",
            Self::TextEmpty => "text.empty",
            Self::SourceUnknown => "source.unknown",
            Self::ReasonUnknown => "reason.unknown",
            Self::ProblemCodeUnknown => "problem_code.unknown",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for Error {}
