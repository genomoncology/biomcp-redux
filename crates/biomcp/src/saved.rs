//! Saved searches, lists and notes on the command line (sdlc/planning/saved-work.md, steps 1,
//! 3 and 4).
//!
//! Only the command line reaches these commands: they stay out of the shared command
//! tree, so MCP, hosted mode and their discovery are unchanged until the typed `saved`
//! tool lands in its own ticket.
use crate::command::{Encoding, Outcome};
use crate::exit::Status as Exit;
use biodata::Text;
use biomcp_query::store::saved::{
    self as saved_store, NAME_CHARS, NOTE_CHARS, NewItem, SavedError, SavedStore,
};
use biomcp_types::{
    Envelope, ErrorEnvelope, ErrorMessage, Saved, Source, Status, UsageArguments, UsageError,
};
use clap::{Arg, ArgAction, ArgMatches, Command};
use std::{ffi::OsString, io, path::Path};

pub(crate) const SAVE_AS: &str = "--save-as";
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

pub(crate) fn node(name: &'static str) -> Command {
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
        .about("Keep searches, lists of records and notes in saved.sqlite in the data folder; a cache clean or upgrade keeps them.")
        .subcommand_required(true)
        .subcommand(
            node("show")
                .about("Show every saved search, list and note, or the search or list with this name and its notes.")
                .after_help("Example: biomcp saved show")
                .arg(name("Saved search or list name, without regard to case.")),
        )
        .subcommand(
            node("changed")
                .about("Re-read each saved search from its source, at most once an hour, and report the IDs it gained, lost or changed since its last check.")
                .after_help("Example: biomcp saved changed melanoma-trials")
                .arg(name("Saved search name; without it, every saved search.")),
        )
        .subcommand(
            node("export")
                .about("Export every saved search, list, note and kept search snapshot as typed JSON.")
                .after_help("Example: biomcp saved export"),
        )
        .subcommand(crate::saved_note::grammar(node("note")))
        .subcommand(crate::saved_note::delete_grammar(node("delete")))
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
                        .arg(Arg::new("note").long("note").allow_hyphen_values(true).help(
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
        ["saved", rest @ ..] if crate::saved_help::asks_help(args) => Some(crate::saved_help::help(
            root,
            settings,
            args,
            &command_path(rest),
        )),
        ["saved", ..] => Some(run(root, settings, args)),
        ["help", "saved", rest @ ..] => Some(crate::saved_help::help(
            root,
            settings,
            args,
            &command_path(rest),
        )),
        _ if saves => Some(save_search(root, settings, args)),
        _ => None,
    }
}

/// The subcommand words before the first option.
fn command_path<'a>(words: &[&'a str]) -> Vec<&'a str> {
    words
        .iter()
        .copied()
        .take_while(|word| !word.starts_with('-'))
        .collect()
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
        // A command that is not a search refuses the option; others report their own error.
        Ok(_) => return usage(encoding, UsageError::argument(SAVE_AS.into())),
        Err(_) => return crate::command::dispatch(root, settings, &rest),
    };
    // A saved search reruns from its first page: a pool, cursor or offset expires.
    if request.path().0 != "search" || !crate::grammar::first_page(&request) {
        return usage(encoding, UsageError::argument(SAVE_AS.into()));
    }
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
        &biomcp_query::store::saved::request_key(&request),
    ) {
        Ok(_) => Ok(outcome),
        Err(error) => failure(encoding, error, "--save-as"),
    }
}

pub(crate) fn checked_name(name: &str) -> Option<Text> {
    Text::new(name)
        .ok()
        .filter(|name| name.as_str().chars().count() <= NAME_CHARS)
}

