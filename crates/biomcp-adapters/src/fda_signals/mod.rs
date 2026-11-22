//! FDA quarterly safety-signal tables, with a shipped copy for first and offline reads.
pub(crate) mod decl;
mod read;
pub use decl::SPEC;
pub use read::{Signals, read};
