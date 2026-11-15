//! Device names as one quoted openFDA phrase, with the characters openFDA refuses spaced.
use biodata::Text;
use biomcp_types::{DiagnosticSearch, openfda_phrase, openfda_words};
/// Text as one quoted openFDA phrase, with refused characters spaced: the device recall name
/// (product_description) and the PMA intended-use phrase (ao_statement). "Alaris® pump" counts
/// the same 49 recall records as "Alaris pump" (live, 2026-11-15).
pub(crate) fn phrase(value: &Text) -> String {
    openfda_phrase(value.as_str())
}
/// The phrase searched for a device recall name whose refused characters became spaces.
pub fn spaced_name(q: &DiagnosticSearch) -> Option<String> {
    q.name()
        .map(|name| openfda_words(name.as_str()))
        .filter(|words| matches!(words, std::borrow::Cow::Owned(_)))
        .map(std::borrow::Cow::into_owned)
}
