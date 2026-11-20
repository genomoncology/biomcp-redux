//! FDA's index joined with the shipped seed; pages the seed lacks are read one at a time, only
//! while FDA's 30-second spacing allows, and a row not yet read is kept, marked, never dropped.
use super::page::{Listed, Page, index, page};
use super::seed::seed;
use super::{INDEX_PATH, SITE, SPEC};
use crate::{Mapped, Refusal};
use biodata::{PartialDate, Text};
use biomcp_fetch::{
    DocumentMedia, Endpoint, Freshness, Input, Leg, PathSegment, Read, RequestError,
};
use biomcp_types::{DrugSafetyCommunication, Problem, ProblemCode, Source};
use std::time::Duration;

const INDEX: Endpoint = SPEC
    .get(INDEX_PATH)
    .bytes(4 * 1024 * 1024)
    .fresh(Freshness::Days(1));
/// A communication's page changes rarely; a read page is kept a month.
const PAGE: Endpoint = SPEC
    .get("")
    .bytes(4 * 1024 * 1024)
    .fresh(Freshness::Days(30));
/// How long a search waits for FDA's spacing before leaving a page for a later call.
const PAGE_WAIT: Duration = Duration::from_secs(1);

/// Every communication BioMCP can list now, newest first.
#[derive(Debug)]
pub struct Communications {
    /// Rows from FDA's index, with seed rows FDA's index no longer lists.
    pub items: Vec<DrugSafetyCommunication>,
    /// The oldest full date on FDA's index.
    pub list_starts: Option<PartialDate>,
    /// Index dates that are not full dates, and index dates a page contradicts.
    pub problems: Vec<Problem>,
    /// Why FDA's index could not be read; the rows are then the seed's alone.
    pub index_error: Option<RequestError>,
}

/// The shipped seed's date.
pub fn seed_date() -> Option<PartialDate> {
    seed().map(|s| s.date)
}

/// FDA's index joined with the seed. Only a missing index with no seed is an error.
pub fn communications(leg: &Leg<'_>) -> Result<Communications, RequestError> {
    let seeded = seed().map_or(&[][..], |s| s.items.as_slice());
    let listed = match read_index(leg) {
        Ok(listed) => listed,
        Err(error) if !seeded.is_empty() => {
            let mut items = seeded.to_vec();
            newest_first(&mut items);
            return Ok(Communications {
                items,
                list_starts: None,
                problems: Vec::new(),
                index_error: Some(error),
            });
        }
        Err(error) => return Err(error),
    };
    let mut problems = Vec::new();
    let mut spaced = false;
    let mut items: Vec<_> = listed
        .iter()
        .map(|listed| {
            let known = seeded.iter().find(|row| row.id == listed.id);
            let read = match known {
                Some(row) => Some(row.clone()),
                None if spaced => None,
                None => {
                    let body = leg.dependency(&SPEC, PAGE_WAIT, |page_leg| {
                        read_page(page_leg, &listed.path)
                    });
                    spaced = body.is_err();
                    body.ok().map(|page| with_page(listed, page))
                }
            };
            row(listed, read, &mut problems)
        })
        .collect();
    items.extend(
        seeded
            .iter()
            .filter(|row| !listed.iter().any(|l| l.id == row.id))
            .cloned(),
    );
    newest_first(&mut items);
    let list_starts = listed.iter().filter_map(|l| l.date).min_by_key(|d| key(*d));
    Ok(Communications {
        items,
        list_starts,
        problems,
        index_error: None,
    })
}

