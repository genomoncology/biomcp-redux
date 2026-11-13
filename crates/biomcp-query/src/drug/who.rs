//! Read-only local WHO prequalification records; no authorization joins or aliases.
use super::{empty, markets};
use crate::{Action, Bundle, Ctx, Outcome, status};
use biomcp_adapters::who_pq;
use biomcp_types::Paged;
use biomcp_types::ReplyBody;
use biodata::{PartialDate, Text};
use biomcp_types::{
    DrugMarket, DrugMarketState, DrugWhoRow, DrugSearch, DrugSource, DrugWhoGet, DrugWhoPage, PageInfo, Request, Source,
    SourceState, StepKind,
};
pub(super) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let mut out = empty(
        (DrugWhoPage {
            items: Box::default(),
            total: 0,
        })
        .with_query(query.clone()),
    );
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&who_pq::SPEC, deadline).fresh(cx.fresh);
    let result = who_pq::search(&leg, query);
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
        out.body = (DrugWhoPage { items, total }).with_query(query.clone());
    }
    sync_step(&mut out);
    status::search(&mut out);
    command.finish(deadline.remaining());
    out
}
pub(crate) fn get(cx: &Ctx<'_>, query: &DrugWhoGet) -> Outcome {
    let mut out = empty(biomcp_types::DrugWhoCard::into_reply(query.clone(), None));
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
        out.body = biomcp_types::DrugWhoCard::into_reply(query.clone(), card);
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
                bundle: Bundle::WHO_PQ,
            },
        ));
    }
}

/// How the WHO row is chosen among the products WHO lists under the name.
const WHO_CHOICE: &str = "earliest Date of Prequalification among WHO rows whose INN, dosage form and strength cell names the input; export order on a tie; undated rows last";

/// The WHO market row for a name: the earliest prequalified product WHO lists under it.
pub(super) fn market(cx: &Ctx<'_>, name: &Text, out: &mut Outcome) -> DrugMarket {
    let mut row = markets::row(DrugSource::WhoPq);
    let Ok(search) = DrugSearch::named(name.clone(), DrugSource::WhoPq, 10) else {
        return row;
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&who_pq::SPEC, deadline).fresh(cx.fresh);
    let result = who_pq::search(&leg, &search);
    let error = result.as_ref().err().copied();
    let read = status::source(Source::WhoPq, leg.finish(), error);
    command.finish(deadline.remaining());
    if matches!(read.state, SourceState::Syncing | SourceState::Stale) {
        out.next.push(crate::next::Step::new(
            StepKind::Sync,
            &[Source::WhoPq],
            Action::Sync {
                bundle: Bundle::WHO_PQ,
            },
        ));
    }
    let rows = match result {
        Ok(rows) => rows,
        Err(_) => {
            row.state = DrugMarketState::Failed;
            row.reason = read.reason;
            out.local = out.local.or(error.and_then(status::local));
            markets::add_source(out, read);
            return row;
        }
    };
    markets::add_source(out, read);
    row.matches = Some(rows.len() as u64);
    let Some(chosen) = earliest(&rows) else {
        return row;
    };
    row.state = DrugMarketState::Found;
    row.record = Some(chosen.reference.clone());
    row.name = markets::text(&chosen.inn_dosage_form_strength);
    row.holder = markets::text(&chosen.applicant);
    row.first_approval = chosen
        .prequalified_on
        .and_then(|date| markets::dated("date_of_prequalification", date));
    row.url = who_pq::product_url(&chosen.reference);
    row.choice = Some(WHO_CHOICE.into());
    markets::section_step(
        out,
        Source::WhoPq,
        Request::GetWhoMedicine(DrugWhoGet::new(chosen.reference.clone())),
        cx.fresh,
    );
    markets::alternatives(out, &row, name, cx.fresh);
    row
}

/// The first row with the earliest prequalification date; undated rows only when none is dated.
fn earliest(rows: &[DrugWhoRow]) -> Option<&DrugWhoRow> {
    let key = |date: &PartialDate| (date.year(), date.month(), date.day());
    rows.iter()
        .filter(|row| row.prequalified_on.is_some())
        .min_by_key(|row| row.prequalified_on.as_ref().map(key))
        .or_else(|| rows.first())
}
