use biomcp_fetch::{LegReport, RequestError};
use biomcp_types::{Source, SourceOutcome, SourceState, Status};
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
    let reason = error
        .and_then(reason)
        .or_else(|| report.failures.first().map(|failure| failure.reason));
    let mut outcome = biomcp_adapters::registry::outcome(
        source,
        if matches!(
            reason,
            Some(biomcp_types::Reason::Disabled | biomcp_types::Reason::NoKey)
        ) {
            SourceState::Skipped
        } else if report.stale {
            SourceState::Stale
        } else if reason == Some(biomcp_types::Reason::Syncing) {
            SourceState::Syncing
        } else if error.is_some() {
            SourceState::Failed
        } else {
            SourceState::Ok
        },
        reason,
    );
    outcome
        .notices
        .extend(report.cache_problems.into_iter().map(|error| {
            biomcp_types::SourceNotice::CacheProblem {
                reason: match error {
                    biomcp_fetch::CacheError::Busy => biomcp_types::CacheProblem::Busy,
                    biomcp_fetch::CacheError::Database => biomcp_types::CacheProblem::Database,
                    biomcp_fetch::CacheError::Version => biomcp_types::CacheProblem::Version,
                },
            }
        }));
    outcome
}
pub(crate) fn local(error: RequestError) -> Option<RequestError> {
    (!matches!(error, RequestError::Source(_))).then_some(error)
}
pub(crate) fn append(
    target: &mut Vec<biomcp_types::Problem>,
    values: impl IntoIterator<Item = biomcp_types::Problem>,
) {
    for value in values {
        if !target.contains(&value) {
            target.push(value);
        }
    }
}

/// Preserve a failed or stale source leg when later metadata from it succeeds.
pub(crate) fn retain_source_state(target: &mut SourceOutcome, incoming: &SourceOutcome) {
    for notice in &incoming.notices {
        if !target.notices.contains(notice) {
            target.notices.push(notice.clone());
        }
    }
    if target.state != SourceState::Failed
        && (target.state != SourceState::Stale || incoming.state == SourceState::Failed)
    {
        target.state = incoming.state;
        target.reason = incoming.reason;
    }
}

/// Preserve provider reasons and distinguish absent replay exchanges from local failures.
pub(crate) fn reason(error: RequestError) -> Option<biomcp_types::Reason> {
    match error {
        RequestError::Source(error) => Some(error.reason),
        RequestError::Recording(
            biomcp_fetch::RecordError::Missing | biomcp_fetch::RecordError::Exhausted,
        ) => Some(biomcp_types::Reason::ReplayMiss),
        _ => None,
    }
}
