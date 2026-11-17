//! STRING functional association partners of one human protein, in STRING's score order.
/// Complete source policy and command declaration.
pub(crate) mod decl;
mod raw;
mod read;
pub use read::{REQUIRED_SCORE, SPEC, VERSION, partners};
