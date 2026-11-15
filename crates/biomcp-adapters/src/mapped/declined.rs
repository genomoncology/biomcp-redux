//! Records left off a page because their identity is unusable.
use crate::raw::Field;
use biomcp_types::{Problem, ProblemCode, Source};

/// A record left off its page because its identity `field` is unusable. It has no canonical
/// identifier, so `record` names it by the raw value openFDA sent, such as
/// `recall_number:D-751-013`; a blank or `N/A` value names it by a native key instead, such as
/// `event_id:100178`. Each declined record keeps its own problem, so a page states how many it
/// declined.
pub(crate) fn declined(
    source: Source,
    field: &str,
    number: Option<&Field<String>>,
    key: &str,
    value: Option<&Field<String>>,
) -> Problem {
    let named = |name: &str, value: Option<&Field<String>>| {
        value
            .and_then(Field::as_ref)
            .map(|v| v.trim())
            .filter(|v| !v.is_empty() && !v.eq_ignore_ascii_case("N/A"))
            .and_then(|v| super::record(format_args!("{name}:{v}")))
    };
    let record = named(field, number).or_else(|| named(key, value));
    Problem {
        source,
        record,
        field: field.into(),
        code: ProblemCode::RecordDeclined,
    }
}

/// Add a record's problems to its page: a rejected field once, each declined record once.
pub(crate) fn page_problems(page: &mut Vec<Problem>, record: Vec<Problem>) {
    for problem in record {
        if problem.code == ProblemCode::RecordDeclined || !page.contains(&problem) {
            page.push(problem);
        }
    }
}
