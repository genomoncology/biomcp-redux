//! Words after a get command's record ID: which trailing words are sections, and the refusal
//! of a word that names no section. The refusal names the word and lists the sections; a near
//! section is offered as the next call and never read in the word's place.
use super::{ParseInvalid, choices, properties};
use biomcp_types::{ErrorMessage, UsageArguments, UsageError};
use clap::Command;
use serde_json::{Map, Value, json};
use std::ffi::OsString;

/// The longest word a refusal names. Longer text is no section anyone meant.
const NAMED: usize = 64;

pub(super) fn split_sections(
    result: &mut Map<String, Value>,
    schema: &Value,
) -> Result<(), ParseInvalid> {
    for (name, property) in properties(schema)? {
        if property.get("x-trailing-section") != Some(&Value::Bool(true))
            || result.contains_key(name)
        {
            continue;
        }
        let item = property.get("items").ok_or_else(ParseInvalid::default)?;
        let item = item
            .get("$ref")
            .and_then(Value::as_str)
            .and_then(|s| schema.pointer(s.trim_start_matches('#')))
            .unwrap_or(item);
        let words = item
            .get("enum")
            .and_then(Value::as_array)
            .ok_or_else(ParseInvalid::default)?;
        let Some(input) = result.get_mut("input").and_then(Value::as_array_mut) else {
            return Err(ParseInvalid::default());
        };
        let mut split = input.len();
        while split > 1
            && input
                .get(split - 1)
                .is_some_and(|value| choices::matching(value, words).is_some())
        {
            split -= 1;
        }
        let section = input.split_off(split);
        if !section.is_empty() {
            result.insert(name.clone(), Value::Array(section));
        }
    }
    Ok(())
}

/// A call that checks, but whose trailing word after the first ID has a section's shape and
/// names no section: it was read as a section, so the refusal says so.
pub(super) fn trailing(
    verb: &str,
    entity: &str,
    value: &Value,
    explicit: bool,
) -> Option<ParseInvalid> {
    refuse(
        (verb, entity),
        value,
        explicit,
        ErrorMessage::ArgumentSectionNotId,
    )
}

/// A refused call whose first word not understood is a section word that names no section.
pub(super) fn unknown(
    verb: &str,
    entity: &str,
    value: &Value,
    explicit: bool,
) -> Option<ParseInvalid> {
    refuse(
        (verb, entity),
        value,
        explicit,
        ErrorMessage::ArgumentSectionUnknown,
    )
}

/// Words past the ID and the one section a command takes, as in `get disease ID extra words
/// here`: the first word not understood, named as a section word that names no section.
pub(super) fn excess(root: &Command, path: &[&str], args: &[OsString]) -> Option<UsageArguments> {
    let &["get", entity] = path else { return None };
    let command = root.find_subcommand("get")?.find_subcommand(entity)?;
    let (mut kept, words) = super::positional::split(root, command, "get", entity, args)?;
    let taken = command.get_positionals().count();
    let extra = words.get(taken)?.clone();
    kept.extend(words.into_iter().take(taken).map(OsString::from));
    // A near-section call would drop the words past it, so help is the next step.
    let mut usage = match super::parse(root, &kept) {
        Ok(_) => {
            let schema = super::validation::schema("get", entity)?;
            let sections = sections(&schema)?;
            if !plain(&extra) || known(&sections, &extra) {
                return None;
            }
            refusal(
                ("get", entity),
                &schema,
                &extra,
                ErrorMessage::ArgumentSectionUnknown,
                None,
            )?
        }
        Err(ParseInvalid {
            usage: Some(UsageError::Arguments(arguments)),
            ..
        }) => arguments,
        Err(_) => return None,
    };
    usage.next = vec![super::validation::help(["get", entity])];
    (usage.message == Some(ErrorMessage::ArgumentSectionUnknown)).then_some(usage)
}

fn refuse(
    path: (&str, &str),
    value: &Value,
    explicit: bool,
    message: ErrorMessage,
) -> Option<ParseInvalid> {
    if path.0 != "get" {
        return None;
    }
    let schema = super::validation::schema(path.0, path.1)?;
    let (word, candidate) = first(&schema, value, explicit)?;
    let candidate = candidate.filter(|candidate| first(&schema, candidate, explicit).is_none());
    Some(ParseInvalid {
        usage: Some(UsageError::Arguments(refusal(
            path,
            &schema,
            &word,
            message,
            candidate.as_ref(),
        )?)),
        ..ParseInvalid::default()
    })
}

