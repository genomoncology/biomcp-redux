//! Native FDA application discovery; product and submission scope remain separate.
use super::{empty, searched, unsupported};
use crate::{Ctx, Outcome, status};
use biodata::Text;
use biomcp_adapters::{drugsfda as source, received_date};
use biomcp_types::ReplyBody;
use biomcp_types::{
    DrugCard, DrugFdaPage, DrugFdaSort, DrugGet, DrugPage, DrugSearch, DrugSection, DrugSource,
    Request, Source, SourceState, SourceStatus, StepKind,
};
mod approvals;
mod letter;
mod purpose;
/// How the names were searched, for the one notice.
const SEARCHED: &str = "one query at the typed name's step";
/// The one notice for a newest-first page: the source orders every match before it pages.
fn newest(total: Option<u64>) -> String {
    let matches = total.map_or("all matching applications".to_owned(), |n| {
        format!("all {n} matching applications")
    });
    format!(
        "--sort newest: openFDA orders {matches}, not just this page, by their latest submission \
         status date, newest first, ties by application number; applications with no dated \
         submission come last. latest_submission shows that submission, which is often a \
         labeling or efficacy supplement, not the original approval; original_approval gives \
         the original approval date."
    )
}
/// How BioMCP ordered or windowed a whole read, which openFDA cannot do itself.
fn whole_notices(query: &DrugSearch, matched: u64, read: Option<u64>, kept: u64) -> Vec<String> {
    if matched > source::WHOLE_MOST {
        return vec![format!(
            "{matched} applications match, more than the {} BioMCP reads to order or window \
             by original approval, so none are returned. Narrow the search with a drug name or \
             a longer --applicant, or drop --sort original-approval and the approval window to \
             page them in openFDA's own order.",
            source::WHOLE_MOST
        )];
    }
    let read = match read {
        Some(read) if read < matched => format!(
            "only {read} of the {matched} matching applications, because a later openFDA read \
             came back empty, so this reply is partial,"
        ),
        _ => format!("all {matched} matching applications"),
    };
    let mut notices = Vec::new();
    if query.sort() == Some(DrugFdaSort::OriginalApproval) {
        notices.push(format!(
            "--sort original-approval: openFDA cannot sort on one submission, so BioMCP read \
             {read} and ordered them by original_approval date, newest first, ties by \
             application number; applications without one come last."
        ));
    }
    if query.approved_from().is_some() || query.approved_to().is_some() {
        notices.push(format!(
            "--approved-from/--approved-to: BioMCP read {read} and kept the {kept} whose \
             original_approval date falls in the window, inclusive; the total counts those."
        ));
    }
    notices
}

pub(super) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let mut out = empty(
        (DrugFdaPage {
            items: Box::default(),
            total: 0,
            source_updated: None,
        })
        .with_query(query.clone()),
    );
    let command = cx.fetch.command_deadline(cx.budget);
    let mut deadline = command.deadline();
    let fits = |names: &[Text]| source::names_fit(names, query.applicant());
    let mut widened = match super::widen::begin(cx, query, &mut deadline, fits, &mut out) {
        super::widen::Start::Typed => None,
        super::widen::Start::Widened(saved) => Some(saved),
        super::widen::Start::Answered => return out,
    };
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let result = match (widened.as_mut(), query.query()) {
        (Some(saved), Some(typed)) => widened_page(&leg, query, typed, saved),
        _ => source::search(
            &leg,
            query.query(),
            query.applicant(),
            query.limit(),
            query.offset(),
            order(query),
        ),
    };
    let error = result.as_ref().err().copied();
    let mut total = None;
    let mut matched = None;
    let mut read = None;
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::Drugsfda, leg.finish(), error));
    if let Ok(page) = result {
        out.include_refusals(page.refusals);
        total = Some(page.total);
        matched = page.matched;
        read = page.read;
        // A whole read that stopped early orders or windows only what it read.
        out.dropped |= read.zip(matched).is_some_and(|(read, matched)| read < matched);
        // Too many matches to order: the source answered, but this search has no rows.
        if page.matched.is_some_and(|n| n > source::WHOLE_MOST)
            && let Some(report) = out
                .sources
                .iter_mut()
                .find(|s| s.source == Source::Drugsfda)
        {
            report.set_status(SourceStatus::Failed(Some(biomcp_types::Reason::TooLarge)));
        }
        // Later pages keep this page's names, so the snapshot is kept only for an answer.
        let sent = super::widen::retain(cx, query, widened.as_ref(), deadline, true, &mut out);
        out.problems.extend(page.problems);
        searched(
            &sent,
            cx.fresh,
            DrugPage {
                items: page.records.iter().map(|r| r.header()).collect(),
                total: Some(page.total),
            },
            &mut out,
        );
        out.body = (DrugFdaPage {
            items: page
                .records
                .iter()
                .map(source::Application::candidate)
                .collect(),
            total: page.total,
            source_updated: page.updated.map(Into::into),
        })
        .with_query(sent.clone());
    }
    notices(query, total, (matched, read), &mut out);
    if let Some(saved) = widened {
        super::widen::finish(query, saved, SEARCHED, &mut out);
    }
    command.finish(deadline.remaining());
    out
}

/// The adapter's order and window for this search.
fn order(query: &DrugSearch) -> source::Order {
    source::Order {
        sort: query.sort(),
        approved_from: query.approved_from(),
        approved_to: query.approved_to(),
    }
}

