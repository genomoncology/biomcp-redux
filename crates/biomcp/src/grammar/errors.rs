//! Recover grammar-owned argument identities without copying rejected values.
use clap::{Command, error::{ContextKind, ContextValue}};
use std::ffi::OsString;
pub(super) fn argument_error(args: &[OsString]) -> Option<String> {
    let parser = super::root().ok()?;
    let error = parser.clone().try_get_matches_from(std::iter::once(OsString::from("biomcp")).chain(args.iter().cloned())).err()?;
    let context = error.get(ContextKind::InvalidArg)?;
    let spellings = match context {
        ContextValue::String(value) => vec![value.as_str()],
        ContextValue::Strings(values) => values.iter().map(String::as_str).collect(),
        _ => return None,
    };
    identity(&parser, &spellings)
}
fn identity(command: &Command, spellings: &[&str]) -> Option<String> {
    command.get_arguments().find_map(|argument| {
        let rendered = argument.to_string();
        spellings.contains(&rendered.as_str()).then(|| argument.get_long().map_or_else(|| argument.get_id().to_string(), |name| format!("--{name}")))
    }).or_else(|| command.get_subcommands().find_map(|child| identity(child, spellings)))
}
