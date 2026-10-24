use crate::{Ctx, Outcome, status};
use biodata::{ArticleItem, Pmid, QueryBody, Source, SourceOutcome, Status};
use biomcp_adapters::icite;
use biomcp_fetch::{Deadline, RequestError};
/// Application citation enrichment window, shared by searches and graph pages.
const MAX_PMIDS: usize = 10_000;
pub(super) struct Counts {
    pub rows: Vec<icite::Publication>,
    pub source: SourceOutcome,
    pub local: Option<RequestError>,
    pub error: Option<RequestError>,
}
pub(super) fn fetch<'a>(cx: &Ctx<'a>, deadline: &mut Deadline<'a>, ids: &[Pmid]) -> Counts {
    let leg=cx.fetch.leg(&icite::SPEC,*deadline).fresh(cx.fresh);
    let started=leg.remaining();
    let mut rows=Vec::new();
    let mut error=None;
    let mut local=None;
    let chunks:Vec<_>=ids.chunks(icite::BATCH_LIMIT).collect();
    // A wave bounds transport callbacks to four, without a second thread policy.
    for wave in chunks.chunks(4) {
        for result in leg.batch(wave, |child,ids|icite::publications(child,ids)) {
            match result {
                Ok(values)=>rows.extend(values),
                Err(e)=> {error.get_or_insert(e); local=local.or(status::local(e));}
            }
        }
    }
    *deadline=deadline.after_stage(started.saturating_sub(leg.remaining()));
    Counts {rows,source:status::source(Source::Icite,leg.finish(),error),error,local}
}
pub(super) fn apply<'a>(cx: &Ctx<'a>, deadline: &mut Deadline<'a>, items: &mut [ArticleItem], out:&mut Outcome) -> Result<bool,RequestError> {
    let ids:Vec<_>=items.iter().filter_map(|i|i.pmid).collect();
    if refuse(ids.len(),out) {return Ok(false);}
    let result=fetch(cx,deadline,&ids);
    let stale=out.sources.iter().any(|s|s.source==Source::Icite && s.state==biodata::SourceState::Stale);
    out.sources.retain(|s|s.source!=Source::Icite);
    let mut source=result.source;
    if stale && result.error.is_none() {source.state=biodata::SourceState::Stale;}
    out.sources.push(source);
    if let Some(error)=result.local {return Err(error);}
    if result.error.is_some() {
        out.status=Status::Failed;
        return Ok(false);
    }
    for item in items.iter_mut() {
        item.cited_by=result.rows.iter().find(|r|Some(r.pmid)==item.pmid).and_then(|r|r.counts.cited_by);
    }
    // Stable sort preserves the native fold order for equal or absent counts.
    items.sort_by_key(|item|std::cmp::Reverse(item.cited_by));
    Ok(true)
}
pub(super) fn refuse(count:usize,out:&mut Outcome)->bool {
    if count<=MAX_PMIDS {return false;}
    out.status=Status::Failed;
    out.body=QueryBody::TooManyPmids {count:count as u64};
    true
}
