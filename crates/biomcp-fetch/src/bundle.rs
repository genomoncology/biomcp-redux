//! Indexed published TSV files, independent of individual reply cache entries.
mod download;
mod native;
mod parse;
mod publish;
mod set;
pub use set::{BundlePart, BundleSet, SetRow};
pub(crate) mod store;
use crate::{Fetch, Freshness, Leg, Mode, RequestError, SourceSpec};
use std::marker::PhantomData;
/// Maximum unencoded published bundle size.
pub const MAX_BUNDLE_BYTES: u64 = 128 * 1024 * 1024;
/// A decoded bundle row supplies its own search keys.
pub trait Row: serde::de::DeserializeOwned {
    /// Source-specific complete transfer bound, capped by the shared bundle ceiling.
    const MAX_BYTES: u64 = MAX_BUNDLE_BYTES;
    /// Decode a complete source document before any row is published.
    fn decode_document(_body: &[u8]) -> Result<Vec<String>, crate::BadReply> {
        Err(crate::BadReply)
    }
    /// Return the row’s exact and word search keys.
    fn keys(&self) -> Keys;
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
    Document,
    /// Tab-separated cells with a named header.
    Tsv,
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
}
impl RowFormat {
    fn media(self) -> &'static str {
        match self {
            Self::Document => "text/html",
            Self::Json { .. } => "application/json",
            Self::Tsv | Self::NativeTsv { .. } => "text/tab-separated-values",
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
    /// Native publisher revision validator, when supplied.
    pub etag: Option<Box<str>>,
    /// Native publisher modification header, when supplied.
    pub last_modified: Option<Box<str>>,
    /// Stable declared bundle name.
    pub name: &'static str,
    /// Complete raw body bytes.
    pub bytes: u64,
    /// Checked decoded row count.
    pub rows: u64,
    /// Whether stored freshness has expired.
    pub fresh: bool,
    /// Computed ordered-part content revision, never a publisher HTTP validator.
    pub content_revision: Option<Box<str>>,
    /// Actual local set synchronization time as UTC Unix seconds.
    pub retrieved_at: Option<i64>,
}
impl Fetch {
    /// Synchronize a declared bundle under the configured independent sync budget.
    pub fn sync<R: Row>(&self, bundle: &Bundle<R>) -> Result<BundleInfo, RequestError> {
        let deadline = self.deadline(self.config.sync_budget);
        let mut leg = self.leg(bundle.spec, deadline);
        leg.deadline = deadline;
        leg.sync_bundle(bundle, false)
    }
}
impl Leg<'_> {
    /// Find checked rows, synchronizing absent or stale data when policy permits.
    pub fn rows<R: Row>(&self, bundle: &Bundle<R>, find: Find<'_>) -> Result<Vec<R>, RequestError> {
        self.bundle_source(bundle)?;
        if self.fetch.cache.is_none() && matches!(self.fetch.mode, Mode::Offline) {
            return Err(self.failure(biomcp_types::Reason::Offline, None).into());
        }
        let cache = self.bundle_cache()?;
        // One command can make several indexed lookups against the same bundle.
        // Fresh revalidates once for this leg, including its concurrent children.
        let mut refreshed = self.fresh.then(|| {
            self.refreshed_bundles
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
        });
        let old = if matches!(self.fetch.mode, Mode::Record { .. }) {
            None
        } else {
            cache.bundle_info(bundle.name, self.budget())?
        };
        let replay = matches!(self.fetch.mode, Mode::Replay { .. });
        let offline = matches!(self.fetch.mode, Mode::Offline);
        let fresh = old
            .as_ref()
            .is_some_and(|m| m.fresh_until > crate::cache::now());
        if offline && old.is_none() {
            return Err(self.failure(biomcp_types::Reason::Offline, None).into());
        }
        let force_fresh = refreshed
            .as_ref()
            .is_some_and(|names| !names.contains(bundle.name));
        let forced = replay || force_fresh || matches!(self.fetch.mode, Mode::Record { .. });
        if !offline && (!fresh || forced) {
            match self.sync_bundle(bundle, true) {
                Ok(_) => {
                    if let Some(names) = &mut refreshed {
                        names.insert(bundle.name);
                    }
                }
                Err(RequestError::Source(error))
                    if old.is_some()
                        && matches!(self.fetch.mode, Mode::Live { .. })
                        && !self.fresh =>
                {
                    let mut report = self
                        .report
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                    report.stale = true;
                    report.failures.push(error);
                }
                Err(error) => return Err(error),
            }
        }
        if !forced && fresh {
            let url = format!("{}{}", self.spec.base.trim_end_matches('/'), bundle.path);
            self.observed(
                &crate::retry::identity(&url),
                Some(200),
                0,
                crate::Shape::default(),
                std::time::Duration::ZERO,
                ("bundle", true, &url, crate::Provenance::default()),
            );
        }
        if offline && !fresh {
            self.report
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .stale = true;
        }
        let rows = cache.bundle_rows(bundle.name, find, self.budget())?;
        let rows = rows
            .into_iter()
            .map(|row| serde_json::from_str(&row).map_err(|_| crate::CacheError::Database.into()))
            .collect::<Result<_, RequestError>>()?;
        Ok(rows)
    }
    /// Read only the last validated local bundle, even when fresh or replay is selected.
    /// Missing data reports `Syncing`; the caller must explicitly synchronize it.
    pub fn stored_rows<R: Row>(
        &self,
        bundle: &Bundle<R>,
        find: Find<'_>,
    ) -> Result<(Vec<R>, BundleInfo), RequestError> {
        self.bundle_source(bundle)?;
        let cache = self.bundle_cache()?;
        let (meta, rows) = cache
            .stored_bundle(bundle.name, find, self.budget())?
            .ok_or_else(|| self.failure(biomcp_types::Reason::Syncing, None))?;
        let rows = rows
            .into_iter()
            .map(|row| serde_json::from_str(&row).map_err(|_| crate::CacheError::Database.into()))
            .collect::<Result<_, RequestError>>()?;
        if meta.fresh_until <= crate::cache::now() {
            self.report
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .stale = true;
        }
        Ok((rows, store::info(bundle, &meta)))
    }
    fn bundle_source<R: Row>(&self, bundle: &Bundle<R>) -> Result<(), RequestError> {
        self.check_source()?;
        if self.budget().expired() {
            return Err(self
                .failure(biomcp_types::Reason::DeadlineExceeded, None)
                .into());
        }
        if !std::ptr::eq(self.spec, bundle.spec) {
            return Err(self.failure(biomcp_types::Reason::Rejected, None).into());
        }
        Ok(())
    }
    fn bundle_cache(&self) -> Result<&crate::cache::Cache, RequestError> {
        self.fetch
            .cache
            .as_deref()
            .ok_or(crate::CacheError::Database.into())
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
    Sha256::digest(body)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
