/// A rejected value, without retaining the input text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
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
