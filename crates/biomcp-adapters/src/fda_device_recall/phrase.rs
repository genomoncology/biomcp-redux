//! Device phrases: the device recall name, PMA intended use and 510(k) classification name,
//! and the device recall selector.
use biodata::Text;
use biomcp_types::{DiagnosticSearch, openfda_phrase, openfda_words};
/// One quoted openFDA phrase with repeated spaces squeezed, as these three phrases have always
/// been sent (0901), so their requests stay the same.
pub(crate) fn phrase(value: &Text) -> String {
    openfda_phrase(
        &value
            .as_str()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" "),
    )
}
/// The phrase searched for a device recall name whose refused characters became spaces:
/// "Alaris® pump" counts the same 49 recall records as "Alaris pump" (live, 2026-11-15).
pub fn spaced_name(q: &DiagnosticSearch) -> Option<String> {
    q.name()
        .map(|name| openfda_words(name.as_str()))
        .filter(|words| matches!(words, std::borrow::Cow::Owned(_)))
        .map(std::borrow::Cow::into_owned)
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
        terms.push(format!("recalling_firm:{}", openfda_phrase(v.as_str())));
    }
    if let Some(v) = q.product_code() {
        terms.push(format!("product_code:{}", openfda_phrase(v.as_str())));
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
