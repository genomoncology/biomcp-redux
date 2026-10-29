use super::{empty, order, page};
use crate::{Action, Ctx, Outcome, store::Store};
use biodata::Text;
use biomcp_fetch::{CacheError, Deadline, RequestError};
use biomcp_types::{ArticleEdges, ArticleItem, ArticleKey, QueryBody, Request, Status, StepKind};
use sha2::{Digest, Sha256};
pub(super) fn run(cx: &Ctx<'_>, query: &ArticleEdges, references: bool) -> Outcome {
    let mut out = empty();
    out.body = QueryBody::ArticleEdges(biomcp_types::ArticleEdgePage {
        items: Box::default(),
    });
    let command = cx.fetch.command_deadline(cx.budget);
    let mut deadline = command.deadline();
    if let Err(error) = execute(cx, query, references, &mut deadline, &mut out) {
        out.local = crate::status::local(error);
        out.status = Status::Failed;
    }
    command.finish(deadline.remaining());
    out
}
fn request(query: ArticleEdges, references: bool) -> Request {
    if references {
        Request::ArticleReferences(query)
    } else {
        Request::ArticleCitations(query)
    }
}
fn restart(query: &ArticleEdges, references: bool, out: &mut Outcome, fresh: bool) {
    out.next.push((
        if fresh {
            StepKind::Fresh
        } else {
            StepKind::Restart
        },
        Action::Run {
            request: request(query.page(0, None), references),
            fresh,
        },
    ));
}
fn execute<'a>(
    cx: &Ctx<'a>,
    query: &ArticleEdges,
    references: bool,
    deadline: &mut Deadline<'a>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let mut store = Store::open(cx.fetch.cache_access()?, *deadline)?;
    let identity = serde_json::to_vec(&request(query.identity(), references))
        .map_err(|_| CacheError::Database)?;
    let key = Sha256::digest([b"biomcp.article_edges/2\0".as_slice(), &identity].concat());
    let explicit = query.pool().map(Text::as_str);
    let pool = if cx.fresh && explicit.is_none() {
        None
    } else {
        store.find(&key, explicit)?
    };
    let pool = match pool {
        Some(pool) if pool.key != key.as_slice() => {
            out.body = QueryBody::PoolError { mismatch: true };
            restart(query, references, out, false);
            return Ok(());
        }
        Some(pool) => pool,
        None if explicit.is_some() => {
            out.body = QueryBody::PoolError { mismatch: false };
            restart(query, references, out, false);
            return Ok(());
        }
        None => {
            let Some(items) = create(cx, query, references, deadline, out)? else {
                return Ok(());
            };
            store.set_deadline(*deadline);
            store.publish(&key, &items, &out.sources, &out.problems)?
        }
    };
    out.sources = pool.sources.clone();
    let result = page::read(
        cx,
        page::Cursor::Edges(query, references),
        &mut store,
        &pool,
        out,
    );
    *deadline = store.deadline();
    result
}

fn create<'a>(
    cx: &Ctx<'a>,
    query: &ArticleEdges,
    references: bool,
    deadline: &mut Deadline<'a>,
    out: &mut Outcome,
) -> Result<Option<Vec<ArticleItem>>, RequestError> {
    if query.source() == biomcp_types::EdgeSource::SemanticScholar {
        return super::semantic_edges::create(cx, query, references, deadline, out);
    }
    let Some(pmid) = query.pmid() else {
        return Ok(None);
    };
    let parent = order::fetch(cx, deadline, &[pmid]);
    status::append(&mut out.problems, parent.problems);
    out.sources.push(parent.source);
    if let Some(error) = parent.local {
        return Err(error);
    }
    if parent.error.is_some() {
        out.status = Status::Failed;
        restart(query, references, out, true);
        return Ok(None);
    }
    let ids = parent
        .rows
        .into_iter()
        .find(|row| row.pmid == pmid)
        .and_then(|row| {
            if references {
                row.references
            } else {
                row.citing
            }
        });
    let Some(ids) = ids else {
        out.status = Status::Failed;
        if let Some(source) = out.sources.last_mut() {
            source.state = biomcp_types::SourceState::Failed;
            source.reason = Some(biomcp_types::Reason::BadReply);
        }
        restart(query, references, out, true);
        return Ok(None);
    };
    if order::refuse(ids.len(), out) {
        return Ok(None);
    }
    let mut items: Vec<_> = ids
        .into_iter()
        .map(|id| ArticleItem {
            key: Some(ArticleKey::Pubmed(id)),
            pmid: Some(id),
            header: None,
            cited_by: None,
            intents: None,
            influential: None,
            contexts: None,
            found_by: Default::default(),
            sentence: None,
        })
        .collect();
    if !order::apply(cx, deadline, &mut items, out)? {
        restart(query, references, out, true);
        return Ok(None);
    }
    Ok(Some(items))
}
