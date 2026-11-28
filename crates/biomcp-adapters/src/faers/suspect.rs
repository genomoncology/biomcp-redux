//! Confirm the name and role on a single drug entry before counting a report.
use super::raw;
use biomcp_fetch::{Leg, Received, RequestError, SourceError};
use biomcp_types::{AdverseEventSearch, FAERS_SUSPECT_CODE, Reason, Source, openfda_words};

pub(super) fn search(
    leg: &Leg<'_>,
    query: &AdverseEventSearch,
    selector: &str,
) -> Result<Received<raw::Response>, RequestError> {
    let limit = biomcp_types::SEARCH_PAGE_MAX;
    let received = super::search::fetch::<raw::Candidate>(leg, selector, 0, limit)?;
    let value = match received.value {
        raw::Response::Page { mut meta, results } => {
            if meta.results.total > u64::from(limit) {
                return Err(SourceError {
                    source: Source::Faers,
                    reason: Reason::TooLarge,
                    status: Some(200),
                    tls_cause: None,
                    retry_after: None,
                }
                .into());
            }
            let matches = matching(query, results);
            meta.results.total = matches.len() as u64;
            let results = matches
                .into_iter()
                .skip(query.offset() as usize)
                .take(query.limit() as usize)
                .collect();
            raw::Response::Page { meta, results }
        }
        raw::Response::Absent { error } => raw::Response::Absent { error },
    };
    Ok(Received {
        value,
        meta: received.meta,
    })
}

fn matching(query: &AdverseEventSearch, reports: Vec<raw::Candidate>) -> Vec<raw::Header> {
    let names: Vec<_> = query
        .text()
        .into_iter()
        .chain(query.drug())
        .map(|name| openfda_words(name.as_str()))
        .collect();
    reports
        .into_iter()
        .filter_map(|report| {
            report
                .patient
                .drug
                .iter()
                .any(|drug| {
                    drug.drugcharacterization.as_deref() == Some(FAERS_SUSPECT_CODE)
                        && names.iter().any(|name| has_name(drug, name))
                })
                .then_some(report.header)
        })
        .collect()
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
                .flatten()
                .map(AsRef::as_ref)
        }))
        .any(|value| value.trim().eq_ignore_ascii_case(name))
}
