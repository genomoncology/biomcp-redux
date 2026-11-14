//! Native product recall records with exact optional enforcement classification.
use biomcp_fetch::{Input, Read};
mod map;
mod raw;
use biodata::{DeviceRecallNumber, Text};
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, Leg, RequestError, SourceSpec, Terms};
use biomcp_types::{DeviceRecallCard, DiagnosticCandidate, DiagnosticSearch, Problem, Source, Use};
use raw::Identity;
use std::{collections::HashMap, time::Duration};
/// Shared openFDA admission policy for recall and its exact enforcement lookup.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: Source::FdaDeviceRecall,
    base: "https://api.fda.gov",
    terms: Terms {
        name: "openFDA terms and license; third-party exceptions",
        url: "https://open.fda.gov/terms/",
        usage: Use::Open,
    },
    quota: &crate::quotas::OPENFDA,
    key: crate::openfda_key::OPTIONAL,
    deadline: Duration::from_secs(15),
    command_budget: None,
};
const RECALL: &str = "/device/recall.json";
const ENFORCEMENT: &str = "/device/enforcement.json";
/// One bounded native recall-record page.
#[derive(Debug)]
pub struct Page {
    /// Native headers, without event deduplication.
    pub items: Box<[DiagnosticCandidate]>,
    /// Native count of recall records; enforcement records under a classification filter.
    pub total: u64,
    /// Rejected optional values with record provenance.
    pub problems: Vec<Problem>,
    /// Rows whose exact identity has no record in the joined openFDA dataset.
    pub unmatched: u32,
    /// A failed join leaves the selected rows and their total unchanged.
    pub join_failure: Option<Failure>,
}
fn literal(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}
/// openFDA answers HTTP 400 to these characters even inside a quoted phrase (live, 2026-11-13).
/// Its tokenizer splits words at them, so a space finds the same words.
const REFUSED: &[char] = &['!', '#', '$', ';', '?', '^', '|', '~', '`'];
/// The device name as one quoted phrase on product_description, with refused punctuation spaced.
fn phrase(value: &Text) -> String {
    let words: String = value
        .as_str()
        .chars()
        .map(|c| if REFUSED.contains(&c) { ' ' } else { c })
        .collect();
    literal(&words.split_whitespace().collect::<Vec<_>>().join(" "))
}
/// Recall selectors; a classification selects enforcement records and their field spellings.
/// Both datasets spell the device name field product_description.
pub fn expression(q: &DiagnosticSearch) -> String {
    let class = q.classification();
    let mut terms = Vec::new();
    if let Some(v) = class {
        terms.push(format!("classification:\"{v}\""));
    }
    if let Some(v) = q.name() {
        terms.push(format!("product_description:{}", phrase(v)));
    }
    if let Some(v) = q.manufacturer() {
        terms.push(format!("recalling_firm:{}", literal(v.as_str())));
    }
    if let Some(v) = q.product_code() {
        terms.push(format!("product_code:{}", literal(v.as_str())));
    }
    if q.initiated_from().is_some() || q.initiated_to().is_some() {
        let day = |d: Option<biodata::PartialDate>| match d {
            Some(d) if class.is_some() => d.to_string().replace('-', ""),
            Some(d) => d.to_string(),
            None => "*".into(),
        };
        let field = if class.is_some() {
            "recall_initiation_date"
        } else {
            "event_date_initiated"
        };
        let (from, to) = (day(q.initiated_from()), day(q.initiated_to()));
        terms.push(format!("{field}:[{from} TO {to}]"));
    }
    terms.join(" AND ")
}
/// Native lot lists (`code_info`, `more_code_info`) reach 9.9 MB in one record and
/// 19 MB across a 50-record page (openFDA bulk export, 2026-11-10), so a smaller
/// bound fails whole pages; a larger page still fails as too-large, never trimmed.
const PAGE_BYTES: u64 = 32 * 1024 * 1024;
fn endpoint(path: &'static str) -> Endpoint {
    SPEC.get(path)
        .fresh(Freshness::Days(1))
        .bytes(PAGE_BYTES)
        .json_absence()
}
fn selected<T: crate::openfda::Item>(
    leg: &Leg<'_>,
    q: &DiagnosticSearch,
    path: &'static str,
    sort: &str,
) -> Result<(Vec<T>, u64), RequestError> {
    let selector = expression(q);
    let mut args = Vec::new();
    if !selector.is_empty() {
        args.push(("search", Arg::Text(&selector)));
    }
    args.extend([
        ("limit", Arg::Num(q.limit().into())),
        ("skip", Arg::Num(q.offset().into())),
        ("sort", Arg::Text(sort)),
    ]);
    let reply = crate::openfda::page(
        Source::FdaDeviceRecall,
        leg,
        &endpoint(path),
        &args,
        q.offset(),
        q.limit(),
        &|reply| counts(reply, q.offset(), q.limit(), false),
    )?;
    Ok(match reply.value {
        raw::Response::Page { meta, results } => (results, meta.results.total),
        raw::Response::Absent { .. } => (Vec::new(), 0),
    })
}
type Joined<A, B> = (Vec<(A, Option<B>)>, u32, Option<Failure>);
/// One exact request pairs a page with its records in the other openFDA recall dataset.
fn joined<A: Identity, B: Identity + crate::openfda::Item>(
    leg: &Leg<'_>,
    found: Vec<A>,
    path: &'static str,
    field: &'static str,
) -> Joined<A, B> {
    let ids: Vec<_> = found.iter().map(|row| row.id().clone()).collect();
    let (other, failure) = match batch::<B>(leg, path, field, &ids) {
        Ok(rows) => (rows, None),
        Err(failure) => (Vec::new(), Some(failure)),
    };
    let mut other: HashMap<_, _> = other
        .into_iter()
        .map(|row| (row.id().clone(), row))
        .collect();
    let rows: Vec<_> = found
        .into_iter()
        .map(|row| {
            let matched = other.remove(row.id());
            (row, matched)
        })
        .collect();
    let unmatched = if failure.is_some() {
        0
    } else {
        rows.iter().filter(|(_, other)| other.is_none()).count() as u32
    };
    (rows, unmatched, failure)
}
/// Search one bounded source-ordered native page and join its classifications in one request.
/// A classification selects enforcement records first, then joins their native recall records.
pub fn search(leg: &Leg<'_>, q: &DiagnosticSearch) -> Result<Page, RequestError> {
    let mut problems = Vec::new();
    let (items, total, unmatched, join_failure) = if q.classification().is_some() {
        let (found, total) =
            selected::<raw::Enforcement>(leg, q, ENFORCEMENT, "recall_initiation_date:desc")?;
        let (rows, unmatched, failure) =
            joined::<_, raw::Record>(leg, found, RECALL, "product_res_number");
        let items = rows
            .into_iter()
            .map(|(class, record)| {
                let row = match record {
                    Some(record) => map::row(record, &mut problems),
                    None => map::bare(class.recall_number.clone()),
                };
                map::candidate(row, Some(class), &mut problems)
            })
            .collect();
        (items, total, unmatched, failure)
    } else {
        let (found, total) = selected::<raw::Record>(leg, q, RECALL, "event_date_initiated:desc")?;
        let (rows, unmatched, failure) =
            joined::<_, raw::Enforcement>(leg, found, ENFORCEMENT, "recall_number");
        let items = rows
            .into_iter()
            .map(|(record, class)| {
                let row = map::row(record, &mut problems);
                map::candidate(row, class, &mut problems)
            })
            .collect();
        (items, total, unmatched, failure)
    };
    Ok(Page {
        items,
        total,
        problems,
        unmatched,
        join_failure,
    })
}
fn counts<T>(reply: &raw::Response<T>, skip: u32, limit: u32, exact: bool) -> Result<(), BadReply> {
    if let raw::Response::Page { meta, .. } = reply
        && (meta.results.skip != skip
            || meta.results.limit != limit
            || exact && !(1..=u64::from(limit)).contains(&meta.results.total))
    {
        return Err(BadReply);
    }
    Ok(())
}
/// An exact lookup failure retains its conflicting identity field.
#[derive(Debug)]
pub struct Failure {
    /// Source or local request failure.
    pub error: RequestError,
    /// Wrong returned recall identity; never admitted to the cache.
    pub identity_field: Option<&'static str>,
}
/// Read exactly the named records; another identity fails before the cache admits it.
fn batch<T: Identity + crate::openfda::Item>(
    leg: &Leg<'_>,
    path: &'static str,
    field: &'static str,
    ids: &[DeviceRecallNumber],
) -> Result<Vec<T>, Failure> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let names: Vec<_> = ids.iter().map(|id| format!("\"{id}\"")).collect();
    let selector = match names.as_slice() {
        [name] => format!("{field}.exact:{name}"),
        names => format!("{field}.exact:({})", names.join(" OR ")),
    };
    let limit = ids.len() as u32;
    let identity = |reply: &raw::Response<T>| !matches!(reply,raw::Response::Page {results,..} if results.iter().any(|r|!ids.contains(r.id())));
    let reply = leg
        .call(
            &endpoint(path),
            Input::Params(&[
                ("search", Arg::Text(&selector)),
                ("limit", Arg::Num(limit.into())),
                ("skip", Arg::Num(0)),
            ]),
            Read::<raw::Response<T>>::json()
                .check(&|reply| counts(reply, 0, limit, true))
                .cache_if(&identity),
        )
        .map(|received| received.value)
        .map_err(|error| Failure {
            error,
            identity_field: None,
        })?;
    if !identity(&reply) {
        return Err(Failure {
            error: biomcp_fetch::SourceError {
                source: Source::FdaDeviceRecall,
                reason: biomcp_types::Reason::BadReply,
                status: Some(200),
                tls_cause: None,
                retry_after: None,
            }
            .into(),
            identity_field: Some(field),
        });
    }
    Ok(match reply {
        raw::Response::Page { results, .. } => results,
        raw::Response::Absent { .. } => Vec::new(),
    })
}
fn exact<T: Identity + crate::openfda::Item>(
    leg: &Leg<'_>,
    path: &'static str,
    field: &'static str,
    id: &DeviceRecallNumber,
) -> Result<Option<T>, Failure> {
    batch(leg, path, field, std::slice::from_ref(id)).map(|rows| rows.into_iter().next())
}
/// Get only the selected native product recall record.
pub fn get(
    leg: &Leg<'_>,
    id: &DeviceRecallNumber,
) -> Result<Option<crate::mapped::Mapped<DeviceRecallCard>>, Failure> {
    exact::<raw::Record>(leg, RECALL, "product_res_number", id).map(|row| row.map(map::mapped))
}
/// One exact enforcement lookup adds its native classification and seriousness fields to the card.
pub fn enforcement(
    leg: &Leg<'_>,
    id: &DeviceRecallNumber,
    card: &mut DeviceRecallCard,
) -> Result<Vec<Problem>, Failure> {
    exact::<raw::Enforcement>(leg, ENFORCEMENT, "recall_number", id)
        .map(|row| row.map_or_else(Vec::new, |row| map::enforcement(row, card)))
}

/// Complete source policy and command declaration.
pub(crate) mod decl;
