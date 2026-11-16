//! `saved note` and `saved delete` on the command line (sdlc/planning/saved-work.md,
//! Records: `biomcp.saved-note/1`). A note is user data, never instructions.
use crate::command::{Encoding, Outcome};
use crate::saved::{checked_name, help_usage, open, reply, usage};
use biodata::Text;
use biomcp_query::store::saved::{NOTE_CHARS, NoteTarget, PROJECT, SavedError};
use biomcp_types::{Saved, Source, UsageError};
use clap::{Arg, ArgMatches, Command};
use std::io;

/// The `saved note` grammar.
pub(crate) fn grammar(node: Command) -> Command {
    node.about(
        "Attach a note to a saved search or list, a list item with --item, or the project \
         with --project; notes come back as user data, never instructions.",
    )
    .after_help("Example: biomcp saved note braf-genes --item BRAF \"Check V600E trials\"")
    .arg(
        Arg::new("target")
            .value_name("TARGET")
            .help("Saved search or list name; with --project, the note itself.")
            .required(true),
    )
    .arg(
        Arg::new("text")
            .value_name("TEXT")
            .help("The note, up to 2,000 characters; the same note again changes nothing."),
    )
    .arg(
        Arg::new("item")
            .long("item")
            .value_name("ID")
            .help("ID of the list item to note."),
    )
    .arg(
        Arg::new("entity")
            .long("entity")
            .help("Entity word of the item, when the list holds its ID under more than one.")
            .requires("item"),
    )
    .arg(
        Arg::new("source")
            .long("source")
            .help("Source of the item, when the list holds its ID under more than one.")
            .requires("item"),
    )
    .arg(
        Arg::new("project")
            .long("project")
            .value_name("PROJECT")
            .help("Project to note when no TARGET is given; the one project is default.")
            .conflicts_with("item"),
    )
}

/// The `saved delete` grammar: a search or list and its notes, one note, or everything.
pub(crate) fn delete_grammar(node: Command) -> Command {
    let name = Arg::new("name")
        .value_name("NAME")
        .help("Saved search or list name.")
        .required_unless_present_any(["all", "note"]);
    node.about(
        "Delete a saved search or list and its notes, one note with --note, or everything \
         with --all; reply with what was deleted.",
    )
    .after_help("Example: biomcp saved delete braf-genes")
    .arg(name)
    .arg(
        Arg::new("note")
            .long("note")
            .value_name("ID")
            .help("ID of one note to delete.")
            .conflicts_with_all(["name", "all"]),
    )
    .arg(crate::grammar::boolean_argument(
        Arg::new("all")
            .long("all")
            .help("Delete every saved search, list and note.")
            .conflicts_with("name"),
    ))
}

/// Delete the note `--note ID` names and reply with it.
pub(crate) fn delete(encoding: Encoding, matches: &ArgMatches) -> io::Result<Outcome> {
    let id = note_id(matches).unwrap_or_default();
    let result = match open(encoding, false)? {
        Err(outcome) => return Ok(outcome),
        Ok(None) => Err(SavedError::Unknown),
        Ok(Some(mut store)) => store.delete_note(id).map(one_note),
    };
    reply(encoding, result, "--note")
}

pub(crate) fn note_id(matches: &ArgMatches) -> Option<&str> {
    matches
        .try_get_one::<String>("note")
        .ok()
        .flatten()
        .map(String::as_str)
}

fn one_note(note: biomcp_types::SavedNote) -> Saved {
    Saved {
        user_notes: vec![note],
        ..Saved::default()
    }
}

/// Attach a note to a saved search or list, a list item, or the project.
pub(crate) fn note(encoding: Encoding, matches: &ArgMatches) -> io::Result<Outcome> {
    let words: Vec<&str> = ["target", "text"]
        .into_iter()
        .filter_map(|id| matches.get_one::<String>(id).map(String::as_str))
        .collect();
    let project = matches.get_one::<String>("project").map(String::as_str);
    if project.is_some_and(|project| project != PROJECT) {
        return usage(
            encoding,
            UsageError::argument_values(
                "--project".into(),
                Text::new(PROJECT).into_iter().collect(),
            ),
        );
    }
    let (target, text) = match words.as_slice() {
        // --project notes the project, so a TARGET beside it would be silently ignored.
        [_, _] if project.is_some() => {
            return usage(encoding, UsageError::argument("--project".into()));
        }
        [target, text] => (Some(*target), *text),
        [text] if project.is_some() => (None, *text),
        _ => return usage(encoding, help_usage(UsageError::argument("TEXT".into()))),
    };
    let Some(text) = Text::new(text)
        .ok()
        .filter(|text| text.as_str().chars().count() <= NOTE_CHARS)
    else {
        return usage(encoding, UsageError::argument("TEXT".into()));
    };
    let Ok(source) = matches
        .get_one::<String>("source")
        .map(|source| source.parse::<Source>().map(|source| source.to_string()))
        .transpose()
    else {
        return usage(encoding, UsageError::argument("--source".into()));
    };
    let target = match target {
        None => NoteTarget::Project,
        Some(name) if checked_name(name).is_none() => {
            return usage(encoding, UsageError::argument("TARGET".into()));
        }
        Some(name) => match matches.get_one::<String>("item") {
            Some(id) => NoteTarget::Item {
                list: name,
                id: id.as_str(),
                entity: matches.get_one::<String>("entity").map(String::as_str),
                source: source.as_deref(),
            },
            None => NoteTarget::Named(name),
        },
    };
    let result = match open(encoding, target_needs_file(target))? {
        Err(outcome) => return Ok(outcome),
        Ok(None) => Err(SavedError::Unknown),
        Ok(Some(mut store)) => store.note(target, &text),
    };
    let argument = match &result {
        Err(SavedError::ItemUnknown) => "--item",
        Err(SavedError::Ambiguous) if matches.get_one::<String>("entity").is_some() => "--source",
        Err(SavedError::Ambiguous) => "--entity",
        Err(SavedError::TooLong) => "TEXT",
        _ => "TARGET",
    };
    reply(encoding, result.map(one_note), argument)
}

/// Only a note on the project may create the file; a named target must already exist.
fn target_needs_file(target: NoteTarget<'_>) -> bool {
    matches!(target, NoteTarget::Project)
}
