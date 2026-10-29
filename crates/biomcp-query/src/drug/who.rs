//! Read-only local WHO prequalification records; no authorization joins or aliases.
use super::empty;
use crate::{Action, Bundle, Ctx, Outcome, status};
use biomcp_adapters::who_pq;
use biomcp_types::{
    DrugSearch, DrugWhoGet, DrugWhoPage, PageInfo, QueryBody, Request, Source, SourceState, Status,
    StepKind,
};
pub(super) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let mut out = empty(QueryBody::DrugWhoPage(DrugWhoPage {
        items: Box::default(),
        total: 0,
    }));
    let Some(name) = query.query() else {
        return super::unsupported(query.source(), out);
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&who_pq::SPEC, deadline).fresh(cx.fresh);
    let result = who_pq::search(&leg, name.as_str());
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::WhoPq, leg.finish(), error));
    if let Ok(rows) = result {
        let total = rows.len() as u64;
        let items: Box<[_]> = rows
            .into_iter()
            .skip(query.offset() as usize)
            .take(query.limit() as usize)
            .collect();
        let returned = items.len() as u32;
        out.status = found(!items.is_empty(), &out);
        out.page = Some(PageInfo {
            offset: query.offset(),
            returned,
            total,
            pool: None,
            truncated: None,
        });
        for source in &mut out.sources {
            source.count = Some(total);
            source.fetched = Some(returned);
        }
        for row in &items {
            out.next.push((
                StepKind::Disambiguate,
                Action::Run {
                    request: Request::GetWhoMedicine(DrugWhoGet::new(row.reference.clone())),
                    fresh: cx.fresh,
                },
            ));
        }
        if let Some(offset) = query
            .offset()
            .checked_add(returned)
            .filter(|next| returned > 0 && u64::from(*next) < total)
        {
            out.next.push((
                StepKind::Page,
                Action::Run {
                    request: Request::SearchDrug(query.at_offset(offset)),
                    fresh: cx.fresh,
                },
            ));
        }
        out.body = QueryBody::DrugWhoPage(DrugWhoPage { items, total });
    }
    sync_step(&mut out);
    command.finish(deadline.remaining());
    out
}
pub(crate) fn get(cx: &Ctx<'_>, query: &DrugWhoGet) -> Outcome {
    let mut out = empty(QueryBody::DrugWhoCard(None));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&who_pq::SPEC, deadline).fresh(cx.fresh);
    let result = who_pq::get(&leg, query.input());
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::WhoPq, leg.finish(), error));
    if let Ok((card, _)) = result {
        out.status = found(card.is_some(), &out);
        out.body = QueryBody::DrugWhoCard(card);
    }
    sync_step(&mut out);
    command.finish(deadline.remaining());
    out
}
fn found(found: bool, out: &Outcome) -> Status {
    if !found {
        Status::Absent
    } else if out.sources.iter().any(|s| s.state == SourceState::Stale) {
        Status::Partial
    } else {
        Status::Complete
    }
}
fn sync_step(out: &mut Outcome) {
    if out
        .sources
        .iter()
        .any(|s| matches!(s.state, SourceState::Syncing | SourceState::Stale))
    {
        out.next.push((
            StepKind::Sync,
            Action::Sync {
                bundle: Bundle::WhoPq,
            },
        ));
    }
}
