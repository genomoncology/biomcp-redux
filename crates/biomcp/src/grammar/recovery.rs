//! Suggest only requests accepted by the registry's existing checked constructors.
use biomcp_types::{Rewrite, Step, StepKind};
use serde_json::Value;

pub(super) fn checked(via: Rewrite, path: (&str, &str), value: Value) -> Option<Step> {
    let request = super::rewrite::request(via, path.0, path.1, value).ok()?;
    run(&request)
}

/// The corrected call, with the options it also takes, so the fix needs no help read.
pub(super) fn run(request: &biomcp_types::Request) -> Option<Step> {
    let mut step = crate::spell::step(
        StepKind::Configure,
        &biomcp_query::Action::Run {
            request: request.clone(),
            fresh: false,
        },
    )
    .ok()?;
    step.options = super::options(request);
    Some(step)
}

pub(super) fn corrected(
    via: Rewrite,
    path: (&str, &str),
    schema: &Value,
    value: &Value,
    names: &[&str],
    allow_omission: bool,
) -> Option<Step> {
    if let Some(step) = checked(via, path, value.clone()) {
        return Some(step);
    }
    for name in names {
        let property = schema.get("properties")?.get(*name)?;
        if allow_omission && let Some(values) = value.get(*name).and_then(Value::as_array) {
            let accepted: Vec<_> = values
                .iter()
                .filter(|item| {
                    let mut candidate = value.clone();
                    let Some(fields) = candidate.as_object_mut() else {
                        return false;
                    };
                    fields.insert((*name).into(), Value::Array(vec![(*item).clone()]));
                    checked(via, path, candidate).is_some()
                })
                .cloned()
                .collect();
            if !accepted.is_empty() {
                let mut candidate = value.clone();
                candidate
                    .as_object_mut()?
                    .insert((*name).into(), Value::Array(accepted));
                if let Some(step) = checked(via, path, candidate) {
                    return Some(step);
                }
            }
        }
        // Ordinary invalid filters must not disappear from the proposed request.
        if allow_omission {
            let mut candidate = value.clone();
            candidate.as_object_mut()?.remove(*name);
            if let Some(step) = checked(via, path, candidate) {
                return Some(step);
            }
        }
        for replacement in candidates(
            schema,
            property,
            value.get(*name).is_none() || allow_omission,
        ) {
            let mut candidate = value.clone();
            candidate
                .as_object_mut()?
                .insert((*name).into(), replacement);
            if let Some(step) = checked(via, path, candidate) {
                return Some(step);
            }
        }
    }
    None
}

fn candidates(schema: &Value, property: &Value, missing: bool) -> Vec<Value> {
    let resolved = super::validation::resolve(schema, property);
    // A supplied value expresses intent. Only a singleton vocabulary is unambiguous.
    if !missing
        && resolved
            .get("enum")
            .and_then(Value::as_array)
            .is_none_or(|v| v.len() != 1)
    {
        return Vec::new();
    }
    let mut values: Vec<_> = property
        .get("default")
        .filter(|_| missing)
        .into_iter()
        .cloned()
        .chain(
            resolved
                .get("enum")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .cloned(),
        )
        .collect();
    if missing && super::property_type(resolved) == Some("integer") {
        values.push(resolved.get("minimum").cloned().unwrap_or(Value::from(0)));
    }
    if super::property_type(property) == Some("array") {
        values = values
            .into_iter()
            .map(|value| {
                if value.is_array() {
                    value
                } else {
                    Value::Array(vec![value])
                }
            })
            .collect();
    }
    values
}

/// A runnable correction replaces help: the refusal's first step is the call to run.
pub(super) fn next(arguments: &mut biomcp_types::UsageArguments, corrected: Option<Step>) {
    if let Some(step) = corrected {
        arguments.next.retain(|step| !is_help(step));
        arguments.next.insert(0, step);
    }
}

fn is_help(step: &Step) -> bool {
    step.args
        .first()
        .is_some_and(|word| word.as_ref() == "help")
}

/// Restart an unbound trial page without discarding its selection or source filters.
pub(super) fn trial_first_page(
    via: Rewrite,
    path: (&str, &str),
    value: &Value,
    cause: &biomcp_types::QueryInvalid,
) -> Option<Step> {
    if !matches!(
        cause.message(),
        Some(
            biomcp_types::ErrorMessage::TrialSectionPool
                | biomcp_types::ErrorMessage::TrialSearchCursor
        )
    ) {
        return None;
    }
    let mut first = value.clone();
    first
        .as_object_mut()?
        .insert("offset".into(), Value::from(0));
    checked(via, path, first)
}

/// A request refused before any source is read, for one argument with named valid values:
/// when exactly one valid value passes the check, that call replaces help. When several
/// pass, any one would be a guess, so help stays.
pub(crate) fn prepared(
    request: &biomcp_types::Request,
    mut error: biomcp_types::UsageError,
) -> biomcp_types::UsageError {
    let biomcp_types::UsageError::Arguments(arguments) = &mut error else {
        return error;
    };
    if arguments.next.iter().any(|step| !is_help(step)) {
        return error;
    }
    let (Some(argument), Some(values)) = (&arguments.argument, &arguments.valid_values) else {
        return error;
    };
    let (verb, entity) = request.path();
    let name = argument.trim_start_matches('-').replace('-', "_");
    let (Some(schema), Ok(value)) = (
        super::validation::schema(verb, entity),
        super::rewrite::query_value(request),
    ) else {
        return error;
    };
    let array = schema
        .pointer(&format!("/properties/{name}"))
        .and_then(super::property_type)
        == Some("array");
    let mut passing = values.iter().filter_map(|choice| {
        let choice = Value::from(choice.as_str());
        let mut candidate = value.clone();
        candidate.as_object_mut()?.insert(
            name.clone(),
            if array {
                Value::Array(vec![choice])
            } else {
                choice
            },
        );
        let request = super::rewrite::request(request.rewrite(), verb, entity, candidate).ok()?;
        // The corrected call must pass the same check that refused this one.
        biomcp_query::Action::Run {
            request: request.clone(),
            fresh: false,
        }
        .prepare()
        .ok()?;
        run(&request)
    });
    let corrected = match (passing.next(), passing.next()) {
        (Some(only), None) => Some(only),
        _ => None,
    };
    next(arguments, corrected);
    error
}
