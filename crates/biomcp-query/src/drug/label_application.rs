//! An FDA application number gets the openFDA label filed under it, or lists its labels (1063).
use super::resolve::{carry, within};
use crate::Action;
use crate::{Ctx, Outcome, status};
use biomcp_types::{
    DrugGet, DrugSearch, DrugSection, DrugSource, ReplyData, Request, SourceState, StepKind,
    UsageError,
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
    let Some(label) = label
        .filter(|_| !found.dropped && found.sources.iter().all(|s| s.state == SourceState::Ok))
    else {
        if query.sections().contains(&DrugSection::Label) {
            read_labels(&mut found);
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

/// Each label offered to choose from is offered as a label read, since a label was asked for.
fn read_labels(found: &mut Outcome) {
    for step in &mut found.next {
        if step.0 != StepKind::Disambiguate {
            continue;
        }
        if let Action::Run {
            request: Request::GetDrug(get),
            ..
        } = &mut step.1
        {
            // A label read of an SPL set the search just offered is always a valid request.
            match get.for_section(DrugSection::Label) {
                Ok(label) => *get = label,
                Err(_) => continue,
            }
        }
    }
}
