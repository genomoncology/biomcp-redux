//! What a refused typed call is told: the argument, the names it could be, and the fix.
use serde_json::{Value, json};
use std::ffi::OsString;

/// The first key the action's schema does not name.
pub(super) fn unknown(input: &Value, values: &Value) -> Option<String> {
    let properties = input.get("properties")?.as_object()?;
    values
        .as_object()?
        .keys()
        .find(|key| !properties.contains_key(*key))
        .cloned()
}

/// Every argument the action takes, under its advertised name.
pub(super) fn names(input: &Value, forms: &[super::forms::Form]) -> Vec<String> {
    input
        .get("properties")
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(|properties| properties.keys())
        .map(|key| super::forms::advertised(forms, key).to_owned())
        .collect()
}

/// The arguments the action's schema does name, for the refusal's own command line.
pub(super) fn known(input: &Value, values: &Value) -> Value {
    let properties = input.get("properties").and_then(Value::as_object);
    Value::Object(
        values
            .as_object()
            .into_iter()
            .flatten()
            .filter(|(key, _)| properties.is_some_and(|properties| properties.contains_key(*key)))
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect(),
    )
}

/// Every unknown key typed as the option it would be on the command line, `q` as `-q` and
/// `who_pq` as `--who-pq`: the fix the command line gives those mistakes, if any. No fix
/// when a key cannot be spelled as an option. A JSON object's keys have no order, and the
/// command line fixes some pairs only one way round (`--who-pq -q x`, not `-q x --who-pq`), so
/// the keys are tried in both orders. The call without the keys is never offered: it would
/// widen the query.
pub(super) fn fix(
    root: &crate::grammar::Root,
    args: &[OsString],
    input: &Value,
    values: &Value,
) -> Option<biomcp_types::Step> {
    let properties = input.get("properties")?.as_object()?;
    let mut options = Vec::new();
    for (key, value) in values.as_object()? {
        if properties.contains_key(key) {
            continue;
        }
        if key.is_empty() || key.contains('=') || key.starts_with('-') {
            return None;
        }
        let spelling = if key.chars().count() == 1 {
            format!("-{key}")
        } else {
            format!("--{}", key.replace('_', "-"))
        };
        let mut words = Vec::new();
        for value in value
            .as_array()
            .map_or_else(|| vec![value], |values| values.iter().collect())
        {
            match value {
                Value::Bool(true) => words.push(OsString::from(&spelling)),
                Value::String(text) if !text.starts_with('-') => {
                    words.extend([OsString::from(&spelling), text.into()]);
                }
                Value::Number(number) => {
                    words.extend([OsString::from(&spelling), number.to_string().into()]);
                }
                _ => return None,
            }
        }
        options.push(words);
    }
    let call = |options: Vec<&Vec<OsString>>| {
        let mut args = args.to_vec();
        let at = args.len().min(2);
        args.splice(at..at, options.into_iter().flatten().cloned());
        crate::grammar::candidate(root, &args)
    };
    let mut step =
        call(options.iter().collect()).or_else(|| call(options.iter().rev().collect()))?;
    // The fix carries the options its call also takes, as a command-line correction does.
    if step.options.is_empty() {
        let words: Vec<OsString> = step
            .args
            .iter()
            .map(|word| OsString::from(&**word))
            .collect();
        if let Ok(parsed) = crate::grammar::parse(root, &words)
            && let Some(biomcp_query::Action::Run { request, .. }) = parsed.action
        {
            step.options = crate::grammar::options(&request);
        }
    }
    Some(step)
}

/// Preserve the parser's specific recovery for a source the lookup does not accept.
pub(super) fn source(
    root: &crate::grammar::Root,
    path: &[&str],
    args: &[OsString],
    values: &Value,
) -> Option<biomcp_types::UsageError> {
    let source = values.get("source")?.as_str()?;
    let mut args = args.to_vec();
    args.insert(2, format!("--source={source}").into());
    crate::grammar::source_refusal::for_command(root, path, Some("--source"), &args)?.usage
}

pub(super) fn arguments(
    refused: &super::forms::Refused,
    help: &[&str],
) -> Option<biomcp_types::UsageError> {
    let valid = refused
        .valid
        .iter()
        .map(|value| biodata::Text::new(value).ok())
        .collect::<Option<Vec<_>>>()?;
    Some(biomcp_types::UsageError::Arguments(
        biomcp_types::UsageArguments {
            argument: Some(refused.argument.as_str().into()),
            message: Some(refused.cause),
            valid_values: (!valid.is_empty()).then(|| valid.into()),
            next: vec![biomcp_types::Step {
                kind: biomcp_types::StepKind::Configure,
                args: help.iter().copied().map(Box::from).collect(),
                options: Default::default(),
            }],
        },
    ))
}

