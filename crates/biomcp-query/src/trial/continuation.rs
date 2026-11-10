//! A rejected registry cursor offers an explicit uncached first-page request.
use crate::{Action, Ctx, Outcome, next::Step, source_snapshot};
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{ReplyData, Request, Source, StepKind, TrialSearch, UsageError};
mod portable;

const PREFIX: &str = "biomcp:ctgov-page:v1:";
const NAMESPACE: &str = "ctgov-continuation/1";

/// The existing capability store authorizes callers and selectors before decoding native cursors.
pub(super) fn restore(
    cx: &Ctx<'_>,
    query: &TrialSearch,
    deadline: Deadline<'_>,
) -> Result<Option<TrialSearch>, RequestError> {
    let Some(token) = query.next_page() else {
        return Ok(Some(query.clone()));
    };
    if let Some(envelope) = token.as_str().strip_prefix(portable::PREFIX) {
        return portable::restore(cx, query, envelope);
    }
    let Some(id) = token.as_str().strip_prefix(PREFIX) else {
        return Ok(None);
    };
    let key = source_snapshot::key(NAMESPACE, &query.first_page())?;
    let saved = source_snapshot::read::<TrialSearch>(cx, &key, id, deadline)?;
    Ok(saved.filter(|saved| saved.offset() == query.offset() && saved.next_page().is_some()))
}

/// Retain the actual sent query and consumed position, never a caller-supplied native expression.
pub(super) fn retain(
    cx: &Ctx<'_>,
    out: &mut Outcome,
    deadline: Deadline<'_>,
) -> Result<(), RequestError> {
    let sources: Vec<_> = out
        .sources
        .iter()
        .filter(|source| source.state != biomcp_types::SourceState::Skipped)
        .cloned()
        .collect();
    for mut step in std::mem::take(&mut out.next) {
        if step.0 == StepKind::Page
            && let Action::Run {
                request: Request::SearchTrial(query),
                ..
            } = &mut step.1
        {
            let saved = retain_token(cx, query, &sources, deadline);
            let Some(id) = crate::publish::keep(saved, out)? else {
                continue;
            };
            *query = query.at_page(id, query.offset());
        }
        out.next.push(step);
    }
    Ok(())
}

fn retain_token(
    cx: &Ctx<'_>,
    query: &TrialSearch,
    sources: &[biomcp_types::SourceOutcome],
    deadline: Deadline<'_>,
) -> Result<biodata::Text, RequestError> {
    if let Some(key) = cx.fetch.trial_cursor_key() {
        return portable::retain(cx, key, query, sources);
    }
    let key = source_snapshot::key(NAMESPACE, &query.first_page())?;
    let id = source_snapshot::write(cx, &key, query, sources, deadline)?;
    Ok(biodata::Text::new(&format!("{PREFIX}{id}"))
        .map_err(|_| biomcp_fetch::CacheError::Encoding)?)
}

pub(super) fn unavailable(query: &TrialSearch, out: &mut Outcome) {
    out.body = ReplyData::Usage(UsageError::TrialContinuationUnavailable);
    restart(query, out);
}

pub(super) fn rejected(query: &TrialSearch, out: &mut Outcome) {
    out.body = ReplyData::Usage(UsageError::TrialContinuationRejected);
    restart(query, out);
}
fn restart(query: &TrialSearch, out: &mut Outcome) {
    out.next.push(Step::run(
        StepKind::Configure,
        &[Source::ClinicalTrialsGov],
        Request::SearchTrial(query.first_page()),
        true,
    ));
}
