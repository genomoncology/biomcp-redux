//! Return a complete biosimilar roster or a page of FDA presentation rows.
use crate::{Ctx, Outcome, status};
use biomcp_adapters::purple_book;
use biomcp_types::{
    DrugPurpleBookPage, DrugPurpleBookProducts, DrugSearch, Paged, PurpleBookPresentation,
    ReplyBody, Request, Source, StepKind,
};

const SOURCE: Source = Source::PurpleBook;

pub(crate) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let details = query.purple_book_presentations();
    let mut out = status::empty(if details {
        DrugPurpleBookPage::into_reply(query.clone(), None)
    } else {
        DrugPurpleBookProducts::into_reply(query.clone(), None)
    });
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&purple_book::SPEC, deadline);
    let result = purple_book::search(&leg, query);
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    let mut source = status::source(SOURCE, leg.finish(), error);
    match result {
        Ok((release_month, rows)) => {
            source
                .notices
                .push(purple_book::notice(Some(&release_month), ""));
            if details {
                presentations(&mut out, query, cx.fresh, release_month, rows);
            } else {
                products(&mut out, query, cx.fresh, release_month, &rows);
            }
            if let Some(page) = &out.page {
                source.count = Some(page.total);
                source.retrieved_count = Some(page.returned);
            }
        }
        Err(_) => source.notices.push(purple_book::notice(None, "")),
    }
    out.sources.push(source);
    status::search(&mut out);
    command.finish(deadline.remaining());
    out
}

fn presentations(
    out: &mut Outcome,
    query: &DrugSearch,
    fresh: bool,
    release_month: Box<str>,
    rows: Vec<PurpleBookPresentation>,
) {
    let total = rows.len() as u64;
    let offset = query.offset();
    let items: Box<[_]> = rows
        .into_iter()
        .skip(offset as usize)
        .take(query.limit() as usize)
        .collect();
    let returned = items.len() as u32;
    out.page = Some(status::page_info(offset, returned, total));
    if let Some(next) = offset
        .checked_add(returned)
        .filter(|next| returned > 0 && u64::from(*next) < total)
    {
        out.next.push(crate::next::Step::run(
            StepKind::Page,
            &[SOURCE],
            Request::SearchDrug(query.at_offset(next)),
            fresh,
        ));
    }
    out.body = DrugPurpleBookPage {
        release_month,
        total,
        items,
    }
    .with_query(query.clone());
}

fn products(
    out: &mut Outcome,
    query: &DrugSearch,
    fresh: bool,
    release_month: Box<str>,
    rows: &[PurpleBookPresentation],
) {
    let roster = purple_book::summarize(release_month, rows);
    out.page = Some(status::page_info(
        0,
        roster.items.len() as u32,
        roster.total,
    ));
    if let Some(details) =
        crate::next::checked(query.purple_book_details(), SOURCE, &mut out.problems)
    {
        out.next.push(crate::next::Step::run(
            StepKind::Section,
            &[SOURCE],
            Request::SearchDrug(details),
            fresh,
        ));
    }
    out.body = roster.with_query(query.clone());
}
