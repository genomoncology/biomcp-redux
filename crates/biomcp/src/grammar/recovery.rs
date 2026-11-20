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

/// A corrected call, and whether it is certain: false only when `allow_omission` let it drop
/// what the caller asked for, which is a guess that keeps help beside it. A listed value is
/// offered only when it is certain; a bare command, or several values that would each pass,
/// get no correction.
pub(super) fn corrected(
    via: Rewrite,
    path: (&str, &str),
    schema: &Value,
    value: &Value,
    names: &[&str],
    allow_omission: bool,
) -> Option<(Step, bool)> {
    if let Some(step) = checked(via, path, value.clone()) {
        return Some((step, true));
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
                    return Some((step, true));
                }
            }
        }
        // Without the refused filter or section the call asks for less than the caller did,
        // so it is a guess that keeps help beside it.
        if allow_omission {
            let mut candidate = value.clone();
            candidate.as_object_mut()?.remove(*name);
            if let Some(step) = checked(via, path, candidate) {
                return Some((step, false));
            }
        }
        let missing = value.get(*name).is_none() || allow_omission;
        let defaulted = missing && property.get("default").is_some();
        let mut passing = candidates(schema, property, missing)
            .into_iter()
            .enumerate()
            .filter_map(|(index, replacement)| {
                let mut candidate = value.clone();
                candidate
                    .as_object_mut()?
                    .insert((*name).into(), replacement);
                checked(via, path, candidate).map(|step| (index, step))
            });
        if let Some((index, step)) = passing.next() {
            // The declared default is certain, and so is the only listed value that passes when
            // the call already names what it is after. Otherwise the value is a guess: on a bare
            // command the call shows no intent to pick it (bare `search drug` runs only on
            // recalls), and among several passing values the first is only the first declared.
            // Help alone answers those refusals.
            let certain = (defaulted && index == 0)
                || (passing.next().is_none() && super::options::names_something(schema, value));
            return certain.then_some((step, true));
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

/// A refusal whose fixed message names its one fix: drug markets takes a name and at most a
/// source, so the same call without the options it refused; a label-history offset needs
/// every version, so the same call reading every version from that offset; a section
/// label-history does not read is read from the label, when it is a label section (1081).
pub(super) fn fixed(
    via: Rewrite,
    path: (&str, &str),
    value: &Value,
    cause: &biomcp_types::QueryInvalid,
) -> Option<Step> {
    let mut candidate = value.clone();
    let fields = candidate.as_object_mut()?;
    match cause.message()? {
        biomcp_types::ErrorMessage::FdaDscGetId => {
            return checked(
                Rewrite::Direct,
                ("search", "drug"),
                serde_json::json!({"query": fields.get("input")?, "source": "fda-dsc"}),
            );
        }
        biomcp_types::ErrorMessage::DrugMarketsSource => {
            for name in cause.arguments() {
                fields.remove(name.as_ref());
            }
        }
        biomcp_types::ErrorMessage::LabelHistoryOffset => {
            fields.insert("every_version".into(), Value::Bool(true));
        }
        biomcp_types::ErrorMessage::LabelHistorySectionUnknown
        | biomcp_types::ErrorMessage::LabelHistoryOffsetSection => {
            let heading = fields.get("label_section").and_then(Value::as_str);
            let heading = biomcp_types::openfda_label_field(heading?)?;
            fields.retain(|name, _| name == "input");
            fields.insert("section".into(), Value::Array(vec!["label".into()]));
            fields.insert("label_section".into(), Value::from(heading));
        }
        _ => return None,
    }
    checked(via, path, candidate)
}

/// A certain correction replaces help: the refusal's only step is the call to run.
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

/// A request refused before any source is read, for one argument with named valid values:
/// when the call already asks for something and exactly one valid value passes the check,
/// that call replaces help. When several pass, or the call names nothing but the command,
/// any value would be a guess, so help stays.
pub(crate) fn prepared(
    request: &biomcp_types::Request,
    mut error: biomcp_types::UsageError,
) -> biomcp_types::UsageError {
    let biomcp_types::UsageError::Arguments(arguments) = &mut error else {
        return error;
    };
    if arguments.next.iter().any(|step| !is_help(step)) || !super::options::asked(request) {
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

/// An option no spelling of this command takes: the same call without it, and without the
/// word after it when that word is not an option, if that call checks. No call is offered
/// when the word after it could also be the caller's own operand.
pub(super) fn dropped(
    root: &clap::Command,
    spelling: &str,
    args: &[std::ffi::OsString],
) -> Option<biomcp_types::Step> {
    let spelling = spelling.split('=').next()?;
    let at = args.iter().position(|word| {
        word.to_str().is_some_and(|word| {
            word == spelling
                || word
                    .strip_prefix(spelling)
                    .is_some_and(|rest| rest.starts_with('='))
        })
    })?;
    if args.iter().take(at).any(|word| word.to_str() == Some("--")) {
        return None;
    }
    let mut without = args.to_vec();
    let flag = without.remove(at);
    if !flag.to_str()?.contains('=')
        && without
            .get(at)
            .and_then(|word| word.to_str())
            .is_some_and(|word| !word.starts_with('-'))
    {
        // The word after the option may be its value or the caller's own search word. When
        // the call also reads with that word kept, which one it is would be a guess.
        if super::parse(root, &without).is_ok() {
            return None;
        }
        without.remove(at);
    }
    let biomcp_query::Action::Run { request, .. } = super::parse(root, &without).ok()?.action?
    else {
        return None;
    };
    // The offered call must also pass the check made before any source is read, or be the
    // one correction that check names.
    let action = biomcp_query::Action::Run {
        request: request.clone(),
        fresh: false,
    };
    match action.prepare() {
        Ok(_) => run(&request),
        Err(error) => match prepared(&request, error) {
            biomcp_types::UsageError::Arguments(arguments) => arguments
                .next
                .into_iter()
                .next()
                .filter(|step| !is_help(step)),
            _ => None,
        },
    }
}

/// A filter or section the selected source refuses, kept with the one source that accepts it
/// and passes the check made before any source is read. It is certain only when the caller
/// named no source, so the default was refused; replacing a source the caller named drops
/// something they asked for, so that swap is a guess that keeps help beside it.
pub(super) fn source_swap(
    via: Rewrite,
    path: (&str, &str),
    schema: &Value,
    value: &Value,
    valid: &[&str],
) -> Option<(Step, bool)> {
    let array = schema
        .pointer("/properties/source")
        .and_then(super::property_type)
        == Some("array");
    let mut passing = valid.iter().filter_map(|source| {
        let source = Value::from(*source);
        let mut candidate = value.clone();
        candidate.as_object_mut()?.insert(
            "source".into(),
            if array {
                Value::Array(vec![source])
            } else {
                source
            },
        );
        let request = super::rewrite::request(via, path.0, path.1, candidate).ok()?;
        biomcp_query::Action::Run {
            request: request.clone(),
            fresh: false,
        }
        .prepare()
        .ok()?;
        run(&request)
    });
    let only = passing.next()?;
    if passing.next().is_some() || !super::options::names_something(schema, value) {
        return None;
    }
    let named = value.get("source").is_some_and(|source| !source.is_null());
    Some((only, !named))
}
