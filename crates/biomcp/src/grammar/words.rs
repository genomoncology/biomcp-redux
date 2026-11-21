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
        // Once a word after the first ID names a section, every word after it is a section.
        let Some(split) = (1..input.len()).find(|index| {
            input
                .get(*index)
                .is_some_and(|value| choices::matching(value, words).is_some())
        }) else {
            continue;
        };
        let section = input.split_off(split);
        if !section.is_empty() {
            result.insert(name.clone(), Value::Array(section));
        }
    }
    Ok(())
}

/// The first word not understood after a get command's ID: a section word that names no
/// section, or a trailing word read as a section because it is near one of this record's
/// sections or names another record's. Any other trailing word is an ID, in any letter case.
pub(super) fn unknown(
    verb: &str,
    entity: &str,
    value: &Value,
    explicit: bool,
) -> Option<ParseInvalid> {
    if verb != "get" {
        return None;
    }
    let schema = super::validation::schema(verb, entity)?;
    let via = biomcp_types::COMMANDS
        .iter()
        .find(|command| (command.verb, command.entity) == (verb, entity))?
        .via;
    let is_id =
        |word: &str| super::rewrite::request(via, verb, entity, json!({"input": [word]})).is_ok();
    let (word, candidate, place) = first(&schema, value, explicit)?;
    let candidate = candidate.filter(|candidate| first(&schema, candidate, explicit).is_none());
    let message = match place {
        Place::Trailing if is_id(&word) => ErrorMessage::ArgumentSectionNotId,
        Place::AfterSection => ErrorMessage::ArgumentIdAfterSection,
        Place::Trailing | Place::Section => ErrorMessage::ArgumentSectionUnknown,
    };
    let next = candidate.and_then(|candidate| runs(via, (verb, entity), &schema, candidate));
    Some(ParseInvalid {
        usage: Some(UsageError::Arguments(refusal(
            (verb, entity),
            &schema,
            &word,
            message,
            next,
        )?)),
        ..ParseInvalid::default()
    })
}

/// Where the word not understood stood: in a section's place, trailing the first ID before
/// any section, or after a trailing section word.
enum Place {
    Section,
    Trailing,
    AfterSection,
}

