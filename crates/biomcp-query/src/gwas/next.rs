//! One source-order navigation policy for an association page.
use crate::Action;
use biomcp_types::{GwasPageQuery, GwasSearch, Request, StepKind};
use serde_json::json;
type Steps = Vec<(StepKind, Action)>;
pub(super) fn search(
    query: &GwasSearch,
    page: Option<&biomcp_types::GwasPage>,
    state: biomcp_types::SourceState,
    fresh: bool,
) -> Steps {
    let mut next = Vec::new();
    let rows = page.map(|page| page.items.as_ref()).unwrap_or_default();
    if let Some(GwasPageQuery::Search(query)) = page.and_then(|page| page.continuation.as_ref()) {
        push(
            &mut next,
            StepKind::Page,
            Request::SearchGwas(query.clone()),
            fresh,
        );
    }
    let before = next.len();
    let mut seen = std::collections::BTreeSet::new();
    for row in rows {
        let mut requests = Vec::new();
        for allele in &row.alleles {
            if let Ok(get) = serde_json::from_value(json!({"input":allele.rs_id})) {
                requests.push(Request::GetVariant(get));
            }
        }
        if let Some(publication) = &row.publication
            && let Ok(get) = serde_json::from_value(json!({"input":[publication]}))
        {
            requests.push(Request::GetArticle(get));
        }
        for request in requests {
            let key = serde_json::to_string(&request).ok();
            if next.len() - before < 3 && key.is_some_and(|key| seen.insert(key)) {
                push(&mut next, StepKind::Pivot, request, fresh);
            }
        }
    }
    if matches!(
        state,
        biomcp_types::SourceState::Failed | biomcp_types::SourceState::Stale
    ) {
        push(
            &mut next,
            StepKind::Fresh,
            Request::SearchGwas(query.clone()),
            true,
        );
    }
    next
}
fn push(next: &mut Steps, kind: StepKind, request: Request, fresh: bool) {
    next.push((kind, Action::Run { request, fresh }));
}
