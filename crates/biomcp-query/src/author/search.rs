use super::{empty, recovery, run, snapshot};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::semantic_scholar::{self, author};
use biomcp_fetch::{Deadline, RequestError, SourceError};
use biomcp_types::{
    AuthorGet, AuthorPage, AuthorSearch, PageInfo, QueryBody, Reason, Request, Source, SourceState,
    Status, StepKind,
};
/// Preserve the source's exact query, candidate order and continuation.
pub fn search(cx: &Ctx<'_>, q: &AuthorSearch) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let mut out = empty(QueryBody::AuthorPage(None));
    if let Err(error) = execute(cx, q, deadline, &mut out) {
        out.local = status::local(error);
        if out.sources.is_empty() {
            out.sources.push(biomcp_adapters::registry::outcome(
                Source::SemanticScholar,
                SourceState::Failed,
                None,
            ));
        }
    }
    recovery(cx, &mut out, Request::SearchAuthor(q.clone()));
    command.finish(deadline.remaining());
    out
}
fn execute(
    cx: &Ctx<'_>,
    q: &AuthorSearch,
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    if cx.fetch.disabled(Source::SemanticScholar) {
        out.sources.push(biomcp_adapters::registry::outcome(
            Source::SemanticScholar,
            SourceState::Skipped,
            Some(Reason::Disabled),
        ));
        return Err(SourceError {
            source: Source::SemanticScholar,
            reason: Reason::Disabled,
            status: None,
            retry_after: None,
        }
        .into());
    }
    let saved = snapshot::load(cx, q, deadline, out)?;
    if matches!(out.body, QueryBody::PoolError { .. }) {
        return Ok(());
    }
    let (mut saved, old_pool) = if let Some((saved, pool)) = saved {
        (saved, Some(pool))
    } else {
        (
            snapshot::Saved {
                pages: vec![],
                source: biomcp_adapters::registry::outcome(
                    Source::SemanticScholar,
                    SourceState::Ok,
                    None,
                ),
            },
            None,
        )
    };
    let cached = saved.pages.iter().any(|p| p.offset == q.offset());
    if !cached {
        let leg = cx
            .fetch
            .leg(&semantic_scholar::SPEC, deadline)
            .fresh(cx.fresh);
        let result = author::search(&leg, q);
        let error = result.as_ref().err().copied();
        let source = status::source(Source::SemanticScholar, leg.finish(), error);
        out.sources.push(source.clone());
        saved.pages.push(result?);
        saved.source = source;
    }
    let pool = if cached {
        old_pool.ok_or(biomcp_fetch::CacheError::Database)?
    } else {
        snapshot::save(cx, q, deadline, &saved, old_pool)?
    };
    let page = saved
        .pages
        .into_iter()
        .find(|p| p.offset == q.offset())
        .ok_or(biomcp_fetch::CacheError::Database)?;
    let items: Box<[_]> = page.data.into_iter().map(|r| r.header()).collect();
    for row in &items {
        out.next.push((
            StepKind::Disambiguate,
            run(Request::GetAuthor(AuthorGet::one(row.id.clone()))),
        ));
    }
    if let Some(offset) = page.next {
        out.next.push((
            StepKind::Page,
            run(Request::SearchAuthor(
                q.at_offset(offset).with_pool(pool.info.id.clone()),
            )),
        ));
    }
    out.page = page.total.map(|total| PageInfo {
        offset: q.offset(),
        returned: items.len() as u32,
        total,
        pool: Some(pool.info),
        truncated: None,
    });
    out.status = if saved.source.state == SourceState::Stale {
        Status::Partial
    } else {
        Status::Complete
    };
    out.sources = vec![saved.source];
    out.body = QueryBody::AuthorPage(Some(AuthorPage {
        items,
        matching_records: page.total,
    }));
    Ok(())
}
