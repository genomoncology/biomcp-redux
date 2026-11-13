//! A section word the command does not have is answered with the call that works.
//!
//! The fix is derived from what the command already declares: a section another name stands
//! for, the sections a named source publishes, the source selector, and the label headings.
//! It is offered as the next call and never run in place of the one given.
use biomcp_types::{ErrorMessage, Source, Step, UsageArguments, UsageError};
use serde_json::{Map, Value, json};

/// Section words agents type for a section under its own name, by entity.
const SYNONYMS: &[(&str, &str, &str)] = &[
    ("article", "text", "fulltext"),
    ("article", "full-text", "fulltext"),
    ("article", "full_text", "fulltext"),
    ("article", "body", "fulltext"),
];

/// Refuse a call whose section word is wrong with the corrected call, when one checks.
pub(super) fn fix(verb: &str, entity: &str, value: &Value) -> Option<super::ParseInvalid> {
    let schema = super::validation::schema(verb, entity)?;
    let property = schema.get("properties")?.get("section")?;
    let words = super::validation::accepted(&schema, property)?;
    let valid = |word: &str| words.iter().any(|known| known.as_str() == word);
    let (word, mut candidate) = wrong_word(value, property, &valid)?;
    replace(verb, entity, &schema, &word, candidate.as_object_mut()?)?;
    refusal((verb, entity), &schema, &candidate)
}

/// Refuse a label read that names a label heading as a second section.
pub(super) fn label(verb: &str, entity: &str, value: &Value) -> Option<super::ParseInvalid> {
    let schema = super::validation::schema(verb, entity)?;
    let candidate = label_word(&schema, value)?;
    refusal((verb, entity), &schema, &candidate)
}

/// The section refusal whose next call is the checked candidate.
fn refusal(path: (&str, &str), schema: &Value, candidate: &Value) -> Option<super::ParseInvalid> {
    let property = schema.get("properties")?.get("section")?;
    let step = super::positional::complete(path, schema, candidate)?;
    let argument = if property.get("x-positional").is_some() {
        "section"
    } else {
        "--section"
    };
    Some(super::ParseInvalid {
        argument: None,
        usage: Some(UsageError::Arguments(UsageArguments {
            argument: Some(argument.into()),
            message: Some(ErrorMessage::ArgumentGuidance),
            valid_values: super::validation::accepted(schema, property),
            next: super::recovery::guidance(step, [path.0, path.1]),
        })),
        prefer_argument_error: true,
    })
}

/// A label read with a second section that names a label heading, as in
/// `label --section indications`: that heading is the label section.
fn label_word(schema: &Value, value: &Value) -> Option<Value> {
    schema.pointer("/properties/label_section")?;
    if value.get("label_section").is_some() {
        return None;
    }
    let mut candidate = value.clone();
    let fields = candidate.as_object_mut()?;
    let items = fields.get_mut("section")?.as_array_mut()?;
    if !items.contains(&json!("label")) {
        return None;
    }
    let index = items.iter().position(|item| {
        item.as_str()
            .is_some_and(|word| word != "label" && heading(word).is_some())
    })?;
    let word = items.remove(index);
    let heading = heading(word.as_str()?)?;
    fields.insert("label_section".into(), json!(heading));
    Some(candidate)
}

/// The first section word the command does not have, and the call without it. A command
/// whose trailing words are sections reads an unknown one as a record ID, so its last
/// record word is the candidate there.
fn wrong_word(
    value: &Value,
    property: &Value,
    valid: &dyn Fn(&str) -> bool,
) -> Option<(String, Value)> {
    let mut candidate = value.clone();
    let fields = candidate.as_object_mut()?;
    let wrong = match fields.get("section") {
        Some(Value::String(word)) if !valid(word) => Some(None),
        Some(Value::Array(items)) => items
            .iter()
            .position(|item| item.as_str().is_some_and(|word| !valid(word)))
            .map(Some),
        _ => None,
    };
    let word = match wrong {
        Some(None) => fields.remove("section")?,
        Some(Some(index)) => {
            let items = fields.get_mut("section")?.as_array_mut()?;
            let word = items.remove(index);
            if items.is_empty() {
                fields.remove("section");
            }
            word
        }
        None if property.get("x-trailing-section") == Some(&json!(true)) => {
            let input = fields.get_mut("input")?.as_array_mut()?;
            if input.len() < 2 {
                return None;
            }
            input.pop()?
        }
        None => return None,
    };
    Some((word.as_str()?.to_owned(), candidate))
}

