//! Published ClinGen validity and dosage bundles, keyed only by HGNC identity.
mod raw;
use crate::quotas;
use biodata::HgncId;
use biomcp_fetch::{
    Bundle, Fetch, Find, Freshness, KeyKind, KeyUse, Leg, RequestError, RowFormat, SourceSpec,
};
use biomcp_types::{ClinGenGene, Source, SyncResult};
use std::{marker::PhantomData, time::Duration};
/// ClinGen published summaries are generated anew for each request.
pub static SPEC: SourceSpec = SourceSpec {
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
    })
}
/// Read complete source assessments for one identity, synchronizing first use.
pub fn gene(leg: &Leg<'_>, id: HgncId) -> Result<Option<ClinGenGene>, RequestError> {
    let text = id.to_string();
    let validity = leg.rows(&VALIDITY, Find::Exact(KeyKind::Id, &text))?;
    let dosage = leg.rows(&DOSAGE, Find::Exact(KeyKind::Id, &text))?;
    if validity.is_empty() && dosage.is_empty() {
        return Ok(None);
    }
    if dosage.len() > 1 {
        return Err(bad());
    }
    let (haploinsufficiency, triplosensitivity, dosage_date) = match dosage.into_iter().next() {
        Some(row) => (
            Some(row.haploinsufficiency.ok_or_else(bad)?),
            Some(row.triplosensitivity.ok_or_else(bad)?),
            Some(row.date.ok_or_else(bad)?),
        ),
        None => (None, None, None),
    };
    Ok(Some(ClinGenGene {
        validity: validity
            .into_iter()
            .map(raw::Validity::checked)
            .collect::<Result<_, _>>()?,
        haploinsufficiency,
        triplosensitivity,
        dosage_date,
    }))
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
