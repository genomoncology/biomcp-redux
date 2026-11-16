//! The saved-work file: saved searches and lists, kept apart from the cache.
//!
//! This file has no drop-and-rebuild path. Its schema only grows through
//! [`MIGRATIONS`], and a file written by a newer build is refused untouched.
mod migrations;
mod snapshot;
mod store;
pub use migrations::MIGRATIONS;
pub use snapshot::{
    FLOOR_SECONDS, NewSnapshot, SNAPSHOT_ROWS, SnapshotRow, compare, failed, not_due, now,
};
pub use store::{
    COMPATIBILITY, NAME_CHARS, NOTE_CHARS, NewItem, PROJECT, SavedError, SavedStore, request_key,
};

#[cfg(test)]
mod tests;
