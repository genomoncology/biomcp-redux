//! Name the command a parser refusal belongs to, and fix an option given without its value.
use clap::{
    Command,
    error::{ContextKind, ContextValue, ErrorKind},
};
use std::ffi::OsString;

/// The refused command and its path. The parser's usage context names it, including when
/// parsing stopped before matches could be returned. An option given without its value
/// carries no usage, so the parser names the path from the same call without that option.
pub(super) fn command<'a>(
    root: &'a Command,
    error: &clap::Error,
    args: &[OsString],
) -> (&'a Command, Vec<&'a str>) {
    let usage = error
        .get(ContextKind::Usage)
        .map(ToString::to_string)
        .unwrap_or_default();
    let mut words = usage
        .split_whitespace()
        .skip_while(|word| *word != root.get_name());
    words.next();
    let mut command = root;
    let mut path = Vec::new();
    for word in words {
        let Some(child) = command.find_subcommand(word) else {
            break;
        };
        path.push(child.get_name());
        command = child;
    }
    if path.is_empty()
        && let Some(found) = stripped(error, args).and_then(|args| parsed(root, &args))
    {
        return found;
    }
    (command, path)
}

/// Whether the parser refused an option for coming with no value at all.
pub(super) fn valueless(error: &clap::Error) -> bool {
    error.kind() == ErrorKind::InvalidValue
        && matches!(error.get(ContextKind::InvalidValue), Some(ContextValue::String(value)) if value.is_empty())
}

/// The call without the option given no value, when the registry's checked constructor
/// accepts it; the option's default, or no filter, then applies.
pub(super) fn without(
    root: &Command,
    path: &[&str],
    error: &clap::Error,
    args: &[OsString],
) -> Option<biomcp_types::Step> {
    let [verb, entity] = path else { return None };
    let args = stripped(error, args)?;
    let matches = root
        .clone()
        .try_get_matches_from(std::iter::once(OsString::from("biomcp")).chain(args))
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

/// The caller's words without the last spelling of the valueless option.
fn stripped(error: &clap::Error, args: &[OsString]) -> Option<Vec<OsString>> {
    if !valueless(error) {
        return None;
    }
    let Some(ContextValue::String(spelling)) = error.get(ContextKind::InvalidArg) else {
        return None;
    };
    let flag = spelling.split_whitespace().next()?;
    let inline = format!("{flag}=");
    let index = args.iter().enumerate().rposition(|(index, word)| {
        let next = args.get(index + 1).and_then(|word| word.to_str());
        word.to_str() == Some(inline.as_str())
            || word.to_str() == Some(flag) && next.is_none_or(|next| next.starts_with('-'))
    })?;
    let mut args = args.to_vec();
    args.remove(index);
    Some(args)
}

fn parsed<'a>(root: &'a Command, args: &[OsString]) -> Option<(&'a Command, Vec<&'a str>)> {
    let matches = root
        .clone()
        .try_get_matches_from(std::iter::once(OsString::from("biomcp")).chain(args.iter().cloned()))
        .ok()?;
    let (mut command, mut found, mut path) = (root, &matches, Vec::new());
    while let Some((name, next)) = found.subcommand() {
        command = command.find_subcommand(name)?;
        path.push(command.get_name());
        found = next;
    }
    (!path.is_empty()).then_some((command, path))
}

#[cfg(test)]
mod tests {
    use biomcp_types::{ErrorMessage, UsageError};

    /// Every option that takes a value, given none, is named as missing with a next step (1064).
    #[test]
    fn every_value_option_given_no_value_is_named_missing() -> Result<(), String> {
        let root = super::super::root().map_err(|_| "grammar unavailable")?;
        let mut failures = Vec::new();
        for (verb, entity, _) in super::super::command_paths() {
            let Some(command) = root
                .find_subcommand(verb)
                .and_then(|command| command.find_subcommand(entity))
            else {
                continue;
            };
            for argument in command.get_arguments() {
                let takes = argument.get_action().takes_values()
                    && argument
                        .get_num_args()
                        .is_some_and(|range| range.min_values() > 0);
                let Some(long) = argument.get_long().filter(|_| takes) else {
                    continue;
                };
                let flag = format!("--{long}");
                let args = [verb, entity, flag.as_str()].map(std::ffi::OsString::from);
                let outcome = super::super::parse(&root, &args);
                let named = match &outcome {
                    Err(super::super::ParseInvalid {
                        usage: Some(UsageError::Arguments(refusal)),
                        ..
                    }) => {
                        refusal.argument.as_deref() == Some(flag.as_str())
                            && refusal.message == Some(ErrorMessage::ArgumentMissing)
                            && !refusal.next.is_empty()
                    }
                    _ => false,
                };
                if !named {
                    let usage = outcome.err().map(|invalid| format!("{:?}", invalid.usage));
                    failures.push(format!("{verb} {entity} {flag}: {usage:?}"));
                }
            }
        }
        if failures.is_empty() {
            Ok(())
        } else {
            Err(failures.join("\n"))
        }
    }
}
