//! Registry-owned name broadening follows a checked empty exact first page.
use biomcp_adapters::ctgov;
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{SourceNotice, TrialSearch};
use std::collections::BTreeMap;

pub(super) fn search(
    leg: &Leg<'_>,
    query: &TrialSearch,
    params: &BTreeMap<String, String>,
    conditions: &[biodata::Text],
) -> (
    Result<ctgov::SearchPage, RequestError>,
    Option<SourceNotice>,
) {
    let mut result = ctgov::search(leg, params, conditions, query.country());
    let mut broader = query.broader_names();
    let notice = if query.next_page().is_none()
        && (query.sponsor().is_some() || query.facility().is_some())
        && result.as_ref().is_ok_and(|page| page.total == Some(0))
    {
        // Replace only the exact name selectors. Keep resolved condition/gene
        // parameters and every other strict filter from the actual first request.
        let mut fallback = ctgov::broader_parameters(query, None);
        for (name, value) in params {
            if name != "filter.advanced" {
                fallback.insert(name.clone(), value.clone());
            }
        }
        let message = format!(
            "ClinicalTrials.gov exact phrase request {params:?} returned zero trials. \
             BioMCP tried the registry's broader name request {fallback:?} once."
        );
        broader = true;
        result = ctgov::search(leg, &fallback, conditions, query.country());
        Some(guidance(message))
    } else if broader {
        Some(guidance(format!(
            "ClinicalTrials.gov broader name search continuation request {params:?}."
        )))
    } else {
        None
    };
    if broader {
        result = result.and_then(wrap_page);
    }
    (result, notice)
}

fn wrap_page(mut page: ctgov::SearchPage) -> Result<ctgov::SearchPage, RequestError> {
    page.next = page
        .next
        .as_ref()
        .map(TrialSearch::broader_page_token)
        .transpose()
        .map_err(|_| {
            RequestError::from(biomcp_fetch::SourceError {
                source: ctgov::SPEC.source,
                reason: biomcp_types::Reason::BadReply,
                status: Some(200),
                retry_after: None,
            })
        })?;
    Ok(page)
}

fn guidance(message: String) -> SourceNotice {
    SourceNotice::RequestGuidance {
        message: message.into(),
        retry_after_seconds: None,
    }
}
