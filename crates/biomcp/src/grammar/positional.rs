//! Words typed where the command takes no text, or one value typed as several words, are
//! answered with the argument at fault and the call that works.
use biomcp_types::{ErrorMessage, Step, UsageArguments};
use clap::Command;
use serde_json::Value;
use std::ffi::OsString;

/// The options a stray text most often belongs to, in the order they are tried.
const TEXT_OPTIONS: [&str; 9] = [
    "query",
    "text",
    "name",
    "marker",
    "biomarker",
    "gene",
    "condition",
    "keyword",
    "drug",
];
/// Option spellings that carry search text on some other command.
const TEXT_FLAGS: [&str; 6] = ["-q", "--query", "-k", "--keyword", "--text", "--search"];
/// The words after the command that no argument took, joined into the value they meant.
pub(super) fn fix(root: &Command, path: &[&str], args: &[OsString]) -> Option<UsageArguments> {
    let &[verb, entity] = path else { return None };
    let schema = super::validation::schema(verb, entity)?;
    let command = root.find_subcommand(verb)?.find_subcommand(entity)?;
    // One text positional takes the words joined; a command without one, a text option.
    let mut positionals = command.get_positionals();
    let single = match (positionals.next(), positionals.next()) {
        (Some(arg), None) => Some(arg.get_id().to_string()).filter(|id| {
            schema
                .pointer(&format!("/properties/{id}"))
                .is_some_and(|property| super::property_type(property) != Some("array"))
        }),
        (None, _) => None,
        _ => return None,
    };
    if single.is_none() && command.get_positionals().next().is_some() {
        return None;
    }
    let (mut kept, words) = split(root, command, verb, entity, args)?;
    let text = words.join(" ");
    if single.is_some() {
        kept.push(text.clone().into());
    }
    let matches = root
        .clone()
        .try_get_matches_from(std::iter::once(OsString::from("biomcp")).chain(kept))
        .ok()?;
    let matches = matches
        .subcommand_matches(verb)?
        .subcommand_matches(entity)?;
    let value = super::request_arguments(verb, entity, matches).ok()?;
    let names = match &single {
        Some(id) => vec![id.as_str()],
        None => TEXT_OPTIONS
            .into_iter()
            .filter(|name| open_text(&schema, &value, name))
            .collect(),
    };
    names.into_iter().find_map(|name| {
        let mut candidate = value.clone();
        candidate
            .as_object_mut()?
            .insert(name.into(), Value::String(text.clone()));
        let step = complete((verb, entity), &schema, &candidate)?;
        let property = schema.get("properties")?.get(name)?;
        let argument = if property.get("x-positional").is_some() {
            name.to_owned()
        } else {
            format!("--{}", name.replace('_', "-"))
        };
        Some(UsageArguments {
            argument: Some(argument.into()),
            message: Some(ErrorMessage::ArgumentGuidance),
            valid_values: super::validation::accepted(&schema, property),
            next: super::recovery::guidance(step, [verb, entity]),
        })
    })
}

/// A search-text flag the command does not have: the same call with its text put where
/// this command takes it.
pub(super) fn text_flag(
    root: &Command,
    path: &[&str],
    spelling: &str,
    args: &[OsString],
) -> Option<Step> {
    if !TEXT_FLAGS.contains(&spelling) {
        return None;
    }
    let index = args
        .iter()
        .position(|word| word.to_str() == Some(spelling))?;
    let mut args = args.to_vec();
    args.remove(index);
    fix(root, path, &args)?.next.into_iter().next()
}

/// An option that takes one free text and that the call left empty.
fn open_text(schema: &Value, value: &Value, name: &str) -> bool {
    value.get(name).is_none()
        && schema
            .pointer(&format!("/properties/{name}"))
            .is_some_and(|property| {
                property.get("x-positional").is_none()
                    && !matches!(
                        super::property_type(property),
                        Some("array" | "boolean" | "integer")
                    )
            })
}