/// Edit the call so it asks for what the wrong word meant.
fn replace(
    verb: &str,
    entity: &str,
    schema: &Value,
    word: &str,
    fields: &mut Map<String, Value>,
) -> Option<()> {
    let lower = word.to_ascii_lowercase();
    if let Some((_, _, section)) = SYNONYMS
        .iter()
        .find(|(owner, synonym, _)| *owner == entity && *synonym == lower)
    {
        return add(fields, schema, section);
    }
    if let Ok(source) = lower.parse::<Source>() {
        return by_source(verb, entity, schema, fields, source);
    }
    let heading = heading(&lower)?;
    schema.pointer("/properties/label_section")?;
    add(fields, schema, "label")?;
    fields.insert("label_section".into(), json!(heading));
    Some(())
}

/// A source named as a section: the sections it publishes here, else the source selector,
/// else the record card that source already fills.
fn by_source(
    verb: &str,
    entity: &str,
    schema: &Value,
    fields: &mut Map<String, Value>,
    source: Source,
) -> Option<()> {
    let property = schema.pointer("/properties/section")?;
    let published = super::validation::accepted(schema, property)?
        .iter()
        .map(|word| word.as_str().to_owned())
        // all and drug markets read every source, so neither is the one a source publishes.
        .filter(|word| word != "all" && !(entity == "drug" && word == "markets"))
        .find(|word| {
            biomcp_types::command_section_sources(verb, entity, word, &[source])
                .is_some_and(|sources| sources.contains(&source))
        });
    if let Some(section) = published {
        return add(fields, schema, &section);
    }
    let name = source.to_string();
    if let Some(selector) = schema.pointer("/properties/source")
        && super::validation::accepted(schema, selector)
            .is_some_and(|values| values.iter().any(|value| value.as_str() == name))
    {
        let value = if super::property_type(selector) == Some("array") {
            json!([name])
        } else {
            json!(name)
        };
        fields.insert("source".into(), value);
        return Some(());
    }
    crate::help::coverage::command_sources(verb, entity, schema)
        .contains(&name)
        .then_some(())
}

/// Add one section word, keeping any the call already named.
fn add(fields: &mut Map<String, Value>, schema: &Value, section: &str) -> Option<()> {
    let property = schema.pointer("/properties/section")?;
    if super::property_type(property) == Some("array") {
        let items = fields
            .entry("section")
            .or_insert_with(|| json!([]))
            .as_array_mut()?;
        if !items.contains(&json!(section)) {
            items.push(json!(section));
        }
    } else {
        fields.insert("section".into(), json!(section));
    }
    Some(())
}

/// The one label heading a word names exactly or begins.
fn heading(word: &str) -> Option<&'static str> {
    let headings = biomcp_types::OPENFDA_LABEL_HEADINGS;
    if let Some(exact) = headings.iter().find(|heading| **heading == word) {
        return Some(exact);
    }
    let prefix = format!("{word}_");
    let mut matches = headings
        .iter()
        .filter(|heading| heading.starts_with(&prefix) && !heading.ends_with("_table"));
    let first = matches.next()?;
    matches.next().is_none().then_some(*first)
}

/// A section word typed after a record word that took the section's place, as in
/// `get pgx G6PD recommendations`: the record word moves to the command's record option.
pub(super) fn misplaced(
    path: (&str, &str),
    schema: &Value,
    value: &Value,
    word: &str,
) -> Option<Step> {
    let property = schema.pointer("/properties/section")?;
    let words = super::validation::accepted(schema, property)?;
    let known = |word: &str| words.iter().any(|known| known.as_str() == word);
    let current = value.get("section")?.as_str()?;
    if !known(word) || known(current) {
        return None;
    }
    let record = ["gene", "input", "id"].into_iter().find(|name| {
        value.get(*name).is_none()
            && schema
                .pointer(&format!("/properties/{name}"))
                .is_some_and(|property| property.get("x-positional").is_none())
    })?;
    let mut candidate = value.clone();
    let fields = candidate.as_object_mut()?;
    fields.insert("section".into(), json!(word));
    fields.insert(record.into(), json!(current));
    super::positional::complete(path, schema, &candidate)
}
