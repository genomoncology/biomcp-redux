//! Search current SPL labels by name or indication.
use super::{SOURCE, names, search_page};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::openfda_label as source;
use biomcp_types::{DrugPage, DrugSearch, ReplyBody};

pub(in crate::drug) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let mut out = super::super::empty(
        (DrugPage {
            items: Box::default(),
            total: None,
        })
        .with_query(query.clone()),
    );
    let command = cx.fetch.command_deadline(cx.budget);
    let mut deadline = command.deadline();
    // A name too long for one bounded request is searched as typed, as it always was.
    let mut expanded = if query.widens_drug_names()
        && (query.name_snapshot().is_some() || !cx.fetch.disabled(SOURCE))
        && names::typed(query).is_some()
    {
        match names::prepare(cx, query, &mut deadline) {
            Ok(Some(saved)) => Some(saved),
            Ok(None) => {
                super::super::widen::missing(query, &mut out);
                return out;
            }
            Err(error) => {
                out.local = Some(error);
                return out;
            }
        }
    } else {
        None
    };
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let discovery = if let Some(saved) = expanded.as_mut() {
        names::search(&leg, query, saved)
    } else {
        match (query.indication(), query.query()) {
            (Some(indication), name) => {
                source::search_indication(&leg, indication, name, query.limit(), query.offset())
            }
            (None, Some(name)) => source::search(&leg, name, query.limit(), query.offset()),
            (None, None) => return super::super::unsupported(query.source(), out),
        }
    };
    let result = discovery.page.and_then(|page| {
        out.problems.extend(page.problems);
        let items = search_page::headers(&page.window.rows, query)?;
        Ok(page.window.with_rows(items))
    });
    let (window, error) = match result {
        Ok(window) => (Some(window), None),
        Err(error) => (None, Some(error)),
    };
    out.local = error.and_then(status::local);
    let remaining = leg.command_remaining();
    out.sources
        .push(status::source(SOURCE, leg.finish(), error));
    let sent = super::super::widen::retain(
        cx,
        query,
        expanded.as_ref(),
        deadline,
        window.is_some(),
        &mut out,
    );
    disclosure(&mut out, discovery.free_text);
    if let Some(window) = window {
        let total = window.total().unwrap_or(0);
        search_page::apply(cx, &sent, window, total, &mut out);
    }
    status::search(&mut out);
    names::finish(query, expanded, &mut out);
    command.finish(remaining);
    out
}
fn disclosure(out: &mut Outcome, selector: Option<biodata::Text>) {
    if let Some(selector) = selector
        && let Some(source) = out.sources.first_mut()
    {
        source.term = Some(selector);
        source
            .notices
            .push(biomcp_types::SourceNotice::RequestGuidance {
            data_date: None,
            message:
                "Exact and word name searches found no matching product; searched label free text."
                    .into(),
            retry_after_seconds: None,
        });
    }
}
