//! FDA receipt-date report counts: openFDA's daily `count=` buckets for the whole range in one
//! request, summed to years or months.
use biodata::PartialDate;
use biomcp_fetch::{Arg, BadReply, Freshness, Input, InputError, Leg, Read, Reply, RequestError};
use biomcp_types::{
    AdverseEventAggregate, AdverseEventReceivedCount, AdverseEventSearch, AdverseEventSource,
    ComponentLabel, Source,
};
use serde::Deserialize;
use std::collections::BTreeMap;

/// Report counts per receipt period, oldest first.
#[derive(Debug)]
pub struct ReceivedPage {
    /// One row per year or month of the requested range.
    pub items: Vec<AdverseEventReceivedCount>,
    /// Publisher dataset date.
    pub updated: Option<PartialDate>,
    /// Original retrieval time in Unix seconds.
    pub fetched: Option<i64>,
    /// Applicable terminology terms.
    pub components: Box<[ComponentLabel]>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Response {
    Page { meta: Meta, results: Vec<Day> },
    Absent { error: crate::openfda::Absence },
}
#[derive(Debug, Deserialize)]
struct Meta {
    last_updated: Option<PartialDate>,
}
#[derive(Debug, Deserialize)]
struct Day {
    time: Box<str>,
    count: u64,
}
impl Reply for Response {
    fn data_date(&self) -> Option<String> {
        match self {
            Self::Page { meta, .. } => meta.last_updated.map(|date| date.to_string()),
            Self::Absent { .. } => None,
        }
    }
    fn check(&self) -> Result<(), BadReply> {
        match self {
            Self::Page { results, .. } if results.iter().all(|day| day_of(&day.time).is_ok()) => {
                Ok(())
            }
            Self::Absent { error } if error.valid() => Ok(()),
            _ => Err(BadReply),
        }
    }
    fn check_status(&self, status: u16) -> Result<(), BadReply> {
        if matches!(
            (self, status),
            (Self::Page { .. }, 200) | (Self::Absent { .. }, 404)
        ) {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}

/// openFDA's native YYYYMMDD bucket time as a checked calendar day.
fn day_of(time: &str) -> Result<PartialDate, BadReply> {
    match (time.get(..4), time.get(4..6), time.get(6..)) {
        (Some(year), Some(month), Some(day)) if time.len() == 8 => format!("{year}-{month}-{day}")
            .parse()
            .map_err(|_| BadReply),
        _ => Err(BadReply),
    }
}

fn date(year: u16, month: u8, day: u8) -> Result<PartialDate, RequestError> {
    PartialDate::new(year, Some(month), Some(day)).map_err(|_| InputError::Serialization.into())
}

fn last_day(year: u16, month: u8) -> Result<PartialDate, RequestError> {
    let day = (28..=31)
        .rev()
        .find(|day| PartialDate::new(year, Some(month), Some(*day)).is_ok())
        .ok_or(InputError::Serialization)?;
    date(year, month, day)
}

/// The source's endpoint, receipt-date field and admitted terminology for this search.
fn source_parts(
    leg: &Leg<'_>,
    query: &AdverseEventSearch,
) -> Result<
    (
        &'static biomcp_fetch::SourceSpec,
        &'static str,
        Box<[ComponentLabel]>,
    ),
    RequestError,
> {
    match query.source() {
        AdverseEventSource::Faers if query.reaction().is_some() => {
            super::components::meddra(leg)?;
            Ok((
                &crate::faers::SPEC,
                "receivedate",
                super::components::labels(Source::Faers, Source::Meddra)?,
            ))
        }
        AdverseEventSource::Faers => Ok((&crate::faers::SPEC, "receivedate", Box::default())),
        AdverseEventSource::Maude => {
            super::components::gmdn(leg)?;
            Ok((
                &crate::maude::SPEC,
                "date_received",
                super::components::labels(Source::Maude, Source::Gmdn)?,
            ))
        }
        AdverseEventSource::Vaers => Err(InputError::Serialization.into()),
    }
}

/// Count matching reports by FDA receipt date with the search's own expression and filters.
pub fn counts(leg: &Leg<'_>, query: &AdverseEventSearch) -> Result<ReceivedPage, RequestError> {
    let (Some(aggregate), Some(from), Some(to)) = (
        query
            .aggregate()
            .filter(|aggregate| aggregate.is_received()),
        query.received_from(),
        query.received_to(),
    ) else {
        return Err(InputError::Serialization.into());
    };
    let (spec, field, components) = source_parts(leg, query)?;
    let selector = match query.source() {
        AdverseEventSource::Maude => crate::maude::expression(query),
        _ => query.faers_expression(),
    };
    let (low, high) = (from.to_string(), to.to_string());
    let endpoint = spec.get("").fresh(Freshness::Days(1)).json_absence();
    // No `limit`: openFDA returns every daily bucket of a date count without one, and refuses a
    // `limit` above 100 without an API key (measured 2026-11-13).
    let received = leg.call(
        &endpoint,
        Input::Params(&[
            ("search", Arg::Text(&selector)),
            ("count", Arg::Text(field)),
        ]),
        Read::<Response>::json().check(&|reply| match reply {
            Response::Page { results, .. }
                if results.iter().any(|day| {
                    day_of(&day.time).is_ok_and(|day| {
                        let day = day.to_string();
                        day < low || day > high
                    })
                }) =>
            {
                Err(BadReply)
            }
            _ => Ok(()),
        }),
    )?;
    let mut months = BTreeMap::<(u16, u8), u64>::new();
    let mut updated = None;
    if let Response::Page { meta, results } = received.value {
        updated = meta.last_updated;
        for day in results {
            let date = day_of(&day.time).map_err(|_| InputError::Serialization)?;
            let month = date.month().ok_or(InputError::Serialization)?;
            let total = months.entry((date.year(), month)).or_default();
            *total = total.saturating_add(day.count);
        }
    }
    Ok(ReceivedPage {
        items: periods(aggregate, (from, to), &months, updated)?,
        updated,
        fetched: received.meta.fetched,
        components,
    })
}

/// Every year or month of the range up to the source's last update, oldest first, with zeros and
/// partial periods marked.
fn periods(
    aggregate: AdverseEventAggregate,
    (from, to): (PartialDate, PartialDate),
    months: &BTreeMap<(u16, u8), u64>,
    updated: Option<PartialDate>,
) -> Result<Vec<AdverseEventReceivedCount>, RequestError> {
    let partial = |first: PartialDate, last: PartialDate| {
        first.to_string() < from.to_string()
            || last.to_string() > to.to_string()
            || updated.is_some_and(|date| date.to_string() < last.to_string())
    };
    // Periods that start after the source's last update hold no reports yet, so they are left out.
    let started =
        |first: PartialDate| updated.is_none_or(|date| first.to_string() <= date.to_string());
    let mut items = Vec::new();
    for year in from.year()..=to.year() {
        if !started(date(year, 1, 1)?) {
            break;
        }
        if aggregate == AdverseEventAggregate::ReceivedYear {
            items.push(AdverseEventReceivedCount {
                received: year.to_string().into(),
                count: months.range((year, 1)..=(year, 12)).map(|(_, n)| n).sum(),
                partial: partial(date(year, 1, 1)?, date(year, 12, 31)?),
            });
            continue;
        }
        let first_month = if year == from.year() {
            from.month().unwrap_or(1)
        } else {
            1
        };
        let last_month = if year == to.year() {
            to.month().unwrap_or(12)
        } else {
            12
        };
        for month in first_month..=last_month {
            if !started(date(year, month, 1)?) {
                break;
            }
            items.push(AdverseEventReceivedCount {
                received: format!("{year:04}-{month:02}").into(),
                count: months.get(&(year, month)).copied().unwrap_or(0),
                partial: partial(date(year, month, 1)?, last_day(year, month)?),
            });
        }
    }
    Ok(items)
}
