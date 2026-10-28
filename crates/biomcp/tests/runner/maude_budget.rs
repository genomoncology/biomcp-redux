//! Feature maude-reports: synthetic output ceilings retain native facts and component labels.
use serde_json::{Value, json};
pub(super) fn check(value: &Value) -> Result<(), String> {
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
    let caps = ceiling(value, schema).ok_or("unbudgeted synthetic MAUDE schema")?;
    if full > caps.0 || bytes > caps.1 {
        return Err(format!(
            "synthetic MAUDE budget exceeded for {schema}: {full}/{bytes} > {caps:?}"
        ));
    }
    Ok(())
}

// j13034 observed each class maximum; ceilings add 15 percent and round up.
fn ceiling(value: &Value, schema: &str) -> Option<(usize, usize)> {
    if value.get("error").is_some() {
        return Some(if value.get("next").is_some() {
            (250, 250)
        } else {
            // j19028: typed argument names, 218 compact bytes plus 15 percent.
            (251, 251)
        });
    }
    if value.get("status") == Some(&json!("failed")) {
        return match schema {
            "biomcp.adverse-event-page/3" => Some((364, 232)),
            "biomcp.adverse-event-maude-card/1" => Some((472, 340)),
            "biomcp.adverse-event-devices/1" => Some((481, 349)),
            "biomcp.adverse-event-narrative/1" => Some((500, 367)),
            _ => None,
        };
    }
    if value.get("status") == Some(&json!("absent")) {
        return Some((733, 601));
    }
    match schema {
        "biomcp.adverse-event-page/3" => Some((948, 816)),
        "biomcp.adverse-event-maude-card/1" => Some((1497, 1364)),
        "biomcp.adverse-event-devices/1" => Some((1241, 1109)),
        "biomcp.adverse-event-narrative/1" => Some((912, 780)),
        _ => None,
    }
}
