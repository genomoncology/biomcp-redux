//! Dated FDA Purple Book snapshots from bundled public data or manual CSV imports.
mod dates;
pub(crate) mod decl;
mod decode;
mod products;
pub use products::summarize;
mod read;
pub use read::SPEC;
pub use read::{notice, search, sync_file};
