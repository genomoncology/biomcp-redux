//! Component restrictions are decided before reading replies or caches.
use biodata::{Reason, Section, Source, SourceOutcome, SourceState};
pub(super) fn restricted<T>() -> super::sections::Result<T> {
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
