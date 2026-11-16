//! Saved work for the typed MCP `saved` tool (sdlc/planning/saved-work.md, Commands and
//! Hosted points 2-3). The tool runs the same saved commands as the command line. Locally
//! they use the data folder's file. In hosted mode the server is started with a saved-work
//! folder, and the file is `<folder>/<account>/saved.sqlite`, where the account is the header
//! the hosting service sets from its verified user. No call names a path, and hosted mode
//! without the header refuses: the core learns nothing of the user beyond that header.
use crate::command::{Context, Encoding, Outcome};
use crate::saved::{At, usage};
use biomcp_types::{ErrorMessage, UsageError};
use serde_json::Value;
use std::{ffi::OsString, io, path::PathBuf};

/// The hosted header naming the account whose saved work a call reads and writes.
pub(crate) const ACCOUNT_HEADER: &str = "x-biomcp-account";
/// Longest account the header may carry.
const ACCOUNT_BYTES: usize = 128;
/// Command-line argument names in saved errors, under the `saved` tool's field names.
const FIELDS: [(&str, &str); 10] = [
    ("NAME", "name"),
    ("TARGET", "name"),
    ("--save-as", "name"),
    ("TEXT", "text"),
    ("ID", "ids"),
    ("--entity", "entity"),
    ("--source", "source"),
    ("--note", "note"),
    ("--item", "item"),
    ("--all", "all"),
];

/// An account the header may carry: an opaque key of letters, digits, `-` and `_`, so it
/// names one folder and can never climb out of the saved-work folder. Accounts compare exactly,
/// so the folder must be on a case-sensitive filesystem (docs/configuration.md).
pub(crate) fn account(value: &str) -> Option<&str> {
    ((1..=ACCOUNT_BYTES).contains(&value.len())
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_')))
    .then_some(value)
}

/// Run a saved command's words, or a search with `--save-as NAME`, for one MCP call.
pub(crate) fn tool(args: &[OsString], context: Context<'_>) -> io::Result<Outcome> {
    let encoding = Encoding::Mcp;
    let file = match file(context) {
        Ok(file) => file,
        Err(error) => return usage(encoding, error),
    };
    let at = At {
        encoding,
        file: file.as_deref(),
        context: Some(context),
    };
    let mut outcome = if args.first().is_some_and(|word| word == "saved") {
        match crate::grammar::saved(args) {
            Ok((_, matches)) => crate::saved::verbs(
                context.root,
                context.settings,
                context.globals,
                at,
                &matches,
            )?,
            Err(invalid) => usage(
                encoding,
                invalid
                    .argument
                    .map_or_else(UsageError::arguments, |argument| {
                        UsageError::argument(argument.into())
                    }),
            )?,
        }
    } else {
        crate::saved::save_search(context.root, context.settings, args, Some(at))?
    };
    rename(outcome.value_mut()?);
    Ok(outcome)
}

/// The saved file for this call: the data folder's locally, or the account's in hosted mode.
fn file(context: Context<'_>) -> Result<Option<PathBuf>, UsageError> {
    if !context.globals.hosted {
        return Ok(None);
    }
    let folder = context
        .globals
        .saved_folder
        .as_ref()
        .ok_or(UsageError::McpUnavailable(ErrorMessage::McpHosted))?;
    let account = context
        .account
        .and_then(account)
        .ok_or(UsageError::McpUnavailable(ErrorMessage::SavedAccount))?;
    Ok(Some(folder.join(account).join("saved.sqlite")))
}

/// Name a refused argument as the tool's field, not the command line's.
fn rename(value: &mut Value) {
    if let Some(option) = value.pointer_mut("/error/option")
        && let Some((_, field)) = FIELDS
            .iter()
            .find(|(argument, _)| Some(*argument) == option.as_str())
    {
        *option = Value::from(*field);
    }
}
