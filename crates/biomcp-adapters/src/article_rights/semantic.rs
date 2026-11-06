//! Citation contexts belong to the citing paper; abstracts follow source terms.
use super::{licenses, permitted, pmid};
use biodata::{License, Pmid};
use biomcp_fetch::{BadReply, Leg};
use biomcp_types::Use;
use serde_json::Value;
use std::collections::BTreeMap;

pub(crate) fn filter(
    leg: &Leg<'_>,
    body: &[u8],
    parent: Option<Pmid>,
) -> Result<Vec<u8>, BadReply> {
    if !leg.disabled_use(Use::PerRecord) {
        return Ok(body.to_vec());
    }
    let root: Value = serde_json::from_slice(body).map_err(|_| BadReply)?;
    if !has_text(&root) {
        return Ok(body.to_vec());
    }
    let mut ids = Vec::new();
    if let Some(parent) = parent {
        ids.push(parent);
    }
    collect(&root, &mut ids);
    let licenses = if ids.is_empty() {
        BTreeMap::new()
    } else {
        licenses(leg, &ids)
    };
    filter_with_licenses(root, parent, &licenses)
}
pub(super) fn filter_with_licenses(
    mut root: Value,
    parent: Option<Pmid>,
    licenses: &BTreeMap<Pmid, License>,
) -> Result<Vec<u8>, BadReply> {
    apply(&mut root, parent, licenses);
    serde_json::to_vec(&root).map_err(|_| BadReply)
}
fn id(value: &Value) -> Option<Pmid> {
    value
        .pointer("/externalIds/PubMed")
        .and_then(pmid)
        .or_else(|| value.get("pmid").and_then(pmid))
        .or_else(|| {
            value
                .pointer("/header/external_ids")
                .and_then(Value::as_array)
                .and_then(|ids| {
                    ids.iter().find_map(|id| {
                        let namespace = id["namespace"].as_str()?;
                        if !namespace.eq_ignore_ascii_case("pubmed") {
                            return None;
                        }
                        id["id"].as_str()?.rsplit(':').next()?.parse().ok()
                    })
                })
        })
}
fn collect(value: &Value, ids: &mut Vec<Pmid>) {
    if let Some(id) = id(value) {
        ids.push(id);
    }
    match value {
        Value::Object(object) => {
            for child in object.values() {
                collect(child, ids);
            }
        }
        Value::Array(rows) => {
            for child in rows {
                collect(child, ids);
            }
        }
        _ => {}
    }
}
fn apply(value: &mut Value, inherited: Option<Pmid>, licenses: &BTreeMap<Pmid, License>) {
    // A nested paper owns its text even when it has no resolvable PMID.
    let own = if value.get("paperId").is_some() || value.get("externalIds").is_some() {
        id(value)
    } else {
        id(value).or(inherited)
    };
    let citing = value.get("citingPaper").map_or(own, id);
    let license = own.and_then(|id| licenses.get(&id).copied());
    match value {
        Value::Object(object) => {
            if !permitted(license, true) {
                object.remove("abstract");
                object.remove("abstract_text");
            }
            let context_license = citing.and_then(|id| licenses.get(&id).copied());
            object.remove("biomcpContextLicense");
            if object.contains_key("contexts")
                && let Some(value) =
                    context_license.and_then(|license| serde_json::to_value(license).ok())
            {
                object.insert("biomcpContextLicense".into(), value);
            }
            if !permitted(context_license, false)
                && object.get("contexts").is_some_and(has_contexts)
            {
                let notice =
                    citing
                        .map(super::withheld)
                        .unwrap_or(biomcp_types::WithheldArticleText {
                            reason: biomcp_types::Reason::Disabled,
                            url: None,
                        });
                if let Ok(notice) = serde_json::to_value(notice) {
                    object.insert("biomcpTextWithheld".into(), notice);
                }
                object.remove("contexts");
            }
            for child in object.values_mut() {
                apply(child, own, licenses);
            }
        }
        Value::Array(rows) => {
            for child in rows {
                apply(child, own, licenses);
            }
        }
        _ => {}
    }
}

fn has_text(value: &Value) -> bool {
    match value {
        Value::Object(object) => {
            ["abstract", "abstract_text"]
                .iter()
                .any(|key| object.get(*key).is_some_and(|value| !value.is_null()))
                || object.get("contexts").is_some_and(has_contexts)
                || object.values().any(has_text)
        }
        Value::Array(rows) => rows.iter().any(has_text),
        _ => false,
    }
}

fn has_contexts(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Array(items) => !items.is_empty(),
        Value::Object(object) => !object.is_empty(),
        Value::String(text) => !text.is_empty(),
        _ => true,
    }
}
