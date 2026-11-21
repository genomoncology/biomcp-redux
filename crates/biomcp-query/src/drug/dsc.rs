//! FDA Drug Safety Communications: search by drug name or text, read one by FDA's id.
use super::empty;
use crate::{Ctx, Outcome, status};
use biodata::Text;
use biomcp_adapters::fda_dsc as source;
use biomcp_fetch::{LegReport, RequestError};
use biomcp_types::{
    DrugGet, DrugSafetyCommunication, DrugSafetyCommunicationCard, DrugSafetyCommunicationPage,
    DrugSafetyMatch, DrugSearch, DrugSource, PageInfo, Paged, ReplyBody, Request, Source, StepKind,
};

/// Communications whose title, linked drug names or text hold the name or one of its other names;
/// with no name, every communication, newest first.
pub(super) fn search(cx: &Ctx<'_>, q: &DrugSearch) -> Outcome {
    let empty_page = DrugSafetyCommunicationPage {
        total: 0,
        searched: None,
        text_searched: None,
        list_starts: None,
        seed_date: None,
        items: Box::default(),
        hint: None,
    };
    let mut out = empty(empty_page.with_query(q.clone()));
    if cx.fetch.disabled(Source::FdaDsc) {
        let error = biomcp_fetch::SourceError::disabled(Source::FdaDsc);
        finish(&mut out, LegReport::default(), Some(error.into()));
        status::search(&mut out);
        return out;
    }
    let command = cx.fetch.source_command_deadline(&source::SPEC, cx.budget);
    let mut deadline = command.deadline();
    let widened: Option<super::widen::Saved<()>> =
        match super::widen::begin(cx, q, &mut deadline, |_| true, &mut out) {
            super::widen::Start::Typed => None,
            super::widen::Start::Widened(saved) => Some(saved),
            super::widen::Start::Answered => return out,
        };
    let names: Vec<Text> = match widened.as_ref() {
        Some(saved) => saved.names.clone(),
        None => q.query().cloned().into_iter().collect(),
    };
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let result = source::communications(&leg);
    let mut report = leg.finish();
    let error = result.as_ref().err().copied();
    // A failed read goes in `sources`; the reply keeps no rows.
    let found = match result {
        Ok(found) => found,
        Err(_) => {
            finish(&mut out, report, error);
            status::search(&mut out);
            if let Some(saved) = widened {
                super::widen::finish(q, saved, SEARCHED, &mut out);
            }
            command.finish(deadline.remaining());
            return out;
        }
    };
    if let Some(failure) = found.index_error {
        // The shipped list answered without FDA's index: a stored copy, marked stale.
        report.stale = true;
        if let RequestError::Source(failure) = failure {
            report.failures.push(failure);
        }
    }
    let sent = super::widen::retain(cx, q, widened.as_ref(), deadline, true, &mut out);
    answer(cx.fresh, &sent, found, &names, &mut out);
    finish(&mut out, report, error);
    status::search(&mut out);
    if let Some(saved) = widened {
        super::widen::finish(q, saved, SEARCHED, &mut out);
    }
    command.finish(deadline.remaining());
    out
}

/// One page of the matching communications, with a read for each and the next page's call.
fn answer(
    fresh: bool,
    sent: &DrugSearch,
    found: source::Communications,
    names: &[Text],
    out: &mut Outcome,
) {
    let searched = found.items.len() as u64;
    let text_searched = found.items.iter().filter(|row| row.text.is_some()).count() as u64;
    let names: Vec<_> = names
        .iter()
        .map(|name| name.as_str().to_lowercase())
        .collect();
    let matching: Vec<_> = found
        .items
        .into_iter()
        .filter_map(|row| matched(row, &names))
        .collect();
    let total = matching.len() as u64;
    let items: Box<[_]> = matching
        .into_iter()
        .skip(sent.offset() as usize)
        .take(sent.limit() as usize)
        .collect();
    let returned = items.len() as u32;
    out.problems.extend(
        found
            .problems
            .into_iter()
            .filter(|p| items.iter().any(|row| p.record.as_ref() == Some(&row.id))),
    );
    record_steps(fresh, &items, out);
    if let Some(offset) = sent
        .offset()
        .checked_add(returned)
        .filter(|n| returned > 0 && u64::from(*n) < total)
    {
        out.next.push(crate::next::Step::run(
            StepKind::Page,
            &[Source::FdaDsc],
            Request::SearchDrug(sent.at_offset(offset)),
            fresh,
        ));
    }
    out.page = Some(PageInfo {
        offset: sent.offset(),
        returned,
        total,
        pool: None,
        truncated: None,
    });
    crate::status::item(out, returned != 0, false, false);
    out.body = DrugSafetyCommunicationPage {
        total,
        searched: Some(searched),
        text_searched: Some(text_searched),
        list_starts: found.list_starts,
        seed_date: Some(found.seed_date),
        items,
        hint: if returned == 0 || text_searched < searched {
            let detail = found.hint.as_ref().map_or("", Text::as_str);
            match Text::new(&format!(
                "Zero matches does not establish that FDA issued no communication about this drug. \
                 Searched {searched} communications, with page text for {text_searched}. FDA index {}. {detail}",
                source::index_url()
            )) {
                Ok(hint) => Some(hint),
                Err(_) => found.hint,
            }
        } else {
            found.hint
        },
    }
    .with_query(sent.clone());
}

