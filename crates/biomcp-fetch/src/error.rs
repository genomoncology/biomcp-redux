use biodata::{Reason, Source};
use std::fmt;

/// A rejected decoded reply; carries no provider text.
#[derive(Clone, Copy, Debug)]
pub struct BadReply;

/// A typed source failure containing no request or provider text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceError {
    /// Source whose request failed.
    pub source: Source,
    /// Stable failure reason.
    pub reason: Reason,
    /// HTTP status when a response was received.
    pub status: Option<u16>,
}

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.source, self.reason)
    }
}

impl std::error::Error for SourceError {}

/// Invalid explicit transport configuration, without retaining supplied text.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum FetchError {
    /// The configured proxy could not be parsed.
    Proxy,
    /// The body limit must allow one byte for overflow detection.
    BodyLimit,
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Proxy => "fetch.proxy",
            Self::BodyLimit => "fetch.body_limit",
        })
    }
}

impl std::error::Error for FetchError {}
