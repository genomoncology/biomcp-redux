//! A rejected identifier uses its entity's declared search input, unchanged.
use biomcp_types::{QueryInvalid, UsageError};
use serde_json::{Map, Value};

pub(super) fn invalid(
    root: &clap::Command,
    verb: &str,
    entity: &str,
    value: &Value,
    cause: &QueryInvalid,
) -> Option<crate::grammar::ParseInvalid> {
    if verb != "get" || !cause.is_decode_failure() {
        return None;
    }
    let get = crate::grammar::validation::schema(verb, entity)?;
    let (name, _) = primary(&get)?;
    if cause.arguments().len() != 1 || cause.arguments().first()?.as_ref() != name {
        return None;
    }
    let search = crate::grammar::validation::schema("search", entity)?;
    let supplied = value.get(name)?;
    let inputs = supplied
        .as_array()
        .map_or_else(|| std::slice::from_ref(supplied), Vec::as_slice);
    let next = inputs
        .iter()
        .map(|input| search_step(root, entity, &search, input.as_str()?))
        .collect::<Option<Vec<_>>>()?;
    if next.is_empty() {
        return None;
    }
    let mut invalid = super::errors::checked_invalid(verb, entity, cause);
    let Some(UsageError::Arguments(arguments)) = invalid.usage.as_mut() else {
        return None;
    };
    arguments.next = next;
    Some(invalid)
}

fn primary(schema: &Value) -> Option<(&str, &Value)> {
    schema
        .get("properties")?
        .as_object()?
        .iter()
        .find_map(|(name, property)| {
            (property.get("x-positional").and_then(Value::as_u64) == Some(0))
                .then_some((name.as_str(), property))
        })
}

fn search_step(
    root: &clap::Command,
    entity: &str,
    schema: &Value,
    input: &str,
) -> Option<biomcp_types::Step> {
    let mut fields = if let Some(template) = schema.get("x-get-search") {
        let mut fields = template.as_object()?.clone();
        for value in fields.values_mut().filter(|value| value.is_null()) {
            *value = input.into();
        }
        fields
    } else {
        let (name, property) = primary(schema)?;
        let value = if crate::grammar::property_type(property) == Some("array") {
            Value::Array(vec![input.into()])
        } else {
            input.into()
        };
        Map::from_iter([(name.into(), value)])
    };
    // Spell options before the positional boundary, so a literal name stays data.
    let mut args: Vec<Box<str>> = vec!["search".into(), entity.into()];
    let positional = primary(schema).and_then(|(name, _)| fields.remove(name));
    for (name, value) in fields {
        args.push(format!("--{}={}", name.replace('_', "-"), value.as_str()?).into());
    }
    if let Some(value) = positional {
        args.push("--".into());
        let values = value
            .as_array()
            .map_or_else(|| std::slice::from_ref(&value), Vec::as_slice);
        for value in values {
            args.push(value.as_str()?.into());
        }
    }
    let argv = args
        .iter()
        .map(|word| std::ffi::OsString::from(word.as_ref()))
        .collect::<Vec<_>>();
    let parsed = crate::grammar::parse(root, &argv).ok()?;
    if !matches!(parsed.action, Some(biomcp_query::Action::Run { .. })) {
        return None;
    }
    Some(biomcp_types::Step {
        kind: biomcp_types::StepKind::Disambiguate,
        args: args.into(),
    })
}
