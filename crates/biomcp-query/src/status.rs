use biodata::{Source, SourceOutcome, SourceState, Status};
use biomcp_fetch::{LegReport, RequestError};
/// Combine input outcomes without hiding a failed input behind a found one.
pub(crate) fn status(items: impl Iterator<Item = Status>) -> Status {
    let values: Vec<_> = items.collect();
    if values.iter().all(|v| *v == Status::Absent) {
        Status::Absent
    } else if values.iter().all(|v| *v == Status::Failed) {
        Status::Failed
    } else if values
        .iter()
        .any(|v| matches!(v, Status::Failed | Status::Partial))
    {
        Status::Partial
    } else {
        Status::Complete
    }
}
pub(crate) fn source(
    source: Source,
    report: LegReport,
    error: Option<RequestError>,
) -> SourceOutcome {
    let reason = match error {
        Some(RequestError::Source(error)) => Some(error.reason),
        _ => report.failures.first().map(|failure| failure.reason),
    };
    SourceOutcome {
        term: None,
        count: None,
        fetched: None,
        notices: Vec::new(),
        source,
        state: if report.stale {
            SourceState::Stale
        } else if reason == Some(biodata::Reason::Syncing) {
            SourceState::Syncing
        } else if error.is_some() {
            SourceState::Failed
        } else {
            SourceState::Ok
        },
        reason,
    }
}
pub(crate) fn local(error: RequestError) -> Option<RequestError> {
    (!matches!(error, RequestError::Source(_))).then_some(error)
}
pub(crate) fn append(
    target: &mut Vec<biodata::Problem>,
    values: impl IntoIterator<Item = biodata::Problem>,
) {
    for value in values {
        if !target.contains(&value) {
            target.push(value);
        }
    }
}
