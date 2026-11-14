//! Order or window a whole Drugs@FDA match set by original approval, which openFDA cannot do:
//! it sorts and filters an application by any of its submissions, never one chosen submission.
use super::{NEWEST, Order, Page, read};
use biodata::PartialDate;
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::DrugFdaSort;
/// Native rows per read. openFDA allows 1,000, but 1,000 originator applications can exceed the
/// 8 MiB body bound (NDA02* measured 12 MB on 2026-11-13); 500 BLAs measured 5.3 MB.
const READ: u32 = 500;
/// Reads per search. The largest single sponsor prefix measured 1,053 applications (WATSON*).
const READS: u64 = 6;
/// The most applications BioMCP reads to order or window one search.
pub const MOST: u64 = READ as u64 * READS;
/// Native order of the whole read, so skip pages stay stable between reads.
const BY_NUMBER: &str = "application_number:asc";

/// Read every match, keep the window, order it, and return one page of it.
pub(super) fn page(
    leg: &Leg<'_>,
    selector: &str,
    order: Order,
    limit: u32,
    offset: u32,
) -> Result<Page, RequestError> {
    let native = if order.sort == Some(DrugFdaSort::Newest) {
        NEWEST
    } else {
        BY_NUMBER
    };
    let mut all = read(leg, selector, Some(native), READ, 0, None)?;
    let matched = all.total;
    if matched > MOST {
        all.records.clear();
        all.matched = Some(matched);
        return Ok(all);
    }
    while (all.records.len() as u64) < matched {
        let next = read(
            leg,
            selector,
            Some(native),
            READ,
            all.records.len() as u32,
            None,
        )?;
        if next.records.is_empty() {
            break;
        }
        all.refusals.include(next.refusals);
        all.problems.extend(next.problems);
        all.records.extend(next.records);
    }
    let mut kept: Vec<_> = all
        .records
        .into_iter()
        .filter(|row| within(order, row.0.original_approval().map(|a| a.date)))
        .collect();
    if order.sort == Some(DrugFdaSort::OriginalApproval) {
        // Newest first; a row without an original approval sorts as None, after every date.
        kept.sort_by_cached_key(|row| {
            (
                std::cmp::Reverse(row.0.original_approval().map(|a| key(a.date, (0, 0)))),
                row.0.application_number.to_string(),
            )
        });
    }
    all.total = kept.len() as u64;
    all.matched = Some(matched);
    all.records = kept
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect();
    Ok(all)
}

/// A date's sort key, filling a missing month or day with the given values.
fn key(date: PartialDate, (month, day): (u8, u8)) -> (u16, u8, u8) {
    (
        date.year(),
        date.month().unwrap_or(month),
        date.day().unwrap_or(day),
    )
}

/// Whether an original approval date falls in the inclusive window; with a window, rows without
/// one fall outside it.
fn within(order: Order, date: Option<PartialDate>) -> bool {
    if order.approved_from.is_none() && order.approved_to.is_none() {
        return true;
    }
    let Some(date) = date.map(|d| key(d, (0, 0))) else {
        return false;
    };
    order
        .approved_from
        .is_none_or(|from| date >= key(from, (1, 1)))
        && order.approved_to.is_none_or(|to| date <= key(to, (12, 31)))
}
