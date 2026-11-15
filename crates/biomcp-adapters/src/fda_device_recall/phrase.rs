//! The device recall name phrase searched once refused characters became spaces.
use biomcp_types::{DiagnosticSearch, openfda_words};
/// The phrase searched for a device recall name whose refused characters became spaces:
/// "Alaris® pump" counts the same 49 recall records as "Alaris pump" (live, 2026-11-15).
pub fn spaced_name(q: &DiagnosticSearch) -> Option<String> {
    q.name()
        .map(|name| openfda_words(name.as_str()))
        .filter(|words| matches!(words, std::borrow::Cow::Owned(_)))
        .map(std::borrow::Cow::into_owned)
}
