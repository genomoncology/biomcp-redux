//! Orphadata Science epidemiology and natural-history files as one local bundle.
mod bundle;
/// Complete source policy and command declaration.
pub(crate) mod decl;
mod raw;
mod read;
pub(crate) use bundle::{HEALTH_PATH, bundle_state};
pub use bundle::{SPEC, sync};
pub use read::{epidemiology, orpha_code};
