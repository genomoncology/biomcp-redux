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
    let own = id(value).or(inherited);
    let citing = value.get("citingPaper").and_then(id).or(own);
    let license = own.and_then(|id| licenses.get(&id).copied());
    match value {
        Value::Object(object) => {
            if !permitted(license, true) {
                object.remove("abstract");
                object.remove("abstract_text");
            }
            let context_license = citing.and_then(|id| licenses.get(&id).copied());
            object.remove("contextLicense");
            if object.contains_key("contexts") {
                if let Some(license) = context_license {
                    if let Ok(value) = serde_json::to_value(license) {
                        object.insert("contextLicense".into(), value);
                    }
                }
            }
            if !permitted(context_license, false) {
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