/// A guessed action that is really a section of the tool's get, such as drug `approvals`:
/// the get command line that reads it, when the call carried the record ID.
pub(super) fn section(
    root: &crate::grammar::Root,
    placed: &[(&str, &str, &str)],
    word: &str,
    fields: &serde_json::Map<String, Value>,
) -> Option<Vec<OsString>> {
    let (_, command, _) = placed.iter().find(|(action, _, _)| *action == "get")?;
    let (verb, entity, id) = crate::grammar::command_paths()
        .into_iter()
        .find(|(verb, entity, _)| super::name(verb, entity) == *command)?;
    let schema = super::schema(id).ok()?;
    let input = super::arguments::input_with_limit(&schema, usize::MAX);
    let forms = super::forms::forms(verb, &schema);
    let sections = forms.iter().find(|form| form.advertised == "section")?;
    let record = forms.iter().find(|form| form.advertised == "id")?;
    let node = input.pointer(&format!("/properties/{}", sections.field))?;
    let listed = node.pointer("/items/enum").or_else(|| node.get("enum"))?;
    if !listed.as_array()?.contains(&json!(word)) {
        return None;
    }
    let mut values = fields.get(word).cloned().unwrap_or_else(|| json!({}));
    super::forms::normalize(&mut values, &forms, &input).ok()?;
    let id = values.get(&record.field)?.clone();
    let values = json!({record.field.as_str(): id, sections.field.as_str(): word});
    super::arguments::cli(root, verb, entity, &schema, &values)
}

/// Accept fields inside the action when both spellings agree or only one is given.
pub(super) fn take_fields(
    top: Option<&Value>,
    action: &mut Value,
) -> Result<Option<String>, biomcp_types::ErrorMessage> {
    use biomcp_types::ErrorMessage;
    let nested = action
        .as_object_mut()
        .and_then(|fields| fields.remove("fields"))
        .filter(|value| !super::arguments::unset(value));
    let top = top.filter(|value| !super::arguments::unset(value));
    if let (Some(top), Some(nested)) = (top, &nested)
        && top != nested
    {
        return Err(ErrorMessage::ArgumentRepeated);
    }
    let Some(value) = top.or(nested.as_ref()) else {
        return Ok(None);
    };
    let names = value
        .as_array()
        .and_then(|names| {
            names
                .iter()
                .map(|name| name.as_str().filter(|name| !name.contains(',')))
                .collect::<Option<Vec<_>>>()
        })
        .ok_or(ErrorMessage::ArgumentValue)?;
    Ok((!names.is_empty()).then(|| names.join(",")))
}

/// A single-ID action with several IDs is answered by one checked call per ID.
pub(super) fn split(
    root: &crate::grammar::Root,
    (verb, entity): (&str, &str),
    schema: &Value,
    values: &Value,
    refused: &super::forms::Refused,
    chosen: Option<&str>,
) -> Vec<biomcp_types::Step> {
    if refused.valid.as_slice() != ["one value per call"] {
        return Vec::new();
    }
    let forms = super::forms::forms(verb, schema);
    let Some(form) = forms
        .iter()
        .find(|form| form.advertised == refused.argument)
    else {
        return Vec::new();
    };
    let Some(values) = values.as_object() else {
        return Vec::new();
    };
    let Some((key, items)) = values.iter().find_map(|(key, value)| {
        (super::forms::field(&forms, key) == form.field)
            .then(|| value.as_array().map(|items| (key, items)))
            .flatten()
    }) else {
        return Vec::new();
    };
    let input = super::arguments::input(schema);
    items
        .iter()
        .map(|item| {
            let mut values = values.clone();
            values.remove(key);
            values.insert(form.field.clone(), item.clone());
            let mut values = Value::Object(values);
            super::forms::normalize(&mut values, &forms, &input).ok()?;
            let mut args = super::arguments::cli(root, verb, entity, schema, &values)?;
            if let Some(chosen) = chosen {
                args.insert(0, format!("--fields={chosen}").into());
            }
            let mut step = crate::grammar::candidate(root, &args)?;
            if let Some(chosen) = chosen
                && !step.args.iter().any(|arg| arg.starts_with("--fields="))
            {
                let mut words = step.args.into_vec();
                words.insert(0, format!("--fields={chosen}").into());
                step.args = words.into();
            }
            let words = step
                .args
                .iter()
                .map(|word| OsString::from(&**word))
                .collect::<Vec<_>>();
            let checked = crate::grammar::parse(root, &words).ok()?;
            crate::fields::check(&checked).ok()?;
            checked.action?.prepare().ok()?;
            Some(step)
        })
        .collect::<Option<Vec<_>>>()
        .unwrap_or_default()
}
