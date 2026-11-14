//! Source-owned FDA device search and document access.
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{DiagnosticCandidate, DiagnosticSearch};
/// Native field the name filter matches.
const NAME: &str = "trade_name";
/// Exact openFDA search expression sent for this request; it names the field each filter matched.
pub fn expression(q: &DiagnosticSearch) -> String {
    crate::device_search::expression(q, NAME)
}
/// Search native FDA submission records, retaining supplements and source totals.
pub fn search(leg: &Leg<'_>, q: &DiagnosticSearch) -> Result<super::SearchPage, RequestError> {
    let (rows, total) =
        crate::device_search::page::<super::raw::Record>(leg, &super::SPEC, q, NAME)?;
    let mut problems = Vec::new();
    let mut items = Vec::new();
    for row in rows {
        let row = row.mapped();
        problems.extend(row.problems);
        items.push(DiagnosticCandidate::PmaSubmission {
            header: super::header(&row.value)?,
        });
    }
    Ok(crate::device_search::Page {
        items: items.into(),
        total,
        problems,
    })
}
