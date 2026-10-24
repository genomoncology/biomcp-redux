/// A rejected value, without retaining the input text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// A subject reference lacks its source identity or notation.
    VariantRefInvalid,
    /// A concept identifier lacks a checked prefix or local part.
    CurieInvalid,
    /// The shared grammar refused the value.
    AccessionDigitsWidth,
    /// The shared grammar refused the value.
    AccessionVersionMissing,
    /// The shared grammar refused the value.
    AccessionVersionLeadingZero,
    /// The shared grammar refused the value.
    AccessionVersionOverflow,
    /// The shared grammar refused the value.
    AccessionLrgInvalid,
    /// The shared grammar refused the value.
    SequenceRna,
    /// The shared grammar refused the value.
    ExpressionSyntaxUnknown,

    /// The common value refused this input.
    IntervalBuildAbsent,
    /// The common value refused this input.
    IntervalBuildMismatch,
    /// The common value refused this input.
    IntervalChromosomeMismatch,
    /// The common value refused this input.
    AccessionVersionPresent,
    /// The common value refused this input.
    ManeStatusUnknown,
    /// The common value refused this input.
    GeneRefLabelEmpty,
    /// The common value refused this input.
    HgvsTextEmpty,
    /// The common value refused this input.
    HgvsReferenceMissing,
    /// The common value refused this input.
    HgvsReferenceInvalid,
    /// The common value refused this input.
    HgvsInnerReferenceInvalid,
    /// The common value refused this input.
    HgvsSyntaxUnknown,
    /// The common value refused this input.
    HgvsBodyEmpty,
    /// The common value refused this input.
    HgvsBodyUnprintable,
    /// The common value refused this input.
    RsIdInvalid,
    /// The common value refused this input.
    CaIdInvalid,
    /// The common value refused this input.
    PaIdInvalid,
    /// The common value refused this input.
    VcvIdInvalid,
    /// The common value refused this input.
    RcvIdInvalid,
    /// The common value refused this input.
    ScvIdInvalid,
    /// The common value refused this input.
    ClinVarVariationIdInvalid,
    /// The common value refused this input.
    ClinVarAlleleIdInvalid,
    /// The common value refused this input.
    CosmicIdInvalid,
    /// The common value refused this input.
    VrsIdInvalid,

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
    /// The checked NcbiGeneIdInvalid value was rejected.
    NcbiGeneIdInvalid,
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
            Self::VariantRefInvalid => "variant.reference.invalid",
            Self::CurieInvalid => "concept.curie.invalid",
            Self::AccessionDigitsWidth => "accession.digits.width",
            Self::AccessionVersionMissing => "accession.version.missing",
            Self::AccessionVersionLeadingZero => "accession.version.leading_zero",
            Self::AccessionVersionOverflow => "accession.version.overflow",
            Self::AccessionLrgInvalid => "accession.lrg.invalid",
            Self::SequenceRna => "sequence.base.rna",
            Self::ExpressionSyntaxUnknown => "expression.syntax.unknown",

            Self::IntervalBuildAbsent => "coord.interval.build_absent",
            Self::IntervalBuildMismatch => "coord.interval.build_mismatch",
            Self::IntervalChromosomeMismatch => "coord.interval.chromosome_mismatch",
            Self::AccessionVersionPresent => "accession.version.present",
            Self::ManeStatusUnknown => "gene.mane.status_unknown",
            Self::GeneRefLabelEmpty => "gene.ref.label_empty",
            Self::HgvsTextEmpty => "hgvs.text.empty",
            Self::HgvsReferenceMissing => "hgvs.reference.missing",
            Self::HgvsReferenceInvalid => "hgvs.reference.invalid",
            Self::HgvsInnerReferenceInvalid => "hgvs.inner_reference.invalid",
            Self::HgvsSyntaxUnknown => "hgvs.syntax.unknown",
            Self::HgvsBodyEmpty => "hgvs.body.empty",
            Self::HgvsBodyUnprintable => "hgvs.body.unprintable",
            Self::RsIdInvalid => "id.rs.invalid",
            Self::CaIdInvalid => "id.ca.invalid",
            Self::PaIdInvalid => "id.pa.invalid",
            Self::VcvIdInvalid => "id.vcv.invalid",
            Self::RcvIdInvalid => "id.rcv.invalid",
            Self::ScvIdInvalid => "id.scv.invalid",
            Self::ClinVarVariationIdInvalid => "id.clinvar_variation.invalid",
            Self::ClinVarAlleleIdInvalid => "id.clinvar_allele.invalid",
            Self::CosmicIdInvalid => "id.cosmic.invalid",
            Self::VrsIdInvalid => "id.vrs.invalid",

            Self::VariantIdInvalid => "id.variant.unknown",
            Self::NotationInvalid => "expression.value.invalid",
            Self::AccessionInvalid => "accession.prefix.unknown",
            Self::ChromosomeInvalid => "coord.chromosome.invalid",
            Self::SequenceInvalid => "sequence.base.invalid",
            Self::AlleleReferenceLength => "allele.reference.length",
            Self::AlleleEmpty => "allele.state.empty",
            Self::VariantInputInvalid => "variant.input.invalid",
            Self::TrialIdInvalid => "id.trial.invalid",
            Self::ArticleIdInvalid => "id.article.invalid",
            Self::LocusTypeUnknown => "gene.locus_type.unknown",
            Self::BuildUnknown => "coord.build.unknown",
            Self::StrandUnknown => "coord.strand.unknown",
            Self::ContigInvalid => "coord.contig.invalid",

            Self::GeneStatusUnknown => "gene.status.unknown",
            Self::LocusGroupUnknown => "gene.locus_group.unknown",
            Self::MatchKindUnknown => "gene.match_kind.unknown",
            Self::SummarySourceUnknown => "gene.summary_source.unknown",
            Self::HgncStatusUnknown => "gene.hgnc_status.unknown",

            Self::NcbiGeneIdInvalid => "id.ncbi_gene.invalid",
            Self::GeneSymbolInvalid => "gene.symbol.invalid",
            Self::OmimIdInvalid => "id.omim.invalid",
            Self::UniProtAccInvalid => "id.uniprot.invalid",
            Self::CytobandInvalid => "cytoband.invalid",
            Self::EnsemblGeneIdInvalid => "id.ensembl_gene.invalid",

            Self::HgncIdInvalid => "id.hgnc.invalid",
            Self::DateInvalid => "date.invalid",
            Self::PositionZero => "coord.position.zero",
            Self::SpanOrder => "coord.span.order",
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
