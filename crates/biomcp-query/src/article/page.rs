use super::consume;
use super::sources;
use crate::{
    Action, Ctx, Outcome, status,
    store::{Pool, Store, StoredItem},
};
use biodata::{
    ArticleEdgePage, ArticleEdges, ArticlePage, ArticleSearch, ArticleSort, PageInfo, QueryBody,
    Request, Source, Status, StepKind, Text,
};
use biomcp_adapters::pubmed;
use biomcp_fetch::RequestError;
#[derive(Clone, Copy)]
pub(super) enum Cursor<'a> {
    Search(&'a ArticleSearch),
    Edges(&'a ArticleEdges, bool),
}
impl Cursor<'_> {
    fn offset(self) -> u32 {
        match self {
            Self::Search(q) => q.offset(),
            Self::Edges(q, _) => q.offset(),
        }
    }
    fn limit(self) -> u32 {
        match self {
            Self::Search(q) => q.limit(),
            Self::Edges(q, _) => q.limit(),
        }
    }
    fn request(self, offset: u32, pool: Option<Text>) -> Request {
        match self {
            Self::Search(q) => Request::SearchArticle(q.page(offset, pool)),
            Self::Edges(q, true) => Request::ArticleReferences(q.page(offset, pool)),
            Self::Edges(q, false) => Request::ArticleCitations(q.page(offset, pool)),
        }
    }
    fn retry(self, pool: Option<Text>, out: &mut Outcome) {
        out.next.push((
            StepKind::Fresh,
            Action::Run {
                request: self.request(self.offset(), pool),
                fresh: true,
            },
        ));
    }
}
pub(super) fn read<'a>(
    cx: &Ctx<'a>,
    query: Cursor<'_>,
    store: &mut Store<'a>,
    pool: &Pool,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let id = pool.info.id.as_str();
    let mut rows = store.page(id, query.offset(), query.limit())?;
    let missing: Vec<_> = rows
        .iter()
        .filter(|r| r.item.header.is_none() && (!r.checked || cx.fresh))
        .filter_map(|r| r.item.pmid)
        .collect();
    if !missing.is_empty() {
        let deadline = store.deadline();
        let leg = cx.fetch.leg(&pubmed::SPEC, deadline).fresh(cx.fresh);
        let started = leg.remaining();
        let result = pubmed::summaries(&leg, &missing, cx.ncbi_email.as_ref());
        store.set_deadline(consume(deadline, started, leg.remaining()));
        match result {
            Ok(mapped) => {
                status::append(&mut out.problems, mapped.problems.clone());
                let source = status::source(Source::PubMed, leg.finish(), None);
                if !out.sources.iter().any(|s| s.source == Source::PubMed) {
                    out.sources.push(source.clone());
                }
                store.hydrate(id, &missing, &mapped.value, &mapped.problems, Some(&source))?;
                rows = store.page(id, query.offset(), query.limit())?;
            }
            Err(error) => {
                let mut source = status::source(Source::PubMed, leg.finish(), Some(error));
                if let Some(original) = pool.sources.iter().find(|s| s.source == Source::PubMed) {
                    source.term = original.term.clone();
                    source.count = original.count;
                    source.fetched = original.fetched;
                    source.notices = original.notices.clone();
                }
                out.sources.retain(|s| s.source != Source::PubMed);
                out.sources.push(source);
                out.status = Status::Failed;
                query.retry(Some(pool.info.id.clone()), out);
                return Err(error);
            }
        }
    }
    finish(query, pool, rows, out);
    Ok(())
}
fn finish(query: Cursor<'_>, pool: &Pool, rows: Vec<StoredItem>, out: &mut Outcome) {
    for source in rows.iter().filter_map(|r| r.outcome.as_ref()) {
        if let Some(original) = out.sources.iter_mut().find(|s| s.source == source.source) {
            if !matches!(original.state, biodata::SourceState::Failed | biodata::SourceState::Stale) {
                original.state = source.state;
                original.reason = source.reason;
            }
        } else {
            out.sources.push(source.clone());
        }
    }
    let stale = out
        .sources
        .iter()
        .any(|s| s.state == biodata::SourceState::Stale);
    let failed = sources::failed(&out.sources);
    let partial = stale
        || failed
        || rows.iter().any(|r| {
            r.item.header.is_none() || r.problems.iter().any(|p| p.source == Source::Icite)
        });
    for row in &rows {
        status::append(&mut out.problems, row.problems.clone());
    }
    out.status = if partial {
        Status::Partial
    } else {
        Status::Complete
    };
    let missing_external = rows
        .iter()
        .any(|r| r.item.pmid.is_none() && r.item.header.is_none());
    let returned = u32::try_from(rows.len()).unwrap_or(u32::MAX);
    let total = u64::from(pool.info.size);
    out.page = Some(PageInfo {
        offset: query.offset(),
        returned,
        total,
        pool: Some(pool.info.clone()),
        truncated: Some(pool.sources.iter().any(|s| {
            s.count
                .zip(s.fetched)
                .is_some_and(|(count, fetched)| count > u64::from(fetched))
        })),
    });
    let items = rows.into_iter().map(|r| r.item).collect();
    out.body = match query {
        Cursor::Search(_) => QueryBody::ArticlePage(ArticlePage { items }),
        Cursor::Edges(_, _) => QueryBody::ArticleEdges(ArticleEdgePage { items }),
    };
    actions(query, pool, returned, total, out);
    if partial {
        query.retry(
            if stale || failed || missing_external {
                None
            } else {
                Some(pool.info.id.clone())
            },
            out,
        );
    }
    if let Cursor::Search(search) = query
        && search.sort() != ArticleSort::Citations
    {
        out.next.push((
            StepKind::Sort,
            Action::Run {
                request: Request::SearchArticle(search.ordered(ArticleSort::Citations)),
                fresh: false,
            },
        ));
    }
}

fn actions(query: Cursor<'_>, pool: &Pool, returned: u32, total: u64, out: &mut Outcome) {
    if let Some(offset) = query
        .offset()
        .checked_add(returned)
        .filter(|offset| u64::from(*offset) < total && returned > 0)
    {
        out.next.push((
            StepKind::Page,
            Action::Run {
                request: query.request(offset, Some(pool.info.id.clone())),
                fresh: false,
            },
        ));
    }
    let items = match &out.body {
        QueryBody::ArticlePage(page) => page.items.as_ref(),
        QueryBody::ArticleEdges(page) => page.items.as_ref(),
        _ => &[],
    };
    if let Some(first) = items.first()
        && let Some(id) = first.pmid
        && let Ok(query) = biodata::ArticleGet::new(vec![biodata::ArticleInput::Pmid(id)].into())
    {
        out.next.push((
            StepKind::Disambiguate,
            Action::Run {
                request: Request::GetArticle(query),
                fresh: false,
            },
        ));
    }
}
