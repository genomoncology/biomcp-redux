//! Join FDA's currently listed reports with the shipped quarterly tables.
use super::{SPEC, decl::INDEX};
use crate::fda_html;
use biodata::{PartialDate, Text};
use biomcp_fetch::{Endpoint, Freshness, Leg, PathSegment, RequestError};
use biomcp_types::{DrugSafetySignal, Source};
use serde::Deserialize;
use std::{collections::BTreeMap, time::Duration};

const SOURCE: Source = Source::FdaSafetySignals;
const INDEX_READ: Endpoint = SPEC.get(INDEX).bytes(1024 * 1024).fresh(Freshness::Days(1));
const PAGE: Endpoint = SPEC
    .get("")
    .bytes(2 * 1024 * 1024)
    .fresh(Freshness::Days(30));
const PAGE_WAIT: Duration = Duration::from_secs(1);

#[derive(Deserialize)]
struct Seed {
    date: PartialDate,
    reports: Vec<SeedReport>,
}

#[derive(Deserialize)]
struct SeedReport {
    quarter: String,
    path: String,
    table: String,
}

/// Quarterly rows and how much of the list was available to search.
pub struct Signals {
    /// Rows, newest quarter first, in FDA's order within a quarter.
    pub items: Vec<DrugSafetySignal>,
    /// Snapshot date when the shipped tables contributed to the search.
    pub seed_date: Option<PartialDate>,
    /// Number of quarters with no readable table.
    pub unread_quarters: u32,
    /// The index failure when only the shipped list was available.
    pub index_error: Option<RequestError>,
}

/// Read FDA's current index and use shipped tables if a quarter cannot be refreshed yet.
pub fn read(leg: &Leg<'_>) -> Result<Signals, RequestError> {
    if leg.disabled_source(SOURCE) {
        return Err(biomcp_fetch::SourceError::disabled(SOURCE).into());
    }
    let seed: Seed = serde_json::from_str(include_str!("seed.json")).map_err(|_| malformed())?;
    let index = fda_html::read(leg, &INDEX_READ, SOURCE, false).and_then(index_reports);
    let mut signals = Signals {
        items: Vec::new(),
        seed_date: None,
        unread_quarters: 0,
        index_error: index.as_ref().err().copied(),
    };
    let mut spaced = false;
    let mut updated = BTreeMap::new();
    for (key, (path, quarter)) in index.unwrap_or_default().into_iter().rev() {
        match leg.dependency(&SPEC, PAGE_WAIT, |child| {
            page_rows(child, &path, &quarter, spaced)
        }) {
            Ok(rows) => {
                updated.insert(key, Some(rows));
            }
            Err(_) => {
                spaced = true;
                updated.insert(key, None);
            }
        }
    }
    for report in seed.reports {
        let key = quarter_key(&report.quarter).ok_or_else(malformed)?;
        let rows = updated.entry(key).or_default();
        if rows.is_none() {
            signals.seed_date = Some(seed.date);
            *rows =
                Some(decode(&report.table, &report.path, &report.quarter).ok_or_else(malformed)?);
        }
    }
    signals.unread_quarters = updated.values().filter(|rows| rows.is_none()).count() as u32;
    signals.items = updated.into_values().rev().flatten().flatten().collect();
    Ok(signals)
}

type Reports = BTreeMap<(u16, u8), (String, String)>;

fn index_reports(body: String) -> Result<Reports, RequestError> {
    let reports: Reports = fda_html::links(&body)
        .filter_map(|(path, quarter)| Some((quarter_key(&quarter)?, (path.to_owned(), quarter))))
        .collect();
    if reports.is_empty() {
        return Err(malformed());
    }
    Ok(reports)
}

fn quarter_key(quarter: &str) -> Option<(u16, u8)> {
    let (months, year) = quarter.rsplit_once(' ')?;
    let year: u16 = year.parse().ok()?;
    let (start, end) = months.split_once(['-', '–', '—'])?;
    let quarter = match (start.trim(), end.trim()) {
        ("January", "March") => 1,
        ("April", "June") => 2,
        ("July", "September") => 3,
        ("October", "December") => 4,
        _ => return None,
    };
    Some((year, quarter))
}

fn page_rows(
    leg: &Leg<'_>,
    path: &str,
    quarter: &str,
    cache_only: bool,
) -> Result<Vec<DrugSafetySignal>, RequestError> {
    let segments = path
        .trim_start_matches('/')
        .split('/')
        .map(PathSegment::new)
        .collect::<Result<Vec<_>, _>>()
        .map_err(RequestError::Input)?;
    let body = fda_html::read(
        leg,
        &PAGE.clone().with_segments(&segments),
        SOURCE,
        cache_only,
    )?;
    decode(&body, path, quarter).ok_or_else(malformed)
}

fn decode(body: &str, path: &str, quarter: &str) -> Option<Vec<DrugSafetySignal>> {
    let quarter = Text::new(quarter).ok()?;
    let url = Text::new(&format!("{}{path}", SPEC.base)).ok()?;
    fda_html::rows(body, |[product, signal, info]| {
        product.to_ascii_lowercase().starts_with("product name")
            && signal.to_ascii_lowercase().contains("potential signal")
            && info
                .to_ascii_lowercase()
                .starts_with("additional information")
    })?
    .into_iter()
    .map(|[product, signal, additional_information]| {
        Some(DrugSafetySignal {
            product: Text::new(&product).ok()?,
            signal: Text::new(&signal).ok()?,
            additional_information: additional_information.into(),
            quarter: quarter.clone(),
            url: url.clone(),
        })
    })
    .collect()
}

fn malformed() -> RequestError {
    biomcp_fetch::bad_reply(SOURCE, None)
}
