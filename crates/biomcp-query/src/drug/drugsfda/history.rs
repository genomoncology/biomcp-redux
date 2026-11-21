//! Run one bounded Drugs@FDA label-history query and offer a continuation when needed.
use super::history_read::{Opened, Search, versions};
use super::history_reply::reply;
use super::history_search::{Path, search};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::drugsfda as source;
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{
    DrugGet, DrugLabelHistory, LabelHistorySection, ReplyBody, Request, Source, StepKind,
};
use std::collections::BTreeMap;

/// Answer one label-history get for the application in `row`.
pub(super) fn select(
    cx: &Ctx<'_>,
    query: &DrugGet,
    row: &source::Application,
    deadline: biomcp_fetch::Deadline<'_>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let (finds, every, section) = query.label_history();
    let section = section.unwrap_or(LabelHistorySection::BoxedWarning);
    let versions = versions(row)?;
    let leg = cx.fetch.leg(&source::DOCUMENTS, deadline).fresh(cx.fresh);
    let mut found = Search {
        leg: &leg,
        versions: &versions,
        finds,
        section,
        opened: BTreeMap::new(),
        stopped: false,
        pdf_busy: false,
    };
    let path = if every {
        let from = query.section_offset().unwrap_or(0) as usize;
        found.in_order(from);
        Path::Every(from)
    } else {
        search(&mut found)
    };
    let (mut body, consistent) = reply(&found, row, section, path)?;
    if found.pdf_busy {
        body.answer = text(
            "BioMCP could not finish the label history because the hosted PDF reader was busy. Retry shortly.",
        )?;
        body.method = text(&format!(
            "Stopped when the next PDF slot was unavailable. {} of {} PDFs read.",
            found.opened.len(),
            found.versions.len(),
        ))?;
        body.first = None;
        body.before = None;
        body.on_or_before = false;
    }
    let open = !consistent
        || found.stopped
        || (body.first.is_none() && body.not_opened > 0)
        || body.on_or_before;
    let unread = open || found.opened.values().any(Opened::gap);
    let resume = if found.pdf_busy {
        None
    } else {
        resume(&found, path)
    };
    drop(found);
    merge(leg, out);
    if open && body.not_opened > 0 {
        status::missing(out, text("/result/not_opened")?, Source::Drugsfda, None);
    }
    if let Some(from) = resume.filter(|from| open && *from < versions.len()) {
        let request = query.every_version_from(from as u32);
        if let Some(request) = crate::next::checked(request, Source::Drugsfda, &mut out.problems) {
            out.next.push(crate::next::Step::run(
                StepKind::Page,
                &[Source::Drugsfda],
                Request::GetDrug(request),
                cx.fresh,
            ));
        }
    }
    status::item(out, true, false, unread);
    out.body = body.with_query(query.clone());
    Ok(())
}

/// An ordered page continues after its last read. A search resumes at the oldest unopened label.
fn resume(search: &Search<'_, '_>, path: Path) -> Option<usize> {
    match path {
        Path::Every(_) => search.opened.keys().next_back().map(|at| at + 1),
        _ => (0..search.versions.len()).find(|at| search.get(*at).is_none()),
    }
}

pub(super) fn text(value: &str) -> Result<biodata::Text, RequestError> {
    biodata::Text::from_source(value)
        .map_err(|_| biomcp_fetch::bad_reply(Source::Drugsfda, Some(200)))
}

/// Fold the document reads' report into the Drugs@FDA source row.
fn merge(leg: Leg<'_>, out: &mut Outcome) {
    let report = status::source(Source::Drugsfda, leg.finish(), None);
    if let Some(row) = out
        .sources
        .iter_mut()
        .find(|s| s.source == Source::Drugsfda)
    {
        row.notices.extend(report.notices);
    }
}

/// An empty label-history reply for a get that found no application.
pub(super) fn empty(query: &DrugGet) -> Outcome {
    super::empty(DrugLabelHistory::into_reply(query.clone(), None))
}
