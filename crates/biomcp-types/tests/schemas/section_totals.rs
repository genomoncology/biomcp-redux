//! Every counted section states the `total` the skill tells agents to read (1010, 1022).
use super::registry;
use serde_json::Value;

// 1010: the skill tells agents to read `total`; a source's own pre-selection count
// (`native_total`) never stands in for it.
fn native_totals_without_total(node: &Value, path: &str, found: &mut Vec<String>) {
    if let Some(properties) = node.get("properties").and_then(Value::as_object)
        && properties.contains_key("native_total")
        && !properties.contains_key("total")
    {
        found.push(path.to_owned());
    }
    match node {
        Value::Object(values) => values.iter().for_each(|(key, value)| {
            native_totals_without_total(value, &format!("{path}/{key}"), found)
        }),
        Value::Array(values) => values
            .iter()
            .for_each(|value| native_totals_without_total(value, &format!("{path}/[]"), found)),
        _ => {}
    }
}

pub(super) fn every_object_with_a_native_total_also_states_its_total() -> Result<(), String> {
    let mut planted = Vec::new();
    native_totals_without_total(
        &serde_json::json!({"$defs":{"X":{"properties":{"native_total":{}}}}}),
        "",
        &mut planted,
    );
    if planted.is_empty() {
        return Err("planted native_total without total was not found".into());
    }
    let mut found = Vec::new();
    for contract in registry() {
        native_totals_without_total(&(contract.generate)(), contract.id, &mut found);
    }
    if found.is_empty() {
        Ok(())
    } else {
        Err(format!("native_total without total: {found:?}"))
    }
}

// 1022: the skill says "Read its `total`", so a card section whose data holds rows carries
// its whole-set count at the top of that data, not only nested in `page` or `items`.
fn sections_with_rows_and_no_total(
    root: &Value,
    node: &Value,
    path: &str,
    found: &mut Vec<String>,
) {
    let data = node
        .get("properties")
        .filter(|p| p.pointer("/state/const") == Some(&Value::from("data")))
        .and_then(|p| p.pointer("/data/$ref"))
        .and_then(Value::as_str)
        .and_then(|r| r.strip_prefix('#'))
        .and_then(|r| root.pointer(r));
    if let Some(properties) = data.and_then(|d| d.get("properties"))
        && properties.get("items").is_some()
        && properties.get("total").is_none()
    {
        found.push(path.to_owned());
    }
    match node {
        Value::Object(values) => values.iter().for_each(|(key, value)| {
            sections_with_rows_and_no_total(root, value, &format!("{path}/{key}"), found)
        }),
        Value::Array(values) => values.iter().for_each(|value| {
            sections_with_rows_and_no_total(root, value, &format!("{path}/[]"), found)
        }),
        _ => {}
    }
}

pub(super) fn every_section_with_rows_states_its_total_at_the_top() -> Result<(), String> {
    let planted = serde_json::json!({
        "$defs": {
            "Rows": {"properties": {"page": {}, "items": {}}},
            "Section": {"oneOf": [{"properties": {
                "state": {"const": "data"}, "data": {"$ref": "#/$defs/Rows"}
            }}]}
        }
    });
    let mut found = Vec::new();
    sections_with_rows_and_no_total(&planted, &planted, "", &mut found);
    if found.is_empty() {
        return Err("planted section with rows and no total was not found".into());
    }
    let mut found = Vec::new();
    for contract in registry() {
        let schema = (contract.generate)();
        sections_with_rows_and_no_total(&schema, &schema, contract.id, &mut found);
    }
    if found.is_empty() {
        Ok(())
    } else {
        Err(format!("section rows without a top-level total: {found:?}"))
    }
}
