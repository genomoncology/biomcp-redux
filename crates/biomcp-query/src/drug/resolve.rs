//! Resolve drug names to exact source records.
#[path = "name_pages.rs"]
mod name_pages;
mod named;
pub(super) use named::{
    Rank, applications, carry, get, get_parts, get_with_name_state, lookup_deadline, within,
};
