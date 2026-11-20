//! Recover empty selected-source lookups using source-stated aliases.
#[path = "alias_lookup.rs"]
mod alias_lookup;
pub(super) use alias_lookup::names;
#[path = "name_evidence.rs"]
pub(super) mod name_evidence;
mod named_get;
pub(super) use named_get::{Aliased, get, get_parts, partner};
mod search;
pub(crate) use search::search;
