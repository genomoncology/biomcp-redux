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
    let counted = ctgov::site_count(&leg, query, gene, conditions);
    let version = match &counted {
        Ok(_) => ctgov::version(&leg),
        Err(error) => Err(*error),
    };
    let error = counted.as_ref().err().or(version.as_ref().err()).copied();
    *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
    let mut source = status::source(Source::ClinicalTrialsGov, leg.finish(), error);
    let missing = error.is_some() || source.state == biomcp_types::SourceState::Stale;
    let (found, identity_failed, unavailable) = super::item_state(earlier, counted.is_ok(), missing);
    status::item(outcome, found, identity_failed, unavailable);
    if let Ok(counts) = &counted {
        source.count = Some(counts.trials_matched);
        source.retrieved_count = u32::try_from(counts.trials_read).ok();
    }
    if let ReplyData::TrialPage {
        body: Some(body), ..
    } = &mut outcome.body
    {
        if let Ok(counts) = counted {
            body.site_counts = Some(counts);
            body.attribution = ATTRIBUTION.into();
        }
        if let Ok(version) = version {
            outcome.problems.extend(version.problems);
            body.source_updated = version.value.map(Into::into);
        }
    }
    outcome.local = error.and_then(status::local);
    source
}
