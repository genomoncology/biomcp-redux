//! Chosen fields: one generic filter keeps named columns, or dotted paths inside them, of
//! each row of a search, a subcommand's list or a named section.
//!
//! Rows come from the reply schema (`rows`). Each row always keeps its required and
//! identifier (`x-identifier`) keys, at every level a path narrows, and the envelope and the
//! card stay whole, so a narrowed reply validates against the same schema.
mod narrow;
mod rows;
mod select;

pub(crate) use select::{Chosen, carried, check, help_line, project};
