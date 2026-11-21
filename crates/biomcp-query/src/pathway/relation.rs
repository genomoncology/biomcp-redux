use super::empty;
use crate::{Ctx, Outcome, source_snapshot, status};
use biodata::{PathwayId, Text};
use biomcp_fetch::{Deadline, RequestError, SourceError};
use biomcp_types::ReplyBody;
use biomcp_types::{
    PathwayEventsQuery, PathwayGenesQuery, PathwayHeader, PathwayPageInfo, Reason, ReplyData,
    Request, Source, StepKind,
};
pub(super) enum Echo {
    Genes(PathwayGenesQuery),
    Events(PathwayEventsQuery),
}
pub(super) struct Selection<'a> {
    pub echo: Echo,
    pub id: &'a PathwayId,
    pub genes: bool,
    pub details: bool,
    pub limit: u32,
    pub offset: u32,
    pub pool: Option<&'a Text>,
    pub fresh: bool,
    pub card: Option<(PathwayHeader, Option<Text>)>,
}
impl Selection<'_> {
    fn source(&self) -> Source {
        if matches!(self.id, PathwayId::Wikipathways(_)) {
            Source::WikiPathways
        } else {
            Source::Reactome
        }
    }
}
/// Page retained source membership with checked gene pivots.
pub(crate) fn genes(cx: &Ctx<'_>, q: &PathwayGenesQuery) -> Outcome {
    execute(
        cx,
        Selection {
            echo: Echo::Genes(q.clone()),
            id: q.id(),
            genes: true,
            details: q.details(),
            limit: q.limit(),
            offset: q.offset(),
            pool: q.pool(),
            fresh: cx.fresh,
            card: None,
        },
    )
}
/// Page retained source event objects and unresolved integer references.
pub(crate) fn events(cx: &Ctx<'_>, q: &PathwayEventsQuery) -> Outcome {
    execute(
        cx,
        Selection {
            echo: Echo::Events(q.clone()),
            id: q.id(),
            genes: false,
            details: false,
            limit: q.limit(),
            offset: q.offset(),
            pool: q.pool(),
            fresh: cx.fresh,
            card: None,
        },
    )
}
fn execute(cx: &Ctx<'_>, q: Selection<'_>) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let out = read(cx, q, deadline);
    command.finish(deadline.remaining());
    out
}
pub(super) fn read(cx: &Ctx<'_>, q: Selection<'_>, deadline: Deadline<'_>) -> Outcome {
    let mut out = empty(match &q.echo {
        Echo::Genes(query) => biomcp_types::PathwayGenes::into_reply(query.clone(), None),
        Echo::Events(query) => biomcp_types::PathwayEvents::into_reply(query.clone(), None),
    });
    if let Err(error) = read_inner(cx, &q, deadline, &mut out) {
        out.local = status::local(error);
        crate::status::item(&mut out, false, true, false);
    }
    out
}
fn restart(q: &Selection<'_>, out: &mut Outcome) {
    out.body = ReplyData::PoolError { mismatch: false };
    let products = matches!(&q.echo, Echo::Genes(query) if query.gene_products());
    let request = if q.genes {
        PathwayGenesQuery::new(q.id.clone(), q.limit)
            .map(|query| query.with_gene_products(products))
            .map(|query| {
                if q.details {
                    query.with_details()
                } else {
                    query
                }
            })
            .map(Request::PathwayGenes)
    } else {
        PathwayEventsQuery::new(q.id.clone(), q.limit).map(Request::PathwayEvents)
    };
    if let Ok(request) = request {
        out.next.push(crate::next::Step::new(
            StepKind::Restart,
            &[q.source()],
            super::run(request),
        ));
    }
}
fn read_inner(
    cx: &Ctx<'_>,
    q: &Selection<'_>,
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let source = q.source();
    if cx.fetch.disabled(source) {
        let error: RequestError = SourceError::disabled(source).into();
        out.sources.push(biomcp_adapters::registry::outcome(
            source,
            biomcp_types::SourceStatus::Skipped(Some(Reason::Disabled)),
        ));
        return Err(error);
    }
    // An offset without a pool reads the current copy, or starts one, as a first page does (1061).
    if q.pool.is_some() && q.fresh {
        restart(q, out);
        return Ok(());
    }
    let loaded = if q.fresh {
        None
    } else {
        source_snapshot::find::<Saved>(
            cx,
            &source_snapshot::key("pathway-relation", &(q.id, q.genes))?,
            q.pool,
            deadline,
        )?
        .0
    };
    let (saved, pool) = match loaded {
        Some((saved, pool)) => (saved, Some(pool)),
        None if q.pool.is_some() => {
            restart(q, out);
            return Ok(());
        }
        None => {
            let fetch = match q.id {
                PathwayId::Wikipathways(_) => super::gpml_relation::fetch,
                PathwayId::Reactome(_) => super::reactome_relation::fetch,
            };
            let Some(saved) = fetch(cx, q, deadline, out)? else {
                // The source answered and doesn't hold this pathway.
                crate::status::item(out, false, false, false);
                return Ok(());
            };
            let pool = crate::publish::keep(
                source_snapshot::publish(
                    cx,
                    &source_snapshot::key("pathway-relation", &(q.id, q.genes))?,
                    &saved,
                    saved
                        .genes
                        .as_ref()
                        .map(|g| g.items.len())
                        .or_else(|| saved.events.as_ref().map(|e| e.items.len()))
                        .unwrap_or(0) as u32,
                    &saved.sources,
                    None,
                    deadline,
                ),
                out,
            )?;
            (saved, pool)
        }
    };
    out.dropped = saved.dropped;
    out.sources = saved.sources;
    out.problems = saved.problems;
    match (&q.echo, saved.genes, saved.events) {
        (Echo::Genes(query), Some(body), _) => {
            super::membership::present(cx, q, query, deadline, body, pool.map(|p| p.info), out)
        }
        (Echo::Events(query), _, Some(body)) => {
            present_events(q, query, body, pool.map(|p| p.info), out)
        }
        _ => {}
    }
    Ok(())
}
fn present_events(
    q: &Selection<'_>,
    query: &PathwayEventsQuery,
    mut body: biomcp_types::PathwayEvents,
    pool: Option<biomcp_types::PoolInfo>,
    out: &mut Outcome,
) {
    body.items = body
        .items
        .iter()
        .skip(q.offset as usize)
        .take(q.limit as usize)
        .cloned()
        .collect();
    body.page = position(body.page, q, body.items.len(), pool);
    // A pool saved before 1022 lacks the top-level total; the page always has it.
    body.total = body.page.total;
    super::relation_steps::events(&body, q.limit, &mut out.next, &mut out.problems);
    // The pathway card was read first, so an empty relation is a complete answer, not an unknown pathway.
    crate::status::item(out, true, false, false);
    out.body = body.with_query(query.clone());
}

pub(super) fn position(
    mut page: PathwayPageInfo,
    q: &Selection<'_>,
    returned: usize,
    pool: Option<biomcp_types::PoolInfo>,
) -> PathwayPageInfo {
    page.offset = q.offset;
    page.returned = returned as u32;
    let next = q.offset.saturating_add(q.limit);
    page.next_offset = (pool.is_some() && u64::from(next) < page.total).then_some(next);
    page.pool = pool;
    page
}
#[derive(serde::Serialize, serde::Deserialize)]
pub(super) struct Saved {
    pub dropped: bool,
    pub genes: Option<biomcp_types::PathwayGenes>,
    pub events: Option<biomcp_types::PathwayEvents>,
    pub sources: Vec<biomcp_types::SourceOutcome>,
    pub problems: Vec<biomcp_types::Problem>,
}

impl crate::source_snapshot::Observation for Saved {
    fn whole(&self) -> bool {
        !self.dropped
            && self.genes.as_ref().is_none_or(|genes| {
                !matches!(genes.header.id, PathwayId::Reactome(_)) || genes.release.is_some()
            })
            && crate::source_snapshot::sources_whole(&self.sources)
    }
}
