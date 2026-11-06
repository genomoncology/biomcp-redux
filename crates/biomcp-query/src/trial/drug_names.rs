//! Substitute source-stated drug names without changing the caller's other filters.
use crate::{Ctx, Outcome, drug::search_names};
use biodata::Text;
use biomcp_types::TrialSearch;

pub(super) fn expand(cx: &Ctx<'_>, query: &TrialSearch, out: &mut Outcome) -> TrialSearch {
    if query.exact_term() || cx.fetch.disabled(query.source().source()) {
        return query.clone();
    }
    let text = query.query().map(|input| expand_term(cx, input, out));
    let interventions = query
        .intervention()
        .iter()
        .map(|input| expand_term(cx, input, out))
        .collect();
    let keywords = query
        .keyword()
        .iter()
        .map(|input| expand_term(cx, input, out))
        .collect();
    query.with_drug_terms(text, interventions, keywords)
}
fn expand_term(cx: &Ctx<'_>, input: &Text, out: &mut Outcome) -> Text {
    let names = search_names::resolve(cx, input, out);
    if names.len() > 1 {
        search_names::alternatives(&names).unwrap_or_else(|| input.clone())
    } else {
        input.clone()
    }
}
