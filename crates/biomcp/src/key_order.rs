//! Name order for JSON that readers and command words expect alphabetical.
//!
//! serde_json keeps insertion order (0972) so MCP replies list fields as CLI JSON does.
//! Markdown, query fields and command words kept their alphabetical order before that.
use serde_json::{Map, Value};

/// A copy whose objects list their names alphabetically, at every depth.
pub(crate) fn sorted(value: &Value) -> Value {
    match value {
        Value::Object(fields) => {
            let mut names: Vec<_> = fields.iter().collect();
            names.sort_unstable_by(|left, right| left.0.cmp(right.0));
            Value::Object(
                names
                    .into_iter()
                    .map(|(name, field)| (name.clone(), sorted(field)))
                    .collect::<Map<_, _>>(),
            )
        }
        Value::Array(items) => Value::Array(items.iter().map(sorted).collect()),
        _ => value.clone(),
    }
}