/// One communication with its page's text, read live from www.fda.gov.
pub(super) fn get(cx: &Ctx<'_>, q: &DrugGet) -> Outcome {
    let mut out = empty(DrugSafetyCommunicationCard::into_reply(q.clone(), None));
    if !q.sections().is_empty() {
        return super::rejected_input(DrugSource::FdaDsc, "section", out);
    }
    let command = cx.fetch.source_command_deadline(&source::SPEC, cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let result = source::communication(&leg, &q.input().to_string());
    let error = result.as_ref().err().copied();
    // A failed read goes in `sources`; the reply keeps no body.
    let mapped = match result {
        Ok(mapped) => mapped,
        Err(_) => {
            finish(&mut out, leg.finish(), error);
            command.finish(deadline.remaining());
            return out;
        }
    };
    out.problems.extend(mapped.problems);
    let found = mapped.value.is_some();
    if let Some((header, text)) = mapped.value {
        let card = DrugSafetyCommunicationCard {
            header,
            text: Some(text),
            hint: source::text_hint(),
        };
        out.body = card.with_query(q.clone());
    }
    crate::status::item(&mut out, found, false, false);
    finish(&mut out, leg.finish(), error);
    command.finish(deadline.remaining());
    out
}

/// Match whole word runs in each field and retain the places that matched.
fn matched(row: source::SearchRow, names: &[String]) -> Option<DrugSafetyCommunication> {
    let mut header = row.header;
    if names.is_empty() {
        return Some(header);
    }
    let matches = |field: &str| {
        let field = field.to_lowercase();
        names.iter().any(|name| whole(&field, name))
    };
    header.matched_in = [
        (DrugSafetyMatch::Title, matches(header.title.as_str())),
        (
            DrugSafetyMatch::LinkedName,
            header.drugs.iter().any(|drug| matches(drug.as_str())),
        ),
        (
            DrugSafetyMatch::Text,
            row.text.as_deref().is_some_and(matches),
        ),
    ]
    .into_iter()
    .filter_map(|(place, found)| found.then_some(place))
    .collect();
    (!header.matched_in.is_empty()).then_some(header)
}

fn whole(field: &str, name: &str) -> bool {
    !name.is_empty()
        && field.match_indices(name).any(|(at, _)| {
            let before = field.get(..at).and_then(|s| s.chars().next_back());
            let after = field.get(at + name.len()..).and_then(|s| s.chars().next());
            !before.is_some_and(char::is_alphanumeric) && !after.is_some_and(char::is_alphanumeric)
        })
}

/// One exact read per returned communication.
fn record_steps(fresh: bool, items: &[DrugSafetyCommunication], out: &mut Outcome) {
    for item in items {
        if let Some(id) =
            crate::next::checked(item.id.as_str().parse(), Source::FdaDsc, &mut out.problems)
            && let Some(get) = crate::next::checked(
                DrugGet::new(id, DrugSource::FdaDsc),
                Source::FdaDsc,
                &mut out.problems,
            )
        {
            out.next.push(crate::next::Step::run(
                StepKind::Disambiguate,
                &[Source::FdaDsc],
                Request::GetDrug(get),
                fresh,
            ));
        }
    }
}

/// How the names were searched, for the one notice.
const SEARCHED: &str = "titles, linked drug names and page text";

fn finish(out: &mut Outcome, report: LegReport, error: Option<RequestError>) {
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::FdaDsc, report, error));
}
