//! Semantic Scholar edge pools retain source order and per-edge evidence.
use crate::{Action, Ctx, Outcome, status};
use biodata::{ArticleEdges, ArticleItem, Reason, Source, Status, StepKind};
use biomcp_adapters::semantic_scholar;
use biomcp_fetch::{Deadline, RequestError, SourceError};
pub(super) fn create<'a>(cx: &Ctx<'a>, query: &ArticleEdges, references: bool,
    deadline: &mut Deadline<'a>, out: &mut Outcome) -> Result<Option<Vec<ArticleItem>>, RequestError> {
    let leg = cx.fetch.leg(&semantic_scholar::SPEC, *deadline).fresh(cx.fresh);
    let started = leg.remaining();
    let result = collect(&leg, query, references);
    *deadline = super::consume(*deadline, started, leg.remaining());
    let error = result.as_ref().err().copied();
    let mut source = status::source(Source::SemanticScholar, leg.finish(), error);
    let (mut items, count, truncated) = match result {
        Ok(value) => value,
        Err(error) => {
            out.sources.push(source);
            out.status = Status::Failed;
            failure(query, references, out);
            return if status::local(error).is_some() { Err(error) } else { Ok(None) };
        }
    };
    source.count = (!truncated).then_some(count);
    source.fetched = Some(count as u32);
    if truncated {
        source.notices.push(biodata::SourceNotice::GraphTruncated {
            fallback: if references { "article references" } else { "article citations" }.parse().map_err(|_| biomcp_fetch::CacheError::Database)?,
        });
    }
    out.sources.push(source);
    if items.iter().any(|item| item.pmid.is_none() && item.header.as_ref().is_some_and(|h|h.doi.is_some())) {
        match super::semantic_scholar::resolve_dois(cx, &mut items, deadline) {
            Ok(source) => out.sources.push(source),
            Err(error) if status::local(error).is_some() => return Err(error),
            Err(_) => {} // Original source identities and headers remain independently useful.
        }
    }
    for item in &mut items { if item.pmid.is_some() && item.header.as_ref().is_some_and(|h|h.types_unknown) { item.header = None; } }
    let mut seen = std::collections::HashSet::new();
    items.retain(|item| seen.insert(serde_json::to_string(&item.key).unwrap_or_default()));
    Ok(Some(items))
}
fn collect(leg: &biomcp_fetch::Leg<'_>, query: &ArticleEdges, references: bool) -> Result<(Vec<ArticleItem>,u64,bool), RequestError> {
    let mut offset = None;
    let mut items = Vec::new();
    let mut count = 0;
    for _ in 0..5 {
        let reply = semantic_scholar::edges(leg, query.input(), references, offset)?;
        count += reply.data.len() as u64;
        for edge in reply.data {
            let item = edge.item(references).map_err(|_| bad_reply())?;
            items.push(item);
        }
        match reply.next {
            None => return Ok((items,count,false)),
            Some(next) if next > offset.unwrap_or(0) => offset = Some(next),
            Some(_) => return Err(bad_reply()),
        }
    }
    Ok((items,count,true))
}
fn bad_reply() -> RequestError {
    SourceError { source: Source::SemanticScholar, reason: Reason::BadReply, status: None, retry_after: None }.into()
}
fn failure(query: &ArticleEdges, references: bool, out: &mut Outcome) {
    if out.sources.iter().any(|s| s.reason == Some(Reason::RateLimited)) {
        out.next.push((StepKind::Key, Action::KeyHelp(Source::SemanticScholar)));
    }
    out.next.push((StepKind::Fresh, Action::Run {
        request: if references { biodata::Request::ArticleReferences(query.page(0,None)) }
            else { biodata::Request::ArticleCitations(query.page(0,None)) }, fresh: true,
    }));
}
