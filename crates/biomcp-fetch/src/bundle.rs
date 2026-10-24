//! Indexed published TSV files, independent of individual reply cache entries.
mod download;
mod parse;
pub(crate) mod store;
use crate::{Fetch, Freshness, Leg, Mode, RequestError, SourceSpec};
use std::marker::PhantomData;
/// Maximum unencoded published bundle size.
pub const MAX_BUNDLE_BYTES: u64 = 128 * 1024 * 1024;
/// A decoded bundle row supplies its own search keys.
pub trait Row: serde::de::DeserializeOwned {
    /// Return the row’s exact and word search keys.
    fn keys(&self) -> Keys;
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
    /// Tab-separated cells with a named header.
    Tsv,
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
    /// Literal key prefix.
    Prefix(KeyKind, &'a str),
    /// Literal words combined with AND.
    Words(&'a str),
}
/// Public bundle metadata contains no local filenames or provider diagnostics.
#[derive(Clone, Debug)]
pub struct BundleInfo {
    /// Stable declared bundle name.
    pub name: &'static str,
    /// Complete raw body bytes.
    pub bytes: u64,
    /// Checked decoded row count.
    pub rows: u64,
    /// Whether stored freshness has expired.
    pub fresh: bool,
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
        if self.fetch.cache.is_none() && matches!(self.fetch.config.mode, Mode::Offline) {
            return Err(self.failure(biodata::Reason::Offline, None).into());
        }
        let cache = self.bundle_cache()?;
        // One command can make several indexed lookups against the same bundle.
        // Fresh revalidates once for this leg, including its concurrent children.
        let mut refreshed = self.fresh.then(|| {
            self.refreshed_bundles
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
        });
        let old = if matches!(self.fetch.config.mode, Mode::Record { .. }) {
            None
        } else {
            cache.bundle_info(bundle.name, self.budget())?
        };
        let replay = matches!(self.fetch.config.mode, Mode::Replay { .. });
        let offline = matches!(self.fetch.config.mode, Mode::Offline);
        let fresh = old
            .as_ref()
            .is_some_and(|m| m.fresh_until > crate::cache::now());
        if offline && old.is_none() {
            return Err(self.failure(biodata::Reason::Offline, None).into());
        }
        let force_fresh = refreshed
            .as_ref()
            .is_some_and(|names| !names.contains(bundle.name));
        let forced = replay || force_fresh || matches!(self.fetch.config.mode, Mode::Record { .. });
        if !offline && (!fresh || forced) {
            match self.sync_bundle(bundle, true) {
                Ok(_) => {
                    if let Some(names) = &mut refreshed {
                        names.insert(bundle.name);
                    }
                }
                Err(RequestError::Source(error))
                    if old.is_some()
                        && matches!(self.fetch.config.mode, Mode::Live { .. })
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
    fn bundle_source<R: Row>(&self, bundle: &Bundle<R>) -> Result<(), RequestError> {
        self.check_source()?;
        if self.budget().expired() {
            return Err(self.failure(biodata::Reason::DeadlineExceeded, None).into());
        }
        if !std::ptr::eq(self.spec, bundle.spec) {
            return Err(self.failure(biodata::Reason::Rejected, None).into());
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
