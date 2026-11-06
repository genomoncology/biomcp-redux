//! Validate schema metadata independently from compatibility assertions.
use serde_json::Value;
type Check<T> = Result<T, String>;

pub(super) fn validate(object: &serde_json::Map<String, Value>) -> Check<()> {
    validate_cli_flags(object.get("x-cli-flags"))?;
    validate_enum_scope(object)?;
    if object
        .get("x-identifier")
        .is_some_and(|value| value != &Value::Bool(true))
    {
        return Err("unsupported identifier annotation".into());
    }
    if object.get("x-extensible-enum").is_some_and(|value| {
        !value
            .as_array()
            .is_some_and(|values| !values.is_empty() && values.iter().all(Value::is_string))
    }) {
        return Err("invalid extensible enum annotation".into());
    }
    if object.get("x-reply-query-titles").is_some_and(|value| {
        !value.as_array().is_some_and(|titles| {
            !titles.is_empty()
                && titles
                    .iter()
                    .all(|title| title.as_str().is_some_and(|title| !title.is_empty()))
        })
    }) {
        return Err("invalid reply query title annotation".into());
    }
    if object
        .get("x-section-rule")
        .is_some_and(|value| value.as_str().is_none_or(str::is_empty))
    {
        return Err("unsupported section-rule annotation".into());
    }
    if object
        .get("x-trailing-section")
        .is_some_and(|value| value != &Value::Bool(true))
    {
        return Err("unsupported trailing-section annotation".into());
    }
    if object
        .get("x-comma-separated")
        .is_some_and(|v| v != &Value::Bool(true))
    {
        return Err("unsupported comma-separated annotation".into());
    }
    Ok(())
}

fn validate_cli_flags(flags: Option<&Value>) -> Check<()> {
    if let Some(flags) = flags {
        let flags = flags.as_object().ok_or("unsupported CLI flag annotation")?;
        if flags.is_empty()
            || flags.iter().any(|(name, value)| {
                !name.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
                    || !name.bytes().all(|b| b.is_ascii_lowercase() || b == b'-')
                    || value.as_str().is_none_or(str::is_empty)
            })
        {
            return Err("unsupported CLI flag annotation".into());
        }
    }
    Ok(())
}

fn validate_enum_scope(object: &serde_json::Map<String, Value>) -> Check<()> {
    let Some(scope) = object.get("x-enum-by-field") else {
        return Ok(());
    };
    let scope = scope.as_object().ok_or("invalid enum field scope")?;
    let field = scope.get("field").and_then(Value::as_str).ok_or("enum scope field")?;
    let default = scope.get("default").and_then(Value::as_str).ok_or("enum scope default")?;
    let choices = scope.get("choices").and_then(Value::as_object).ok_or("enum scope choices")?;
    let values = object.get("enum").and_then(Value::as_array).ok_or("enum scope values")?;
    if scope.len() != 3 || field.is_empty() || !choices.contains_key(default)
        || choices.iter().any(|(key, list)| {
            key.is_empty() || list.as_array().is_none_or(|list| {
                list.is_empty() || list.iter().any(|v| !v.is_string() || !values.contains(v))
            })
        })
    {
        return Err("invalid enum field scope".into());
    }
    Ok(())
}
