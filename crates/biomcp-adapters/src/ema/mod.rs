//! Canonical EMA source declaration.
mod map;
mod raw;
use biodata::DrugId;
use biomcp_fetch::{
    Bundle, BundleInfo, Fetch, Find, Freshness, KeyKind, KeyUse, Leg, RequestError, RowFormat,
    SourceSpec, Terms,
};
use biomcp_types::Source;
use biomcp_types::{DrugCard, DrugHeader, DrugRecords, DrugRegulatory, Section};
use std::marker::PhantomData;
use std::time::Duration;
/// Publisher policy shared by health, request admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: Source::Ema,
    base: "https://www.ema.europa.eu",
    terms: Terms {
        name: crate::drug_sources::EMA_TERMS.name,
        url: crate::drug_sources::EMA_TERMS.url,
        usage: crate::drug_sources::EMA_TERMS.usage,
    },
    quota: &crate::quotas::EMA,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};

const MEDICINES: Bundle<raw::Medicine> = Bundle {
    spec: &SPEC,
    name: "ema",
    path: "/en/documents/report/medicines-output-medicines_json-report_en.json",
    format: RowFormat::Json {
        rows: "data",
        metadata: "meta",
    },
    fresh: Freshness::Days(1),
    row: PhantomData,
};
/// Explicit complete-file synchronization through the shared atomic bundle store.
pub fn sync(fetch: &Fetch) -> Result<BundleInfo, RequestError> {
    fetch.sync(&MEDICINES)
}
/// Literal medicine-name candidates in native bundle order; never synchronize on reads.
pub fn search(leg: &Leg<'_>, query: &str) -> Result<Vec<DrugHeader>, RequestError> {
    let (rows, _) = leg.stored_rows(&MEDICINES, Find::Words(query))?;
    Ok(rows.iter().map(map::header).collect())
}
fn exact(leg: &Leg<'_>, id: &DrugId) -> Result<Option<raw::Medicine>, RequestError> {
    let (mut rows, _) = leg.stored_rows(&MEDICINES, Find::Exact(KeyKind::Id, &id.to_string()))?;
    // SQLite indexes fold keys; exact get still requires the published identifier itself.
    Ok(rows.pop().filter(|row| row.id == *id))
}
/// Exact native EMA identity card without authorization events.
pub fn card(leg: &Leg<'_>, id: &DrugId) -> Result<Option<DrugCard>, RequestError> {
    Ok(exact(leg, id)?.as_ref().map(map::card))
}
/// Source-native dates and status for only the selected EMA application record.
pub fn regulatory(leg: &Leg<'_>, id: &DrugId) -> Result<Option<DrugRegulatory>, RequestError> {
    Ok(exact(leg, id)?.map(|row| DrugRegulatory {
        identity: map::header(&row),
        regulatory: Section::Data {
            source: Source::Ema,
            data: DrugRecords {
                records: vec![map::event(&row)].into(),
                offset: 0,
                total: Some(1),
                truncated: None,
                version: Some(row.meta.timestamp),
                retrieved: None,
            },
        },
    }))
}

/// Inspect this source's validated local catalog without reading rows or synchronizing.
pub(crate) fn bundle_state(leg: &Leg<'_>) -> Result<biomcp_types::LocalBundleState, RequestError> {
    leg.stored_bundle_state(&MEDICINES)
}
