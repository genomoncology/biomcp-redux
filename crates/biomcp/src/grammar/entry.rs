//! Public help aliases and recovery paths use only command grammar identities.
use clap::Command;
use std::ffi::OsString;

#[derive(Clone, Copy)]
pub(crate) enum EntryAlias {
    Help,
    Version,
}
impl EntryAlias {
    pub(crate) fn flags(self) -> &'static [&'static str] {
        match self {
            Self::Help => &["-h", "--help"],
            Self::Version => &["--version"],
        }
    }
    pub(crate) fn description(self) -> &'static str {
        match self {
            Self::Help => "Print scoped command help without source requests.",
            Self::Version => "Print the installed version without source requests.",
        }
    }
    fn parse(word: &str) -> Option<Self> {
        entry_aliases().iter().copied().find(|alias| alias.flags().contains(&word))
    }
}

pub(crate) fn entry_aliases() -> &'static [EntryAlias] {
    &[EntryAlias::Help, EntryAlias::Version]
}

pub(super) fn arguments(args: &[OsString], root: &Command) -> Vec<OsString> {
    if args.is_empty() {
        return vec!["help".into()];
    }
    let mut command = root;
    let mut path = Vec::new();
    let mut globals = Vec::new();
    let mut words = args.iter().peekable();
    while let Some(word) = words.next() {
        let Some(text) = word.to_str() else { break };
        if text == "--" {
            break;
        }
        if matches!(EntryAlias::parse(text), Some(EntryAlias::Help)) {
            globals.push("help".into());
            globals.extend(path.into_iter().filter(|name| *name != "help").map(OsString::from));
            return globals;
        }
        if matches!(EntryAlias::parse(text), Some(EntryAlias::Version)) && path.is_empty() {
            globals.push("version".into());
            globals.extend(words.cloned());
            return globals;
        }
        if let Some(child) = command.find_subcommand(text) {
            path.push(child.get_name());
            command = child;
            continue;
        }
        if !text.starts_with('-') && command.get_subcommands().next().is_some() {
            return args.to_vec();
        }
        if let Some((argument, inline)) = argument(command, text) {
            let is_global = argument.is_global_set();
            if is_global {
                globals.push(word.clone());
            }
            if takes_value(argument, &mut words) && !inline {
                if let Some(value) = words.next() {
                    if is_global {
                        globals.push(value.clone());
                    }
                }
            }
        }
    }
    args.to_vec()
}

fn argument<'a>(command: &'a Command, word: &str) -> Option<(&'a clap::Arg, bool)> {
    let (name, inline) = word.split_once('=').map_or((word, false), |(name, _)| (name, true));
    command.get_arguments().find(|arg| {
        name.strip_prefix("--").is_some_and(|name| arg.get_long() == Some(name))
            || name.strip_prefix('-').is_some_and(|name| {
                name.len() == 1 && arg.get_short().is_some_and(|short| name.starts_with(short))
            })
    }).map(|arg| (arg, inline))
}

fn takes_value(
    argument: &clap::Arg,
    words: &mut std::iter::Peekable<std::slice::Iter<'_, OsString>>,
) -> bool {
    argument.get_action().takes_values()
        && !(argument.get_num_args().is_some_and(|range| range.min_values() == 0)
            && words.peek().is_some_and(|word| word.to_str().is_some_and(|word| word.starts_with('-'))))
}

pub(super) fn unknown_command(args: &[OsString], root: &Command) -> Option<Vec<String>> {
    if super::command_words(args).next() == Some("schema") {
        return None;
    }
    let mut command = root;
    let mut path: Vec<String> = Vec::new();
    let mut words = args.iter().peekable();
    while let Some(word) = words.next().and_then(|word| word.to_str()) {
        if word == "--" || path.first().is_some_and(|word| word == "schema") {
            return None;
        }
        if let Some((argument, inline)) = argument(command, word) {
            if takes_value(argument, &mut words) && !inline {
                words.next();
            }
            continue;
        }
        if let Some(child) = command.find_subcommand(word) {
            if path.is_empty() && word == "help" {
                continue;
            }
            path.push(child.get_name().to_owned());
            command = child;
            continue;
        }
        if word.starts_with('-') && path.first().is_none_or(|name| name != "serve") {
            return None;
        }
        if command.get_subcommands().next().is_some() {
            let nearest = command.get_subcommands().min_by_key(|child| {
                distance(word, child.get_name())
            })?;
            path.push(nearest.get_name().to_owned());
            return Some(path);
        }
        return (path.first().is_some_and(|name| name == "serve") && word.starts_with('-'))
            .then_some(path);
    }
    None
}

// Bound comparisons of untrusted text while keeping allocation proportional to known names.
fn distance(input: &str, known: &str) -> usize {
    let mut row: Vec<_> = (0..=known.len()).collect();
    for (index, left) in input.chars().take(128).enumerate() {
        let mut diagonal = row[0];
        row[0] = index + 1;
        for (column, right) in known.bytes().enumerate() {
            let prior = row[column + 1];
            row[column + 1] = (row[column] + 1)
                .min(prior + 1)
                .min(diagonal + usize::from(left != char::from(right)));
            diagonal = prior;
        }
    }
    row[known.len()]
}

pub(super) fn help_path(verb: &str, matches: &clap::ArgMatches) -> Result<Option<Vec<String>>, ()> {
    Ok(if verb == "help" {
        match matches.subcommand() {
            Some(("key", matches)) if matches.get_one::<String>("source").is_some() => None,
            Some((name @ ("version" | "mcp-config" | "sources" | "config"), _)) => {
                Some(vec![name.into()])
            }
            Some(("author", matches)) if matches.subcommand().is_none() => {
                Some(vec!["author".into()])
            }
            Some(("network", matches)) => Some(
                std::iter::once("network".into())
                    .chain(matches.get_one::<String>("setting").cloned())
                    .collect(),
            ),
            Some((verb, matches)) => {
                Some(std::iter::once(verb.into())
                    .chain(matches.subcommand().map(|(name, _)| name.into()))
                    .collect())
            }
            None => Some(Vec::new()),
        }
    } else {
        None
    })
}
