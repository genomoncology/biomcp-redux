//! Dated FDA Purple Book snapshots from bundled public data or manual CSV imports.
mod cber;
pub use cber::{CberProduct, cber_product};
mod dates;
pub(crate) mod decl;
mod decode;
mod products;
pub use products::summarize;
mod read;
pub use read::SPEC;
pub use read::{SnapshotSearch, search, sync_file, unavailable_notice};
