use super::papers_snapshot as snapshot;
use super::{empty, recovery, run};
use crate::{Ctx, Outcome, status};
use biodata::{AuthorId, AuthorPaperHeader};
use biomcp_adapters::semantic_scholar::{self, author};
use biomcp_fetch::{Deadline, RequestError, SourceError};
use biomcp_types::{
    ArticleGet, ArticleInput, AuthorPaperDetails, AuthorPapers, AuthorPapersQuery, PageInfo,
    QueryBody, Reason, Request, Source, SourceState, Status, StepKind,
};
/// Read one exact author's native paper page and retain checked continuation state.
pub(super) fn papers(cx: &Ctx<'_>, q: &AuthorPapersQuery) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let mut out = empty(body(q, Box::default()));
    if let Err(error) = execute(cx, q, deadline, &mut out) {
        out.local = status::local(error);
        if matches!(error, RequestError::Source(e) if e.reason == Reason::NotFound) {
            out.status = Status::Absent;
        }
        if out.sources.is_empty() {
            out.sources.push(biomcp_adapters::registry::outcome(
                Source::SemanticScholar,
                SourceState::Failed,
                None,
            ));
        }
    }
    recovery(cx, &mut out, Request::AuthorPapers(q.restart()));
    command.finish(deadline.remaining());
    out
}
fn execute(
    cx: &Ctx<'_>,
    q: &AuthorPapersQuery,
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
                key_needed: false,
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
        let AuthorId::SemanticScholar(id) = q.id() else {
            return Err(biomcp_fetch::CacheError::Database.into());
        };
        let leg = cx
            .fetch
            .leg(&semantic_scholar::SPEC, deadline)
            .fresh(cx.fresh);
        let result = author::papers(&leg, id, q);
        let error = result.as_ref().err().copied();
        let source = status::source(Source::SemanticScholar, leg.finish(), error);
        out.sources.push(source.clone());
        saved.key_needed |= source.reason == Some(Reason::RateLimited)
            && !cx
                .fetch
                .has_key(biomcp_fetch::SecretName::SemanticScholarApiKey);
        saved.pages.push(result?);
        status::retain_source_state(&mut saved.source, &source);
    }
    let pool = if cached {
        old_pool.ok_or(biomcp_fetch::CacheError::Database)?
    } else {
        snapshot::save(cx, q, deadline, &saved, old_pool)?
    };
    present(q, out, saved, pool)
}
fn present(
    q: &AuthorPapersQuery,
    out: &mut Outcome,
    saved: snapshot::Saved,
    pool: crate::store::Pool,
) -> Result<(), RequestError> {
    let page = saved
        .pages
        .into_iter()
        .find(|p| p.offset == q.offset())
        .ok_or(biomcp_fetch::CacheError::Database)?;
    out.problems.extend(page.problems);
    let items: Box<[_]> = page
        .data
        .into_iter()
        .map(|r| r.detail(&mut out.problems))
        .collect::<Result<_, _>>()
        .map_err(|_| SourceError {
            source: Source::SemanticScholar,
            reason: Reason::BadReply,
            status: None,
            retry_after: None,
        })?;
    for row in &items {
        pivot(&row.header, out);
    }
    if let Some(offset) = page.next {
        out.next.push((
            StepKind::Page,
            run(Request::AuthorPapers(
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
    out.status = if saved.source.state == SourceState::Stale || !out.problems.is_empty() {
        Status::Partial
    } else {
        Status::Complete
    };
    if saved.key_needed && saved.source.reason == Some(Reason::RateLimited) {
        super::key_step(out);
    }
    out.sources = vec![saved.source];
    out.body = body(q, items);
    Ok(())
}
fn body(q: &AuthorPapersQuery, items: Box<[biodata::AuthorPaperDetail]>) -> QueryBody {
    if q.section().is_some() {
        QueryBody::AuthorPaperDetails(Some(AuthorPaperDetails {
            author: q.id().clone(),
            items,
        }))
    } else {
        QueryBody::AuthorPapers(Some(AuthorPapers {
            author: q.id().clone(),
            items: items.into_iter().map(|r| r.header).collect(),
        }))
    }
}
fn pivot(row: &AuthorPaperHeader, out: &mut Outcome) {
    let input = row
        .pmid
        .map(ArticleInput::Pmid)
        .or_else(|| row.doi.clone().map(ArticleInput::Doi));
    if let Some(input) = input
        && let Ok(q) = ArticleGet::with_sections(vec![input].into(), Box::default())
    {
        out.next
            .push((StepKind::Pivot, run(Request::GetArticle(q))));
    }
}
