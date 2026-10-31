//! Read-only local WHO prequalification records; no authorization joins or aliases.
use super::empty;
use crate::{Action, Bundle, Ctx, Outcome, status};
use biomcp_adapters::who_pq;
use biomcp_types::{
    DrugSearch, DrugWhoGet, DrugWhoPage, PageInfo, ReplyData, Request, Source, SourceState,
    StepKind,
};
pub(super) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let mut out = empty(ReplyData::DrugWhoPage {
        query: query.clone(),
        body: Some(DrugWhoPage {
            items: Box::default(),
            total: 0,
        }),
    });
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
        crate::status::item(&mut out, !items.is_empty(), false, false);
        out.page = Some(PageInfo {
            offset: query.offset(),
            returned,
            total,
            pool: None,
            truncated: None,
        });
        for source in &mut out.sources {
            source.count = Some(total);
            source.retrieved_count = Some(returned);
        }
        for row in &items {
            out.next.push(crate::next::Step::run(
                StepKind::Disambiguate,
                &[biomcp_types::Source::WhoPq],
                Request::GetWhoMedicine(DrugWhoGet::new(row.reference.clone())),
                cx.fresh,
            ));
        }
        if let Some(offset) = query
            .offset()
            .checked_add(returned)
            .filter(|next| returned > 0 && u64::from(*next) < total)
        {
            out.next.push(crate::next::Step::run(
                StepKind::Page,
                &[biomcp_types::Source::WhoPq],
                Request::SearchDrug(query.at_offset(offset)),
                cx.fresh,
            ));
        }
        out.body = ReplyData::DrugWhoPage {
            query: query.clone(),
            body: Some(DrugWhoPage { items, total }),
        };
    }
    sync_step(&mut out);
    status::search(&mut out);
    command.finish(deadline.remaining());
    out
}
pub(crate) fn get(cx: &Ctx<'_>, query: &DrugWhoGet) -> Outcome {
    let mut out = empty(ReplyData::DrugWhoCard {
        query: query.clone(),
        body: None,
    });
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&who_pq::SPEC, deadline).fresh(cx.fresh);
    let result = who_pq::get(&leg, query.input());
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::WhoPq, leg.finish(), error));
    if let Ok((card, _)) = result {
        crate::status::item(&mut out, card.is_some(), false, false);
        out.body = ReplyData::DrugWhoCard {
            query: query.clone(),
            body: card,
        };
    }
    sync_step(&mut out);
    command.finish(deadline.remaining());
    out
}

fn sync_step(out: &mut Outcome) {
    if out
        .sources
        .iter()
        .any(|s| matches!(s.state, SourceState::Syncing | SourceState::Stale))
    {
        out.next.push(crate::next::Step::new(
            StepKind::Sync,
            &[biomcp_types::Source::WhoPq],
            Action::Sync {
                bundle: Bundle::WhoPq,
            },
        ));
    }
}
