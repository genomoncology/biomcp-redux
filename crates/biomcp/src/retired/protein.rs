//! Protein migration guidance never repairs or rewrites a query.
use biomcp_types::ErrorMessage;
pub(super) const SEARCH: &[(&str, ErrorMessage)] = &[
    ("-q", ErrorMessage::ProteinQuery),
    ("--query", ErrorMessage::ProteinQuery),
    ("--all-species", ErrorMessage::ProteinSpecies),
    ("--disease", ErrorMessage::ProteinSelector),
    ("--existence", ErrorMessage::ProteinSelector),
    ("--next-page", ErrorMessage::ProteinPage),
    ("--include-unreviewed", ErrorMessage::ProteinUnreviewed),
];
pub(super) fn message(words: &super::Words<'_>) -> Option<ErrorMessage> {
    match words.positional.as_slice() {
        ["search", "protein", ..] => words.retired_option,
        _ => None,
    }
}
