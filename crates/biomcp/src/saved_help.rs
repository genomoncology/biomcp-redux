//! Command-line help for saved work, kept out of the shared help MCP reads.
use crate::command::{Encoding, Outcome};
use crate::exit::Status as Exit;
use crate::saved::{SAVE_AS, grammar, help_usage, usage};
use biomcp_types::{
    Envelope, Help, HelpArgument, HelpCommand, HelpQuery, ResponseBody, Saved, Status, UsageError,
};
use clap::{Arg, ArgAction, Command};
use std::{ffi::OsString, io};

/// Command-line help for saved work and the search option that saves.
pub(crate) fn help(
    root: &crate::grammar::Root,
    settings: &crate::config::Settings,
    args: &[OsString],
    rest: &[&str],
) -> io::Result<Outcome> {
    let encoding = Encoding::cli(root, settings, args);
    let mut commands = vec![HelpCommand {
        command: vec![
            "search".into(),
            "<entity>".into(),
            SAVE_AS.into(),
            "NAME".into(),
        ],
        description: "Save a search under NAME when it succeeds; its words, without options \
            such as --format or --config, are what runs again. biomcp saved show lists it."
            .into(),
        reply_schemas: Vec::new(),
        arguments: Vec::new(),
        sections: Vec::new(),
        section_rule: None,
        examples: vec![
            [
                "search",
                "trial",
                "--condition",
                "melanoma",
                SAVE_AS,
                "melanoma-trials",
            ]
            .map(String::from)
            .to_vec(),
        ],
    }];
    walk(&grammar(), vec!["saved".into()], &mut commands);
    let path: Vec<String> = std::iter::once("saved")
        .chain(rest.iter().copied())
        .map(String::from)
        .collect();
    commands.retain(|command| path.len() == 1 || command.command.starts_with(&path));
    if commands.is_empty() {
        return usage(encoding, help_usage(UsageError::arguments()));
    }
    encoding.encode(
        Envelope::new(
            Status::Complete,
            HelpQuery { command: path },
            Some(Help { commands }),
        ),
        Exit::Success,
    )
}

fn walk(command: &Command, path: Vec<String>, commands: &mut Vec<HelpCommand>) {
    if command.get_subcommands().next().is_none() {
        commands.push(HelpCommand {
            command: path.clone(),
            description: command
                .get_about()
                .map(ToString::to_string)
                .unwrap_or_default(),
            reply_schemas: vec![Saved::SCHEMA.into()],
            arguments: command.get_arguments().map(argument).collect(),
            sections: Vec::new(),
            section_rule: None,
            examples: command
                .get_after_help()
                .map(ToString::to_string)
                .unwrap_or_default()
                .lines()
                .filter_map(|line| line.strip_prefix("Example: biomcp "))
                .filter_map(shlex::split)
                .collect(),
        });
    }
    for child in command.get_subcommands() {
        let mut child_path = path.clone();
        child_path.push(child.get_name().into());
        walk(child, child_path, commands);
    }
}

fn argument(arg: &Arg) -> HelpArgument {
    HelpArgument {
        name: arg.get_long().map_or_else(
            || arg.get_id().to_string().to_uppercase(),
            |name| format!("--{name}"),
        ),
        description: arg.get_help().map(ToString::to_string).unwrap_or_default(),
        value_type: if matches!(arg.get_action(), ArgAction::SetTrue) {
            "boolean"
        } else {
            "string"
        }
        .into(),
        aliases: Vec::new(),
        default: None,
        required: arg.is_required_set(),
        position: arg.get_index().map(|index| (index - 1) as u64),
        minimum: None,
        maximum: None,
        choices: Vec::new(),
    }
}

/// The command-line root help line for saved work; MCP help never carries it.
pub(crate) fn root_help() -> HelpCommand {
    HelpCommand {
        command: vec!["saved".into()],
        description: "Keep named searches (search ... --save-as NAME), lists of record IDs and \
            notes on this machine; command line only. Run biomcp help saved."
            .into(),
        reply_schemas: vec![Saved::SCHEMA.into()],
        arguments: Vec::new(),
        sections: Vec::new(),
        section_rule: None,
        examples: Vec::new(),
    }
}

/// `-h` or `--help` under `saved` answers as `help saved ...` does, unless it is an option's value.
pub(crate) fn asks_help(args: &[OsString]) -> bool {
    let parser = crate::grammar::saved_parser();
    let mut node = &parser;
    let mut words = args.iter().take_while(|arg| *arg != "--");
    while let Some(word) = words.next() {
        let Some(word) = word.to_str() else {
            continue;
        };
        if word == "-h" || word == "--help" {
            return true;
        }
        if let Some(command) = node.find_subcommand(word) {
            node = command;
        } else if let Some(long) = word.strip_prefix("--").filter(|word| !word.contains('='))
            && parser
                .get_arguments()
                .chain(node.get_arguments())
                .find(|arg| arg.get_long() == Some(long))
                .is_some_and(|arg| matches!(arg.get_action(), ArgAction::Set | ArgAction::Append))
        {
            words.next();
        }
    }
    false
}
