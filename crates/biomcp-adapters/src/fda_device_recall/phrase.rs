//! Device phrases: the device recall name, PMA intended use and 510(k) classification name.
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
