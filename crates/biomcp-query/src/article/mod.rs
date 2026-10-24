//! PubMed snapshots and stable pages through one checked command consumer.
mod fold;
mod edges;
mod order;
mod page;
pub mod get;
mod next;
mod sections;
mod sources;
mod term;
use crate::{
    Action, Ctx, Outcome, status,
    store::Store,
};
use biodata::{
    ArticlePage, ArticleSearch, QueryBody, Request, Source, Status, StepKind, Text,
};
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
            let term = if query.sources().contains(&biodata::ArticleSource::Pubmed) {
                let Some(term) = term::resolve(cx, query, deadline, out)? else {
                    return Ok(());
                };
                term
            } else {
                String::new()
            };
            let Some(mut items) = sources::search(cx, query, &term, deadline, out)? else {
                retry(query, None, out);
                return Ok(());
            };
            if query.sort() == biodata::ArticleSort::Citations && !order::apply(cx, deadline, &mut items, out)? {
                retry(query, None, out);
                return Ok(());
            }
            store.set_deadline(*deadline);
            store.publish(&key, &items, &out.sources)?
        }
    };
    out.sources = pool.sources.clone();
    let result = page::read(cx, page::Cursor::Search(query), &mut store, &pool, out);
    *deadline = store.deadline();
    result
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
pub(super) fn consume<'a>(
    deadline: Deadline<'a>,
    started: std::time::Duration,
    remaining: std::time::Duration,
) -> Deadline<'a> {
    deadline.after_stage(started.saturating_sub(remaining))
}

/// Page one citation graph direction.
pub fn edges(cx: &Ctx<'_>, query:&biodata::ArticleEdges,references:bool)->Outcome {edges::run(cx,query,references)}
