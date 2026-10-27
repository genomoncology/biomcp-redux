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
            Some("schema" | "help")
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
    for (name, argument) in [
        (
            "schema",
            Arg::new("artifact-id")
                .index(1)
                .help("Artifact ID to print; omit it to list available schemas."),
        ),
        (
            "help",
            Arg::new("command")
                .index(1)
                .help("Command path or discovery topic to describe."),
        ),
    ] {
        if let Some(entry) = commands.iter_mut().find(|entry| entry.command == [name]) {
            entry.arguments.push(metadata(&argument));
        }
    }
    Ok(())
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
        commands.push(HelpCommand {
            reply_schemas: crate::schema::local_replies(&path),
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
        required: arg.is_required_set(),
        position: arg.get_index().map(|i| (i - 1) as u64),
        minimum: None,
        maximum: None,
        choices: Vec::new(),
    }
}
