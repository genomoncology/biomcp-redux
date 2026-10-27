//! Complete checked catalog replacement; reads never synchronize.
mod raw;
use super::SPEC;
use biomcp_fetch::{
    Bundle, BundleInfo, Fetch, Find, Freshness, KeyKind, Leg, RequestError, RowFormat,
};
use biomcp_types::{StudySearch, StudySnapshot, SyncResult};
use std::marker::PhantomData;
const CATALOG: Bundle<raw::CatalogRow> = Bundle {
    spec: &SPEC,
    name: "cbioportal-studies",
    path: "/studies?projection=DETAILED",
    format: RowFormat::Document { media: biomcp_fetch::DocumentMedia::Json },
    fresh: Freshness::Days(7),
    row: PhantomData,
};
fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: biomcp_types::Source::Cbioportal,
        reason: biomcp_types::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
/// Acquire one complete metadata catalog and publish only after all rows pass.
pub fn sync(fetch: &Fetch) -> Result<SyncResult, RequestError> {
    let info = fetch.sync(&CATALOG)?;
    Ok(SyncResult {
        snapshot: Some(snapshot(&info)?),
        etag: info.etag,
        last_modified: info.last_modified,
        rows: info.rows,
        bytes: info.bytes,
        duplicate_identities: None,
    })
}
/// Return literal matches in original catalog order from the accepted local copy.
pub fn search(
    leg: &Leg<'_>,
    query: &StudySearch,
) -> Result<(Vec<biodata::StudyHeader>, BundleInfo), RequestError> {
    let (rows, info) = leg.stored_rows(&CATALOG, Find::Exact(KeyKind::Band, "catalog"))?;
    let items = rows
        .into_iter()
        .filter(|r| r.matches(query))
        .map(|r| r.header().map_err(|_| bad()))
        .collect::<Result<Vec<_>, _>>()?;
    Ok((items, info))
}
/// Read an exact case-sensitive metadata card from the stored catalog.
pub fn card(
    leg: &Leg<'_>,
    id: &biodata::StudyId,
) -> Result<(Option<biodata::Study>, BundleInfo), RequestError> {
    let (rows, info) = leg.stored_rows(&CATALOG, Find::Exact(KeyKind::Id, id.as_str()))?;
    let row = rows
        .into_iter()
        .map(|r| r.card().map_err(|_| bad()))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .find(|v| v.header.study_id == *id);
    Ok((row, info))
}
/// Catalog provenance never substitutes acquisition time for a publisher revision.
pub fn snapshot(info: &BundleInfo) -> Result<StudySnapshot, RequestError> {
    Ok(StudySnapshot {
        revision: info
            .content_revision
            .as_deref()
            .ok_or_else(bad)?
            .parse()
            .map_err(|_| bad())?,
        acquired: info
            .retrieved_at
            .and_then(|v| u64::try_from(v).ok())
            .ok_or_else(bad)?,
    })
}

mod sections;
pub use sections::{Records, profiles, sample_lists};
mod files;
pub use files::{Directory, files};

/// Inspect this source's validated local catalog without reading rows or synchronizing.
pub(crate) fn bundle_state(leg: &Leg<'_>) -> Result<biomcp_types::LocalBundleState, RequestError> {
    leg.stored_bundle_state(&CATALOG)
}
