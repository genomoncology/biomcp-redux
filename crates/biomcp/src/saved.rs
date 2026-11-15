//! Saved searches and lists on the command line (sdlc/planning/saved-work.md, step 1).
//!
//! Only the command line reaches these commands: they stay out of the shared command
//! tree, so MCP, hosted mode and their discovery are unchanged until the typed `saved`
//! tool lands in its own ticket.
use crate::command::{Encoding, Outcome};
use crate::exit::Status as Exit;
use biodata::Text;
use biomcp_query::saved::{NAME_CHARS, NOTE_CHARS, NewItem, SavedError, SavedStore};
use biomcp_types::{
    Envelope, ErrorEnvelope, ErrorMessage, Help, HelpArgument, HelpCommand, HelpQuery,
    ResponseBody, Saved, Source, Status, UsageArguments, UsageError,
};
use clap::{Arg, ArgAction, ArgMatches, Command};
use std::{ffi::OsString, io, path::Path};

const SAVE_AS: &str = "--save-as";
/// Process options that say how to run, not what to search; a saved search keeps none.
const RUN_OPTIONS: &[&str] = &[
    "--config",
    "--replay",
    "--record",
    "--case-cache",
    "--observe",
    "--offline",
    "--fresh",
    "--format",
    "--source_pacing",
];

fn node(name: &'static str) -> Command {
    Command::new(name)
        .disable_help_flag(true)
        .disable_help_subcommand(true)
}

pub(crate) fn grammar() -> Command {
    let name = |help: &'static str| Arg::new("name").value_name("NAME").help(help);
    let ids = |help: &'static str| {
        Arg::new("id")
            .value_name("ID")
            .help(help)
            .required(true)
            .num_args(1..)
    };
    node("saved")
        .about("Keep searches and lists of records in saved.sqlite in the data folder; a cache clean or upgrade keeps them.")
        .subcommand_required(true)
        .subcommand(
            node("show")
                .about("Show every saved search and list, or the one with this name.")
                .after_help("Example: biomcp saved show")
                .arg(name("Saved search or list name, without regard to case.")),
        )
        .subcommand(
            node("export")
                .about("Export every saved search and list as typed JSON.")
                .after_help("Example: biomcp saved export"),
        )
        .subcommand(
            node("delete")
                .about("Delete the saved search or list with this name, or everything with --all; reply with what was deleted.")
                .after_help("Example: biomcp saved delete braf-genes")
                .arg(name("Saved search or list name.").required_unless_present("all"))
                .arg(crate::grammar::boolean_argument(
                    Arg::new("all")
                        .long("all")
                        .help("Delete every saved search and list.")
                        .conflicts_with("name"),
                )),
        )
        .subcommand(
            node("list")
                .about("Add records to a named list or remove them.")
                .subcommand_required(true)
                .subcommand(
                    node("add")
                        .about("Add record IDs that get ENTITY accepts to a list, making the list when it is new.")
                        .after_help("Example: biomcp saved list add braf-genes --entity gene BRAF")
                        .arg(name("List name, up to 80 characters.").required(true))
                        .arg(
                            Arg::new("entity")
                                .long("entity")
                                .help("Entity word of the get command, such as gene or trial.")
                                .required(true),
                        )
                        .arg(Arg::new("source").long("source").help(
                            "Source the IDs belong to, when the get command's IDs are source-specific.",
                        ))
                        .arg(Arg::new("note").long("note").help(
                            "A note kept with each added item, up to 2,000 characters.",
                        ))
                        .arg(ids("Exact IDs the get command accepts.")),
                )
                .subcommand(
                    node("remove")
                        .about("Remove items with these IDs from a list.")
                        .after_help("Example: biomcp saved list remove braf-genes BRAF")
                        .arg(name("List name.").required(true))
                        .arg(ids("IDs to remove.")),
                ),
        )
}

