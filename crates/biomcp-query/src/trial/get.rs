//! Exact trial cards and immutable source section continuations.
use biomcp_types::ReplyBody;
mod arms;
pub(in crate::trial) mod page;
use crate::{Ctx, Outcome, TrialArmProjection, source_snapshot, status};
use biodata::NctId;
use biomcp_adapters::ctgov;
use biomcp_fetch::{CacheError, RequestError};
use biomcp_types::{
    ReplyData, Request, Source, SourceState, Status, StepKind, TrialCardItem, TrialCards, TrialGet,
};
// Older protocol-only projections cannot supply posted results; restart before decoding them.
const SNAPSHOT_NAMESPACE: &str = "trial-results/1";

/// Retrieve independent input states under one shared command deadline.
pub(crate) fn get(
    cx: &Ctx<'_>,
    query: &TrialGet,
    arm_projection: Option<&TrialArmProjection<'_>>,
) -> Outcome {
    if query.source() == biomcp_types::TrialSource::Nci {
        return super::nci::get(cx, query);
    }

    let mut out = empty(query);
    // Each trial is its own request, spaced by BioMCP's pacing; give a batch that time (1003).
    let budget = cx
        .fetch
        .paced_budget(&ctgov::SPEC, cx.budget, query.input().len());
    let command = cx.fetch.command_deadline(budget);
    let deadline = command.deadline();
    let result = if query.pool().is_some() {
        restore(cx, query, deadline, &mut out, arm_projection)
    } else {
        fetch(cx, query, deadline, &mut out, arm_projection)
    };
    if let Err(error) = result {
        crate::status::item(&mut out, false, true, false);
        out.local = status::local(error);
    }
    command.finish(deadline.remaining());
    out
}
fn fetch(
    cx: &Ctx<'_>,
    query: &TrialGet,
    deadline: biomcp_fetch::Deadline<'_>,
    out: &mut Outcome,
    arm_projection: Option<&TrialArmProjection<'_>>,
) -> Result<(), RequestError> {
    let leg = cx
        .fetch
        .paced_leg(&ctgov::SPEC, deadline, query.input().len())
        .fresh(cx.fresh);
    let rows: Vec<_> = query
        .input()
        .iter()
        .map(|id| ctgov::detail::get(&leg, *id))
        .collect();
    let version = rows.iter().any(Result::is_ok).then(|| ctgov::version(&leg));
    let error = rows
        .iter()
        .find_map(|r| r.as_ref().err().copied().filter(|e| !not_found(*e)))
        .or_else(|| version.as_ref().and_then(|v| v.as_ref().err().copied()));
    let source = status::source(Source::ClinicalTrialsGov, leg.finish(), error);
    out.local = error.and_then(status::local);
    out.sources.push(source.clone());
    let version = version.and_then(Result::ok);
    let processing_problems = version
        .as_ref()
        .map_or_else(Vec::new, |version| version.problems.clone());
    let source_updated = version.and_then(|version| version.value);
    let mut items = Vec::new();
    let mut unread = Vec::new();
    for (id, row) in query.input().iter().zip(rows) {
        match row {
            Ok((detail, metadata)) => {
                let canonical = detail.card.nct_id;
                if *id != canonical {
                    out.sources[0]
                        .notices
                        .push(ctgov::alias_notice(*id, canonical));
                }
                let mut own_source = source.clone();
                if own_source.state == SourceState::Failed && source_updated.is_some() {
                    own_source.set_status(biomcp_types::SourceStatus::Ok);
                }
                let snapshot = Snapshot {
                    detail,
                    fetched: metadata.fetched.and_then(|value| u64::try_from(value).ok()),
                    version: source_updated.clone(),
                    processing_problems: processing_problems.clone(),
                    sources: vec![own_source],
                };
                // A character offset is a text position, not a row, so past the text it is refused.
                if let Some(length) = page::past_text(query, &snapshot) {
                    out.body = ReplyData::Usage(biomcp_types::UsageError::offset_past_text(length));
                    return Ok(());
                }
                let pool = if query.sections().is_empty() {
                    None
                } else {
                    crate::publish::keep(
                        source_snapshot::write(
                            cx,
                            &source_snapshot::key(SNAPSHOT_NAMESPACE, &canonical)?,
                            &snapshot,
                            &snapshot.sources,
                            deadline,
                        ),
                        out,
                    )?
                };
                items.push(page::render(
                    query,
                    *id,
                    snapshot,
                    pool,
                    out,
                    arm_projection,
                )?);
            }
            Err(e) if not_found(e) => items.push(unresolved(
                *id,
                crate::status::item_status(false, false, false),
            )),
            Err(e) => {
                let reason = status::reason(e);
                unread.push((*id, reason));
                let mut item = unresolved(*id, crate::status::item_status(false, true, false));
                item.failure = Some(failure(reason));
                items.push(item);
            }
        }
    }
    say_unread(query, &unread, out);
    finish(query, items, source_updated, out);
    Ok(())
}
/// Say on the item itself why it was not read, and whether the cause is BioMCP's own limit.
fn failure(reason: Option<biomcp_types::Reason>) -> biomcp_types::TrialItemFailure {
    use biomcp_types::Reason;
    const AGAIN: &str = "Read it again on its own or in smaller batches.";
    let (cause, again) = match reason {
        Some(Reason::DeadlineExceeded) => (
            "BioMCP's own time limit for this command ran out before this trial was read.",
            true,
        ),
        Some(Reason::LocalQuota) => (
            "BioMCP's own request pacing for ClinicalTrials.gov left no time to read this trial.",
            true,
        ),
        Some(Reason::TooLarge) => (
            "ClinicalTrials.gov's reply for this trial was larger than BioMCP's reply cap.",
            false,
        ),
        Some(Reason::Disabled | Reason::Offline | Reason::NoKey | Reason::Tls | Reason::Proxy) => (
            "A local BioMCP setting stopped the request for this trial; reading it again will not help until the setting changes.",
            false,
        ),
        Some(Reason::BadReply) => (
            "ClinicalTrials.gov answered for this trial, but BioMCP could not use the reply.",
            false,
        ),
        Some(Reason::RateLimited) => (
            "ClinicalTrials.gov limited the request rate before this trial was read; wait a moment first.",
            true,
        ),
        Some(Reason::ReplayMiss) => ("No recorded reply matched this trial's request.", false),
        Some(_) => ("ClinicalTrials.gov did not return this trial.", true),
        None => (
            "A local error in BioMCP stopped this trial from being read.",
            true,
        ),
    };
    let problem = if again {
        format!("{cause} {AGAIN}")
    } else {
        cause.to_owned()
    };
    biomcp_types::TrialItemFailure {
        source: Source::ClinicalTrialsGov,
        reason,
        problem: problem.into(),
    }
}
/// A batch item that failed carries only its input and status; name each one, its reason
/// and a retry, so a missing card or section never reads as an empty one.
fn say_unread(
    query: &TrialGet,
    unread: &[(NctId, Option<biomcp_types::Reason>)],
    out: &mut Outcome,
) {
    // A single get's source reason already names the cause.
    if unread.is_empty() || query.input().len() < 2 {
        return;
    }
    let named: Vec<String> = unread
        .iter()
        .map(|(id, reason)| match reason {
            Some(reason) => format!("{id} ({reason})"),
            None => format!("{id} (local error)"),
        })
        .collect();
    let message = format!(
        "Not read in this batch: {}. These trials carry no card or sections in this reply; read them again with the fresh step, or in smaller batches.",
        named.join(", ")
    );
    if let Some(source) = out
        .sources
        .iter_mut()
        .find(|source| source.source == Source::ClinicalTrialsGov)
    {
        source
            .notices
            .push(biomcp_types::SourceNotice::RequestGuidance {
                data_date: None,
                message: message.into(),
                retry_after_seconds: None,
            });
    }
    let ids: Vec<NctId> = unread.iter().map(|(id, _)| *id).collect();
    out.next.insert(
        0,
        crate::next::Step::run(
            StepKind::Fresh,
            &[Source::ClinicalTrialsGov],
            Request::GetTrial(query.identity().with_input(ids)),
            true,
        ),
    );
}
fn restore(
    cx: &Ctx<'_>,
    query: &TrialGet,
    deadline: biomcp_fetch::Deadline<'_>,
    out: &mut Outcome,
    arm_projection: Option<&TrialArmProjection<'_>>,
) -> Result<(), RequestError> {
    let id = query.input().first().copied().ok_or(CacheError::Internal)?;
    if cx.fetch.disabled(Source::ClinicalTrialsGov) {
        let leg = cx.fetch.leg(&ctgov::SPEC, deadline);
        let result = ctgov::detail::get(&leg, id);
        let error = result.err();
        out.sources.push(status::source(
            Source::ClinicalTrialsGov,
            leg.finish(),
            error,
        ));
        finish(
            query,
            vec![unresolved(
                id,
                crate::status::item_status(false, true, false),
            )],
            None,
            out,
        );
        return Ok(());
    }
    let snapshot: Option<Snapshot> = source_snapshot::read(
        cx,
        &source_snapshot::key(SNAPSHOT_NAMESPACE, &id)?,
        query.pool().map_or("", |p| p.as_str()),
        deadline,
    )?;
    let Some(snapshot) = snapshot else {
        out.body = ReplyData::PoolError { mismatch: false };
        out.next.push(crate::next::Step::run(
            StepKind::Restart,
            &[query.source().source()],
            Request::GetTrial(query.identity()),
            false,
        ));
        return Ok(());
    };
    if snapshot.detail.card.nct_id != id {
        return Err(CacheError::Internal.into());
    }
    if let Some(length) = page::past_text(query, &snapshot) {
        out.body = ReplyData::Usage(biomcp_types::UsageError::offset_past_text(length));
        return Ok(());
    }
    let version = snapshot.version.clone();
    out.sources = snapshot.sources.clone();
    if let Some(version) = &version
        && let Some(source) = out
            .sources
            .iter_mut()
            .find(|source| source.source == Source::ClinicalTrialsGov)
    {
        cx.fetch.retain_data_date(source, version.value.as_str());
    }
    let item = page::render(
        query,
        id,
        snapshot,
        query.pool().cloned(),
        out,
        arm_projection,
    )?;
    finish(query, vec![item], version, out);
    Ok(())
}
fn not_found(error: RequestError) -> bool {
    matches!(error,RequestError::Source(e) if e.reason==biomcp_types::Reason::NotFound)
}
fn unresolved(input: NctId, status: Status) -> TrialCardItem {
    TrialCardItem {
        input,
        status,
        card: None,
        fetched: None,
        sections: None,
        more: Box::default(),
        failure: None,
    }
}
fn finish(
    query: &TrialGet,
    items: Vec<TrialCardItem>,
    source_updated: Option<biomcp_types::SourceTimestamp>,
    out: &mut Outcome,
) {
    status::get(out, items.iter().map(|i| i.status));
    out.body = (TrialCards {
        items: items.into(),
        source_updated: source_updated.map(Into::into),
        attribution: "ClinicalTrials.gov; registry protocol text, no eligibility assessment".into(),
    })
    .with_query(query.clone());
}
fn empty(query: &TrialGet) -> Outcome {
    crate::status::empty(
        (TrialCards {
            items: Box::default(),
            source_updated: None,
            attribution: "ClinicalTrials.gov".into(),
        })
        .with_query(query.clone()),
    )
}
fn partial(snapshot: &Snapshot) -> bool {
    snapshot.detail.refusals == biomcp_adapters::Refusal::Refused
        || snapshot.sources.iter().any(|s| {
            matches!(
                s.state,
                SourceState::Failed | SourceState::Stale | SourceState::Syncing
            )
        })
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(super) struct Snapshot {
    pub(super) detail: biomcp_adapters::ctgov::detail::Detail,
    pub(super) fetched: Option<u64>,
    pub(super) version: Option<biomcp_types::SourceTimestamp>,
    pub(super) processing_problems: Vec<biomcp_types::Problem>,
    pub(super) sources: Vec<biomcp_types::SourceOutcome>,
}
