//! The saved-work file: saved searches and lists, kept apart from the cache.
//!
//! This file has no drop-and-rebuild path. Its schema only grows through
//! [`MIGRATIONS`], and a file written by a newer build is refused untouched.
mod migrations;
mod store;
pub use migrations::MIGRATIONS;
pub use store::{NAME_CHARS, NOTE_CHARS, NewItem, PROJECT, SavedError, SavedStore, request_key};

#[cfg(test)]
mod tests;
