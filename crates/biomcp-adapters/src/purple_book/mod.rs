//! Dated FDA Purple Book snapshots from bundled public data or manual CSV imports.
pub(crate) mod decl;
mod decode;
mod read;
pub use read::SPEC;
pub use read::{notice, search, sync_file};
