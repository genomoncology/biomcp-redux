use super::{empty, recovery, run};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::semantic_scholar::{self, author};
use biomcp_types::{
    AuthorGet, AuthorPage, AuthorSearch, PageInfo, QueryBody, Request, Source, SourceState, Status,
    StepKind,
};
/// Preserve the source's exact query, candidate order and continuation.
pub fn search(cx: &Ctx<'_>, q: &AuthorSearch) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx
        .fetch
        .leg(&semantic_scholar::SPEC, deadline)
        .fresh(cx.fresh);
    let mut out = empty(QueryBody::AuthorPage(None));
    let result = author::search(&leg, q);
    let error = result.as_ref().err().copied();
    let source = status::source(Source::SemanticScholar, leg.finish(), error);
    if let Ok(page) = result {
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
                run(Request::SearchAuthor(q.at_offset(offset))),
            ));
        }
        out.page = Some(PageInfo {
            offset: q.offset(),
            returned: items.len() as u32,
            total: page.total.unwrap_or(items.len() as u64),
            pool: None,
            truncated: None,
        });
        out.status = if source.state == SourceState::Stale {
            Status::Partial
        } else if items.is_empty() {
            Status::Absent
        } else {
            Status::Complete
        };
        out.body = QueryBody::AuthorPage(Some(AuthorPage {
            items,
            matching_records: page.total,
        }));
    }
    out.sources.push(source);
    out.local = error.and_then(status::local);
    recovery(cx, &mut out, Request::SearchAuthor(q.clone()));
    command.finish(deadline.remaining());
    out
}
