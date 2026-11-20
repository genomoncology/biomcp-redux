//! FDA Drug Safety Communications: search by drug name or title words, read one by FDA's id.
use super::empty;
use crate::{Ctx, Outcome, status};
use biodata::Text;
use biomcp_adapters::fda_dsc as source;
use biomcp_fetch::{LegReport, RequestError};
use biomcp_types::{
    DrugGet, DrugSafetyCommunication, DrugSafetyCommunicationCard, DrugSafetyCommunicationPage,
    DrugSearch, DrugSource, PageInfo, Paged, ReplyBody, Request, Source, StepKind,
};

const TERMS: &str = "FDA's website content is public domain unless otherwise noted: \
                     https://www.fda.gov/about-fda/about-website/website-policies";

/// Communications whose title or linked drug names hold the name or one of its other names.
pub(super) fn search(cx: &Ctx<'_>, q: &DrugSearch) -> Outcome {
    let seed_date = source::seed_date();
    let page = |items, total, list_starts| DrugSafetyCommunicationPage {
        total,
        list_starts,
        seed_date,
        items,
        hint: None,
    };
    let mut out = empty(page(Box::default(), 0, None).with_query(q.clone()));
    let command = cx.fetch.source_command_deadline(&source::SPEC, cx.budget);
    let mut deadline = command.deadline();
    let mut widened: Option<super::widen::Saved<()>> =
        match super::widen::begin(cx, q, &mut deadline, |_| true, &mut out) {
            super::widen::Start::Typed => None,
            super::widen::Start::Widened(saved) => Some(saved),
            super::widen::Start::Answered => return out,
        };
    let names: Vec<Text> = match widened.as_mut() {
        Some(saved) => saved.names.clone(),
        None => q.query().cloned().into_iter().collect(),
    };
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let result = source::communications(&leg);
    let mut report = leg.finish();
    let error = result.as_ref().err().copied();
    if let Ok(found) = result {
        if let Some(RequestError::Source(failure)) = found.index_error {
            // The seed answered without FDA's index: a stored copy, so marked stale.
            report.stale = true;
            report.failures.push(failure);
        }
        let matching: Vec<_> = found
            .items
            .into_iter()
            .filter(|row| matches(row, &names))
            .collect();
        let total = matching.len() as u64;
        let items: Box<[_]> = matching
            .into_iter()
            .skip(q.offset() as usize)
            .take(q.limit() as usize)
            .collect();
        let returned = items.len() as u32;
        let sent = super::widen::retain(cx, q, widened.as_ref(), deadline, true, &mut out);
        out.problems.extend(
            found
                .problems
                .into_iter()
                .filter(|p| items.iter().any(|row| p.record.as_ref() == Some(&row.id))),
        );
        record_steps(cx.fresh, &items, &mut out);
        if let Some(offset) = sent
            .offset()
            .checked_add(returned)
            .filter(|n| returned > 0 && u64::from(*n) < total)
        {
            out.next.push(crate::next::Step::run(
                StepKind::Page,
                &[Source::FdaDsc],
                Request::SearchDrug(sent.at_offset(offset)),
                cx.fresh,
            ));
        }
        out.page = Some(PageInfo {
            offset: sent.offset(),
            returned,
            total,
            pool: None,
            truncated: None,
        });
        let mut body = page(items, total, found.list_starts);
        body.hint = Text::from_source(&hint(found.list_starts)).ok();
        crate::status::item(&mut out, returned != 0, false, false);
        out.body = body.with_query(sent);
    }
    finish(&mut out, report, error);
    status::search(&mut out);
    if let Some(saved) = widened {
        super::widen::finish(q, saved, SEARCHED, &mut out);
    }
    command.finish(deadline.remaining());
    out
}

/// One communication with its page's text, read live from www.fda.gov.
pub(super) fn get(cx: &Ctx<'_>, q: &DrugGet) -> Outcome {
    let mut out = empty(DrugSafetyCommunicationCard::into_reply(q.clone(), None));
    let id = q.input().to_string();
    if q.is_name() {
        return super::rejected_input(DrugSource::FdaDsc, "id", out);
    }
    let command = cx.fetch.source_command_deadline(&source::SPEC, cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let result = source::communication(&leg, &id);
    let error = result.as_ref().err().copied();
    let result = result.map(|mapped| {
        out.problems.extend(mapped.problems);
        mapped.value
    });
    match result {
        Ok(Some((header, text))) => {
            crate::status::item(&mut out, true, false, false);
            let hint = Text::from_source(&format!(
                "FDA's text as FDA wrote it, read from its page. {TERMS}."
            ))
            .ok();
            out.body = DrugSafetyCommunicationCard {
                header,
                text: Some(text),
                hint,
            }
            .with_query(q.clone());
        }
        Ok(None) => crate::status::item(&mut out, false, false, false),
        Err(_) => {}
    }
    finish(&mut out, leg.finish(), error);
    command.finish(deadline.remaining());
    out
}

fn hint(list_starts: Option<biodata::PartialDate>) -> String {
    let starts = list_starts.map_or_else(|| "its first row".to_owned(), |d| d.to_string());
    format!(
        "FDA's index lists communications from {starts}; for earlier ones FDA links an archived copy, which BioMCP does not read. \
         A row matches by its title and the drug names its page links to Drugs@FDA. {TERMS}."
    )
}

/// A name matches a whole word run in the title or one linked drug name, ignoring case; no
/// name lists every communication.
fn matches(row: &DrugSafetyCommunication, names: &[Text]) -> bool {
    if names.is_empty() {
        return true;
    }
    let fields = std::iter::once(row.title.as_str()).chain(row.drugs.iter().map(Text::as_str));
    let fields: Vec<String> = fields.map(str::to_lowercase).collect();
    names.iter().any(|name| {
        let name = name.as_str().to_lowercase();
        fields.iter().any(|field| whole(field, &name))
    })
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
const SEARCHED: &str = "titles and linked drug names";

fn finish(out: &mut Outcome, report: LegReport, error: Option<RequestError>) {
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::FdaDsc, report, error));
}
