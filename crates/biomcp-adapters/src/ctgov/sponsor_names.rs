//! Check a low-count lead sponsor search for a registry spelling with dotted initials.
use super::{SearchPage, filters, search_page, search_query};
use biomcp_fetch::Leg;
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
    condition: Option<&biodata::Text>,
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
    leg.dependency(&super::SPEC, SUGGESTION_BUDGET, |probe| {
        let mut params = super::parameters(query, gene);
        if let Some(condition) = condition {
            params.insert("query.cond".into(), format!("({condition})"));
        }
        params.insert("pageSize".into(), "1".into());
        params.insert("fields".into(), "NCTId,LeadSponsorName".into());
        let mut read = |sponsor| {
            params.insert(
                "filter.advanced".into(),
                filters::advanced(query, false, None, Some(sponsor)),
            );
            search_page(probe, &params, &[], &[], None, &[], None).ok()
        };
        let candidate = read(&dotted)?;
        if candidate.total? < total * COUNT_RATIO {
            return None;
        }
        let registered = candidate.items.first()?.sponsor.as_ref()?.as_str();
        if registered.eq_ignore_ascii_case(name) || !search_query::name_fits(registered) {
            return None;
        }
        let count = read(registered)?.total?;
        (count >= total * COUNT_RATIO && !probe.take_report().stale).then(|| {
            search_query::guidance(format!(
                "ClinicalTrials.gov lead sponsor search for {name:?} yielded a count of {total}. \
                 Searching the registry spelling {registered:?} with the same filters yielded a count of {count}."
            ))
        })
    })
}

fn dotted_initials(name: &str) -> Option<String> {
    let (initials, rest) = name.split_once(char::is_whitespace)?;
    if !(2..=4).contains(&initials.len()) || !initials.bytes().all(|c| c.is_ascii_uppercase()) {
        return None;
    }
    let dotted = initials.chars().flat_map(|c| [c, '.']).collect::<String>();
    Some(format!("{dotted} {}", rest.trim_start()))
}
