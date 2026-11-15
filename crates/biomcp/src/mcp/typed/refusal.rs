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

/// An unknown key typed as the option it would be on the command line, `q` as `-q` and
/// `who_pq` as `--who-pq`: the first fix the command line gives that mistake, if any.
pub(super) fn fix(
    root: &crate::grammar::Root,
    args: &[OsString],
    key: &str,
    values: &Value,
) -> Option<biomcp_types::Step> {
    let value = values.get(key)?;
    let spelling = if key.chars().count() == 1 {
        format!("-{key}")
    } else {
        format!("--{}", key.replace('_', "-"))
    };
    let mut words = Vec::new();
    for value in value.as_array().map_or_else(|| vec![value], |values| values.iter().collect()) {
        match value {
            Value::Bool(true) => words.push(OsString::from(&spelling)),
            Value::String(text) => words.extend([OsString::from(&spelling), text.into()]),
            Value::Number(number) => {
                words.extend([OsString::from(&spelling), number.to_string().into()]);
            }
            _ => return None,
        }
    }
    let mut args = args.to_vec();
    let at = args.len().min(2);
    args.splice(at..at, words);
    crate::grammar::candidate(root, &args)
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
            valid_values: (!valid.is_empty()).then(|| valid.into()),
            next: vec![biomcp_types::Step {
                kind: biomcp_types::StepKind::Configure,
                args: help.iter().copied().map(Box::from).collect(),
            }],
            ..Default::default()
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