/// Run a saved command, its help, or a search with `--save-as`; other commands return None.
pub(crate) fn cli(
    root: &crate::grammar::Root,
    settings: &crate::config::Settings,
    args: &[OsString],
) -> Option<io::Result<Outcome>> {
    let words: Vec<&str> = crate::grammar::command_words(root, args).collect();
    let saves = save_as_position(args).is_some();
    match words.as_slice() {
        ["saved", ..] => Some(run(root, args)),
        ["help", "saved", rest @ ..] => Some(help(root, settings, args, rest)),
        ["search", ..] if saves => Some(save_search(root, settings, args)),
        _ if saves => Some(usage(
            Encoding::cli(root, settings, args),
            UsageError::argument(SAVE_AS.into()),
        )),
        _ => None,
    }
}

fn save_as_position(args: &[OsString]) -> Option<usize> {
    args.iter().take_while(|arg| *arg != "--").position(|arg| {
        arg.to_str()
            .is_some_and(|arg| arg == SAVE_AS || arg.starts_with("--save-as="))
    })
}

/// Split `--save-as NAME` from the search words.
fn split_save_as(args: &[OsString]) -> Option<(String, Vec<OsString>)> {
    let position = save_as_position(args)?;
    let flag = args.get(position)?.to_str()?;
    let (name, width) = match flag.strip_prefix("--save-as=") {
        Some(name) => (name.to_owned(), 1),
        None => (args.get(position + 1)?.to_str()?.to_owned(), 2),
    };
    let mut rest = args.to_vec();
    rest.drain(position..position + width);
    if save_as_position(&rest).is_some() {
        return None;
    }
    Some((name, rest))
}

/// The search words a saved search keeps: everything but options that say how to run.
fn kept_words(root: &clap::Command, args: &[OsString]) -> Option<Vec<String>> {
    let mut kept = Vec::new();
    let mut words = args.iter();
    while let Some(word) = words.next() {
        let word = word.to_str()?;
        if word == "--" {
            kept.push(word.to_owned());
            for word in words.by_ref() {
                kept.push(word.to_str()?.to_owned());
            }
            break;
        }
        let flag = word.split('=').next().unwrap_or_default();
        if RUN_OPTIONS.contains(&flag) {
            let takes_value = root
                .get_arguments()
                .find(|arg| arg.get_long() == flag.strip_prefix("--"))
                .is_some_and(|arg| matches!(arg.get_action(), ArgAction::Set | ArgAction::Append));
            if takes_value && !word.contains('=') {
                words.next();
            }
            continue;
        }
        kept.push(word.to_owned());
    }
    Some(kept)
}

fn save_search(
    root: &crate::grammar::Root,
    settings: &crate::config::Settings,
    args: &[OsString],
) -> io::Result<Outcome> {
    let fallback = Encoding::cli(root, settings, args);
    let Some((name, rest)) = split_save_as(args) else {
        return usage(fallback, UsageError::argument(SAVE_AS.into()));
    };
    let encoding = Encoding::cli(root, settings, &rest);
    let Some(name) = checked_name(&name) else {
        return usage(encoding, UsageError::argument(SAVE_AS.into()));
    };
    let request = match crate::grammar::parse(root, &rest).map(|parsed| parsed.action) {
        Ok(Some(biomcp_query::Action::Run { request, .. })) => request,
        // The ordinary command reports its own usage error.
        _ => return crate::command::dispatch(root, settings, &rest),
    };
    let Some(words) = kept_words(root, &rest) else {
        return usage(encoding, UsageError::arguments());
    };
    let mut store = match open(encoding, true)? {
        Ok(Some(store)) => store,
        Ok(None) => return failure(encoding, SavedError::Database, "--save-as"),
        Err(outcome) => return Ok(outcome),
    };
    if let Err(error) = store.check_search_name(name.as_str(), &words) {
        return failure(encoding, error, "--save-as");
    }
    let outcome = crate::command::dispatch(root, settings, &rest)?;
    if !matches!(outcome.status, Exit::Success) {
        return Ok(outcome);
    }
    match store.save_search(
        &name,
        &words,
        request.schema(),
        &biomcp_query::saved::request_key(&request),
    ) {
        Ok(_) => Ok(outcome),
        Err(error) => failure(encoding, error, "--save-as"),
    }
}

