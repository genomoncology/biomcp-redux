use super::{empty, retry, run, snapshot};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::semantic_scholar::{self, author};
use biomcp_fetch::{Deadline, RequestError, SourceError};
use biomcp_types::{
    AuthorGet, AuthorPage, AuthorSearch, PageInfo, Reason, ReplyData, Request, Source, SourceState,
    StepKind,
};
/// Preserve the source's exact query, candidate order and continuation.
pub(crate) fn search(cx: &Ctx<'_>, q: &AuthorSearch) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let mut out = empty(ReplyData::AuthorPage {
        query: q.clone(),
        body: None,
    });
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
    let key = out
        .next
        .iter()
        .position(|step| step.0 == StepKind::Key)
        .map(|index| out.next.remove(index));
    retry(&mut out, Request::SearchAuthor(q.restart()));
    if let Some(key) = key {
        out.next.push(key);
    }
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
    if matches!(out.body, ReplyData::PoolError { .. }) {
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
    let cached = saved.pages.iter().any(|p| p.offset() == q.offset());
    if !cached {
        let leg = cx
            .fetch
            .leg(&semantic_scholar::SPEC, deadline)
            .fresh(cx.fresh);
        let result = author::search(&leg, q);
        let error = result.as_ref().err().copied();
        let source = status::source(Source::SemanticScholar, leg.finish(), error);
        out.sources.push(source.clone());
        let key_needed = source.reason == Some(Reason::RateLimited)
            && !cx
                .fetch
                .has_key(biomcp_fetch::SecretName::SemanticScholarApiKey);
        if key_needed {
            super::key_step(out);
        }
        saved.key_needed |= key_needed;
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
    q: &AuthorSearch,
    out: &mut Outcome,
    saved: snapshot::Saved,
    pool: crate::store::Pool,
) -> Result<(), RequestError> {
    let page = saved
        .pages
        .into_iter()
        .find(|p| p.offset() == q.offset())
        .ok_or(biomcp_fetch::CacheError::Database)?;
    let next = page.next();
    let total = page.total();
    let mapped = page.headers();
    out.include_refusals(mapped.refusals);
    out.problems.extend(mapped.problems);
    let items = mapped.value;
    for row in &items {
        out.next.push(crate::next::Step::new(
            StepKind::Disambiguate,
            &[super::next::author(&row.id)],
            run(Request::GetAuthor(AuthorGet::one(row.id.clone()))),
        ));
    }
    if let Some(offset) = next {
        out.next.push(crate::next::Step::new(
            StepKind::Page,
            &[biomcp_types::Source::SemanticScholar],
            run(Request::SearchAuthor(
                q.at_offset(offset).with_pool(pool.info.id.clone()),
            )),
        ));
    }
    out.page = total.map(|total| PageInfo {
        offset: q.offset(),
        returned: items.len() as u32,
        total,
        pool: Some(pool.info),
        truncated: None,
    });
    {
        let unavailable = saved.source.state == SourceState::Stale;
        crate::status::item(out, true, false, unavailable);
    };
    if saved.key_needed && saved.source.reason == Some(Reason::RateLimited) {
        super::key_step(out);
    }
    out.sources = vec![saved.source];
    out.body = ReplyData::AuthorPage {
        query: q.clone(),
        body: Some(AuthorPage {
            items,
            matching_records: total,
        }),
    };
    Ok(())
}
