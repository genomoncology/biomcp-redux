//! NIH RePORTER project search: one row per project and fiscal year.
//!
//! Only named fields are asked for, so grantee-written abstracts are never read or kept.
mod search;
pub use search::{SPEC, search};

/// Complete source policy and command declaration.
pub(crate) mod decl;
