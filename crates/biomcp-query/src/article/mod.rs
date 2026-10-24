//! PubMed snapshots and stable pages through one checked command consumer.
mod term;
use crate::{Ctx,Outcome,Action,store::{Store,Pool,StoredItem},status};
use biodata::{ArticleSearch,ArticlePage,ArticleItem,ArticleKey,QueryBody,Request,Source,Status,PageInfo,StepKind,Text};
use biomcp_adapters::pubmed;
use biomcp_fetch::{RequestError,Deadline,CacheError};
use sha2::{Sha256,Digest};
/// Run one article command under a single absolute deadline.
pub fn search(cx:&Ctx<'_>,query:&ArticleSearch)->Outcome {
    let mut out=empty();
    let deadline=cx.fetch.deadline(cx.budget);
    let result=execute(cx,query,deadline,&mut out);
    if let Err(error)=result {out.local=status::local(error);out.status=Status::Failed;}
    out
}
fn execute(cx:&Ctx<'_>,query:&ArticleSearch,deadline:Deadline<'_>,out:&mut Outcome)->Result<(),RequestError> {
    let mut store=Store::open(cx.fetch.cache_access()?,deadline)?;
    let bytes=serde_json::to_vec(&query.identity()).map_err(|_|CacheError::Database)?;
    let key=Sha256::digest([b"biomcp.article_page/1\0".as_slice(),&bytes].concat());
    let explicit=query.pool().map(Text::as_str);
    let stored=if cx.fresh && explicit.is_none(){None}else{store.find(&key,explicit)?};
    let pool=match stored {
        Some(pool) if pool.key!=key.as_slice()=>{pool_error(query,true,out);return Ok(());},
        Some(pool)=>pool,
        None if explicit.is_some()=>{pool_error(query,false,out);return Ok(());},
        None=>{
            let Some(term)=term::resolve(cx,query,deadline,out)? else{return Ok(());};
            let leg=cx.fetch.leg(&pubmed::SPEC,deadline).fresh(cx.fresh);
            let result=pubmed::search_ids(&leg,&term,cx.ncbi_email.as_ref());
            let mut source=status::source(Source::PubMed,leg.finish(),result.as_ref().err().copied());
            source.term=Text::new(&term).ok();
            match result {
                Ok(ids)=>{
                    source.count=Some(ids.count);source.fetched=Some(u32::try_from(ids.ids.len()).map_err(|_|CacheError::Database)?);source.notices=ids.notices;
                    out.sources.push(source);
                    store.publish(&key,&ids.ids,&out.sources)?
                },
                Err(error)=>{out.sources.push(source);out.status=Status::Failed;retry(query,None,out);return Err(error);}
            }
        }
    };
    out.sources=pool.sources.clone();
    read_page(cx,query,deadline,&mut store,&pool,out)
}
fn read_page(cx:&Ctx<'_>,query:&ArticleSearch,deadline:Deadline<'_>,store:&mut Store,pool:&Pool,out:&mut Outcome)->Result<(),RequestError> {
    let id=pool.info.id.as_str();
    let mut rows=store.page(id,query.offset(),query.limit())?;
    let missing:Vec<_>=rows.iter().filter(|r|r.header.is_none() && (!r.checked || cx.fresh)).map(|r|r.id).collect();
    if !missing.is_empty() {
        let leg=cx.fetch.leg(&pubmed::SPEC,deadline).fresh(cx.fresh);
        let result=pubmed::summaries(&leg,&missing,cx.ncbi_email.as_ref());
        match result {
            Ok(mapped)=>{store.hydrate(id,&missing,&mapped.value,&mapped.problems)?;rows=store.page(id,query.offset(),query.limit())?;},
            Err(error)=>{
                let mut source=status::source(Source::PubMed,leg.finish(),Some(error));
                if let Some(original)=pool.sources.iter().find(|s|s.source==Source::PubMed) {source.term=original.term.clone();source.count=original.count;source.fetched=original.fetched;source.notices=original.notices.clone();}
                out.sources.retain(|s|s.source!=Source::PubMed);out.sources.push(source);
                out.status=Status::Failed;retry(query,Some(pool.info.id.clone()),out);return Err(error);
            }
        }
    }
    finish(query,pool,rows,out);
    Ok(())
}
fn finish(query:&ArticleSearch,pool:&Pool,rows:Vec<StoredItem>,out:&mut Outcome) {
    let partial=rows.iter().any(|r|r.header.is_none());
    for row in &rows {status::append(&mut out.problems,row.problems.clone());}
    out.status=if partial {Status::Partial}else{Status::Complete};
    let returned=u32::try_from(rows.len()).unwrap_or(u32::MAX);
    let total=u64::from(pool.info.size);
    out.page=Some(PageInfo {offset:query.offset(),returned,total,pool:Some(pool.info.clone()),truncated:Some(pool.sources.iter().any(|s|s.count.zip(s.fetched).is_some_and(|(count,fetched)|count>u64::from(fetched))))});
    out.body=QueryBody::ArticlePage(ArticlePage{items:rows.into_iter().map(|r|ArticleItem{key:ArticleKey::Pubmed(r.id),pmid:r.id,header:r.header}).collect()});
    if let Some(offset)=query.offset().checked_add(returned).filter(|offset|u64::from(*offset)<total && returned>0) {
        out.next.push((StepKind::Page,Action::Run{request:Request::SearchArticle(query.page(offset,Some(pool.info.id.clone()))),fresh:false}));
    }
    if partial {retry(query,Some(pool.info.id.clone()),out);}
}
fn pool_error(query:&ArticleSearch,mismatch:bool,out:&mut Outcome) {
    out.body=QueryBody::PoolError{mismatch};
    out.next.push((StepKind::Restart,Action::Run{request:Request::SearchArticle(query.page(0,None)),fresh:false}));
}
fn retry(query:&ArticleSearch,pool:Option<Text>,out:&mut Outcome) {out.next.push((StepKind::Fresh,Action::Run{request:Request::SearchArticle(query.page(query.offset(),pool)),fresh:true}));}
fn empty()->Outcome {Outcome{status:Status::Complete,body:QueryBody::ArticlePage(ArticlePage{items:Box::default()}),page:None,sources:Vec::new(),problems:Vec::new(),next:Vec::new(),local:None}}
