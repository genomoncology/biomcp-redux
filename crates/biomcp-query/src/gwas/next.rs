//! One source-order navigation policy for an association page.
use crate::Action;
use biomcp_types::{GwasAssociation, GwasGet, GwasPageQuery, GwasSearch, Request, StepKind};
use serde::de::DeserializeOwned;
use serde_json::json;
type Steps = Vec<(StepKind, Action)>;
pub(super) fn search(_query: &GwasSearch, rows: &[GwasAssociation], continuation: Option<&GwasPageQuery>, fresh: bool) -> Steps {
    let mut next = Vec::new();
    for row in rows {
        if let Some(get) = checked::<GwasGet>(json!({"kind":"association", "id":format!("association:{}", row.id)})) {
            push(&mut next, StepKind::Disambiguate, Request::GetGwas(get), fresh);
        }
    }
    if let Some(GwasPageQuery::Search(query)) = continuation {
        push(&mut next, StepKind::Page, Request::SearchGwas(query.clone()), fresh);
    }
    let before = next.len();
    let mut seen = std::collections::BTreeSet::new();
    for row in rows {
        let mut requests = Vec::new();
        if let Some(get) = checked::<GwasGet>(json!({"kind":"study","id":row.study})) { requests.push(Request::GetGwas(get)); }
        for allele in &row.alleles {
            if let Some(get) = checked(json!({"input":allele.rs_id})) { requests.push(Request::GetVariant(get)); }
        }
        if let Some(publication) = &row.publication
            && let Some(get) = checked(json!({"input":[publication]})) { requests.push(Request::GetArticle(get)); }
        for request in requests {
            let key = serde_json::to_string(&request).ok();
            if next.len() - before < 3 && key.is_some_and(|key| seen.insert(key)) {
                push(&mut next, StepKind::Pivot, request, fresh);
            }
        }
    }
    next
}
fn checked<T: DeserializeOwned>(value: serde_json::Value) -> Option<T> { serde_json::from_value(value).ok() }
fn push(next: &mut Steps, kind: StepKind, request: Request, fresh: bool) {
    next.push((kind, Action::Run { request, fresh }));
}