fn checked_name(name: &str) -> Option<Text> {
    Text::new(name)
        .ok()
        .filter(|name| name.as_str().chars().count() <= NAME_CHARS)
}

/// Open the saved file; a read of a missing file opens nothing and creates nothing.
fn open(encoding: Encoding, create: bool) -> io::Result<Result<Option<SavedStore>, Outcome>> {
    let path = match crate::config::saved_path() {
        Ok(Some(path)) => path,
        Ok(None) => return failure(encoding, SavedError::Database, "").map(Err),
        Err(error) => {
            return encoding
                .encode(ErrorEnvelope::configuration(error.0), Exit::Local)
                .map(Err);
        }
    };
    if !create && !path.exists() {
        return Ok(Ok(None));
    }
    if let Some(parent) = path.parent().filter(|parent| !parent.exists())
        && crate::config::create_private_directory(parent).is_err()
    {
        return failure(encoding, SavedError::Database, "").map(Err);
    }
    Ok(match SavedStore::open(Path::new(&path)) {
        Ok(store) => Ok(Some(store)),
        Err(error) => Err(failure(encoding, error, "")?),
    })
}

fn run(root: &crate::grammar::Root, args: &[OsString]) -> io::Result<Outcome> {
    let Ok((globals, matches)) = crate::grammar::saved(args) else {
        return usage(Encoding::Json, help_usage(UsageError::arguments()));
    };
    let settings = crate::config::file::read_file(globals.config.as_ref());
    let encoding = match globals.format.or_else(|| crate::config::format(&settings)) {
        Some(crate::grammar::Format::Md) => Encoding::Markdown,
        _ => Encoding::Json,
    };
    let Some((verb, matches)) = matches.subcommand() else {
        return usage(encoding, help_usage(UsageError::arguments()));
    };
    let name = matches
        .try_get_one::<String>("name")
        .ok()
        .flatten()
        .map(String::as_str);
    let result = match verb {
        "show" | "export" => match open(encoding, false)? {
            Err(outcome) => return Ok(outcome),
            Ok(None) if name.is_some() => Err(SavedError::Unknown),
            Ok(None) => Ok(Saved::default()),
            Ok(Some(store)) => store.show(name),
        },
        "delete" => match open(encoding, true)? {
            Err(outcome) => return Ok(outcome),
            Ok(None) => Err(SavedError::Database),
            Ok(Some(mut store)) => store.delete(name),
        },
        "list" => return list(root, encoding, matches),
        _ => return usage(encoding, help_usage(UsageError::arguments())),
    };
    reply(encoding, result, "NAME")
}

fn list(
    root: &crate::grammar::Root,
    encoding: Encoding,
    matches: &ArgMatches,
) -> io::Result<Outcome> {
    let Some((verb, matches)) = matches.subcommand() else {
        return usage(encoding, help_usage(UsageError::arguments()));
    };
    let name = matches
        .get_one::<String>("name")
        .map(String::as_str)
        .and_then(checked_name);
    let Some(name) = name else {
        return usage(encoding, UsageError::argument("NAME".into()));
    };
    let ids: Vec<&String> = matches
        .get_many::<String>("id")
        .into_iter()
        .flatten()
        .collect();
    if verb == "remove" {
        let Some(ids) = ids
            .iter()
            .map(|id| Text::new(id).ok())
            .collect::<Option<Vec<_>>>()
        else {
            return usage(encoding, UsageError::argument("ID".into()));
        };
        let result = match open(encoding, false)? {
            Err(outcome) => return Ok(outcome),
            Ok(None) => Err(SavedError::Unknown),
            Ok(Some(mut store)) => store.list_remove(name.as_str(), &ids),
        };
        return reply(encoding, result.map(one_list), "NAME");
    }
    let items = match new_items(root, matches, &ids) {
        Ok(items) => items,
        Err(error) => return usage(encoding, error),
    };
    let result = match open(encoding, true)? {
        Err(outcome) => return Ok(outcome),
        Ok(None) => Err(SavedError::Database),
        Ok(Some(mut store)) => store.list_add(&name, &items),
    };
    reply(encoding, result.map(one_list), "NAME")
}