/// The corrected call when it passes every check a run makes before reading a source. A call
/// that needs a source the caller did not name is offered with the one source that fits.
fn runs(
    via: biomcp_types::Rewrite,
    path: (&str, &str),
    schema: &Value,
    candidate: Value,
) -> Option<biomcp_types::Step> {
    let check = |value: Value| {
        let request = super::rewrite::request(via, path.0, path.1, value).ok()?;
        biomcp_query::Action::Run {
            request: request.clone(),
            fresh: false,
        }
        .prepare()
        .ok()?;
        super::recovery::run(&request)
    };
    if let Some(step) = check(candidate.clone()) {
        return Some(step);
    }
    if candidate.get("source").is_some() {
        return None;
    }
    let property = schema.pointer("/properties/source")?;
    let list = super::property_type(property) == Some("array");
    let fits: Vec<_> = super::validation::accepted(schema, property)?
        .iter()
        .filter_map(|source| {
            let mut value = candidate.clone();
            let source = if list {
                json!([source.as_str()])
            } else {
                json!(source.as_str())
            };
            value.as_object_mut()?.insert("source".into(), source);
            check(value)
        })
        .collect();
    match <[_; 1]>::try_from(fits) {
        Ok([step]) => Some(step),
        Err(_) => None,
    }
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

/// The refusal: the word, the record's sections, and the checked corrected call, or help.
fn refusal(
    path: (&str, &str),
    schema: &Value,
    word: &str,
    message: ErrorMessage,
    next: Option<biomcp_types::Step>,
) -> Option<UsageArguments> {
    let next = next.unwrap_or_else(|| super::validation::help([path.0, path.1]));
    Some(UsageArguments {
        argument: Some(word.into()),
        message: Some(message),
        valid_values: Some(sections(schema)?),
        next: vec![next],
    })
}

/// The first word not understood, where it stood, and the corrected call: the nearest section
/// in its place, or a word after a section moved before the sections.
fn first(schema: &Value, value: &Value, explicit: bool) -> Option<(String, Option<Value>, Place)> {
    let property = schema.pointer("/properties/section")?;
    let sections = sections(schema)?;
    let trailing = property.get("x-trailing-section") == Some(&json!(true)) && !explicit;
    let mut candidate = value.clone();
    let fields = candidate.as_object_mut()?;
    let given: Vec<&str> = match fields.get("section") {
        Some(Value::String(word)) => vec![word],
        Some(Value::Array(items)) => items.iter().filter_map(Value::as_str).collect(),
        _ => Vec::new(),
    };
    if let Some(index) = given.iter().position(|word| !known(&sections, word)) {
        let word = (*given.get(index)?).to_owned();
        if !plain(&word) {
            return None;
        }
        let near = near(&word, &sections).map(|near| json!(near));
        let place = if trailing && near.is_none() {
            Place::AfterSection
        } else {
            Place::Section
        };
        let mut moved = None;
        match (fields.get_mut("section")?, near) {
            (Value::Array(items), Some(near)) => *items.get_mut(index)? = near,
            (section @ Value::String(_), Some(near)) => *section = near,
            (Value::Array(items), None) if trailing => moved = Some(items.remove(index)),
            _ => return Some((word, None, place)),
        }
        if let Some(moved) = moved {
            fields.get_mut("input")?.as_array_mut()?.push(moved);
        }
        return Some((word, Some(candidate), place));
    }
    if !trailing {
        return None;
    }
    let input = fields.get_mut("input")?.as_array_mut()?;
    let others = if input.len() > 1 {
        elsewhere()
    } else {
        Vec::new()
    };
    let index = (1..input.len()).find(|index| {
        input
            .get(*index)
            .and_then(Value::as_str)
            .is_some_and(|word| {
                plain(word)
                    && !known(&sections, word)
                    && (near(word, &sections).is_some() || known(&others, word))
            })
    })?;
    let word = input.remove(index).as_str()?.to_owned();
    match (
        fields.entry("section").or_insert_with(|| json!([])),
        near(&word, &sections),
    ) {
        (Value::Array(items), Some(near)) => items.push(json!(near)),
        _ => return Some((word, None, Place::Trailing)),
    }
    Some((word, Some(candidate), Place::Trailing))
}

fn sections(schema: &Value) -> Option<Box<[biodata::Text]>> {
    super::validation::accepted(schema, schema.pointer("/properties/section")?)
}

fn known(sections: &[biodata::Text], word: &str) -> bool {
    sections
        .iter()
        .any(|section| section.as_str().eq_ignore_ascii_case(word))
}

/// Every get command's sections, which a trailing word may name for another record.
fn elsewhere() -> Vec<biodata::Text> {
    super::command_paths()
        .iter()
        .filter(|(verb, _, _)| *verb == "get")
        .filter_map(|(verb, entity, _)| sections(&super::validation::schema(verb, entity)?))
        .flat_map(|list| list.into_vec())
        .collect()
}

/// A section's spelling without case, dashes, underscores or a plural s.
fn form(word: &str) -> String {
    let word: String = word
        .chars()
        .filter(|c| !matches!(c, '-' | '_'))
        .map(|c| c.to_ascii_lowercase())
        .collect();
    match word.strip_suffix('s') {
        Some(stem) if word.len() > 3 => stem.to_owned(),
        _ => word,
    }
}

/// The section a word is near: the same spelling up to case, dashes, underscores and a plural
/// s, or the one nearest within one edit of a name of five letters or more, or two edits of a
/// name of eight or more. Shorter names are matched by spelling only, so no gene symbol such
/// as ALK is read as `all`.
fn near<'a>(word: &str, sections: &'a [biodata::Text]) -> Option<&'a str> {
    if let Some(section) = sections
        .iter()
        .find(|section| form(section.as_str()) == form(word))
    {
        return Some(section.as_str());
    }
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
    let allowed = |name: &str| match name.len() {
        8.. => 2,
        5..=7 => 1,
        _ => 0,
    };
    match scored.as_slice() {
        [(distance, best), rest @ ..]
            if (1..=allowed(best)).contains(distance)
                && rest.first().is_none_or(|(other, _)| other > distance) =>
        {
            Some(best)
        }
        _ => None,
    }
}

/// A word a refusal may name: short, of letters, digits, dashes and underscores only.
fn plain(word: &str) -> bool {
    (1..=NAMED).contains(&word.len())
        && word
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
}