/// State the applicant prefix searched and any requested order on the source.
fn notices(
    query: &DrugSearch,
    total: Option<u64>,
    (matched, read): (Option<u64>, Option<u64>),
    out: &mut Outcome,
) {
    let Some(report) = out.sources.first_mut() else {
        return;
    };
    let applicant = query.applicant().map(|applicant| {
        format!(
            "--applicant matches sponsor names starting with {:?}, ignoring letter case.",
            source::applicant_prefix(applicant)
        )
    });
    let sort = (query.sort() == Some(DrugFdaSort::Newest)).then(|| newest(total));
    let whole = matched.map(|n| whole_notices(query, n, read, total.unwrap_or(0)));
    for message in applicant
        .into_iter()
        .chain(sort)
        .chain(whole.into_iter().flatten())
    {
        report
            .notices
            .push(biomcp_types::SourceNotice::RequestGuidance {
                data_date: None,
                message: message.into(),
                retry_after_seconds: None,
            });
    }
}

/// Search the name group at its step; after a refusal, search the typed name alone and keep it.
fn widened_page(
    leg: &biomcp_fetch::Leg<'_>,
    query: &DrugSearch,
    typed: &Text,
    saved: &mut super::widen::Saved<source::NameStep>,
) -> Result<source::Page, biomcp_fetch::RequestError> {
    let page = (query.limit(), query.offset());
    let (result, step) = source::search_widened(
        leg,
        (&saved.names, typed),
        query.applicant(),
        page,
        saved.step,
        order(query),
    );
    saved.step = Some(step);
    if saved.names.len() < 2 || !super::search_names::rejected(result.as_ref().err()) {
        return result;
    }
    saved.reject(query);
    let (result, step) = source::search_widened(
        leg,
        (&saved.names, typed),
        query.applicant(),
        page,
        None,
        order(query),
    );
    saved.step = Some(step);
    result
}
/// Fail the item. A failed reply whose source still reads ok would name no cause, so the
/// source takes the reason.
fn fail(out: &mut Outcome, error: biomcp_fetch::RequestError) {
    crate::status::item(out, false, true, false);
    out.local = status::local(error);
    if let Some(reason) = status::reason(error)
        && let Some(source) = out
            .sources
            .iter_mut()
            .find(|s| s.source == Source::Drugsfda && s.state == SourceState::Ok)
    {
        source.set_status(biomcp_types::SourceStatus::Failed(Some(reason)));
    }
}
pub(super) fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let (is_approvals, mut out) = empty_get(query);
    if !query.supports_sections() {
        return unsupported(query.source(), out);
    }
    let id = match query.native_input() {
        Ok(id) => id,
        Err(_) => return super::rejected_input(query.source(), "input", out),
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let result = source::get(&leg, &id);
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::Drugsfda, leg.finish(), error));
    if let Ok(page) = result {
        if let Some(source) = out.sources.first_mut() {
            source.count = Some(page.total);
            source.retrieved_count = Some(page.records.len() as u32);
        }
        out.include_refusals(page.refusals);
        out.problems.extend(page.problems);
        {
            let (found, identity_failed, unavailable) = if page.records.is_empty() {
                (false, false, false)
            } else if out.sources.iter().any(|s| s.state == SourceState::Stale) {
                (true, false, true)
            } else {
                (true, false, false)
            };
            crate::status::item(&mut out, found, identity_failed, unavailable);
        };
        if let Some(row) = page.records.first() {
            label_steps(cx, row, &mut out);
            if is_approvals {
                if let Err(error) = approvals::select(
                    cx,
                    query,
                    row,
                    (page.metadata.fetched, page.updated),
                    deadline,
                    &mut out,
                ) {
                    fail(&mut out, error);
                }
            } else {
                if let Some(request) = crate::next::checked(
                    query.for_section(DrugSection::Approvals),
                    biomcp_types::Source::Drugsfda,
                    &mut out.problems,
                ) {
                    out.next.push(crate::next::Step::run(
                        StepKind::Section,
                        &[biomcp_types::Source::Drugsfda],
                        Request::GetDrug(request),
                        cx.fresh,
                    ));
                }
                out.body = (DrugCard {
                    header: row.header(),
                    alternate_names: Box::default(),
                    official_names: Box::default(),
                    cross_references: Box::default(),
                    parents: Box::default(),
                    products: Box::default(),
                    formulations: Box::default(),
                    version: None,
                    updated: None,
                    retrieved: received_date(page.metadata.fetched),
                    available_sections: vec![DrugSection::Approvals].into(),
                    next: Box::default(),
                })
                .with_query(query.clone());
            }
        }
    }
    command.finish(deadline.remaining());
    out
}
fn empty_get(query: &DrugGet) -> (bool, Outcome) {
    let is_approvals = query.sections() == [DrugSection::Approvals];
    let out = empty(if is_approvals {
        biomcp_types::DrugApprovals::into_reply(query.clone(), None)
    } else {
        biomcp_types::DrugCard::into_reply(query.clone(), None)
    });
    (is_approvals, out)
}

fn label_steps(cx: &Ctx<'_>, row: &source::Application, out: &mut Outcome) {
    for id in row.label_ids() {
        if let Some(request) = crate::next::checked(
            DrugGet::new(id.clone(), DrugSource::OpenfdaLabel),
            biomcp_types::Source::Drugsfda,
            &mut out.problems,
        ) {
            out.next.push(crate::next::Step::run(
                StepKind::Disambiguate,
                &[request.source().source()],
                Request::GetDrug(request),
                cx.fresh,
            ));
        }
    }
}
