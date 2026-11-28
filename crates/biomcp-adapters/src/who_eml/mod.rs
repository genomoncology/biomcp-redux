//! WHO Model List of Essential Medicines listings, read live from the WHO eEML's JSON-LD pages.
pub(crate) mod decl;
mod matching;
mod raw;
mod read;
pub use read::{SPEC, essential_medicines, medicines};
