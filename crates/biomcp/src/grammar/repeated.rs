//! An option given more than once: name that cause, and join a comma-separated option's values.
use clap::{
    Command,
    error::{ContextKind, ContextValue, ErrorKind},
};
use std::ffi::OsString;

/// Whether the parser refused one option for appearing twice, not two options together.
pub(super) fn is(error: &clap::Error) -> bool {
    error.kind() == ErrorKind::ArgumentConflict
        && matches!(
            (error.get(ContextKind::InvalidArg), error.get(ContextKind::PriorArg)),
            (Some(ContextValue::String(arg)), Some(ContextValue::String(prior))) if arg == prior
        )
}

/// The same call with a repeated comma-separated option given once, its values joined,
/// offered only when the registry's checked constructor accepts it.
pub(super) fn joined(
    root: &Command,
    path: &[&str],
    error: &clap::Error,
    args: &[OsString],
) -> Option<biomcp_types::Step> {
    let [verb, entity] = path else { return None };
    let Some(ContextValue::String(spelling)) = error.get(ContextKind::InvalidArg) else {
        return None;
    };
    if !is(error) {
        return None;
    }
    let command = root.find_subcommand(verb)?.find_subcommand(entity)?;
    let argument = command
        .get_arguments()
        .find(|argument| argument.to_string() == *spelling)?;
    let schema = super::validation::schema(verb, entity)?;
    schema
        .get("properties")?
        .get(argument.get_id().as_str())?
        .get("x-comma-separated")?;
    let long = format!("--{}", argument.get_long()?);
    let inline = format!("{long}=");
    let (mut kept, mut values) = (Vec::new(), Vec::new());
    let mut words = args.iter();
    while let Some(word) = words.next() {
        let text = word.to_str()?;
        if text == long {
            values.push(words.next()?.to_str()?);
        } else if let Some(value) = text.strip_prefix(&inline) {
            values.push(value);
        } else {
            kept.push(word.clone());
        }
    }
    kept.extend([OsString::from(&long), OsString::from(values.join(","))]);
    let matches = root
        .clone()
        .try_get_matches_from(std::iter::once(OsString::from("biomcp")).chain(kept))
        .ok()?;
    let matches = matches
        .subcommand_matches(verb)?
        .subcommand_matches(entity)?;
    let value = super::request_arguments(verb, entity, matches).ok()?;
    let declaration = biomcp_types::COMMANDS
        .iter()
        .find(|command| (command.verb, command.entity) == (*verb, *entity))?;
    super::recovery::checked(declaration.via, (verb, entity), value)
}
