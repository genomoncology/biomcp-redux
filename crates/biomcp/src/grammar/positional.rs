//! Words typed where the command takes no text, or one value typed as several words, are
//! answered with the argument at fault and the call that works.
use biomcp_types::{ErrorMessage, Step, UsageArguments};
use clap::{Arg, ArgAction, Command};
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
const STRAY: &str = "stray_text";

/// The words after the command that no argument took, joined into the value they meant.
pub(super) fn fix(root: &Command, path: &[&str], args: &[OsString]) -> Option<UsageArguments> {
    let &[verb, entity] = path else { return None };
    let schema = super::validation::schema(verb, entity)?;
    let command = root.find_subcommand(verb)?.find_subcommand(entity)?;
    // A single text positional takes several words; a command without one gains a slot.
    let single = command
        .get_positionals()
        .find(|arg| arg.get_index() == Some(1))
        .filter(|arg| {
            schema
                .pointer(&format!("/properties/{}", arg.get_id()))
                .is_some_and(|property| super::property_type(property) != Some("array"))
        })
        .map(|arg| arg.get_id().to_string());
    let positionals = command.get_positionals().count();
    let widened = root.clone().mut_subcommand(verb, |command| {
        command.mut_subcommand(entity, |command| match &single {
            Some(id) => command.mut_arg(id, |arg| arg.num_args(1..)),
            None => command.arg(
                Arg::new(STRAY)
                    .index(positionals + 1)
                    .num_args(1..)
                    .action(ArgAction::Append),
            ),
        })
    });
    let matches = widened
        .try_get_matches_from(std::iter::once(OsString::from("biomcp")).chain(args.iter().cloned()))
        .ok()?;
    let matches = matches
        .subcommand_matches(verb)?
        .subcommand_matches(entity)?;
    let id = single.as_deref().unwrap_or(STRAY);
    let text = matches
        .get_many::<String>(id)?
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ");
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
