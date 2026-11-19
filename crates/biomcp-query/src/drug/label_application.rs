//! An FDA application number gets the openFDA label filed under it, or lists its labels (1063).
use super::resolve::{carry, within};
use crate::Action;
use crate::{Ctx, Outcome, status};
use biomcp_types::{
    DrugGet, DrugSearch, DrugSection, DrugSource, QueryInvalid, ReplyData, Request, Source,
    SourceNotice, SourceState, StepKind, UsageError,
};

/// openFDA's largest page; a longer list pages on from the reply's next step.
const LABELS: u32 = 50;

pub(super) fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let mut out = get_parts(cx, std::slice::from_ref(query));
    out.pop()
        .unwrap_or_else(|| status::empty(ReplyData::Usage(UsageError::argument("input".into()))))
}

/// Get every part from the application's one label, or answer once with the labels to choose
/// from when it has none or several.
#[expect(
    clippy::disallowed_methods,
    reason = "Times nested commands, which charge their own command clocks; it opens no leg (1038)"
)]
pub(super) fn get_parts(cx: &Ctx<'_>, queries: &[DrugGet]) -> Vec<Outcome> {
    let Some(query) = queries.first() else {
        return Vec::new();
    };
    let deadline = cx.fetch.deadline(cx.budget);
    let search = match DrugSearch::named(query.input().clone(), DrugSource::OpenfdaLabel, LABELS) {
        Ok(search) => search,
        Err(_) => {
            return vec![status::empty(ReplyData::Usage(UsageError::argument(
                "input".into(),
            )))];
        }
    };
    let mut found = super::search(&within(cx, deadline.remaining()), &search);
    let label = match &found.body {
        ReplyData::DrugPage {
            body: Some(page), ..
        } if page.total == Some(1) => page.items.first().map(|label| label.id.clone()),
        _ => None,
    };
    let read = !found.dropped && found.sources.iter().all(|s| s.state == SourceState::Ok);
    let none = matches!(&found.body, ReplyData::DrugPage { body: Some(page), .. } if page.total == Some(0));
    let Some(label) = label.filter(|_| read) else {
        if read && none {
            no_label(cx, query, &mut found);
        } else {
            choices(queries, &mut found);
        }
        return vec![found];
    };
    let resolved = match queries
        .iter()
        .map(|query| query.with_native_id(label.clone()))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(resolved) => resolved,
        Err(_) => return vec![super::rejected_input(query.source(), "input", found)],
    };
    super::selected_parts(&within(cx, deadline.remaining()), &resolved)
        .into_iter()
        .map(|mut out| {
            carry(&found, &mut out);
            out
        })
        .collect()
}

/// Each label offered to choose from is offered as the read that was asked for: the same
/// sections, label headings and section paging, of that one label.
fn choices(queries: &[DrugGet], found: &mut Outcome) {
    let Some(query) = queries.first() else {
        return;
    };
    let headings: Vec<&str> = queries
        .iter()
        .filter_map(|query| query.label_section())
        .map(|heading| heading.as_str())
        .collect();
    for step in &mut found.next {
        if step.0 != StepKind::Disambiguate {
            continue;
        }
        if let Action::Run {
            request: Request::GetDrug(get),
            ..
        } = &mut step.1
        {
            // The search just offered this SPL set, so the same read of it is a valid request.
            match asked(query, &headings, get) {
                Ok(read) => *get = read,
                Err(_) => continue,
            }
        }
    }
}

fn asked(query: &DrugGet, headings: &[&str], label: &DrugGet) -> Result<DrugGet, QueryInvalid> {
    let read = query.with_native_id(label.native_input()?)?;
    match headings {
        [_, _, ..] => read.for_label(
            biodata::Text::new(&headings.join(","))
                .map_err(|_| QueryInvalid::new(&["label_section"]))?,
        ),
        _ => Ok(read),
    }
}

/// openFDA lists no label under the application: say so, and offer the application's
/// Drugs@FDA approvals in place of a step that would repeat this read.
fn no_label(cx: &Ctx<'_>, query: &DrugGet, found: &mut Outcome) {
    status::item(found, false, false, false);
    if let Some(source) = found.sources.first_mut() {
        source.notices.push(SourceNotice::RequestGuidance {
            data_date: None,
            message: "openFDA lists no label under this application number.".into(),
            retry_after_seconds: None,
        });
    }
    found.next.clear();
    let approvals = query
        .native_input()
        .and_then(|id| DrugGet::new(id, DrugSource::Drugsfda))
        .and_then(|get| get.for_section(DrugSection::Approvals));
    if let Some(approvals) = crate::next::checked(approvals, Source::Drugsfda, &mut found.problems)
    {
        found.next.push(crate::next::Step::run(
            StepKind::Section,
            &[Source::Drugsfda],
            Request::GetDrug(approvals),
            cx.fresh,
        ));
    }
}