/// The checked call, with any required argument that has one possible value filled in.
pub(super) fn complete(path: (&str, &str), schema: &Value, value: &Value) -> Option<Step> {
    let via = biomcp_types::COMMANDS
        .iter()
        .find(|command| (command.verb, command.entity) == path)?
        .via;
    let missing = schema
        .get("required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .filter(|name| value.get(*name).is_none())
        .collect::<Vec<_>>();
    super::recovery::corrected(via, path, schema, value, &missing, false)
}

/// The call's words without the positional words after its command, and those words.
fn split(
    root: &Command,
    command: &Command,
    verb: &str,
    entity: &str,
    args: &[OsString],
) -> Option<(Vec<OsString>, Vec<String>)> {
    let start = args
        .windows(2)
        .position(|pair| pair[0].to_str() == Some(verb) && pair[1].to_str() == Some(entity))?
        + 2;
    let mut kept = args.get(..start)?.to_vec();
    let mut words = Vec::new();
    let mut rest = args.get(start..)?.iter();
    while let Some(word) = rest.next() {
        let text = word.to_str()?;
        if text == "--" {
            return None;
        }
        if !text.starts_with('-') {
            words.push(text.to_owned());
            continue;
        }
        kept.push(word.clone());
        let argument =
            super::entry::argument(command, text).or_else(|| super::entry::argument(root, text));
        if let Some((argument, false)) = argument
            && argument.get_action().takes_values()
            && let Some(value) = rest.next()
        {
            kept.push(value.clone());
        }
    }
    (!words.is_empty()).then_some((kept, words))
}

/// A positional word no argument took and no fix places: the command's last positional
/// argument, or `positional` where it takes none, with its options and help.
pub(super) fn unplaced(command: &Command, path: &[&str]) -> Option<UsageArguments> {
    let [_, _] = path else { return None };
    let argument = command
        .get_positionals()
        .last()
        .map_or_else(|| "positional".to_owned(), |arg| arg.get_id().to_string());
    let options = command
        .get_arguments()
        .filter(|argument| !argument.is_hide_set() && !argument.is_positional())
        .filter_map(|argument| argument.get_long())
        .map(|name| biodata::Text::new(&format!("--{name}")).ok())
        .collect::<Option<Vec<_>>>()?;
    Some(UsageArguments {
        argument: Some(argument.into()),
        message: None,
        valid_values: (!options.is_empty()).then(|| options.into()),
        next: vec![super::validation::help(path.iter().copied())],
    })
}

/// A source typed as an option, as in `--who-pq`, is that `--source`; a `--source` the
/// command does not take is left out when that source already fills the record.
pub(super) fn source_flag(
    root: &Command,
    path: &[&str],
    spelling: &str,
    args: &[OsString],
) -> Option<Step> {
    let &[verb, entity] = path else { return None };
    let schema = super::validation::schema(verb, entity)?;
    let index = args
        .iter()
        .position(|word| word.to_str() == Some(spelling))?;
    let mut args = args.to_vec();
    if spelling == "--source" {
        let value = args.get(index + 1)?.to_str()?.to_owned();
        if !crate::help::coverage::command_sources(verb, entity, &schema).contains(&value) {
            return None;
        }
        args.drain(index..index + 2);
    } else {
        let name = spelling.strip_prefix("--")?;
        let selector = schema.pointer("/properties/source")?;
        let known = super::validation::accepted(&schema, selector)?
            .iter()
            .any(|value| value.as_str() == name);
        if !known || args.iter().any(|word| word.to_str() == Some("--source")) {
            return None;
        }
        args.splice(index..=index, ["--source".into(), OsString::from(name)]);
    }
    let parsed = super::parse(root, &args).ok()?;
    match parsed.action? {
        action @ biomcp_query::Action::Run { .. } => {
            crate::spell::step(biomcp_types::StepKind::Configure, &action).ok()
        }
        _ => None,
    }
}
