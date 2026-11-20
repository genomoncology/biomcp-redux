//! Page presentation rows from one dated, local FDA Purple Book snapshot.
use crate::{Ctx, Outcome, status};
use biomcp_adapters::purple_book;
use biomcp_types::{
    DrugPurpleBookPage, DrugSearch, PageInfo, Paged, ReplyBody, Request, Source, StepKind,
};

pub(crate) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    const SOURCE: Source = Source::PurpleBook;
    let mut out = status::empty(DrugPurpleBookPage::into_reply(query.clone(), None));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&purple_book::SPEC, deadline);
    let result = purple_book::search(&leg, query);
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    let mut source = status::source(SOURCE, leg.finish(), error);
    let month = result.as_ref().ok().map(|(month, _)| month.as_ref());
    source.notices.push(purple_book::notice(month, ""));
    if let Ok((release_month, rows)) = result {
        let total = rows.len() as u64;
        let offset = query.offset();
        let items: Box<[_]> = rows
            .into_iter()
            .skip(offset as usize)
            .take(query.limit() as usize)
            .collect();
        let returned = items.len() as u32;
        source.count = Some(total);
        source.retrieved_count = Some(returned);
        out.page = Some(PageInfo {
            offset,
            returned,
            total,
            pool: None,
            truncated: None,
        });
        if let Some(next) = offset
            .checked_add(returned)
            .filter(|next| returned > 0 && u64::from(*next) < total)
        {
            out.next.push(crate::next::Step::run(
                StepKind::Page,
                &[SOURCE],
                Request::SearchDrug(query.at_offset(next)),
                cx.fresh,
            ));
        }
        out.body = DrugPurpleBookPage {
            release_month,
            total,
            items,
        }
        .with_query(query.clone());
    }
    out.sources.push(source);
    status::search(&mut out);
    command.finish(deadline.remaining());
    out
}
