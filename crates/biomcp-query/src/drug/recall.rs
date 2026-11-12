//! Source-native drug recall search and exact retrieval.
use super::empty;
use crate::{Ctx, Outcome, status};
use biomcp_adapters::fda_drug_recall as source;
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::Paged;
use biomcp_types::ReplyBody;
use biomcp_types::{
    DrugGet, DrugRecallPage, DrugSearch, DrugSource, PageInfo, Request, Source, SourceState,
    Status, StepKind,
};
pub(super) fn search(cx: &Ctx<'_>, q: &DrugSearch) -> Outcome {
    let mut out = empty(
        (DrugRecallPage {
            count_unit: biomcp_types::DrugRecallCountUnit::RecallRecords,
            items: Box::default(),
            total: 0,
        })
        .with_query(q.clone()),
    );
    let command = cx.fetch.command_deadline(cx.budget);
    let mut deadline = command.deadline();
    let query = q;
    let fits = |names: &[biodata::Text]| source::names_fit(q, names);
    let mut widened = match super::widen::begin(cx, q, &mut deadline, fits, &mut out) {
        super::widen::Start::Typed => None,
        super::widen::Start::Widened(saved) => Some(saved),
        super::widen::Start::Answered => return out,
    };
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let result = match widened.as_mut() {
        Some(saved) => widened_page(&leg, q, saved),
        None => source::search(&leg, q),
    };
    let error = result.as_ref().err().copied();
    if let Ok(page) = result {
        let returned = page.items.len() as u32;
        // Later pages keep this page's names, so the snapshot is kept only for an answer.
        let sent = super::widen::retain(cx, q, widened.as_ref(), deadline, true, &mut out);
        let q = &sent;
        {
            let found = returned != 0;
            crate::status::item(&mut out, found, false, false);
        };
        out.problems = page.problems;
        record_steps(cx.fresh, &page.items, &mut out);
        if let Some(offset) = q
            .offset()
            .checked_add(returned)
            .filter(|n| returned > 0 && u64::from(*n) < page.total)
        {
            out.next.push(crate::next::Step::run(
                StepKind::Page,
                &[biomcp_types::Source::FdaDrugRecall],
                Request::SearchDrug(q.at_offset(offset)),
                cx.fresh,
            ));
        }
        out.page = Some(PageInfo {
            offset: q.offset(),
            returned,
            total: page.total,
            pool: None,
            truncated: None,
        });
        out.body = (DrugRecallPage {
            count_unit: biomcp_types::DrugRecallCountUnit::RecallRecords,
            items: page.items,
            total: page.total,
        })
        .with_query(q.clone());
    }
    finish(&mut out, leg, error);
    status::search(&mut out);
    if let Some(saved) = widened {
        super::widen::finish(query, saved, SEARCHED, &mut out);
    }
    command.finish(deadline.remaining());
    out
}

/// One exact read per returned recall record.
fn record_steps(fresh: bool, items: &[biomcp_types::DrugRecallHeader], out: &mut Outcome) {
    for item in items {
        if let Some(id) = crate::next::checked(
            item.recall_number.to_string().parse(),
            biomcp_types::Source::FdaDrugRecall,
            &mut out.problems,
        ) && let Some(get) = crate::next::checked(
            DrugGet::new(id, DrugSource::FdaDrugRecall),
            biomcp_types::Source::FdaDrugRecall,
            &mut out.problems,
        ) {
            out.next.push(crate::next::Step::run(
                StepKind::Disambiguate,
                &[biomcp_types::Source::FdaDrugRecall],
                Request::GetDrug(get),
                fresh,
            ));
        }
    }
}

/// How the names were searched, for the one notice.
const SEARCHED: &str = "one recall query";

/// Search the name group; after a refusal, search the typed name alone and keep it.
fn widened_page(
    leg: &Leg<'_>,
    q: &DrugSearch,
    saved: &mut super::widen::Saved<()>,
) -> Result<source::Page, RequestError> {
    let result = source::search_names(leg, q, &saved.names);
    if saved.names.len() < 2 || !super::search_names::rejected(result.as_ref().err()) {
        return result;
    }
    saved.reject(q);
    source::search_names(leg, q, &saved.names)
}
pub(super) fn get(cx: &Ctx<'_>, q: &DrugGet) -> Outcome {
    let mut out = empty(biomcp_types::DrugRecallCard::into_reply(q.clone(), None));
    let Some(id) = q.recall_number() else {
        return out;
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let mut error = None;
    match source::get(&leg, id) {
        Ok(Some(row)) => {
            crate::status::item(&mut out, true, false, false);
            out.include_refusals(row.refusals);
            out.problems = row.problems;
            out.body = (row.value).with_query(q.clone());
        }
        Ok(None) => crate::status::item(&mut out, false, false, false),
        Err(failure) => {
            error = Some(failure.error);
            if let Some(field) = failure.identity_field {
                out.problems.push(biomcp_types::Problem {
                    source: Source::FdaDrugRecall,
                    record: q.input().to_string().parse().ok(),
                    field: field.into(),
                    code: biomcp_types::ProblemCode::IdentityConflict,
                });
            }
        }
    }
    finish(&mut out, leg, error);
    command.finish(deadline.remaining());
    out
}
fn finish(out: &mut Outcome, leg: Leg<'_>, error: Option<RequestError>) {
    out.local = error.and_then(status::local);
    let source = status::source(Source::FdaDrugRecall, leg.finish(), error);
    if out.status() == Status::Complete && (source.state == SourceState::Stale || out.dropped) {
        crate::status::item(out, true, false, true);
    }
    out.sources.push(source);
}
