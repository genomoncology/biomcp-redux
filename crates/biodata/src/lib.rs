#![doc = include_str!("../README.md")]

mod response;
mod source;
mod text;

pub use response::{Problem, ProblemCode};
pub use source::{Reason, Source};
pub use text::Text;

/// A rejected value, without retaining the input text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
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