/// The refusal: the word, the record's sections, and the call with a near section, or help.
fn refusal(
    path: (&str, &str),
    schema: &Value,
    word: &str,
    message: ErrorMessage,
    candidate: Option<&Value>,
) -> Option<UsageArguments> {
    let next = candidate
        .and_then(|candidate| super::positional::complete(path, schema, candidate))
        .unwrap_or_else(|| super::validation::help([path.0, path.1]));
    Some(UsageArguments {
        argument: Some(word.into()),
        message: Some(message),
        valid_values: Some(sections(schema)?),
        next: vec![next],
    })
}

/// The first word not understood, given as a section or trailing the first ID, and the call
/// with the nearest section in its place when one is near.
fn first(schema: &Value, value: &Value, explicit: bool) -> Option<(String, Option<Value>)> {
    let property = schema.pointer("/properties/section")?;
    let sections = sections(schema)?;
    let mut candidate = value.clone();
    let fields = candidate.as_object_mut()?;
    let given: Vec<&str> = match fields.get("section") {
        Some(Value::String(word)) => vec![word],
        Some(Value::Array(items)) => items.iter().filter_map(Value::as_str).collect(),
        _ => Vec::new(),
    };
    let index = given.iter().position(|word| !known(&sections, word));
    let word = if let Some(index) = index {
        let word = (*given.get(index)?).to_owned();
        let near = near(&word, &sections).map(|near| json!(near));
        match (fields.get_mut("section")?, near) {
            (Value::Array(items), Some(near)) => *items.get_mut(index)? = near,
            (section @ Value::String(_), Some(near)) => *section = near,
            _ => return plain(&word).then_some((word, None)),
        }
        word
    } else if property.get("x-trailing-section") == Some(&json!(true)) && !explicit {
        let input = fields.get_mut("input")?.as_array_mut()?;
        let index = (1..input.len()).find(|index| {
            input
                .get(*index)
                .and_then(Value::as_str)
                .is_some_and(|word| shaped(word) && !known(&sections, word))
        })?;
        let word = input.remove(index).as_str()?.to_owned();
        let near = near(&word, &sections);
        match (fields.entry("section").or_insert_with(|| json!([])), near) {
            (Value::Array(items), Some(near)) => items.push(json!(near)),
            _ => return plain(&word).then_some((word, None)),
        }
        word
    } else {
        return None;
    };
    plain(&word).then_some((word, Some(candidate)))
}

fn sections(schema: &Value) -> Option<Box<[biodata::Text]>> {
    super::validation::accepted(schema, schema.pointer("/properties/section")?)
}

fn known(sections: &[biodata::Text], word: &str) -> bool {
    sections
        .iter()
        .any(|section| section.as_str().eq_ignore_ascii_case(word))
}

/// The one section within two edits of the word, and nearer than every other.
fn near<'a>(word: &str, sections: &'a [biodata::Text]) -> Option<&'a str> {
    let word = word.to_ascii_lowercase();
    let mut scored: Vec<_> = sections
        .iter()
        .map(|section| {
            (
                super::entry::distance(&word, section.as_str()),
                section.as_str(),
            )
        })
        .collect();
    scored.sort_unstable();
    match scored.as_slice() {
        [(distance, best), rest @ ..]
            if *distance <= 2
                && 3 * distance <= best.len()
                && rest.first().is_none_or(|(other, _)| other > distance) =>
        {
            Some(best)
        }
        _ => None,
    }
}

/// Lowercase letters and dashes, as every section word is spelled; an ID has a capital or digit.
fn shaped(word: &str) -> bool {
    word.starts_with(|c: char| c.is_ascii_lowercase())
        && word.chars().all(|c| c.is_ascii_lowercase() || c == '-')
}

/// A word a refusal may name: short, of letters, digits, dashes and underscores only.
fn plain(word: &str) -> bool {
    (1..=NAMED).contains(&word.len())
        && word
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
}
