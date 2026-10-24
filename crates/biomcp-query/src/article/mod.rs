//! PubMed snapshots and stable pages through one checked command consumer.
mod term;
use crate::{
    Action, Ctx, Outcome, status,
    store::{Pool, Store, StoredItem},
};
use biodata::{
    ArticleItem, ArticleKey, ArticlePage, ArticleSearch, PageInfo, QueryBody, Request, Source,
    Status, StepKind, Text,
};
use biomcp_adapters::pubmed;
use biomcp_fetch::{CacheError, Deadline, RequestError};
use sha2::{Digest, Sha256};
/// Run one article command under a single absolute deadline.
pub fn search(cx: &Ctx<'_>, query: &ArticleSearch) -> Outcome {
    let mut out = empty();
    let command = cx.fetch.command_deadline(cx.budget);
    let mut deadline = command.deadline();
    let result = execute(cx, query, &mut deadline, &mut out);
    command.finish(deadline.remaining());
    if let Err(error) = result {
        out.local = status::local(error);
        out.status = Status::Failed;
    }
    out
}
fn execute<'a>(
    cx: &Ctx<'a>,
    query: &ArticleSearch,
    deadline: &mut Deadline<'a>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let mut store = Store::open(cx.fetch.cache_access()?, *deadline)?;
    let bytes = serde_json::to_vec(&query.identity()).map_err(|_| CacheError::Database)?;
    let key = Sha256::digest([b"biomcp.article_page/1\0".as_slice(), &bytes].concat());
    let explicit = query.pool().map(Text::as_str);
    let stored = if cx.fresh && explicit.is_none() {
        None
    } else {
        store.find(&key, explicit)?
    };
    let pool = match stored {
        Some(pool) if pool.key != key.as_slice() => {
            pool_error(query, true, out);
            return Ok(());
        }
        Some(pool) => pool,
        None if explicit.is_some() => {
            pool_error(query, false, out);
            return Ok(());
        }
        None => {
            let Some(term) = term::resolve(cx, query, deadline, out)? else {
                return Ok(());
            };
            store.set_deadline(*deadline);
            *deadline = store.deadline();
            let leg = cx.fetch.leg(&pubmed::SPEC, *deadline).fresh(cx.fresh);
            let started = leg.remaining();
            let result = pubmed::search_ids(&leg, &term, query.sort(), cx.ncbi_email.as_ref());
            *deadline = consume(*deadline, started, leg.remaining());
            store.set_deadline(*deadline);
            let mut source =
                status::source(Source::PubMed, leg.finish(), result.as_ref().err().copied());
            source.term = Text::new(&term).ok();
            match result {
                Ok(ids) => {
                    source.count = Some(ids.count);
                    source.fetched =
                        Some(u32::try_from(ids.ids.len()).map_err(|_| CacheError::Database)?);
                    source.notices = ids.notices;
                    out.sources.push(source);
                    store.publish(&key, &ids.ids, &out.sources)?
                }
                Err(error) => {
                    out.sources.push(source);
                    out.status = Status::Failed;
                    retry(query, None, out);
                    return Err(error);
                }
            }
        }
    };
    out.sources = pool.sources.clone();
    let result = read_page(cx, query, &mut store, &pool, out);
    *deadline = store.deadline();
    result
}
fn read_page<'a>(
    cx: &Ctx<'a>,
    query: &ArticleSearch,
    store: &mut Store<'a>,
    pool: &Pool,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let id = pool.info.id.as_str();
    let mut rows = store.page(id, query.offset(), query.limit())?;
    let missing: Vec<_> = rows
        .iter()
        .filter(|r| r.header.is_none() && (!r.checked || cx.fresh))
        .map(|r| r.id)
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
                let stale = (source.state == biodata::SourceState::Stale).then_some(&source);
                store.hydrate(id, &missing, &mapped.value, &mapped.problems, stale)?;
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
                retry(query, Some(pool.info.id.clone()), out);
                return Err(error);
            }
        }
    }
    finish(query, pool, rows, out);
    Ok(())
}
fn finish(query: &ArticleSearch, pool: &Pool, rows: Vec<StoredItem>, out: &mut Outcome) {
    for source in rows.iter().filter_map(|r| r.outcome.as_ref()) {
        if let Some(original) = out.sources.iter_mut().find(|s| s.source == source.source) {
            original.state = source.state;
            original.reason = source.reason;
        }
    }
    let stale = out
        .sources
        .iter()
        .any(|s| s.state == biodata::SourceState::Stale);
    let partial = stale || rows.iter().any(|r| r.header.is_none());
    for row in &rows {
        status::append(&mut out.problems, row.problems.clone());
    }
    out.status = if partial {
        Status::Partial
    } else {
        Status::Complete
    };
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
    out.body = QueryBody::ArticlePage(ArticlePage {
        items: rows
            .into_iter()
            .map(|r| ArticleItem {
                key: ArticleKey::Pubmed(r.id),
                pmid: r.id,
                header: r.header,
            })
            .collect(),
    });
    if let Some(offset) = query
        .offset()
        .checked_add(returned)
        .filter(|offset| u64::from(*offset) < total && returned > 0)
    {
        out.next.push((
            StepKind::Page,
            Action::Run {
                request: Request::SearchArticle(query.page(offset, Some(pool.info.id.clone()))),
                fresh: false,
            },
        ));
    }
    if partial {
        retry(
            query,
            if stale {
                None
            } else {
                Some(pool.info.id.clone())
            },
            out,
        );
    }
}
fn pool_error(query: &ArticleSearch, mismatch: bool, out: &mut Outcome) {
    out.body = QueryBody::PoolError { mismatch };
    out.next.push((
        StepKind::Restart,
        Action::Run {
            request: Request::SearchArticle(query.page(0, None)),
            fresh: false,
        },
    ));
}
fn retry(query: &ArticleSearch, pool: Option<Text>, out: &mut Outcome) {
    out.next.push((
        StepKind::Fresh,
        Action::Run {
            request: Request::SearchArticle(query.page(query.offset(), pool)),
            fresh: true,
        },
    ));
}
fn empty() -> Outcome {
    Outcome {
        status: Status::Complete,
        body: QueryBody::ArticlePage(ArticlePage {
            items: Box::default(),
        }),
        page: None,
        sources: Vec::new(),
        problems: Vec::new(),
        next: Vec::new(),
        local: None,
    }
}

// Fetch legs deliberately own independent replay clocks. A sequential article
// command debits each leg before starting another; real deadlines remain absolute.
fn consume<'a>(
    deadline: Deadline<'a>,
    started: std::time::Duration,
    remaining: std::time::Duration,
) -> Deadline<'a> {
    deadline.after_stage(started.saturating_sub(remaining))
}
