//! Component restrictions are decided before reading replies or caches.
use biodata::{Problem, Reason, Section, Source, SourceOutcome, SourceState};
use biomcp_fetch::RequestError;
pub(super) type Result<T> = (
    Section<T>,
    Vec<SourceOutcome>,
    Vec<Problem>,
    Option<RequestError>,
);
pub(super) fn restricted<T>() -> Result<T> {
    (
        Section::Unavailable {
            source: Source::MyVariant,
            reason: Reason::RestrictedTerms,
        },
        vec![SourceOutcome {
            source: Source::MyVariant,
            state: SourceState::Failed,
            reason: Some(Reason::RestrictedTerms),
            term: None,
            count: None,
            fetched: None,
            notices: Vec::new(),
        }],
        Vec::new(),
        None,
    )
}
