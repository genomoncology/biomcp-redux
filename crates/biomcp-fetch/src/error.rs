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
    /// Provider-stated retry delay, when present on a rate-limit refusal.
    pub retry_after: Option<std::time::Duration>,
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
    /// The explicit cache could not be opened safely.
    Cache(CacheError),
    /// Local recording ownership or content failure.
    Recording(RecordError),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Proxy => "fetch.proxy",
            Self::BodyLimit => "fetch.body_limit",
            Self::Cache(error) => return error.fmt(f),
            Self::Recording(error) => return error.fmt(f),
        })
    }
}

impl std::error::Error for FetchError {}

/// Local cache failures carry no SQL, file paths or provider text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheError {
    /// Database access failed.
    Database,
    /// The file has no valid transport ownership or a different schema version.
    Version,
    /// The connection or database remained busy until the request deadline.
    Busy,
}
impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Database => "fetch.cache_database",
            Self::Version => "fetch.cache_version",
            Self::Busy => "fetch.cache_busy",
        })
    }
}
impl std::error::Error for CacheError {}

/// A provider failure or a distinct local cache failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RequestError {
    /// The provider request failed.
    Source(SourceError),
    /// Local cache access or validation failed.
    Cache(CacheError),
    /// Local recording ownership or content failure.
    Recording(RecordError),
}
impl From<SourceError> for RequestError {
    fn from(error: SourceError) -> Self {
        Self::Source(error)
    }
}
impl From<CacheError> for RequestError {
    fn from(error: CacheError) -> Self {
        Self::Cache(error)
    }
}
impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source(error) => error.fmt(f),
            Self::Cache(error) => error.fmt(f),
            Self::Recording(error) => error.fmt(f),
        }
    }
}
impl std::error::Error for RequestError {}

/// Local recording failures never retain paths, URLs or provider text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecordError {
    /// The capture directory is nonempty or already owned.
    Conflict,
    /// Local recording I/O failed.
    Io,
    /// No file matches this source and request identity.
    Missing,
    /// Every exchange for this request was consumed.
    Exhausted,
    /// Recording metadata or exchange content is invalid.
    Malformed,
    /// Recording size or sequence bounds were exceeded.
    Limit,
    /// Binary capture needs a future sidecar representation.
    NonUtf8,
}
impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Conflict => "fetch.record_conflict",
            Self::Io => "fetch.record_io",
            Self::Missing => "fetch.record_missing",
            Self::Exhausted => "fetch.record_exhausted",
            Self::Malformed => "fetch.record_malformed",
            Self::Limit => "fetch.record_limit",
            Self::NonUtf8 => "fetch.record_non_utf8",
        })
    }
}
impl std::error::Error for RecordError {}
impl From<RecordError> for RequestError {
    fn from(error: RecordError) -> Self { Self::Recording(error) }
}
