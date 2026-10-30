//! Exact trial cards and immutable source section continuations.
pub(in crate::trial) mod page;
mod snapshot;
use self::snapshot::Snapshot;
use crate::{Action, Ctx, Outcome, status, store::Store};
use biodata::NctId;
use biomcp_adapters::ctgov;
use biomcp_fetch::{CacheError, RequestError};
use biomcp_types::{
    QueryBody, Request, Source, SourceState, Status, StepKind, TrialCardItem, TrialCards, TrialGet,
};
/// Retrieve independent input states under one shared command deadline.
pub(crate) fn get(cx: &Ctx<'_>, query: &TrialGet) -> Outcome {
    if query.source() == biomcp_types::TrialSource::Nci {
        return super::nci::get(cx, query);
    }

    let mut out = empty();
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let result = if query.pool().is_some() {
        restore(cx, query, deadline, &mut out)
    } else {
        fetch(cx, query, deadline, &mut out)
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
) -> Result<(), RequestError> {
    let leg = cx.fetch.leg(&ctgov::SPEC, deadline).fresh(cx.fresh);
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
    let mut store = if query.sections().is_empty() || !rows.iter().any(Result::is_ok) {
        None
    } else {
        Some(Store::open(cx.fetch.cache_access()?, deadline)?)
    };
    for (id, row) in query.input().iter().zip(rows) {
        match row {
            Ok((detail, metadata)) => {
                let mut own_source = source.clone();
                if own_source.state == SourceState::Failed && source_updated.is_some() {
                    own_source.state = SourceState::Ok;
                    own_source.reason = None;
                }
                let snapshot = Snapshot {
                    detail,
                    fetched: metadata.fetched.and_then(|value| u64::try_from(value).ok()),
                    version: source_updated.clone(),
                    processing_problems: processing_problems.clone(),
                    sources: vec![own_source],
                };
                let pool = if let Some(store) = store.as_mut() {
                    Some(store.save_source_snapshot(
                        &Snapshot::key(*id),
                        &snapshot.encode()?,
                        &snapshot.sources,
                    )?)
                } else {
                    None
                };
                items.push(page::render(query, snapshot, pool, out)?);
            }
            Err(e) => items.push(unresolved(
                *id,
                if not_found(e) {
                    crate::status::item_status(false, false, false)
                } else {
                    crate::status::item_status(false, true, false)
                },
            )),
        }
    }
    finish(items, source_updated, out);
    Ok(())
}
fn restore(
    cx: &Ctx<'_>,
    query: &TrialGet,
    deadline: biomcp_fetch::Deadline<'_>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let id = query.input().first().copied().ok_or(CacheError::Database)?;
    if cx.fetch.disabled(Source::ClinicalTrialsGov) {
        let leg = cx.fetch.leg(&ctgov::SPEC, deadline);
        let result = ctgov::detail::get(&leg, id);
        let error = result.err();
        out.sources.push(status::source(
            Source::ClinicalTrialsGov,
            leg.finish(),
            error,
        ));
        finish(vec![unresolved(id, crate::status::item_status(false, true, false))], None, out);
        return Ok(());
    }
    let store = Store::open(cx.fetch.cache_access()?, deadline)?;
    let payload = store.source_snapshot(
        query.pool().map_or("", |p| p.as_str()),
        &Snapshot::key(id),
        |s| cx.fetch.disabled(s),
    )?;
    let Some(payload) = payload else {
        out.body = QueryBody::PoolError { mismatch: false };
        out.next.push((
            StepKind::Restart,
            Action::Run {
                request: Request::GetTrial(query.identity()),
                fresh: false,
            },
        ));
        return Ok(());
    };
    let snapshot = Snapshot::decode(&payload)?;
    if snapshot.detail.card.nct_id != id {
        return Err(CacheError::Database.into());
    }
    if !page::valid(query, &snapshot) {
        out.body = QueryBody::TextPositionError;
        return Ok(());
    }
    let version = snapshot.version.clone();
    out.sources = snapshot.sources.clone();
    let item = page::render(query, snapshot, query.pool().cloned(), out)?;
    finish(vec![item], version, out);
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
    }
}
fn finish(
    items: Vec<TrialCardItem>,
    source_updated: Option<biomcp_types::SourceTimestamp>,
    out: &mut Outcome,
) {
    status::get(out, items.iter().map(|i| i.status));
    out.body = QueryBody::TrialCards(TrialCards {
        items: items.into(),
        source_updated: source_updated.map(Into::into),
        attribution: "ClinicalTrials.gov; registry protocol text, no eligibility assessment".into(),
    });
}
fn empty() -> Outcome {
    crate::status::finish([crate::status::item_status(false, true, false)], crate::status::Parts {
        dropped: false,
        body: QueryBody::TrialCards(TrialCards {
            items: Box::default(),
            source_updated: None,
            attribution: "ClinicalTrials.gov".into(),
        }),
        page: None,
        sources: Vec::new(),
        problems: Vec::new(),
        next: Vec::new(),
        local: None,
    })
}
fn partial(snapshot: &Snapshot) -> bool {
    !snapshot.detail.problems.is_empty()
        || snapshot.version.is_none()
        || snapshot
            .sources
            .iter()
            .any(|s| s.state == SourceState::Stale)
}
