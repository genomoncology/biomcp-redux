/// A rejected value, without retaining the input text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Empty source subject reference.
    VariantRefInvalid,
    /// A published variant identifier was invalid.
    VariantIdInvalid,
    /// The source notation did not match its declared form.
    NotationInvalid,
    /// A versioned sequence accession was invalid.
    AccessionInvalid,
    /// A canonical primary chromosome was invalid.
    ChromosomeInvalid,
    /// A DNA base or sequence was invalid.
    SequenceInvalid,
    /// Allele reference length differs from its span.
    AlleleReferenceLength,
    /// An allele has neither reference nor alternate bases.
    AlleleEmpty,
    /// Input is not a published variant identifier.
    VariantInputInvalid,
    /// A trial identifier failed its namespace syntax.
    TrialIdInvalid,
    /// An article identifier failed its namespace syntax.
    ArticleIdInvalid,
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
    /// The GeneStatusUnknown boundary rejected a value.
    GeneStatusUnknown,
    /// The LocusGroupUnknown boundary rejected a value.
    LocusGroupUnknown,
    /// The MatchKindUnknown boundary rejected a value.
    MatchKindUnknown,
    /// The SummarySourceUnknown boundary rejected a value.
    SummarySourceUnknown,
    /// The HgncStatusUnknown boundary rejected a value.
    HgncStatusUnknown,
    /// The ManeVersionRequired boundary rejected a value.
    ManeVersionRequired,
    /// The LocusTypeUnknown boundary rejected a value.
    LocusTypeUnknown,
    /// The BuildUnknown boundary rejected a value.
    BuildUnknown,
    /// The StrandUnknown boundary rejected a value.
    StrandUnknown,
    /// The ContigInvalid boundary rejected a value.
    ContigInvalid,
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
            Self::VariantRefInvalid => "variant_ref.invalid",
            Self::VariantIdInvalid => "id.variant.invalid",
            Self::NotationInvalid => "notation.invalid",
            Self::AccessionInvalid => "sequence.accession.invalid",
            Self::ChromosomeInvalid => "chromosome.invalid",
            Self::SequenceInvalid => "sequence.invalid",
            Self::AlleleReferenceLength => "allele.reference_length",
            Self::AlleleEmpty => "allele.empty",
            Self::VariantInputInvalid => "variant.input.invalid",
            Self::TrialIdInvalid => "id.trial.invalid",
            Self::ArticleIdInvalid => "id.article.invalid",
            Self::LocusTypeUnknown => "gene.locus_type.unknown",
            Self::BuildUnknown => "build.unknown",
            Self::StrandUnknown => "strand.unknown",
            Self::ContigInvalid => "contig.invalid",

            Self::GeneStatusUnknown => "gene.status.unknown",
            Self::LocusGroupUnknown => "gene.locus_group.unknown",
            Self::MatchKindUnknown => "gene.match_kind.unknown",
            Self::SummarySourceUnknown => "gene.summary_source.unknown",
            Self::HgncStatusUnknown => "gene.hgnc_status.unknown",
            Self::ManeVersionRequired => "gene.mane.version_required",

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
