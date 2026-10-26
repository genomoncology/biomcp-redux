//! Retained UniProt windows bind literal selectors and hide native cursors.
use super::next;
use crate::{Action, Ctx, Outcome, status, store::Store};
use biodata::Text;
use biomcp_adapters::uniprot;
use biomcp_fetch::{CacheError, Deadline, RequestError};
use biomcp_types::{
    PageInfo, ProteinPage, ProteinSearch, QueryBody, Request, Source, SourceOutcome, Status,
    StepKind,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[derive(Serialize, Deserialize)]
struct Saved {
    page: uniprot::SearchPage,
    sources: Vec<SourceOutcome>,
    problems: Vec<biomcp_types::Problem>,
}
struct Loaded {
    saved: Saved,
    prior: Option<(Text, String)>,
}
/// Walk the retained collection; an invalid pool never silently refetches.
pub fn search(cx: &Ctx<'_>, q: &ProteinSearch) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let mut out = super::empty(QueryBody::ProteinPage(ProteinPage {
        items: Box::default(),
    }));
    if let Err(error) = read(cx, q, deadline, &mut out) {
        out.local = status::local(error);
        out.next.push((
            StepKind::Fresh,
            Action::Run {
                request: Request::SearchProtein(q.restart()),
                fresh: true,
            },
        ));
    }
    command.finish(deadline.remaining());
    out
}
fn key(q: &ProteinSearch) -> Vec<u8> {
    Sha256::digest(format!(
        "biomcp.protein_pool/1\0{}\0{}\0reviewed-human-uniprot",
        q.query().map_or("", Text::as_str),
        q.gene().map_or("", biodata::GeneSymbol::as_str)
    ))
    .to_vec()
}
fn restart(q: &ProteinSearch, out: &mut Outcome, mismatch: bool) {
    out.body = QueryBody::PoolError { mismatch };
    out.next.push((
        StepKind::Restart,
        Action::Run {
            request: Request::SearchProtein(q.restart()),
            fresh: false,
        },
    ));
}
fn read(
    cx: &Ctx<'_>,
    q: &ProteinSearch,
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let leg = cx.fetch.leg(&uniprot::SPEC, deadline).fresh(cx.fresh);
    if cx.fetch.disabled(Source::UniProt) {
        let error = biomcp_fetch::SourceError {
            source: Source::UniProt,
            reason: biomcp_types::Reason::Disabled,
            status: None,
            retry_after: None,
        }
        .into();
        out.sources
            .push(status::source(Source::UniProt, leg.finish(), Some(error)));
        return Err(error);
    }
    if q.pool().is_none() && q.offset() != 0 || q.pool().is_some() && cx.fresh {
        restart(q, out, false);
        return Ok(());
    }
    let mut store = Store::open(cx.fetch.cache_access()?, deadline)?;
    let loaded = load(q, &leg, &store, cx, out);
    let Loaded { mut saved, prior } = match loaded {
        Ok(Some(saved)) => saved,
        Ok(None) => return Ok(()),
        Err(error) => {
            out.sources
                .push(status::source(Source::UniProt, leg.finish(), Some(error)));
            return Err(error);
        }
    };
    if u64::from(q.offset()) > saved.page.total {
        restart(q, out, false);
        return Ok(());
    }
    let error = extend(&leg, q, &mut saved);
    let incoming = status::source(Source::UniProt, leg.finish(), error);
    if let Some(source) = saved.sources.first_mut() {
        status::retain_source_state(source, &incoming);
    }
    out.sources = saved.sources.clone();
    out.problems = saved.problems.clone();
    out.local = error.and_then(status::local);
    let payload = serde_json::to_string(&saved).map_err(|_| CacheError::Database)?;
    let pool = if prior.as_ref().is_some_and(|(_, old)| old == &payload) {
        prior.map(|(id, _)| id).ok_or(CacheError::Database)?
    } else {
        store.publish_source_pool(&key(q), &payload, &out.sources)?
    };
    let info = store.source_snapshot_info(pool.clone(), saved.page.items.len() as u32)?;
    render(q, saved, pool, info, out);
    Ok(())
}
fn load(
    q: &ProteinSearch,
    leg: &biomcp_fetch::Leg<'_>,
    store: &Store<'_>,
    cx: &Ctx<'_>,
    out: &mut Outcome,
) -> Result<Option<Loaded>, RequestError> {
    let id = if let Some(id) = q.pool() {
        Some(id.clone())
    } else if !cx.fresh {
        store.current_source_pool(&key(q))?
    } else {
        None
    };
    if let Some(id) = id {
        match store.source_snapshot_matches(id.as_str(), &key(q))? {
            Some(true) => {}
            Some(false) => {
                restart(q, out, true);
                return Ok(None);
            }
            None => {
                restart(q, out, false);
                return Ok(None);
            }
        }
        let Some(payload) =
            store.source_snapshot(id.as_str(), &key(q), |s| cx.fetch.disabled(s))?
        else {
            restart(q, out, false);
            return Ok(None);
        };
        let saved = serde_json::from_str::<Saved>(&payload).map_err(|_| CacheError::Database)?;
        return Ok(Some(Loaded {
            saved,
            prior: Some((id, payload)),
        }));
    }
    let mapped = uniprot::search(leg, q, None)?;
    Ok(Some(Loaded {
        saved: Saved {
            page: mapped.value,
            sources: vec![status::source(Source::UniProt, Default::default(), None)],
            problems: mapped.problems,
        },
        prior: None,
    }))
}
fn extend(
    leg: &biomcp_fetch::Leg<'_>,
    q: &ProteinSearch,
    saved: &mut Saved,
) -> Option<RequestError> {
    let needed = q.offset().saturating_add(q.limit()) as usize;
    while saved.page.items.len() < needed {
        let Some(cursor) = saved.page.cursor.clone() else {
            break;
        };
        let page = match uniprot::search(leg, q, Some(&cursor)) {
            Ok(v) => v,
            Err(e) => return Some(e),
        };
        if page.value.total != saved.page.total
            || page.value.cursor.as_ref() == Some(&cursor)
            || page.value.items.iter().any(|row| {
                saved
                    .page
                    .items
                    .iter()
                    .any(|old| old.protein.accession == row.protein.accession)
            })
        {
            return Some(
                biomcp_fetch::SourceError {
                    source: Source::UniProt,
                    reason: biomcp_types::Reason::BadReply,
                    status: Some(200),
                    retry_after: None,
                }
                .into(),
            );
        }
        status::append(&mut saved.problems, page.problems);
        saved.page.items.extend(page.value.items);
        saved.page.cursor = page.value.cursor;
        if saved.page.cursor.is_none() {
            break;
        }
    }
    None
}
fn render(
    q: &ProteinSearch,
    saved: Saved,
    pool: Text,
    info: biomcp_types::PoolInfo,
    out: &mut Outcome,
) {
    let rows: Vec<_> = saved
        .page
        .items
        .into_iter()
        .skip(q.offset() as usize)
        .take(q.limit() as usize)
        .collect();
    out.status = if saved
        .sources
        .iter()
        .any(|s| s.state != biomcp_types::SourceState::Ok)
        || !out.problems.is_empty()
    {
        if rows.is_empty()
            && saved
                .sources
                .iter()
                .any(|s| s.state == biomcp_types::SourceState::Failed)
        {
            Status::Failed
        } else {
            Status::Partial
        }
    } else {
        Status::Complete
    };
    let returned = rows.len() as u32;
    let offset = q.offset().saturating_add(returned);
    out.next = next::search(
        q,
        &rows,
        pool,
        offset,
        returned > 0 && u64::from(offset) < saved.page.total,
        out.status != Status::Complete,
    );
    out.page = Some(PageInfo {
        offset: q.offset(),
        returned,
        total: saved.page.total,
        pool: Some(info),
        truncated: Some(saved.page.cursor.is_some()),
    });
    out.body = QueryBody::ProteinPage(ProteinPage { items: rows.into() });
}
