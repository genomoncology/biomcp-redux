//! Biomarker discovery pages one complete NCI observation locally.
use crate::source_snapshot as snapshot;
use crate::{Action, Ctx, Outcome, status};
use biomcp_adapters::nci;
use biomcp_fetch::{Deadline, RequestError, SourceError};
use biomcp_types::{
    BiomarkerPage, BiomarkerSearch, QueryBody, Reason, Request, Source, Status, StepKind,
};
/// Find native biomarker concept headers without interpretation.
pub fn search(cx: &Ctx<'_>, q: &BiomarkerSearch) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let mut out = empty();
    if let Err(error) = execute(cx, q, deadline, &mut out) {
        if out.sources.is_empty() {
            out.sources.push(biomcp_adapters::registry::outcome(
                Source::Nci,
                if status::reason(error) == Some(Reason::Disabled) {
                    biomcp_types::SourceState::Skipped
                } else {
                    biomcp_types::SourceState::Failed
                },
                status::reason(error),
            ));
        }
        if status::reason(error) == Some(Reason::TooLarge) {
            out.body = QueryBody::BiomarkerTooBroad;
        }
        out.local = status::local(error);
    }
    if out.sources.iter().any(|s| s.reason == Some(Reason::NoKey)) {
        out.next
            .insert(0, (StepKind::Key, Action::KeyHelp(Source::Nci)));
    }
    command.finish(deadline.remaining());
    out
}
fn execute(
    cx: &Ctx<'_>,
    q: &BiomarkerSearch,
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    if cx.fetch.disabled(Source::Nci) {
        return Err(SourceError {
            source: Source::Nci,
            reason: Reason::Disabled,
            status: None,
            retry_after: None,
        }
        .into());
    }
    let saved = snapshot::load(
        cx,
        "biomcp.biomarker-snapshot/1",
        &q.snapshot_query(),
        q.pool(),
        deadline,
    )?;
    if q.pool().is_some() && !cx.fresh && saved.is_none() {
        out.body = QueryBody::PoolError { mismatch: false };
        out.next.push((StepKind::Restart, run(q.restart())));
        return Ok(());
    }
    let saved = match saved {
        Some(saved) => saved,
        None => {
            let leg = cx.fetch.leg(&nci::SPEC, deadline).fresh(cx.fresh);
            let reply = nci::biomarker::search(&leg, q);
            out.sources.push(status::source(
                Source::Nci,
                leg.finish(),
                reply.as_ref().err().copied(),
            ));
            let reply = reply?;
            let mut sources = std::mem::take(&mut out.sources);
            if let Some(source) = sources.first_mut() {
                source.count = Some(reply.value.len() as u64);
                source.retrieved_count = Some(reply.value.len() as u32);
            }
            snapshot::Saved {
                items: reply.value,
                sources,
                problems: reply.problems,
            }
        }
    };
    let pool = if let Some(pool) = q.pool().filter(|_| !cx.fresh) {
        pool.clone()
    } else {
        snapshot::save(
            cx,
            "biomcp.biomarker-snapshot/1",
            &q.snapshot_query(),
            deadline,
            &saved,
        )?
    };
    page(q, saved, pool, out);
    Ok(())
}
fn page(
    q: &BiomarkerSearch,
    saved: snapshot::Saved<biomcp_types::BiomarkerCard>,
    pool: biodata::Text,
    out: &mut Outcome,
) {
    let total = saved.items.len() as u64;
    let items: Box<[_]> = saved
        .items
        .into_iter()
        .skip(q.offset() as usize)
        .take(q.limit() as usize)
        .map(|card| card.header)
        .collect();
    for header in &items {
        for id in &header.codes {
            out.next.push((
                StepKind::Pivot,
                Action::Run {
                    request: Request::SearchTrial(biomcp_types::TrialSearch::for_nci_biomarker(
                        id.clone(),
                    )),
                    fresh: false,
                },
            ));
            out.next.push((
                StepKind::Disambiguate,
                Action::Run {
                    request: Request::GetBiomarker(biomcp_types::BiomarkerGet::one(id.clone())),
                    fresh: false,
                },
            ));
        }
    }
    let returned = items.len() as u32;
    let next = q.offset().saturating_add(returned);
    if returned > 0 && u64::from(next) < total {
        out.next
            .insert(0, (StepKind::Page, run(q.at_page(next, pool))));
    }
    {
        let (found, identity_failed, unavailable) = if saved
        .sources
        .iter()
        .any(|s| s.state == biomcp_types::SourceState::Stale)
        || !saved.problems.is_empty()
    {
        (true, false, true)
    } else if total == 0 {
        (false, false, false)
    } else {
        (true, false, false)
    };
        crate::status::item(out, found, identity_failed, unavailable);
    };
    out.page = Some(biomcp_types::PageInfo {
        offset: q.offset(),
        returned,
        total,
        pool: None,
        truncated: None,
    });
    out.sources = saved.sources;
    out.problems = saved.problems;
    out.body = QueryBody::BiomarkerPage(BiomarkerPage { items });
}
fn empty() -> Outcome {
    crate::status::finish([Status::Failed], crate::status::Parts {
        dropped: false,
        body: QueryBody::BiomarkerPage(BiomarkerPage {
            items: Box::default(),
        }),
        page: None,
        sources: Vec::new(),
        problems: Vec::new(),
        next: Vec::new(),
        local: None,
    })
}
fn run(q: BiomarkerSearch) -> Action {
    Action::Run {
        request: Request::SearchBiomarker(q),
        fresh: false,
    }
}
