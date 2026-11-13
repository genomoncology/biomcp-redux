//! Orphadata Science epidemiology and natural-history files as one local bundle.
mod raw;
use biodata::{Attribution, Curie, Text};
use biomcp_fetch::{
    BundlePart, BundleSet, DocumentMedia, Fetch, Find, Freshness, KeyKind, KeyUse, Leg,
    RequestError, RowFormat, SetRow, SourceSpec,
};
use biomcp_types::{OrphanetEpidemiology, OrphanetPrevalence, Source, SyncResult};
use raw::{Native, NativePrevalence};
use std::{marker::PhantomData, time::Duration};

/// Orphadata's file host and its CC BY 4.0 terms.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: Source::Orphadata,
    base: "https://www.orphadata.com",
    terms: crate::disease_sources::components::ORPHANET,
    quota: &crate::quotas::ORPHADATA,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
const PREVALENCE: &str = "prevalence";
const AGES: &str = "ages";
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

/// The ORPHAcode of an Orphanet identifier in any of its common spellings.
pub fn orpha_code(id: &Curie) -> Option<&str> {
    let (prefix, code) = id.as_str().split_once(':')?;
    (["Orphanet", "ORPHA"]
        .iter()
        .any(|p| p.eq_ignore_ascii_case(prefix))
        && !code.is_empty()
        && code.bytes().all(|b| b.is_ascii_digit()))
    .then_some(code)
}

/// Read the stored estimates, onset and inheritance for each Orphanet code, in request order.
/// A code absent from both files has no item.
pub fn epidemiology(
    leg: &Leg<'_>,
    ids: &[Curie],
) -> Result<Vec<OrphanetEpidemiology>, RequestError> {
    let mut codes: Vec<&str> = Vec::new();
    for code in ids.iter().filter_map(orpha_code) {
        if !codes.contains(&code) {
            codes.push(code);
        }
    }
    if codes.is_empty() {
        return Ok(Vec::new());
    }
    let keys: Vec<String> = codes.iter().map(|code| Native::key(code)).collect();
    let keys: Vec<&str> = keys.iter().map(String::as_str).collect();
    let (rows, _) = leg.stored_set_rows(&SET, Find::AnyExact(KeyKind::Id, &keys))?;
    codes
        .into_iter()
        .filter_map(|code| {
            let rows: Vec<&SetRow<Native>> = rows.iter().filter(|r| r.value.code == code).collect();
            (!rows.is_empty()).then(|| item(code, &rows))
        })
        .collect()
}

fn text(value: Option<&String>) -> Option<Text> {
    value.and_then(|v| crate::mapped::source_text(v).ok())
}
fn texts(values: &[String]) -> Box<[Text]> {
    values
        .iter()
        .filter_map(|v| crate::mapped::source_text(v).ok())
        .collect()
}
fn item(code: &str, rows: &[&SetRow<Native>]) -> Result<OrphanetEpidemiology, RequestError> {
    let invalid = || RequestError::Input(biomcp_fetch::InputError::Serialization);
    let id: Curie = Native::key(code).parse().map_err(|_| invalid())?;
    let first = rows.first().map(|r| &r.value);
    let part = |name: &str| rows.iter().find(|r| &*r.part == name).map(|r| &r.value);
    let prevalence = part(PREVALENCE);
    let ages = part(AGES);
    Ok(OrphanetEpidemiology {
        label: text(first.and_then(|r| r.name.as_ref())),
        disorder_type: text(first.and_then(|r| r.disorder_type.as_ref())),
        prevalence: prevalence
            .map(|r| r.prevalence.iter().map(estimate).collect())
            .unwrap_or_default(),
        onset: ages.map(|r| texts(&r.onset)).unwrap_or_default(),
        inheritance: ages.map(|r| texts(&r.inheritance)).unwrap_or_default(),
        attribution: Attribution {
            approved: None,
            published: None,
            source: crate::mapped::text(SPEC.source.into()).map_err(|_| invalid())?,
            record: crate::mapped::text(id.as_str()).ok(),
            // Both files of one release share the version; the dates differ by seconds.
            version: text(first.and_then(|r| r.version.as_ref())),
            release: text(prevalence.or(first).map(|r| &r.date)),
            submitter: None,
            evaluated: None,
            url: None,
        },
        id,
    })
}
fn estimate(row: &NativePrevalence) -> OrphanetPrevalence {
    // A class-only estimate has no mean value; the file writes 0.0 in its place.
    let class_only = row.qualification.as_deref() == Some("Class only");
    let value = row
        .value
        .as_ref()
        .filter(|v| !(class_only && v.as_str() == "0.0"));
    OrphanetPrevalence {
        id: text(row.id.as_ref()),
        kind: text(row.kind.as_ref()),
        qualification: text(row.qualification.as_ref()),
        class: text(row.class.as_ref()),
        value: text(value),
        geography: text(row.geography.as_ref()),
        source: text(row.source.as_ref()),
        pmids: row.source.as_deref().map(pmids).unwrap_or_default(),
        validation: text(row.validation.as_ref()),
    }
}
/// PubMed IDs written as `NNNN[PMID]` in Orphanet's underscore-separated source text.
fn pmids(source: &str) -> Box<[Text]> {
    source
        .split('_')
        .filter_map(|part| part.trim().strip_suffix("[PMID]"))
        .filter(|id| !id.is_empty() && id.bytes().all(|b| b.is_ascii_digit()))
        .filter_map(|id| crate::mapped::text(id).ok())
        .collect()
}

/// Complete source policy and command declaration.
pub(crate) mod decl;
