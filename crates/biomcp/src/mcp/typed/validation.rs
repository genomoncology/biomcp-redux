//! Validate the compact schema subset advertised by this adapter before CLI parsing.
use serde_json::Value;
pub(super) fn valid(schema: &Value, value: &Value) -> bool {
    if let Some(choices) = schema.get("enum").and_then(Value::as_array)
        && !choices.contains(value)
    {
        return false;
    }
    if let Some(expected) = schema.get("const")
        && value != expected
    {
        return false;
    }
    for key in ["anyOf", "oneOf"] {
        if let Some(choices) = schema.get(key).and_then(Value::as_array) {
            let count = choices.iter().filter(|choice| valid(choice, value)).count();
            if count == 0 || key == "oneOf" && count != 1 {
                return false;
            }
        }
    }
    match schema.get("type").and_then(Value::as_str) {
        Some("null") => value.is_null(),
        Some("string") => value.is_string(),
        Some("boolean") => value.is_boolean(),
        Some("integer") => value.as_u64().is_some_and(|n| {
            schema
                .get("minimum")
                .and_then(Value::as_u64)
                .is_none_or(|min| n >= min)
                && schema
                    .get("maximum")
                    .and_then(Value::as_u64)
                    .is_none_or(|max| n <= max)
        }),
        Some("array") => value.as_array().is_some_and(|values| {
            schema
                .get("minItems")
                .and_then(Value::as_u64)
                .is_none_or(|min| values.len() as u64 >= min)
                && schema
                    .get("maxItems")
                    .and_then(Value::as_u64)
                    .is_none_or(|max| values.len() as u64 <= max)
                && values
                    .iter()
                    .all(|value| schema.get("items").is_none_or(|items| valid(items, value)))
        }),
        Some("object") => value.as_object().is_some_and(|fields| {
            let properties = schema.get("properties");
            schema
                .get("required")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .all(|key| fields.contains_key(key))
                && fields.iter().all(|(key, value)| {
                    properties.and_then(|p| p.get(key)).map_or(
                        schema.get("additionalProperties") != Some(&Value::Bool(false)),
                        |property| valid(property, value),
                    )
                })
        }),
        None => true,
        _ => false,
    }
}