/// One communication with its page's text, read live; `None` when FDA lists no such id.
pub fn communication(
    leg: &Leg<'_>,
    id: &str,
) -> Result<Mapped<Option<(DrugSafetyCommunication, Text)>>, RequestError> {
    let seeded = seed().and_then(|s| s.items.iter().find(|row| row.id.as_str() == id));
    let listed = match seeded {
        Some(row) => Listed {
            id: row.id.clone(),
            title: row.title.clone(),
            date_text: row.date.map(|d| d.to_string()).unwrap_or_default(),
            date: row.date,
            path: row
                .url
                .as_str()
                .strip_prefix(SITE)
                .unwrap_or_default()
                .to_owned(),
            url: row.url.clone(),
        },
        None => match read_index(leg)?.into_iter().find(|l| l.id.as_str() == id) {
            Some(listed) => listed,
            None => {
                return Ok(Mapped {
                    value: None,
                    problems: Vec::new(),
                    refusals: Refusal::None,
                });
            }
        },
    };
    let page = read_page(leg, &listed.path)?;
    let text = crate::native_text::admit(&page.text).map_err(|_| bad_reply())?;
    let mut problems = Vec::new();
    let value = Some((
        row(&listed, Some(with_page(&listed, page)), &mut problems),
        text,
    ));
    Ok(Mapped {
        value,
        problems,
        refusals: Refusal::None,
    })
}

fn read_index(leg: &Leg<'_>) -> Result<Vec<Listed>, RequestError> {
    let body = leg
        .call(
            &INDEX,
            Input::Params(&[]),
            Read::bytes().document_media(DocumentMedia::Html),
        )?
        .value
        .0;
    index(&String::from_utf8_lossy(&body))
        .filter(|rows| !rows.is_empty())
        .ok_or_else(bad_reply)
}

fn read_page(leg: &Leg<'_>, path: &str) -> Result<Page, RequestError> {
    let segments = path
        .split('/')
        .filter(|s| !s.is_empty())
        .map(PathSegment::new)
        .collect::<Result<Vec<_>, _>>()
        .map_err(RequestError::Input)?;
    if !path.starts_with("/drugs/") || segments.is_empty() {
        return Err(bad_reply());
    }
    let endpoint = PAGE.clone().with_segments(&segments);
    let body = leg
        .call(
            &endpoint,
            Input::Params(&[]),
            Read::bytes().document_media(DocumentMedia::Html),
        )?
        .value
        .0;
    page(&String::from_utf8_lossy(&body)).ok_or_else(bad_reply)
}

/// A row from a page just read, before the index's facts are laid over it.
fn with_page(listed: &Listed, page: Page) -> DrugSafetyCommunication {
    let texts = |values: Vec<String>| {
        values
            .iter()
            .filter_map(|v| Text::from_source(v).ok())
            .collect()
    };
    DrugSafetyCommunication {
        id: listed.id.clone(),
        title: listed.title.clone(),
        date: listed.date,
        issued: page.issued,
        updated: page.updated,
        url: listed.url.clone(),
        applications: texts(page.applications),
        drugs: texts(page.drugs),
        page_read: true,
    }
}

/// FDA's index states the title, date and address; a page read adds what it states.
fn row(
    listed: &Listed,
    read: Option<DrugSafetyCommunication>,
    problems: &mut Vec<Problem>,
) -> DrugSafetyCommunication {
    let mut row = read.unwrap_or_else(|| DrugSafetyCommunication {
        id: listed.id.clone(),
        title: listed.title.clone(),
        date: None,
        issued: None,
        updated: None,
        url: listed.url.clone(),
        applications: Box::default(),
        drugs: Box::default(),
        page_read: false,
    });
    row.title = listed.title.clone();
    row.url = listed.url.clone();
    row.date = listed.date;
    let problem = |field: &str, code| Problem {
        source: Source::FdaDsc,
        record: Some(row.id.clone()),
        field: field.into(),
        code,
    };
    if listed.date.is_none() && !listed.date_text.is_empty() {
        problems.push(problem("date", ProblemCode::ValueUnknown));
    }
    if row.date.zip(row.issued).is_some_and(|(a, b)| a != b) {
        problems.push(problem("issued", ProblemCode::IdentityConflict));
    }
    row
}

fn newest_first(items: &mut [DrugSafetyCommunication]) {
    items.sort_by_key(|row| std::cmp::Reverse(row.issued.or(row.date).map(key)));
}

fn key(date: PartialDate) -> (u16, u8, u8) {
    (
        date.year(),
        date.month().unwrap_or_default(),
        date.day().unwrap_or_default(),
    )
}

fn bad_reply() -> RequestError {
    biomcp_fetch::bad_reply(Source::FdaDsc, None)
}
