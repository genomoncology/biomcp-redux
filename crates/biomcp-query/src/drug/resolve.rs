//! Resolve drug names to exact source records.
#[path = "name_pages.rs"]
mod name_pages;
mod named;
pub(super) use named::{Rank, carry, get, get_parts, lookup_deadline, within};
