//! Published ClinGen validity and dosage bundles, keyed only by HGNC identity.
mod raw;
use crate::quotas;
use biodata::{ClinGenGene, HgncId, Source, SyncResult};
use biomcp_fetch::{
    Bundle, Fetch, Find, Freshness, KeyKind, KeyUse, Leg, RequestError, RowFormat, SourceSpec,
};
use std::{marker::PhantomData, time::Duration};
/// ClinGen published summaries are generated anew for each request.
pub static SPEC: SourceSpec = SourceSpec {
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
    let dosage = dosage.into_iter().next();
    Ok(Some(ClinGenGene {
        validity: validity
            .into_iter()
            .map(raw::Validity::checked)
            .collect::<Result<_, _>>()?,
        haploinsufficiency: dosage.as_ref().and_then(|v| v.haploinsufficiency.clone()),
        triplosensitivity: dosage.as_ref().and_then(|v| v.triplosensitivity.clone()),
        dosage_date: dosage.and_then(|v| v.date),
    }))
}
fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::ClinGen,
        reason: biodata::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
