//! Feature maude-reports: compact output observations keep native facts and component labels.
use serde_json::{Value, json};
use std::io::Write;
pub(super) fn observe(value: &Value) -> Result<(), String> {
    let full = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    let mut comparable = value.clone();
    if let Some(outcomes) = comparable.get_mut("sources").and_then(Value::as_array_mut) {
        for row in outcomes {
            let fields = row.as_object_mut().ok_or("source outcome")?;
            let source: biomcp_types::Source =
                serde_json::from_value(fields.get("source").ok_or("source identity")?.clone())
                    .map_err(|e| e.to_string())?;
            let terms = biomcp_adapters::registry::terms(source).ok_or("source declaration")?;
            if fields.get("use") != Some(&json!(terms.usage))
                || fields.get("terms") != Some(&json!(terms.name))
                || fields.get("terms_url") != Some(&json!(terms.url))
            {
                return Err("source terms changed before budget comparison".into());
            }
            for key in ["use", "terms", "terms_url"] {
                fields.remove(key).ok_or("source terms field")?;
            }
        }
    }
    let bytes = serde_json::to_vec(&comparable)
        .map_err(|e| e.to_string())?
        .len();
    let schema = value
        .get("schema")
        .and_then(Value::as_str)
        .ok_or("schema identity")?;
    writeln!(
        std::io::stderr(),
        "MAUDE_OUTPUT {schema} full={full} comparable={bytes}"
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
