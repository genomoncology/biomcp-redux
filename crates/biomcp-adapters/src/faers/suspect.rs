//! Confirm the name and role on a single drug entry before counting a report.
use super::raw;
use biomcp_fetch::{RequestError, SourceError};
use biomcp_types::{AdverseEventSearch, FAERS_SUSPECT_CODE, Reason, Source, openfda_words};

pub(super) fn filter(
    query: &AdverseEventSearch,
    reports: Vec<raw::Header>,
    candidates: u64,
) -> Result<(Vec<raw::Header>, u64), RequestError> {
    if candidates > u64::from(biomcp_types::SEARCH_PAGE_MAX) {
        return Err(SourceError {
            source: Source::Faers,
            reason: Reason::TooLarge,
            status: Some(200),
            tls_cause: None,
            retry_after: None,
        }
        .into());
    }
    let names: Vec<_> = query
        .text()
        .into_iter()
        .chain(query.drug())
        .map(|name| openfda_words(name.as_str()))
        .collect();
    let mut matches = Vec::new();
    for report in reports {
        let patient = report
            .patient
            .as_ref()
            .and_then(crate::raw::Field::as_ref)
            .ok_or_else(|| biomcp_fetch::bad_reply(Source::Faers, Some(200)))?;
        if patient.drug.iter().any(|drug| {
            drug.drugcharacterization.as_deref() == Some(FAERS_SUSPECT_CODE)
                && names.iter().any(|name| has_name(drug, name))
        }) {
            matches.push(report);
        }
    }
    let total = matches.len() as u64;
    let page = matches
        .into_iter()
        .skip(query.offset() as usize)
        .take(query.limit() as usize)
        .collect();
    Ok((page, total))
}

fn has_name(drug: &raw::Drug, name: &str) -> bool {
    drug.medicinalproduct
        .as_deref()
        .into_iter()
        .chain(drug.openfda.iter().flat_map(|names| {
            names
                .generic_name
                .iter()
                .chain(&names.brand_name)
                .map(AsRef::as_ref)
        }))
        .any(|value| value.trim().eq_ignore_ascii_case(name))
}
