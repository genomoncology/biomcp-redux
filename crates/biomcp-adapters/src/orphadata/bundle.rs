//! The two Orphadata files, replaced together as one stored set.
use super::raw::Native;
use biomcp_fetch::{
    BundleInfo, BundlePart, BundleSet, DocumentMedia, Fetch, Find, Freshness, KeyUse, Leg,
    RequestError, RowFormat, SetRow, SourceSpec,
};
use biomcp_types::{Source, SyncResult};
use std::{marker::PhantomData, time::Duration};

/// Orphadata's CC BY 4.0 terms in plain words. Its legal notice, checked 2026-11-13, asks an
/// AI tool or service offered to others to state that it uses Orphanet/Orphadata data, that its
/// results may not truly reflect that data, and that Orphanet is not responsible for them.
const TERMS: biomcp_fetch::Terms = biomcp_fetch::Terms {
    name: "CC BY 4.0; cite Orphadata. BioMCP uses Orphanet/Orphadata data; its results may not \
           truly reflect that data, and Orphanet is not responsible for their quality",
    url: "https://www.orphadata.com/legal-notice/",
    usage: biomcp_types::Use::Attribution,
};
/// Orphadata's file host and its CC BY 4.0 terms.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: Source::Orphadata,
    base: "https://www.orphadata.com",
    terms: TERMS,
    quota: &crate::quotas::ORPHADATA,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
pub(super) const PREVALENCE: &str = "prevalence";
pub(super) const AGES: &str = "ages";
// Orphadata publishes these files twice a year; checked 2026-11-13 at
// https://www.orphadata.com/epidemiology/ (July 2026 release, files dated 23 Jun 2026).
const SET: BundleSet<Native> = BundleSet {
    spec: &SPEC,
    name: "orphadata",
    fresh: Freshness::Days(30),
    row: PhantomData,
    format: RowFormat::Document {
        media: DocumentMedia::Xml,
    },
    parts: &[
        BundlePart {
            name: PREVALENCE,
            path: "/data/xml/en_product9_prev.xml",
            format: None,
        },
        BundlePart {
            name: AGES,
            path: "/data/xml/en_product9_ages.xml",
            format: None,
        },
    ],
};
/// Publisher licence page; health never downloads the epidemiology files.
pub(crate) const HEALTH_PATH: &str = "/legal-notice/";

/// Replace both files atomically.
pub fn sync(fetch: &Fetch) -> Result<crate::Mapped<SyncResult>, RequestError> {
    fetch
        .sync_set(&SET)
        .map(|info| crate::mapped::bundle_sync(SPEC.source, info))
}
/// Inspect the stored files without a network request.
pub(crate) fn bundle_state(leg: &Leg<'_>) -> Result<biomcp_types::LocalBundleState, RequestError> {
    SET.state(leg)
}
pub(super) fn read(
    leg: &Leg<'_>,
    find: Find<'_>,
) -> Result<(Vec<SetRow<Native>>, BundleInfo), RequestError> {
    leg.stored_set_rows(&SET, find)
}
