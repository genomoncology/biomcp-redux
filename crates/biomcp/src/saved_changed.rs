//! `saved changed` (sdlc/planning/saved-work.md, Change detection): re-read due saved searches
//! and report what each gained, lost or changed.
use crate::command::Outcome;
use crate::saved::{At, failure, open, reply, usage};
use biomcp_query::store::saved::{self as saved_store, SavedError};
use biomcp_types::{Saved, UsageError};
use std::io;

/// Check saved searches for changes. A search checked within the hour answers `not_due`
/// and runs nothing; a due one re-reads its source through the reviewed fresh path, keeps
/// the new snapshot and reports against the one before it.
pub(crate) fn changed(
    root: &crate::grammar::Root,
    settings: &crate::config::Settings,
    globals: &crate::grammar::Globals,
    at: At<'_>,
    name: Option<&str>,
) -> io::Result<Outcome> {
    let encoding = at.encoding;
    let mut store = match open(at, false)? {
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
    let mut skipped = Vec::new();
    for search in &searches {
        let kept = match store.snapshots(search.id.as_str(), &mut skipped) {
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
        let snapshot = match crate::command::recheck(
            root,
            settings,
            globals,
            &search.args,
            encoding,
            at.context,
        )? {
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
            skipped,
            ..Saved::default()
        }),
        "NAME",
    )
}
