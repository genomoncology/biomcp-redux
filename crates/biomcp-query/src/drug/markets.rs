//! Market-specific drug records and the fields they supply.
#[path = "market_values.rs"]
mod market_values;
pub(super) use market_values::{dated, kept, row, text};
mod read;
pub(crate) use read::get;
pub(super) use read::{add_source, alternatives, section_step};
