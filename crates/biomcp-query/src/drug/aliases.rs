//! Recover empty selected-source lookups using source-stated aliases.
#[path = "alias_lookup.rs"]
mod alias_lookup;
pub(super) use alias_lookup::names;
mod label_fallback;
#[path = "name_evidence.rs"]
pub(super) mod name_evidence;
mod named_get;
pub(super) use named_get::{Aliased, get, get_parts, listing_name, partner};
mod search;
pub(crate) use search::search;
