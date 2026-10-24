//! Required publisher provenance is measured separately from unchanged payload caps.
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("source-controls/outcomes-retain-registry-terms", || {
            envelopes().map_err(Into::into)
        }),
        Trial::test("runner/terms-subtraction-keeps-payload-ceilings", || {
            subtraction().map_err(Into::into)
        }),
    ]
}

fn strip_row(row: &mut Value) -> Result<(), String> {
    let outcome: biomcp_types::SourceOutcome = serde_json::from_value(row.clone()).map_err(|e| e.to_string())?;
    let source = outcome.source;
    let spec = biomcp_adapters::registry::SOURCES.iter()
        .find(|spec| spec.source == source).ok_or("unregistered outcome source")?;
    let object = row.as_object_mut().ok_or("source outcome is not an object")?;
    for (field, expected) in [
        ("use", json!(spec.terms.usage)),
        ("terms", json!(spec.terms.name)),
        ("terms_url", json!(spec.terms.url)),
    ] {
        if object.remove(field) != Some(expected) {
            return Err(format!("{source}: missing or changed registry {field}"));
        }
    }
    Ok(())
}

/// Validate every source outcome and remove exactly its three new registry fields.
pub(super) fn comparable(value: &Value, pointer: &str) -> Result<Value, String> {
    let mut result = value.clone();
    if pointer.is_empty() {
        if let Some(rows) = result.get_mut("sources").and_then(Value::as_array_mut) {
            for row in rows { strip_row(row)?; }
        }
    } else if pointer == "/sources" {
        for row in result.as_array_mut().ok_or("sources is not an array")? { strip_row(row)?; }
    } else if pointer.strip_prefix("/sources/").is_some_and(|p| p.parse::<usize>().is_ok()) {
        strip_row(&mut result)?;
    }
    Ok(result)
}

pub(super) fn size(value: &Value, pointer: &str) -> Result<usize, String> {
    serde_json::to_vec(&comparable(value, pointer)?).map(|b| b.len()).map_err(|e| e.to_string())
}

pub(super) fn raw_size(bytes: &[u8]) -> Result<usize, String> {
    let Ok(value) = serde_json::from_slice::<Value>(bytes) else { return Ok(bytes.len()); };
    let compact = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
    Ok(bytes.len() - (compact - size(&value, "")?))
}

fn envelopes() -> Result<(), String> {
    let registry = super::schema::Registry::embedded()?;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/cases");
    for case in ["gene/braf.yaml", "article/get/card.yaml", "trial/condition.yaml", "gene/failure-500.yaml"] {
        let value = super::run_case(&root.join(case), &registry)?;
        let rows = value.get("sources").and_then(Value::as_array).ok_or("missing source outcomes")?;
        if rows.is_empty() { return Err("no source outcomes tested".into()); }
        let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
        let payload = size(&value, "")?;
        use std::io::Write;
        writeln!(std::io::stdout(), "{case}: actual={full}, new_terms={}, comparable={payload}", full - payload).map_err(|e| e.to_string())?;
    }
    let schema: Value = serde_json::from_str(include_str!("../../../biomcp-types/schemas/source.json"))
        .map_err(|e| e.to_string())?;
    for name in schema.get("enum").and_then(Value::as_array).ok_or("missing source enum")? {
        let source = serde_json::from_value(name.clone()).map_err(|e| e.to_string())?;
        let outcome = biomcp_adapters::registry::outcome(source, biomcp_types::SourceState::Ok, None);
        let mut value = serde_json::to_value(outcome).map_err(|e| e.to_string())?;
        strip_row(&mut value)?;
    }
    Ok(())
}

fn subtraction() -> Result<(), String> {
    let outcome = biomcp_adapters::registry::outcome(biomcp_types::Source::Hgnc, biomcp_types::SourceState::Ok, None);
    let mut envelope = json!({"sources":[outcome], "result":{"value":1}});
    let cap = size(&envelope, "")?;
    envelope.get_mut("result").and_then(Value::as_object_mut).ok_or("missing result")?.insert("added".into(), json!("ordinary payload"));
    if size(&envelope, "")? <= cap { return Err("ordinary payload earned a terms deduction".into()); }
    envelope.pointer_mut("/sources/0").and_then(Value::as_object_mut).ok_or("missing source")?.insert("terms".into(), json!("wrong terms"));
    if size(&envelope, "").is_ok() { return Err("malformed terms earned a deduction".into()); }
    envelope.pointer_mut("/sources/0").and_then(Value::as_object_mut).ok_or("missing row")?.remove("terms");
    if size(&envelope, "").is_ok() { return Err("missing terms earned a deduction".into()); }
    Ok(())
}
