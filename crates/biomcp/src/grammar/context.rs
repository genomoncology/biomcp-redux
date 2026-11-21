//! Name the command a parser refusal belongs to, and fix an option given without its value.
use biomcp_types::{
    DiseaseSection, ErrorMessage, GeneSection, Rewrite, Source, Step, UsageArguments, UsageError,
};
use clap::{
    Command,
    error::{ContextKind, ContextValue, ErrorKind},
};
use std::ffi::OsString;

/// A publisher passed as --source can name a gene section or a disease curation lookup.
pub(super) fn gene_source_refusal(
    root: &Command,
    path: &[&str],
    spelling: Option<&str>,
    args: &[OsString],
) -> Option<super::ParseInvalid> {
    let [verb @ ("get" | "search"), "gene"] = path else {
        return None;
    };
    if spelling?.split('=').next() != Some("--source") {
        return None;
    }
    let index = args
        .iter()
        .take_while(|word| *word != "--")
        .position(|word| {
            word.to_str().and_then(|word| word.split('=').next()) == Some("--source")
        })?;
    let (name, remaining) = super::positional::source_value(args, index)?;
    let source = name.to_ascii_lowercase().parse::<Source>().ok()?;
    if *verb == "get" {
        return super::sections::gene_source_refusal(root, remaining, source);
    }
    let section = match source {
        Source::ClinGen => GeneSection::ClinGen,
        Source::Gencc => GeneSection::Gencc,
        _ => return None,
    };
    let help = super::validation::help(["get", "gene", section.into()]);
    let next = disease_curation(root, remaining, source)
        .into_iter()
        .chain([help])
        .collect();
    Some(super::ParseInvalid {
        usage: Some(UsageError::Arguments(UsageArguments {
            argument: Some("--source".into()),
            message: Some(ErrorMessage::GeneSearchSource),
            valid_values: None,
            next,
        })),
        ..Default::default()
    })
}

fn disease_curation(root: &Command, args: Vec<OsString>, source: Source) -> Option<Step> {
    let matches = root
        .clone()
        .try_get_matches_from(std::iter::once(OsString::from("biomcp")).chain(args))
        .ok()?;
    let matches = matches
        .subcommand_matches("search")?
        .subcommand_matches("gene")?;
    let search = super::request_arguments("search", "gene", matches).ok()?;
    let name = search.get("query")?.as_str()?;
    super::recovery::checked(
        Rewrite::Direct,
        ("get", "disease"),
        serde_json::json!({"id": name, "section": DiseaseSection::GeneValidity, "source": source}),
    )
}

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
        && let Some(found) = stripped(root, error, args).and_then(|args| parsed(root, &args))
    {
        return found;
    }
    (command, path)
}

/// Whether the parser refused an option for coming with no value at all.
pub(super) fn valueless(error: &clap::Error) -> bool {
    matches!(
        error.kind(),
        ErrorKind::InvalidValue | ErrorKind::ValueValidation
    ) && matches!(error.get(ContextKind::InvalidValue), Some(ContextValue::String(value)) if value.is_empty())
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
    let args = stripped(root, error, args)?;
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

/// The caller's words without the last spelling of the valueless option: its long or
/// one-letter form or an alias, alone or with an empty inline value.
fn stripped(root: &Command, error: &clap::Error, args: &[OsString]) -> Option<Vec<OsString>> {
    if !valueless(error) {
        return None;
    }
    let Some(ContextValue::String(spelling)) = error.get(ContextKind::InvalidArg) else {
        return None;
    };
    let mut flags = Vec::new();
    spellings(root, spelling, &mut flags);
    let index = args.iter().enumerate().rposition(|(index, word)| {
        let word = word.to_str().unwrap_or_default();
        let next = args.get(index + 1).and_then(|word| word.to_str());
        flags.iter().any(|flag| {
            word.strip_prefix(flag.as_str()) == Some("=")
                || word == flag && next.is_none_or(|next| next.starts_with('-'))
        })
    })?;
    let mut args = args.to_vec();
    args.remove(index);
    Some(args)
}

/// Every way a caller writes the refused option, on any command that declares it.
pub(super) fn spellings(command: &Command, spelling: &str, flags: &mut Vec<String>) {
    for argument in command
        .get_arguments()
        .filter(|argument| argument.to_string() == spelling)
    {
        let longs = argument
            .get_long()
            .into_iter()
            .chain(argument.get_all_aliases().into_iter().flatten());
        let shorts = argument
            .get_short()
            .into_iter()
            .chain(argument.get_all_short_aliases().into_iter().flatten());
        flags.extend(
            longs
                .map(|long| format!("--{long}"))
                .chain(shorts.map(|short| format!("-{short}"))),
        );
    }
    for child in command.get_subcommands() {
        spellings(child, spelling, flags);
    }
}

/// The command path the parser reads from the call, ignoring its other mistakes, such as a
/// required argument the call also left out.
fn parsed<'a>(root: &'a Command, args: &[OsString]) -> Option<(&'a Command, Vec<&'a str>)> {
    let matches = lenient(root.clone())
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

/// The parser with every command's errors ignored.
fn lenient(command: Command) -> Command {
    command.ignore_errors(true).mut_subcommands(lenient)
}
