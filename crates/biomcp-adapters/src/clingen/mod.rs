//! Published ClinGen validity and dosage bundles, keyed only by HGNC identity.
mod raw;
use crate::quotas;
use biodata::HgncId;
use biomcp_fetch::{
    Bundle, Fetch, Find, Freshness, KeyKind, KeyUse, Leg, RequestError, RowFormat, SourceSpec,
};
use biomcp_types::{ClinGenValidity, Source, SyncResult};
use std::{marker::PhantomData, time::Duration};
/// ClinGen published summaries are generated anew for each request.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "CC0 1.0",
        url: "https://clinicalgenome.org/docs/terms-of-use/",
        usage: biomcp_types::Use::Open,
    },
    source: Source::ClinGen,
    base: "https://search.clinicalgenome.org",
    key: KeyUse::None,
    quota: &quotas::CLINGEN,
    deadline: Duration::from_secs(8),
};
const VALIDITY: Bundle<raw::Validity> = Bundle {
    spec: &SPEC,
    name: "clingen-validity",
    path: "/kb/gene-validity/download",
    format: RowFormat::Csv { banner_lines: 4 },
    fresh: Freshness::Days(1),
    row: PhantomData,
};
const DOSAGE: Bundle<raw::Dosage> = Bundle {
    spec: &SPEC,
    name: "clingen-dosage",
    path: "/kb/gene-dosage/download",
    format: RowFormat::Csv { banner_lines: 4 },
    fresh: Freshness::Days(1),
    row: PhantomData,
};
/// Synchronize both required files and sum their published metadata.
pub fn sync(fetch: &Fetch) -> Result<SyncResult, RequestError> {
    let validity = fetch.sync(&VALIDITY)?;
    let dosage = fetch.sync(&DOSAGE)?;
    Ok(SyncResult {
        snapshot: None,
        etag: None,
        last_modified: None,
        duplicate_identities: None,
        rows: validity.rows + dosage.rows,
        bytes: validity.bytes + dosage.bytes,
        downloaded_bytes: validity.downloaded_bytes + dosage.downloaded_bytes,
    })
}
/// Read gene–disease validity without depending on the dosage summary.
pub fn validity(leg: &Leg<'_>, id: HgncId) -> Result<Box<[ClinGenValidity]>, RequestError> {
    leg.rows(&VALIDITY, Find::Exact(KeyKind::Id, &id.to_string()))?
        .into_iter()
        .map(raw::Validity::checked)
        .collect()
}
/// Checked dosage fields from the independent dosage summary.
#[derive(Debug)]
pub struct Dosage {
    /// Published haploinsufficiency assessment.
    pub haploinsufficiency: biodata::Text,
    /// Published triplosensitivity assessment.
    pub triplosensitivity: biodata::Text,
    /// Published assessment date.
    pub date: biodata::Text,
}
/// Read dosage sensitivity without depending on the validity summary.
pub fn dosage(leg: &Leg<'_>, id: HgncId) -> Result<Option<Dosage>, RequestError> {
    let rows = leg.rows(&DOSAGE, Find::Exact(KeyKind::Id, &id.to_string()))?;
    if rows.len() > 1 {
        return Err(bad());
    }
    rows.into_iter()
        .next()
        .map(|row| {
            Ok(Dosage {
                haploinsufficiency: row.haploinsufficiency.ok_or_else(bad)?,
                triplosensitivity: row.triplosensitivity.ok_or_else(bad)?,
                date: row.date.ok_or_else(bad)?,
            })
        })
        .transpose()
}
fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::ClinGen,
        reason: biomcp_types::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}

/// Small publisher landing page, never a validity or dosage export.
pub(crate) const HEALTH_PATH: &str = "/kb";
/// Both source-owned stored files are required for complete local readiness.
pub(crate) fn bundle_state(leg: &Leg<'_>) -> Result<biomcp_types::LocalBundleState, RequestError> {
    Ok(leg
        .stored_bundle_state(&VALIDITY)?
        .with(leg.stored_bundle_state(&DOSAGE)?))
}
