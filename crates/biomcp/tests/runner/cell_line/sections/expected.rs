//! Expected native facts come from unchanged publisher fixtures.
use super::*;
fn rows(value: &Value, key: &str) -> Vec<Value> {
    value
        .get(key)
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}
fn xref(value: &Value) -> Value {
    let mut value = value.clone();
    if let Some(object) = value.as_object_mut() {
        object.remove("discontinued");
    }
    value
}
fn variation(value: &Value) -> Value {
    let mut result = json!({"variation_type":value["variation-type"], "description":value["mutation-description"], "genes":[], "references":[], "provenance":[]});
    for (source, target) in [
        ("mutation-type", "mutation_type"),
        ("zygosity-type", "zygosity"),
        ("variation-note", "note"),
    ] {
        if let Some(field) = value.get(source) {
            result[target] = field.clone();
        }
    }
    let mut genes = Vec::new();
    let mut references = Vec::new();
    for reference in rows(value, "xref-list") {
        if reference.get("database") == Some(&json!("HGNC")) {
            genes.push(json!({"reference":xref(&reference), "hgnc":reference["accession"]}));
        } else {
            references.push(xref(&reference));
        }
    }
    result["genes"] = json!(genes);
    result["references"] = json!(references);
    result["provenance"] = json!(
        rows(value, "source-list")
            .into_iter()
            .map(|mut source| {
                if let Some(reference) = source.get_mut("reference")
                    && let Some(object) = reference.as_object_mut()
                    && let Some(value) = object.remove("resource-internal-ref")
                {
                    object.insert("resource_internal_ref".into(), value);
                }
                source
            })
            .collect::<Vec<_>>()
    );
    result
}
pub(super) fn page(
    source: &Value,
    section: &str,
    value: &Value,
    offset: usize,
    limit: usize,
) -> Result<(), String> {
    path(value, "/result/offset", json!(offset))?;
    if section == "identity" {
        for (native, field) in [("species-list", "species"), ("disease-list", "diseases")] {
            collection(value, field, rows(source, native), offset, limit)?;
        }
        let warnings = rows(source, "comment-list")
            .into_iter()
            .filter(|r| {
                matches!(
                    r.get("category").and_then(Value::as_str),
                    Some("Caution" | "Problematic cell line")
                )
            })
            .map(|r| json!({"scope":r["category"],"text":r["value"]}))
            .collect();
        return collection(value, "warnings", warnings, offset, limit);
    }
    let items = match section {
        "names" => rows(source, "name-list")
            .into_iter()
            .map(|r| json!({"kind":r["type"],"value":r["value"]}))
            .collect(),
        "xrefs" => rows(source, "xref-list").iter().map(xref).collect(),
        "variants" => rows(source, "sequence-variation-list")
            .iter()
            .map(variation)
            .collect(),
        _ => return Err("unknown section".into()),
    };
    collection(value, "items", items, offset, limit)
}
fn collection(
    value: &Value,
    field: &str,
    rows: Vec<Value>,
    offset: usize,
    limit: usize,
) -> Result<(), String> {
    path(value, &format!("/result/{field}/total"), json!(rows.len()))?;
    path(
        value,
        &format!("/result/{field}/items"),
        json!(
            rows.into_iter()
                .skip(offset)
                .take(limit)
                .collect::<Vec<_>>()
        ),
    )
}
