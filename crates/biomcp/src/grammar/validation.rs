//! Derive accepted values and forms from the command's owning schema.
use biomcp_types::{QueryInvalid, UsageArguments};
use serde_json::Value;

pub(super) fn schema(verb: &str, entity: &str) -> Option<Value> {
    let id = super::command_paths()
        .iter()
        .find(|(v, e, _)| *v == verb && *e == entity)?
        .2;
    serde_json::from_str(crate::schema::find(id)?).ok()
}

pub(super) fn checked(
    verb: &str,
    entity: &str,
    value: &Value,
    cause: &QueryInvalid,
) -> Option<UsageArguments> {
    use biomcp_types::ErrorMessage;
    let schema = schema(verb, entity)?;
    let properties = schema.get("properties")?;
    let named: Vec<_> = cause
        .arguments()
        .iter()
        .filter_map(|name| Some((name.as_ref(), properties.get(name.as_ref())?)))
        .collect();
    let &(name, property) = named.first()?;
    let given = |name: &str| value.get(name).is_some_and(|value| !value.is_null());
    let options = || {
        named
            .iter()
            .map(|(name, property)| biodata::Text::new(&identity(name, property)).ok())
            .collect::<Option<Vec<_>>>()
            .map(Into::into)
    };
    // Several named options mean the refusal is about how they go together.
    let (message, valid_values) = if let Some(message) = cause.message() {
        (message, None)
    } else if !given(name) {
        (ErrorMessage::ArgumentMissing, None)
    } else if named.len() > 1 {
        (ErrorMessage::ArgumentCombination, Some(options()?))
    } else {
        (ErrorMessage::ArgumentValue, None)
    };
    Some(UsageArguments {
        argument: Some(identity(name, property).into()),
        message: Some(message),
        valid_values: valid_values
            .or_else(|| choices(name).or_else(|| accepted(&schema, property))),
        next: vec![help([verb, entity])],
    })
}

/// How a caller writes an argument: a positional by its name, an option as its flag.
fn identity(name: &str, property: &Value) -> String {
    if property.get("x-positional").is_some() {
        name.into()
    } else {
        format!("--{}", name.replace('_', "-"))
    }
}

/// Name the real choices of a free-text argument whose schema carries only a type description.
fn choices(name: &str) -> Option<Box<[biodata::Text]>> {
    let values: &[&str] = match name {
        "label_section" => &biomcp_types::OPENFDA_LABEL_HEADINGS,
        _ => return None,
    };
    values
        .iter()
        .map(|value| biodata::Text::new(value).ok())
        .collect::<Option<Vec<_>>>()
        .map(Into::into)
}

/// The help call for a command path; a path already under help is not prefixed twice.
pub(super) fn help<'a>(path: impl IntoIterator<Item = &'a str>) -> biomcp_types::Step {
    let mut path = path.into_iter().peekable();
    path.next_if_eq(&"help");
    biomcp_types::Step {
        kind: biomcp_types::StepKind::Configure,
        args: std::iter::once("help")
            .chain(path)
            .map(Box::from)
            .collect::<Vec<_>>()
            .into(),
        options: Default::default(),
    }
}

pub(super) fn resolve<'a>(schema: &'a Value, property: &'a Value) -> &'a Value {
    if let Some(reference) = property.get("$ref").and_then(Value::as_str) {
        return schema
            .pointer(reference.trim_start_matches('#'))
            .unwrap_or(property);
    }
    if let Some(choices) = property.get("anyOf").and_then(Value::as_array)
        && let Some(choice) = choices
            .iter()
            .find(|choice| choice.get("type").and_then(Value::as_str) != Some("null"))
    {
        return resolve(schema, choice);
    }
    if let Some(items) = property.get("items") {
        return resolve(schema, items);
    }
    property
}

pub(super) fn accepted(schema: &Value, property: &Value) -> Option<Box<[biodata::Text]>> {
    let value = resolve(schema, property);
    let forms = if let Some(values) = value.get("enum").and_then(Value::as_array) {
        values
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect()
    } else if super::property_type(value) == Some("integer") {
        let mut form = String::from("integer");
        for (key, operator) in [("minimum", ">="), ("maximum", "<=")] {
            if let Some(bound) = value.get(key) {
                form.push_str(&format!(" {operator} {bound}"));
            }
        }
        vec![form]
    } else if plain_text(property) {
        // The shared text type's description says nothing about this option; its own does.
        let own = property.get("description").and_then(Value::as_str);
        let mut form = own
            .and_then(|text| text.lines().next())
            .unwrap_or("text")
            .trim()
            .to_owned();
        if property.get("x-comma-separated").is_some() {
            form.push_str(" Separate several with commas.");
        }
        vec![form]
    } else {
        let description = value
            .get("description")
            .or_else(|| property.get("description"))
            .and_then(Value::as_str)
            .or_else(|| super::property_type(value))?;
        let mut form = description.split_whitespace().collect::<Vec<_>>().join(" ");
        for (key, label) in [
            ("minimum", "Minimum"),
            ("maximum", "Maximum"),
            ("minLength", "Minimum length"),
            ("maxLength", "Maximum length"),
        ] {
            if let Some(bound) = value.get(key) {
                form.push_str(&format!(" {label}: {bound}."));
            }
        }
        vec![form]
    };
    forms
        .into_iter()
        .map(|form| biodata::Text::new(&form).ok())
        .collect::<Option<Vec<_>>>()
        .map(Into::into)
}

/// Whether a property is the shared bounded text type, directly, nullable or as list items.
fn plain_text(property: &Value) -> bool {
    property.get("$ref").and_then(Value::as_str) == Some("#/$defs/Text")
        || property
            .get("anyOf")
            .and_then(Value::as_array)
            .is_some_and(|choices| choices.iter().any(plain_text))
        || property.get("items").is_some_and(plain_text)
}
