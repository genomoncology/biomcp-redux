//! Read exact PMA details and native supplement pages.
use super::{SPEC, raw};
use biodata::DeviceSubmission;
use biomcp_fetch::{Arg, BadReply, Freshness, Input, Leg, Read, RequestError};
use biomcp_types::{DiagnosticGet, DiagnosticSection, Source};
use std::collections::BTreeSet;

const NATIVE_PAGE: u32 = 100;

/// Checked native rows for a PMA detail reply.
#[derive(Debug)]
pub struct Records {
    /// Application members, or the original row and just the requested large-application page.
    pub rows: Vec<DeviceSubmission>,
    /// Rejected nonidentity source fields.
    pub problems: Vec<biomcp_types::Problem>,
    /// Exact total for an already paged supplement collection.
    pub supplement_total: Option<u64>,
    /// Exact native identity field that conflicts with the request.
    pub identity_conflict: Option<&'static str>,
}

/// Read the original application or one exact selected supplement.
pub fn get(leg: &Leg<'_>, query: &DiagnosticGet) -> Result<Records, RequestError> {
    let mut selector = format!("pma_number:\"{}\"", query.id());
    if let Some(supplement) = query.supplement() {
        selector.push_str(&format!(" AND supplement_number:\"{supplement}\""));
    }
    let reply = page(leg, query, &selector, 0, NATIVE_PAGE, None)?;
    let raw::Response::Page { meta, results } = reply else {
        return Ok(mapped(vec![], None, None));
    };
    let conflict = identity(&results, query);
    if conflict.is_some() || meta.results.total <= u64::from(NATIVE_PAGE) {
        return Ok(mapped(results, None, conflict));
    }
    large_application(leg, query, &selector, meta.results.total)
}

fn large_application(
    leg: &Leg<'_>,
    query: &DiagnosticGet,
    selector: &str,
    total: u64,
) -> Result<Records, RequestError> {
    // The empty supplement cell sorts before every numbered supplement.
    let mut rows = sorted_page(leg, query, selector, 0, 2, total)?;
    let conflict = identity(&rows, query);
    rows.truncate(1);
    if conflict.is_some() || query.section() != Some(DiagnosticSection::Supplements) {
        return Ok(mapped(rows, None, conflict));
    }
    let supplement_total = total - 1;
    if u64::from(query.section_offset()) < supplement_total {
        let skip = query
            .section_offset()
            .checked_add(1)
            .ok_or_else(|| biomcp_fetch::bad_reply(Source::FdaPma, Some(200)))?;
        let limit = query.section_limit().min(NATIVE_PAGE);
        let supplements = sorted_page(leg, query, selector, skip, limit, total)?;
        let conflict = identity(&supplements, query);
        rows.extend(supplements);
        return Ok(mapped(rows, Some(supplement_total), conflict));
    }
    Ok(mapped(rows, Some(supplement_total), None))
}

fn sorted_page(
    leg: &Leg<'_>,
    query: &DiagnosticGet,
    selector: &str,
    skip: u32,
    limit: u32,
    total: u64,
) -> Result<Vec<raw::Record>, RequestError> {
    match page(leg, query, selector, skip, limit, Some(total))? {
        raw::Response::Page { results, .. } => Ok(results),
        raw::Response::Absent { .. } => Err(biomcp_fetch::bad_reply(Source::FdaPma, Some(404))),
    }
}

fn page(
    leg: &Leg<'_>,
    query: &DiagnosticGet,
    selector: &str,
    skip: u32,
    limit: u32,
    sorted_total: Option<u64>,
) -> Result<raw::Response, RequestError> {
    let endpoint = SPEC
        .get("")
        .fresh(Freshness::Days(1))
        .bytes(10 * 1024 * 1024)
        .json_absence();
    let mut args = vec![
        ("search", Arg::Text(selector)),
        ("limit", Arg::Num(limit.into())),
        ("skip", Arg::Num(skip.into())),
    ];
    if sorted_total.is_some() {
        args.push(("sort", Arg::Text("supplement_number:asc")));
    }
    leg.call(
        &endpoint,
        Input::Params(&args),
        Read::<raw::Response>::json()
            .check(&|reply| {
                let raw::Response::Page { meta, results } = reply else {
                    return Ok(());
                };
                if meta.results.skip != skip || meta.results.limit != limit {
                    return Err(BadReply);
                }
                check_rows(results, skip, sorted_total, meta.results.total)
            })
            .cache_if(&|reply| match reply {
                raw::Response::Page { results, .. } => identity(results, query).is_none(),
                raw::Response::Absent { .. } => sorted_total.is_none(),
            }),
    )
    .map(|reply| reply.value)
}

fn check_rows(
    rows: &[raw::Record],
    skip: u32,
    sorted_total: Option<u64>,
    total: u64,
) -> Result<(), BadReply> {
    if sorted_total.is_some_and(|expected| expected != total) {
        return Err(BadReply);
    }
    let mut seen = BTreeSet::new();
    let mut previous = None;
    for (index, row) in rows.iter().enumerate() {
        let cell = row.supplement_number.as_ref().ok_or(BadReply)?.as_str();
        if !seen.insert(cell) {
            return Err(BadReply);
        }
        if sorted_total.is_some()
            && (cell.is_empty() != (skip == 0 && index == 0)
                || previous.is_some_and(|last| last > cell))
        {
            return Err(BadReply);
        }
        previous = Some(cell);
    }
    Ok(())
}

fn mapped(
    rows: Vec<raw::Record>,
    supplement_total: Option<u64>,
    identity_conflict: Option<&'static str>,
) -> Records {
    let mut problems = Vec::new();
    let rows = rows
        .into_iter()
        .map(|row| {
            let row = row.mapped();
            problems.extend(row.problems);
            row.value
        })
        .collect();
    Records {
        rows,
        problems,
        supplement_total,
        identity_conflict,
    }
}

fn identity(rows: &[raw::Record], query: &DiagnosticGet) -> Option<&'static str> {
    if rows
        .iter()
        .any(|row| row.pma_number.as_str() != query.id().as_str())
    {
        return Some("pma_number");
    }
    if query.supplement().is_some_and(|wanted| {
        rows.iter().any(|row| {
            row.supplement_number
                .as_ref()
                .is_some_and(|cell| cell.as_str() != wanted.as_str())
        })
    }) {
        return Some("supplement_number");
    }
    None
}
