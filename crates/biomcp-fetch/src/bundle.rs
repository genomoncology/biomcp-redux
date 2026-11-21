//! Indexed published TSV files, independent of individual reply cache entries.
mod download;
mod first;
mod manual;
mod native;
mod parse;
mod publish;
mod read;
pub(crate) mod refresh;
mod replay;
mod set;
mod streamed;
mod transfer;
pub use set::{BundlePart, BundleSet, SetRow};
pub(crate) mod store;
use crate::{Fetch, Freshness, RequestError, SourceSpec};
use std::marker::PhantomData;
/// Maximum unencoded published bundle size.
pub const MAX_BUNDLE_BYTES: u64 = 128 * 1024 * 1024;
/// Ceiling for one streamed gzip XML part, counted apart from `MAX_BUNDLE_BYTES` (0931: GTR's XML is 176 MiB).
pub(crate) const MAX_STREAMED_BYTES: u64 = 512 * 1024 * 1024;
/// Ceiling for one streamed part's decoded XML, which is read once and never held (0931: GTR's is 1.8 GiB).
pub(crate) const MAX_STREAMED_DECODED_BYTES: u64 = 8 * 1024 * 1024 * 1024;
/// A decoded bundle row supplies its own search keys.
pub trait Row: serde::de::DeserializeOwned {
    /// Refuse a refresh when native rows exist but none survive admission.
    const REQUIRE_ACCEPTED_ROWS: bool = false;
    /// Source-specific complete transfer bound, capped by the shared bundle ceiling.
    const MAX_BYTES: u64 = MAX_BUNDLE_BYTES;
    /// Decode a complete source document before any row is published.
    fn decode_document(_body: &[u8]) -> Result<Vec<String>, crate::BadReply> {
        Err(crate::BadReply)
    }
    /// Read a streamed XML part once, emitting each kept row; `None` records a rejected element.
    fn stream_xml(
        _xml: &mut dyn std::io::BufRead,
        _emit: &mut dyn FnMut(Option<String>),
    ) -> Result<(), crate::BadReply> {
        Err(crate::BadReply)
    }
    /// Return the row’s exact and word search keys.
    fn keys(&self) -> Keys;
    /// Shared publisher dataset date embedded in this admitted bundle row.
    fn data_date(&self) -> Option<&str> {
        None
    }
    /// Validate this row against its declared member of a complete source set.
    fn check_part(&self, _part: &str) -> Result<(), crate::BadReply> {
        Ok(())
    }
    /// Validate typed shared metadata against the complete decoded JSON row count.
    fn check_bundle(&self, _rows: usize) -> Result<(), crate::BadReply> {
        Ok(())
    }
}
/// A declared local data source.
#[derive(Debug)]
pub struct Bundle<R: Row + 'static> {
    /// Provider declaration.
    pub spec: &'static SourceSpec,
    /// Stable bundle name.
    pub name: &'static str,
    /// Path under the declared base URL.
    pub path: &'static str,
    /// Wire row format.
    pub format: RowFormat,
    /// Checked-row freshness.
    pub fresh: Freshness,
    /// Decoded row type.
    pub row: PhantomData<R>,
}
/// Supported published row encoding.
#[derive(Clone, Copy, Debug)]
pub enum RowFormat {
    /// Source-owned whole-document decoding, validated before publication.
    Document {
        /// Representation requested from the publisher.
        media: DocumentMedia,
    },
    /// Tab-separated cells with a named header.
    Tsv,
    /// Tab-separated cells after leading `#` banner lines; the last banner line names the columns.
    CommentedTsv,
    /// Strict quoted TSV preserving all cells, optionally gzip-encoded.
    NativeTsv {
        /// Decode the published gzip member before reading cells.
        gzip: bool,
        /// Maximum decoded bytes, bounded by the shared native TSV ceiling.
        max_decoded_bytes: u64,
    },
    /// JSON object with a declared row array and shared metadata object.
    Json {
        /// Object field holding the complete row array.
        rows: &'static str,
        /// Object field copied into each stored row as `_bundle_meta`.
        metadata: &'static str,
    },
    /// Generated comma-separated records after physical banner lines.
    Csv {
        /// Number of physical lines preceding the header.
        banner_lines: usize,
    },
    /// Native CSV with required cells, including empty strings, and a source bound.
    NativeCsv {
        /// Maximum complete published body size.
        max_bytes: u64,
    },
    /// Gzip XML decoded as a stream by `Row::stream_xml`; only emitted rows are kept.
    StreamedXml {
        /// Maximum gzip body size, bounded by `MAX_STREAMED_BYTES` instead of the set ceiling.
        max_bytes: u64,
        /// Maximum decoded XML size, bounded by `MAX_STREAMED_DECODED_BYTES`.
        max_decoded_bytes: u64,
    },
}
/// Publisher representation consumed by a source-owned document decoder.
#[derive(Clone, Copy, Debug)]
pub enum DocumentMedia {
    /// An HTML document.
    Html,
    /// A JSON document.
    Json,
    /// An XML document.
    Xml,
    /// A PDF document.
    Pdf,
}
impl RowFormat {
    pub(crate) fn media(self) -> &'static str {
        match self {
            Self::Document { media } => media.media(),
            Self::Json { .. } => "application/json",
            Self::StreamedXml { .. } => "application/xml",
            Self::Tsv | Self::CommentedTsv | Self::NativeTsv { .. } => "text/tab-separated-values",
            Self::Csv { .. } | Self::NativeCsv { .. } => "text/csv",
        }
    }
}
/// Searchable keys produced by a decoded row.
#[derive(Debug)]
pub struct Keys {
    /// Namespaced exact keys, folded in the shared index.
    pub exact: Vec<(KeyKind, Box<str>)>,
    /// Literal text indexed by SQLite’s tokenizer.
    pub words: Vec<Box<str>>,
}
/// Exact-key namespace.
#[derive(Clone, Copy, Debug)]
pub enum KeyKind {
    /// Primary identifier.
    Id,
    /// Current symbol.
    Symbol,
    /// Previous symbol.
    PreviousSymbol,
    /// Alternative symbol.
    Alias,
    /// External reference.
    Xref,
    /// Indexed band text.
    Band,
}
/// One literal indexed lookup.
#[derive(Clone, Copy, Debug)]
pub enum Find<'a> {
    /// Case-insensitive complete key.
    Exact(KeyKind, &'a str),
    /// Find a complete key, or return all rows if the bundle predates this key namespace.
    /// The caller must check the exact key on returned rows.
    ExactOrUnindexed(KeyKind, &'a str),
    /// Select several complete keys in one indexed snapshot.
    AnyExact(KeyKind, &'a [&'a str]),
    /// Literal key prefix.
    Prefix(KeyKind, &'a str),
    /// Literal words combined with AND.
    Words(&'a str),
}
/// Public bundle metadata contains no local filenames or provider diagnostics.
#[derive(Clone, Debug)]
pub struct BundleInfo {
    /// Publisher dataset date retained in admitted rows, never a local synchronization time.
    pub data_date: Option<Box<str>>,
    /// Rejected logical source rows, retained across synchronization reuse.
    pub rejections: Vec<BundleRejection>,
    /// Native publisher revision validator, when supplied.
    pub etag: Option<Box<str>>,
    /// Native publisher modification header, when supplied.
    pub last_modified: Option<Box<str>>,
    /// Stable declared bundle name.
    pub name: &'static str,
    /// Complete raw body bytes.
    pub bytes: u64,
    /// Source body bytes received during this synchronization, excluding reused or replayed bytes.
    pub downloaded_bytes: u64,
    /// Checked decoded row count.
    pub rows: u64,
    /// Whether stored freshness has expired.
    pub fresh: bool,
    /// Computed ordered-part content revision, never a publisher HTTP validator.
    pub content_revision: Option<Box<str>>,
    /// Actual local set synchronization time as UTC Unix seconds.
    pub retrieved_at: Option<i64>,
}
/// Publication decision made after native row admission.
#[derive(Clone, Debug)]
pub enum BundleSync {
    /// The checked catalog is active, including a valid native empty catalog.
    Published(BundleInfo),
    /// Every native row was refused; the active catalog is unchanged.
    Refused {
        /// Native rows examined before admission.
        native_rows: u64,
        /// Redacted refusal facts from this refresh.
        rejections: Vec<BundleRejection>,
        /// Complete refused source body bytes.
        bytes: u64,
        /// Bytes transferred during this refresh.
        downloaded_bytes: u64,
        /// Last good metadata, separate from the refused refresh.
        retained: Option<BundleInfo>,
    },
}
impl BundleSync {
    pub(super) fn published(self, leg: &crate::Leg<'_>) -> Result<BundleInfo, RequestError> {
        match self {
            Self::Published(info) => Ok(info),
            Self::Refused { .. } => Err(leg
                .failure(biomcp_types::Reason::BadReply, Some(200))
                .into()),
        }
    }
}
impl Fetch {
    /// Configured time limit for one explicit bundle synchronization and each read it makes.
    pub fn sync_budget(&self) -> std::time::Duration {
        self.config.sync_budget
    }
    /// Start a leg on the configured sync time limit, not capped by the source's own request
    /// deadline, so a sync and any read it makes of its own store use that limit (0951).
    pub fn sync_leg(&self, spec: &'static SourceSpec) -> crate::Leg<'_> {
        let deadline = self.deadline(self.config.sync_budget);
        let mut leg = self.leg(spec, deadline);
        leg.deadline = deadline;
        leg
    }
    /// Synchronize while preserving a refused refresh separately from the active catalog.
    pub fn sync_catalog<R: Row>(&self, bundle: &Bundle<R>) -> Result<BundleSync, RequestError> {
        self.sync_leg(bundle.spec).sync_bundle(bundle, false, None)
    }
    /// Synchronize a declared bundle under the configured independent sync budget.
    pub fn sync<R: Row>(&self, bundle: &Bundle<R>) -> Result<BundleInfo, RequestError> {
        let leg = self.sync_leg(bundle.spec);
        let mut info = leg.sync_bundle(bundle, false, None)?.published(&leg)?;
        // A busy cache must not fail a sync that already published; the date stays unknown.
        info.data_date = match leg.stored_data_date(bundle) {
            Err(RequestError::Cache(crate::CacheError::Busy)) => None,
            other => other?,
        };
        Ok(info)
    }
}
/// Serialize a typed source document row for the shared checked bundle store.
pub fn encode_bundle_row<T: serde::Serialize>(row: &T) -> Result<String, crate::BadReply> {
    serde_json::to_string(row).map_err(|_| crate::BadReply)
}

/// Decode a complete typed JSON catalog inside the transport JSON boundary.
pub fn decode_bundle_document<T: serde::de::DeserializeOwned>(
    body: &[u8],
) -> Result<T, crate::BadReply> {
    serde_json::from_slice(body).map_err(|_| crate::BadReply)
}

/// Compute a catalog receipt hash from the complete checked native bytes.
pub fn bundle_sha256(body: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    digest_hex(&Sha256::digest(body))
}

fn digest_hex(digest: &[u8]) -> String {
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

impl DocumentMedia {
    pub(crate) fn media(self) -> &'static str {
        match self {
            Self::Html => "text/html",
            Self::Json => "application/json",
            Self::Xml => "application/xml",
            Self::Pdf => "application/pdf",
        }
    }
}

/// A redacted rejection at a stable logical source row.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BundleRejection {
    /// One-based logical row number, counting the header.
    pub row: u64,
    /// Fixed reason without provider row contents.
    pub reason: String,
}

fn query_word(word: &str) -> bool {
    word.chars().any(char::is_alphanumeric)
}
