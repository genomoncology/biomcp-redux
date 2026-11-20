//! Cross-source aliases for an empty drug search and ChEMBL's native synonym fallback.
use super::{hint, lookup_deadline, notice, recovery, usable, with_aliases, within};
use crate::{Ctx, Outcome, status};
use biodata::Text;
use biomcp_types::{DrugNameSelector, DrugSearch, DrugSource, Request};

pub(crate) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let deadline = lookup_deadline(cx);
    let mut out = search_spelling(cx, query);
    // --exact-term searches the typed text only, so no other source's name stands in for it.
    if query.exact_term() || query.offset() != 0 || !zero(&out, query.source()) {
        return out;
    }
    let Some(input) = query.query() else {
        return out;
    };
    let (found, stated, evidence) = with_aliases(
        &within(cx, deadline.remaining()),
        input,
        query.source(),
        |stated, name, _| {
            if query.explicit_name_selector() {
                return None;
            }
            let retry = query.with_name(name.clone());
            let mut found = search_spelling(&within(cx, deadline.remaining()), &retry);
            if zero(&found, query.source()) {
                return None;
            }
            notice(
                &mut found,
                query.source(),
                format!(
                    "The search for {:?} returned no rows. {} record {} supplies the name {:?}; \
                     tried the selected source with that name. \
                     Returned rows and totals belong to the selected source.",
                    input.as_str(),
                    stated.source,
                    stated.id,
                    name.as_str()
                ),
            );
            Some(found)
        },
    );
    if let Some(mut found) = found {
        evidence.append(&mut found, query.source());
        if evidence.lookup_failed() {
            status::search(&mut found);
            recovery(&mut found, Request::SearchDrug(query.clone()));
        }
        return found;
    }
    evidence.append(&mut out, query.source());
    if evidence.lookup_failed() {
        status::search(&mut out);
        recovery(&mut out, Request::SearchDrug(query.clone()));
    } else if let Some(stated) = stated {
        hint(&mut out, &stated, input, query.source(), cx.fresh);
    }
    out
}

fn zero(out: &Outcome, source: DrugSource) -> bool {
    usable(out, source)
        && !out.dropped
        && out
            .page
            .as_ref()
            .is_some_and(|page| page.total == 0 && page.returned == 0)
}

fn search_spelling(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let deadline = lookup_deadline(cx);
    let mut out = super::super::search(cx, query);
    if query.source() != DrugSource::Chembl
        || query.explicit_name_selector()
        || query.offset() != 0
        || !zero(&out, query.source())
    {
        return out;
    }
    let Some(synonym) = crate::next::checked(
        query.with_name_selector(DrugNameSelector::Synonym),
        query.source().source(),
        &mut out.problems,
    ) else {
        return out;
    };
    let mut found = super::super::search(&within(cx, deadline.remaining()), &synonym);
    notice(
        &mut found,
        query.source(),
        format!(
            "The preferred-name search for {:?} returned no rows; tried the native exact synonym filter.",
            query.query().map_or("", Text::as_str)
        ),
    );
    found
}
