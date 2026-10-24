//! Match completed endpoint replies, preserving retry and cache evidence.
use super::drift;
use biomcp_fetch::Observation;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
const MAX_CHANGES: usize = 16;
const MAX_EXCHANGES: usize = 32;

pub(crate) fn compare(
    before: &[Observation], after: &[Observation], changes: &mut Vec<String>,
    sources: &mut BTreeMap<String, Value>,
) -> Value {
    for item in after.iter().filter(|item| !item.cached) {
        let name = serde_json::to_value(item.source).unwrap_or(Value::Null)
            .as_str().unwrap_or("unknown").to_owned();
        let totals = sources.entry(name).or_insert(json!({"attempts": 0, "bytes": 0, "elapsed_ms": 0}));
        for (key, value) in [("attempts", 1), ("bytes", item.bytes), ("elapsed_ms", item.elapsed_ms)] {
            let previous = totals.get(key).and_then(Value::as_u64).unwrap_or(0);
            if let Some(slot) = totals.get_mut(key) { *slot = json!(previous.saturating_add(value)); }
        }
    }
    let mut evidence = Vec::new();
    let mut used = BTreeSet::new();
    let mut found = Vec::new();
    // A retry is evidence, never the successful endpoint contract.
    for old in terminal(before) {
        let exact = terminal(after).into_iter().find(|item| item.identity == old.identity);
        let paired = exact.or_else(|| terminal(after).into_iter().find(|item|
            item.source == old.source && item.role == old.role && !used.contains(&item.identity)));
        let kind = match paired {
            Some(item) => {
                used.insert(item.identity.clone());
                if item.identity != old.identity { "request-plan-changed" }
                else if item.cached { "cache-reused" } else { "same-request" }
            }
            None => "request-plan-removed",
        };
        if let Some(item) = paired {
            if item.status != old.status {
                found.push(format!("{}: HTTP status {:?} -> {:?}", old.identity, old.status, item.status));
            } else if !item.cached && item.identity == old.identity {
                found.extend(drift(&old.shape, &item.shape).into_iter().map(|s| format!("{}: {s}", item.identity)));
            }
        }
        if kind.starts_with("request-plan") { found.push(format!("{}: {kind}", old.identity)); }
        evidence.push(json!({"kind": kind, "source": old.source, "role": old.role,
            "before_identity": old.identity, "after_identity": paired.map(|item| &item.identity),
            "before_statuses": statuses(before, &old.identity),
            "after_statuses": paired.map(|item| statuses(after, &item.identity))}));
    }
    for item in terminal(after).into_iter().filter(|item| !before.iter().any(|old| old.identity == item.identity)) {
        if !used.contains(&item.identity) {
            found.push(format!("{}: request-plan-added", item.identity));
            evidence.push(json!({"kind": "request-plan-added", "source": item.source, "role": item.role,
                "after_identity": item.identity, "after_statuses": statuses(after, &item.identity)}));
        }
    }
    changes.extend(found);
    let omitted = changes.len().saturating_sub(MAX_CHANGES);
    changes.truncate(MAX_CHANGES);
    if omitted > 0 { changes.push(format!("{omitted} more drift details omitted")); }
    let omitted = evidence.len().saturating_sub(MAX_EXCHANGES);
    evidence.truncate(MAX_EXCHANGES);
    json!({"items": evidence, "omitted": omitted})
}
fn terminal(items: &[Observation]) -> Vec<&Observation> {
    let mut result: Vec<&Observation> = Vec::new();
    for item in items {
        if let Some(index) = result.iter().position(|old| old.identity == item.identity) {
            result[index] = item;
        } else { result.push(item); }
    }
    result
}
fn statuses(items: &[Observation], identity: &str) -> Value {
    let matching: Vec<_> = items.iter().filter(|item| item.identity == identity).collect();
    json!({"values": matching.iter().take(16).map(|item| item.status).collect::<Vec<_>>(),
        "omitted": matching.len().saturating_sub(16)})
}
