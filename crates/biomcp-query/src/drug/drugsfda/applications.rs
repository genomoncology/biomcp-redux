//! Search Drugs@FDA applications by name, sponsor and submission order.
use super::super::{empty, search_names, searched, widen};
use crate::{Ctx, Outcome, status};
use biodata::Text;
use biomcp_adapters::drugsfda as source;
use biomcp_types::{
    DrugFdaPage, DrugFdaSort, DrugPage, DrugSearch, ReplyBody, Source, SourceStatus,
};

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

pub(in crate::drug) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
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
    let mut widened = match widen::begin(cx, query, &mut deadline, fits, &mut out) {
        widen::Start::Typed => None,
        widen::Start::Widened(saved) => Some(saved),
        widen::Start::Answered => return out,
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
        out.dropped |= read
            .zip(matched)
            .is_some_and(|(read, matched)| read < matched);
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
        let sent = widen::retain(cx, query, widened.as_ref(), deadline, true, &mut out);
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
        widen::finish(query, saved, SEARCHED, &mut out);
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
        let prefix = source::applicant_prefix(applicant);
        if prefix.is_empty() {
            // openFDA refuses every character of this applicant, so it matched no sponsor (0841).
            return "--applicant held no letter or digit openFDA accepts, so it matched no sponsor."
                .into();
        }
        format!("--applicant matches sponsor names starting with {prefix:?}, ignoring letter case.")
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
    saved: &mut widen::Saved<source::NameStep>,
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
    if saved.names.len() < 2 || !search_names::rejected(result.as_ref().err()) {
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
