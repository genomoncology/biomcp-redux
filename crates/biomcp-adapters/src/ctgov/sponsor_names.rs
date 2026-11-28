//! Check a low-count lead sponsor search for a registry spelling with dotted initials.
use super::{SearchPage, filters, search_page, search_query};
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{SourceNotice, SponsorRole, TrialSearch};
use std::time::Duration;

const SMALL_TOTAL: u64 = 10;
const COUNT_RATIO: u64 = 10;
const SUGGESTION_BUDGET: Duration = Duration::from_secs(4);

/// Give first-page spelling guidance using two one-row registry reads within the search budget.
pub fn sponsor_notice(
    leg: &Leg<'_>,
    query: &TrialSearch,
    gene: Option<&str>,
    conditions: &[biodata::Text],
    page: &SearchPage,
) -> Option<SourceNotice> {
    let total = page
        .total
        .filter(|total| (1..=SMALL_TOTAL).contains(total))?;
    if query.id().is_some()
        || query.offset() != 0
        || query.next_page().is_some()
        || query.sponsor_role() == Some(&SponsorRole::Any)
    {
        return None;
    }
    let name = query.sponsor()?.as_str();
    let dotted = dotted_initials(name)?;
    let (checked, stale) = leg.dependency(&super::SPEC, SUGGESTION_BUDGET, |probe| {
        let checked = check_spelling(probe, query, gene, conditions, name, &dotted, total);
        (checked, probe.take_report().stale)
    });
    match (checked, stale) {
        (Ok(notice), false) => notice,
        _ => Some(search_query::guidance(format!(
            "ClinicalTrials.gov could not check a spelling suggestion for lead sponsor {name:?}."
        ))),
    }
}

fn check_spelling(
    probe: &Leg<'_>,
    query: &TrialSearch,
    gene: Option<&str>,
    conditions: &[biodata::Text],
    name: &str,
    dotted: &str,
    total: u64,
) -> Result<Option<SourceNotice>, RequestError> {
    let bad_reply = biomcp_fetch::bad_reply(super::SPEC.source, Some(200));
    let mut params = search_query::resolved_parameters(query, gene, conditions, None);
    params.insert("pageSize".into(), "1".into());
    params.insert("fields".into(), "NCTId,LeadSponsorName".into());
    let mut read = |sponsor| {
        params.insert(
            "filter.advanced".into(),
            filters::advanced(query, false, None, Some(sponsor)),
        );
        let page = search_page(probe, &params, &[], &[], None, &[], None)?;
        let count = page.total.ok_or(bad_reply)?;
        Ok::<_, RequestError>((page.items, count))
    };
    let (candidates, count) = read(dotted)?;
    let minimum = total * COUNT_RATIO;
    if count < minimum {
        return Ok(None);
    }
    let registered = candidates
        .first()
        .and_then(|row| row.sponsor.as_ref())
        .ok_or(bad_reply)?
        .as_str();
    if registered.eq_ignore_ascii_case(name) {
        return Ok(None);
    }
    if !search_query::name_fits(registered) {
        return Err(bad_reply);
    }
    let (_, count) = read(registered)?;
    Ok((count >= minimum).then(|| {
        search_query::guidance(format!(
            "ClinicalTrials.gov lead sponsor search for {name:?} yielded a count of {total}. \
             Searching the registry spelling {registered:?} with the same filters yielded a count of {count}."
        ))
    }))
}

fn dotted_initials(name: &str) -> Option<String> {
    let (initials, rest) = name.split_once(char::is_whitespace)?;
    if !(2..=4).contains(&initials.len()) || !initials.bytes().all(|c| c.is_ascii_uppercase()) {
        return None;
    }
    let dotted = initials.chars().flat_map(|c| [c, '.']).collect::<String>();
    Some(format!("{dotted} {}", rest.trim_start()))
}
