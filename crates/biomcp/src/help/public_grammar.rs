//! Derive public global and local argument metadata from the command parser.
use biomcp_types::{HelpArgument, HelpCommand};
use clap::{Arg, Command};
use std::io;
pub(super) fn augment(commands: &mut Vec<HelpCommand>) -> io::Result<()> {
    let mut root =
        crate::grammar::root().map_err(|()| io::Error::other("command grammar unavailable"))?;
    root.build();
    for entry in commands.iter_mut() {
        match entry.command.first().map(String::as_str) {
            Some("help")
                if entry
                    .command
                    .get(1)
                    .is_some_and(|s| s.starts_with(['<', '['])) =>
            {
                entry.command.truncate(1)
            }
            Some("sync") if entry.command.get(1).is_some_and(|s| s.starts_with('[')) => {
                entry.command.truncate(1)
            }
            Some("key") => entry.command.truncate(2),
            _ => {}
        }
    }
    add(commands, &root, Vec::new(), true);
    for child in root.get_subcommands() {
        let path = vec![child.get_name().into()];
        add(commands, child, path.clone(), false);
        if child.get_name() == "key" {
            for grandchild in child.get_subcommands() {
                add(
                    commands,
                    grandchild,
                    vec!["key".into(), grandchild.get_name().into()],
                    false,
                );
            }
        }
    }
    if let Some(entry) = commands.iter_mut().find(|entry| entry.command == ["help"]) {
        entry.arguments.push(metadata(
            &Arg::new("command")
                .index(1)
                .help("Command path or discovery topic to describe."),
        ));
    }
    augment_entries(commands, &root);
    if let Some(index) = commands.iter_mut().find(|c| c.command.is_empty()) {
        index.description = root
            .get_subcommands()
            .map(|c| {
                format!(
                    "{}: {}",
                    c.get_name(),
                    c.get_about()
                        .map(ToString::to_string)
                        .unwrap_or_default()
                        .split(". ")
                        .next()
                        .unwrap_or_default()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
    }
    Ok(())
}
fn augment_entries(commands: &mut [HelpCommand], root: &Command) {
    for entry in commands.iter_mut() {
        let parser = entry
            .command
            .iter()
            .try_fold(root, |parent, name| parent.find_subcommand(name));
        if let Some(parser) = parser {
            if entry.examples.is_empty()
                && let Some(help) = parser.get_after_help()
            {
                entry.examples = help
                    .to_string()
                    .lines()
                    .filter_map(|line| line.strip_prefix("Example: biomcp ").and_then(shlex::split))
                    .collect();
            }
            for arg in parser
                .get_arguments()
                .filter(|a| !a.is_hide_set() && !a.is_global_set())
            {
                let value = metadata(arg);
                if let Some(existing) = entry.arguments.iter_mut().find(|a| a.name == value.name) {
                    existing.aliases = value.aliases;
                    if existing.default.is_none() {
                        existing.default = value.default;
                    }
                } else {
                    entry.arguments.push(value);
                }
            }
        }
    }
}
fn add(commands: &mut Vec<HelpCommand>, parser: &Command, path: Vec<String>, root: bool) {
    let mut arguments: Vec<_> = parser
        .get_arguments()
        .filter(|arg| !arg.is_hide_set() && (root || !arg.is_global_set()))
        .map(metadata)
        .collect();
    if root {
        for alias in crate::grammar::entry_aliases() {
            arguments.push(HelpArgument {
                name: alias.flags().join(", "),
                description: alias.description().into(),
                value_type: "boolean".into(),
                aliases: Vec::new(),
                default: Some("false".into()),
                required: false,
                position: None,
                minimum: None,
                maximum: None,
                choices: Vec::new(),
            });
        }
    }
    if let Some(entry) = commands.iter_mut().find(|entry| entry.command == path) {
        entry.arguments = arguments;
    } else {
        let mut reply_schemas = crate::schema::local_replies(&path);
        if !path.is_empty() {
            reply_schemas.extend(
                commands
                    .iter()
                    .filter(|entry| entry.command.starts_with(&path))
                    .flat_map(|entry| entry.reply_schemas.iter().cloned()),
            );
        }
        reply_schemas.sort();
        reply_schemas.dedup();
        commands.push(HelpCommand {
            reply_schemas,
            command: path,
            description: parser
                .get_about()
                .map(ToString::to_string)
                .unwrap_or_default(),
            arguments,
            sections: Vec::new(),
            section_rule: None,
            examples: Vec::new(),
        });
    }
}
fn metadata(arg: &Arg) -> HelpArgument {
    HelpArgument {
        name: arg
            .get_long()
            .map_or_else(|| arg.get_id().to_string(), |name| format!("--{name}")),
        description: arg.get_help().map(ToString::to_string).unwrap_or_default(),
        value_type: if arg.get_action().takes_values() {
            "string"
        } else {
            "boolean"
        }
        .into(),
        aliases: arg
            .get_short()
            .map(|c| format!("-{c}"))
            .into_iter()
            .collect(),
        default: default(arg),
        required: arg.is_required_set(),
        position: arg.get_index().map(|i| (i - 1) as u64),
        minimum: None,
        maximum: None,
        choices: arg
            .get_value_parser()
            .possible_values()
            .into_iter()
            .flatten()
            .map(|v| v.get_name().to_owned())
            .collect(),
    }
}

pub(super) fn scoped_globals(commands: &mut [HelpCommand]) {
    let mut parser = crate::grammar::global_arguments();
    parser.build();
    for command in commands {
        for argument in parser.get_arguments().filter(|arg| !arg.is_hide_set()) {
            let value = metadata(argument);
            if !command.arguments.iter().any(|arg| arg.name == value.name) {
                command.arguments.push(value);
            }
        }
    }
}

fn default(arg: &Arg) -> Option<String> {
    if let Some(value) = arg.get_default_values().first() {
        return Some(value.to_string_lossy().into_owned());
    }
    // Ask the parser for an optional flag's omitted value instead of copying it.
    if arg.get_action().takes_values()
        && arg
            .get_num_args()
            .is_some_and(|range| range.min_values() == 0)
        && let Some(name) = arg.get_long()
        && let Ok(matches) = Command::new("help")
            .arg(arg.clone())
            .try_get_matches_from(["help".to_owned(), format!("--{name}")])
    {
        return matches
            .try_get_one::<String>(arg.get_id().as_str())
            .ok()
            .flatten()
            .cloned();
    }
    None
}