fn one_list(list: biomcp_types::SavedList) -> Saved {
    Saved {
        searches: Vec::new(),
        lists: vec![list],
    }
}

/// Check each ID the way `get ENTITY ID` would read it, before anything is written.
fn new_items(
    root: &crate::grammar::Root,
    matches: &ArgMatches,
    ids: &[&String],
) -> Result<Vec<NewItem>, UsageError> {
    let entities: Vec<&str> = biomcp_types::COMMANDS
        .iter()
        .filter(|command| command.verb == "get")
        .map(|command| command.entity)
        .collect();
    let entity = matches
        .get_one::<String>("entity")
        .map(String::as_str)
        .filter(|entity| entities.contains(entity))
        .ok_or_else(|| {
            UsageError::argument_values(
                "--entity".into(),
                entities.iter().filter_map(|e| Text::new(e).ok()).collect(),
            )
        })?;
    let source = matches
        .get_one::<String>("source")
        .map(|source| source.parse::<Source>())
        .transpose()
        .map_err(|_| UsageError::argument("--source".into()))?;
    let note = matches
        .get_one::<String>("note")
        .map(|note| {
            Text::new(note)
                .ok()
                .filter(|note| note.as_str().chars().count() <= NOTE_CHARS)
                .ok_or_else(|| UsageError::argument("--note".into()))
        })
        .transpose()?;
    let mut items = Vec::new();
    for id in ids {
        let mut words: Vec<OsString> = vec!["get".into(), entity.into(), id.as_str().into()];
        if let Some(source) = &source {
            words.extend(["--source".into(), source.to_string().into()]);
        }
        let readable = matches!(
            crate::grammar::parse(root, &words).map(|parsed| parsed.action),
            Ok(Some(biomcp_query::Action::Run { .. }))
        );
        let id = Text::new(id).ok().filter(|_| readable);
        let (Some(id), Ok(entity)) = (id, Text::new(entity)) else {
            return Err(UsageError::argument("ID".into()));
        };
        items.push(NewItem {
            entity,
            id,
            source: source.clone(),
            note: note.clone(),
        });
    }
    Ok(items)
}

fn reply(
    encoding: Encoding,
    result: Result<Saved, SavedError>,
    argument: &str,
) -> io::Result<Outcome> {
    match result {
        Ok(saved) => encoding.encode(
            Envelope::new(Status::Complete, biomcp_types::EmptyQuery {}, Some(saved)),
            Exit::Success,
        ),
        Err(error) => failure(encoding, error, argument),
    }
}

fn failure(encoding: Encoding, error: SavedError, argument: &str) -> io::Result<Outcome> {
    let message = match error {
        SavedError::Newer => {
            return encoding.encode(
                ErrorEnvelope::local_message(ErrorMessage::SavedNewer),
                Exit::Local,
            );
        }
        SavedError::Database => return encoding.encode(ErrorEnvelope::local(), Exit::Local),
        SavedError::Taken => ErrorMessage::SavedNameTaken,
        SavedError::Unknown => ErrorMessage::SavedUnknownName,
    };
    usage(
        encoding,
        UsageError::Arguments(UsageArguments {
            argument: Some(argument.into()),
            message: Some(message),
            valid_values: None,
            next: Vec::new(),
        }),
    )
}

fn usage(encoding: Encoding, error: UsageError) -> io::Result<Outcome> {
    crate::command::usage_envelope(error, encoding)
}

fn help_usage(error: UsageError) -> UsageError {
    match error {
        UsageError::Arguments(mut arguments) => {
            arguments.next = vec![biomcp_types::Step {
                kind: biomcp_types::StepKind::Configure,
                args: vec!["help".into(), "saved".into()].into(),
                options: Default::default(),
            }];
            UsageError::Arguments(arguments)
        }
        other => other,
    }
}

/// Command-line help for saved work and the search option that saves.
fn help(
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
