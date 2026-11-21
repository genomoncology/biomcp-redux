//! Native FDA application discovery; product and submission scope remain separate.
mod applications;
mod approvals;
pub(super) use applications::search;
mod history;
mod history_read;
mod history_reply;
mod history_search;
mod history_settle;
mod label_contents;
mod label_plain;
mod label_sections;
#[cfg(test)]
mod label_sections_tests;
mod label_text;
mod letter;
mod lookup;
pub(super) use lookup::get;
mod purpose;
use super::empty;