/// Open the saved file; a read of a missing file opens nothing and creates nothing.
pub(crate) fn open(
    encoding: Encoding,
    create: bool,
) -> io::Result<Result<Option<SavedStore>, Outcome>> {
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

fn run(
    root: &crate::grammar::Root,
    settings: &crate::config::Settings,
    args: &[OsString],
) -> io::Result<Outcome> {
    let (globals, matches) = match crate::grammar::saved(args) {
        Ok(parsed) => parsed,
        Err(invalid) => {
            let error = match invalid.argument {
                Some(argument) => UsageError::argument(argument.into()),
                None => UsageError::arguments(),
            };
            return usage(Encoding::Json, help_usage(error));
        }
    };
    let encoding = match globals.format.or_else(|| crate::config::format(settings)) {
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
        "show" => match open(encoding, false)? {
            Err(outcome) => return Ok(outcome),
            Ok(None) if name.is_some() => Err(SavedError::Unknown),
            Ok(None) => Ok(Saved::default()),
            Ok(Some(store)) => store.show(name),
        },
        "export" => match open(encoding, false)? {
            Err(outcome) => return Ok(outcome),
            Ok(None) => Ok(Saved::default()),
            Ok(Some(store)) => store.show(None).and_then(|saved| {
                Ok(Saved {
                    snapshots: store.all_snapshots()?,
                    ..saved
                })
            }),
        },
        "changed" => return changed(root, settings, &globals, encoding, name),
        "delete" if crate::saved_note::note_id(matches).is_some() => {
            return crate::saved_note::delete(encoding, matches);
        }
        "note" => return crate::saved_note::note(encoding, matches),
        "delete" => match open(encoding, false)? {
            Err(outcome) => return Ok(outcome),
            Ok(None) if name.is_some() => Err(SavedError::Unknown),
            Ok(None) => Ok(Saved::default()),
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
        return reply(
            encoding,
            result.map(|(list, missing)| Saved {
                missing,
                ..one_list(list)
            }),
            "NAME",
        );
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
        lists: vec![list],
        ..Saved::default()
    }
}

/// Check saved searches for changes. A search checked within the hour answers `not_due`
/// and runs nothing; a due one re-reads its source through the reviewed fresh path, keeps
/// the new snapshot and reports against the one before it.
fn changed(
    root: &crate::grammar::Root,
    settings: &crate::config::Settings,
    globals: &crate::grammar::Globals,
    encoding: Encoding,
    name: Option<&str>,
) -> io::Result<Outcome> {
    let mut store = match open(encoding, false)? {
        Err(outcome) => return Ok(outcome),
        Ok(None) if name.is_some() => return failure(encoding, SavedError::Unknown, "NAME"),
        Ok(None) => return reply(encoding, Ok(Saved::default()), "NAME"),
        Ok(Some(store)) => store,
    };
    let searches = match store.searches(name) {
        Ok(searches) => searches,
        Err(error) => return failure(encoding, error, "NAME"),
    };
    let now = saved_store::now();
    let mut due = Vec::new();
    let mut changes = Vec::new();
    for search in &searches {
        let kept = match store.snapshots(search.id.as_str()) {
            Ok(kept) => kept,
            Err(error) => return failure(encoding, error, "NAME"),
        };
        match saved_store::not_due(search, kept.first(), now) {
            Some(report) => changes.push(report),
            None => due.push((search, kept.into_iter().next())),
        }
    }
    // A re-check reads the source; offline would read only the cache. Refuse before any runs.
    if globals.offline && !due.is_empty() {
        return usage(encoding, UsageError::argument("--offline".into()));
    }
    for (search, newest) in due {
        let snapshot =
            match crate::command::recheck(root, settings, globals, &search.args, encoding)? {
                Ok(snapshot) => snapshot,
                // One named search answers its own reply; among several, its failure is
                // its report, so the others' changes are still shown.
                Err(outcome) if name.is_some() => return Ok(outcome),
                Err(_) => {
                    changes.push(saved_store::failed(search, newest.as_ref(), now));
                    continue;
                }
            };
        match store.record_check(search, &snapshot) {
            Ok(report) => changes.push(report),
            Err(error) => return failure(encoding, error, "NAME"),
        }
    }
    changes.sort_by_key(|report| report.search.as_str().to_lowercase());
    reply(
        encoding,
        Ok(Saved {
            changes,
            ..Saved::default()
        }),
        "NAME",
    )
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
            source,
            note: note.clone(),
        });
    }
    Ok(items)
}

pub(crate) fn reply(
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

pub(crate) fn failure(
    encoding: Encoding,
    error: SavedError,
    argument: &str,
) -> io::Result<Outcome> {
    let message = match error {
        SavedError::Newer => {
            return encoding.encode(
                ErrorEnvelope::local_message(ErrorMessage::SavedNewer),
                Exit::Local,
            );
        }
        SavedError::Database => return encoding.encode(ErrorEnvelope::local(), Exit::Local),
        SavedError::Taken => ErrorMessage::SavedNameTaken,
        SavedError::Unknown if argument == "--note" => ErrorMessage::SavedNoteUnknown,
        SavedError::Unknown => ErrorMessage::SavedRecordUnknown,
        SavedError::TooLong | SavedError::ItemUnknown | SavedError::Ambiguous => {
            return usage(encoding, UsageError::argument(argument.into()));
        }
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

pub(crate) fn usage(encoding: Encoding, error: UsageError) -> io::Result<Outcome> {
    crate::command::usage_envelope(error, encoding)
}

pub(crate) fn help_usage(error: UsageError) -> UsageError {
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
