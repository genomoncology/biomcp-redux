//! A site count answers in place of trial rows, from the same registry selectors.
use crate::{Outcome, status};
use biomcp_adapters::ctgov;
use biomcp_types::{ReplyData, Source, SourceOutcome, TrialSearch};

const ATTRIBUTION: &str = "ClinicalTrials.gov; facility names counted exactly as registered, \
    never merged; no eligibility assessment";

/// Count the matched trials' facilities and the registry version beside them.
pub(super) fn count<'a>(
    leg: biomcp_fetch::Leg<'_>,
    query: &TrialSearch,
    gene: Option<&str>,
    conditions: &[biodata::Text],
    earlier: bool,
    deadline: &mut biomcp_fetch::Deadline<'a>,
    outcome: &mut Outcome,
) -> SourceOutcome {
    let started = leg.remaining();
    // The first failure becomes the source's error; a version failure keeps the counts.
    let (counts, version, error) = match ctgov::site_count(&leg, query, gene, conditions) {
        Ok(counts) => match ctgov::version(&leg) {
            Ok(version) => (Some(counts), Some(version), None),
            Err(failed) => (Some(counts), None, Some(failed)),
        },
        Err(failed) => (None, None, Some(failed)),
    };
    *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
    let mut source = status::source(Source::ClinicalTrialsGov, leg.finish(), error);
    let missing = error.is_some() || source.state == biomcp_types::SourceState::Stale;
    let (found, identity_failed, unavailable) =
        super::item_state(earlier, counts.is_some(), missing);
    status::item(outcome, found, identity_failed, unavailable);
    outcome.local = error.and_then(status::local);
    let ReplyData::TrialPage {
        body: Some(body), ..
    } = &mut outcome.body
    else {
        return source;
    };
    if let Some(counts) = counts {
        source.count = Some(counts.trials_matched);
        // At most five pages of 1,000 trials are read.
        source.retrieved_count = Some(u32::try_from(counts.trials_read).unwrap_or(u32::MAX));
        body.site_counts = Some(counts);
        body.attribution = ATTRIBUTION.into();
    }
    if let Some(version) = version {
        outcome.problems.extend(version.problems);
        body.source_updated = version.value.map(Into::into);
    }
    source
}
