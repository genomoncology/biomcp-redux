//! Chosen fields: one generic filter keeps named columns, or dotted paths inside them, of
//! each row of a search, a subcommand's list or a named section.
//!
//! Rows come from the reply schema (`rows`). Each row always keeps its required and
//! identifier (`x-identifier`) and `x-kept` keys, at every level a path narrows, and the envelope and the
//! card stay whole, so a narrowed reply validates against the same schema.
mod arm;
mod narrow;
mod rows;
mod section_rows;
mod sections;
mod select;

pub(crate) use arm::arm_projection;
pub(crate) use select::{Chosen, carried, carry_to_correction, check, help_line, project};
